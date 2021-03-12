use wasm_encoder::Instruction;

use crate::{
    codegen::*,
    core::ast::{Expression, FunctionCall, Name},
};

impl<'a> Walker<FunctionCall> for Context<'a> {
    fn walk(&mut self, operator: FunctionCall) -> Result<(), Error> {
        for params in &operator.1 {
            self.walk(params.clone())?;
        }
        if let Expression::Name(Name::Ident(name)) = operator.0.as_ref() {
            let id = self.resolve(&name.content, name.span.clone())?;
            self.instructions.push(Instruction::Call(id));
        } else {
            return Err(Error::unimplemented(&operator));
        }
        // TODO
        Ok(())
    }
}
