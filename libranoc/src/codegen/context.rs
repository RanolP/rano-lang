use wasm_encoder::Module;

pub struct Context {
    pub(super) module: Module,
}

impl Context {
    pub fn new() -> Self {
        Context {
            module: Module::new(),
        }
    }
}
