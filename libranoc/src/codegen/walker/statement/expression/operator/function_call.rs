use wasm_encoder::Instruction;

use crate::{codegen::*, core::ast::FunctionCall};

impl<'a> Walker<FunctionCall> for Context<'a> {
    fn walk(&mut self, operator: FunctionCall) -> Result<(), Error> {
        for params in operator.1 {
            self.walk(params)?;
        }
        self.instructions.push(Instruction::CallIndirect);
        // TODO
        Ok(())
    }
}
