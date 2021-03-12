use crate::{codegen::*, core::ast::Expression};

mod name;
mod operator;

pub use name::*;
pub use operator::*;

impl<'a> Walker<Expression> for Context<'a> {
    fn walk(&mut self, expression: Expression) -> Result<(), Error> {
        match expression {
            Expression::Match => {
                todo!("match is not implemented now")
            }
            Expression::Closure => {
                todo!("closure is not implemented now")
            }
            Expression::Literal(_) => {
                todo!("literal is not implemented now")
            }
            Expression::Path => {
                todo!("path is not implemented now")
            }
            Expression::Array => {
                todo!("array is not implemented now")
            }
            Expression::Tuple(_) => {
                todo!("tuple is not implemented now")
            }
            Expression::Init => {
                todo!("struct/union init is not implemented now")
            }
            Expression::Operator(operator) => self.walk(operator),
            Expression::Name(name) => self.walk(name),
        }
    }
}
