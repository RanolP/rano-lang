use crate::core::ast::Module;

mod context;
mod walker;

pub(super) use context::*;
pub(super) use walker::*;

pub fn compile_wasm(module: Module) -> Vec<u8> {
    let mut context = Context::new();

    walk_module(&mut context, module);

    context.module.finish()
}
