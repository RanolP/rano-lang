use crate::{codegen::*, core::ast::Expression};

mod operator;
mod name;

pub use operator::*;
pub use name::*;

pub fn walk_expression(context: &mut Context, expression: Expression) -> Result<(), Error> {
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
        Expression::Operator(operator) => walk_operator(context, operator),
        Expression::Name(_) => {
            todo!("name is not implemented now")
        }
    }
}
