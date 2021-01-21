use libranoc::{codegen, syntax};
use wasmer::{imports, Function, Instance, Module, Store};

mod console {
    fn show(i: i32) {
        println!("{}", i);
    }
}

fn main() -> anyhow::Result<()> {
    let src = r#"
        extern fn show(i: i32);
        fn main {
            console.show 42;
        }
    "#;

    let tokens = syntax::tokenize(src);
    let ast = syntax::parse(&tokens)?;

    dbg!(&ast);
    let mut context = codegen::Context::new();
    let wasm_bytes = codegen::compile_wasm(context, ast);

    let store = Store::new();
    let module = Module::new(&store, &wasm_bytes)?;

    let import_object = imports! {
        "console" => {
            "show" => Function::new_native(&store, console::show)
        }
    };
    let instance = Instance::new(&module, &import_object)?;

    let main = instance.exports.get_function("main")?;
    let result = main.call(&[]);

    Ok(())
}
