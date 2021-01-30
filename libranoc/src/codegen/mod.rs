use crate::core::ast::Module;

mod context;
mod walker;

pub(super) use crate::core::Error;
pub(super) use context::*;
pub(super) use walker::*;

pub fn compile_wasm(module: Module) -> (Vec<u8>, Vec<Error>) {
    let mut context = Context::new();

    walk_module(&mut context, module);

    context.finish()
}
