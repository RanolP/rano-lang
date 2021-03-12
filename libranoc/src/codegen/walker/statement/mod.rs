use crate::{codegen::*, core::ast::Statement};

mod declaration;
mod expression;

pub use declaration::*;
pub use expression::*;
use wasm_encoder::Instruction;

impl<'a> Walker<Statement> for Context<'a> {
    fn walk(&mut self, statement: Statement) -> Result<(), Error> {
        match statement {
            Statement::Declaration(declaration) => self.walk(declaration),
            Statement::Expression(expression) => {
                self.walk(expression)?;
                self.instructions.push(Instruction::Drop);
                Ok(())
            }
        }
    }
}
