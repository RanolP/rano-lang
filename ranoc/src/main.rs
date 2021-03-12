use std::{fs, path::PathBuf};

use anyhow::{anyhow, bail};
use libranoc::{codegen, core::Error, syntax};
use wasmer::{imports, Function, Instance, Module, Store};

mod external {
    pub fn show(i: i32) -> i32 {
        println!("{}", i);
        0
    }
    pub fn add(lhs: i32, rhs: i32) -> i32 {
        dbg!(&lhs, &rhs);
        lhs + rhs
    }
}

mod ops {
    pub fn add_i32_i32(lhs: i32, rhs: i32) -> i32 {
        lhs + rhs
    }
}

fn report_error(src: &String, error: Error) -> anyhow::Result<()> {
    use codespan_reporting::{
        diagnostic::{Diagnostic, Label},
        files::SimpleFiles,
        term::{
            self,
            termcolor::{ColorChoice, StandardStream},
        },
    };
    let mut files = SimpleFiles::new();
    let file_id = files.add("main.rano", src);
    let diagnostic = Diagnostic::error()
        .with_message(error.message)
        .with_code(format!("E{:04}", error.code as u16))
        .with_labels(
            error
                .labels
                .iter()
                .map(|label| {
                    let mut diagnostic_label = Label::primary(
                        file_id,
                        match &label.location {
                            libranoc::core::Location::Eof => src.len()..(src.len() + 1),
                            libranoc::core::Location::Known(span) => span.range.clone(),
                        },
                    );
                    if let Some(message) = &label.message {
                        diagnostic_label = diagnostic_label.with_message(message.clone());
                    }
                    diagnostic_label
                })
                .collect(),
        );
    let mut writer = StandardStream::stderr(ColorChoice::Always);
    let config = term::Config::default();
    term::emit(&mut writer, &config, &files, &diagnostic)?;

    Ok(())
}

fn main() -> anyhow::Result<()> {
    println!("Reading main.rano");
    let src = fs::read_to_string(PathBuf::from("main.rano"))?;

    println!("Parsing main.rano");
    let tokens = syntax::tokenize(&src);
    let ast = match syntax::parse(tokens) {
        Ok(ast) => ast,
        Err(error) => {
            report_error(&src, error)?;
            bail!("Failed to parse sources");
        }
    };

    println!("Compiling main.rano");
    let (wasm_bytes, errors) = codegen::compile_wasm(ast);

    if errors.len() > 0 {
        for error in errors {
            report_error(&src, error)?;
        }
        bail!("Failed to compile sources");
    }

    println!("Running main.rano");
    let store = Store::default();
    let module = Module::new(&store, &wasm_bytes)?;

    let import_object = imports! {
        "extern" => {
            "show" => Function::new_native(&store, external::show),
            "add" => Function::new_native(&store, external::add),
            "Add__i32_i32" => Function::new_native(&store, ops::add_i32_i32),
        },
    };
    let instance = Instance::new(&module, &import_object)?;

    let main = instance
        .exports
        .get_function("main")
        .map_err(|_| anyhow!("Failed to find main function"))?;
    let result = main.call(&[])?;

    dbg!(&result);

    Ok(())
}
