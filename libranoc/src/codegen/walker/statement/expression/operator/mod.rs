use crate::{codegen::*, core::ast::Operator};

mod function_call;
mod postfix;

pub use function_call::*;
pub use postfix::*;

impl<'a> Walker<Operator> for Context<'a> {
    fn walk(&mut self, operator: Operator) -> Result<(), Error> {
        match operator {
            Operator::Prefix(operator) => {
                todo!("Prefix operator is not implemented");
            }
            Operator::Infix(operator) => {
                todo!("Infix operator is not implemented");
            }
            Operator::Postfix(operator) => self.walk(operator),
        }
    }
}
