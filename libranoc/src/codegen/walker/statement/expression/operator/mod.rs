use crate::{codegen::*, core::ast::Operator};

// mod prefix;
mod infix;
mod postfix;

impl<'a> Walker<Operator> for Context<'a> {
    fn walk(&mut self, operator: Operator) -> Result<(), Error> {
        match operator {
            Operator::Prefix(operator) => {
                todo!("Prefix operator is not implemented");
            }
            Operator::Infix(operator) => self.walk(operator),
            Operator::Postfix(operator) => self.walk(operator),
        }
    }
}
