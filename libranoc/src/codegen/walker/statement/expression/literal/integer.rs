use wasm_encoder::Instruction;

use crate::{codegen::*, core::ast::Integer};

impl<'a> Walker<Integer> for Context<'a> {
    fn walk(&mut self, Integer(s): Integer) -> Result<(), Error> {
        let v: i32 = s.parse().unwrap();
        self.instructions.push(Instruction::I32Const(v));
        Ok(())
    }
}
