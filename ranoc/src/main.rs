use std::{fs, path::PathBuf};

use anyhow::bail;
use libranoc::{codegen, syntax};
use wasmer::{imports, Function, Instance, Module, Store};

mod console {
    pub fn show(i: i32) {
        println!("{}", i);
    }
}

fn main() -> anyhow::Result<()> {
    let src = fs::read_to_string(PathBuf::from("main.rano"))?;

    let tokens = syntax::tokenize(&src);
    let ast = match syntax::parse(&tokens) {
        Ok(ast) => ast,
        Err(err) => {
            use codespan_reporting::{
                diagnostic::{Diagnostic, Label},
                files::SimpleFiles,
                term::{
                    self,
                    termcolor::{ColorChoice, StandardStream},
                },
            };
            let mut files = SimpleFiles::new();
            let file_id = files.add("main.rano", &src);
            let diagnostic = Diagnostic::error()
                .with_message(err.message)
                .with_code(format!("E{:04}", err.code as u16))
                .with_labels(
                    err.labels
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
            bail!("Failed to parse sources");
        }
    };

    let wasm_bytes = codegen::compile_wasm(ast);

    let store = Store::default();
    let module = Module::new(&store, &wasm_bytes)?;

    let import_object = imports! {
        "console" => {
            "show" => Function::new_native(&store, console::show)
        }
    };
    let instance = Instance::new(&module, &import_object)?;

    let main = instance.exports.get_function("main")?;
    let result = main.call(&[])?;

    dbg!(&result);

    Ok(())
}
