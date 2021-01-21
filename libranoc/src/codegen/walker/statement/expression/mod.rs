use crate::{codegen::*, core::ast::Expression};

pub fn walk_expression(context: &mut Context, expression: Expression) {
    match expression {
        Expression::Match => {
            todo!("match is not implemented now")
        }
        Expression::Closure => {
            todo!("closure is not implemented now")
        }
        Expression::Literal(_) => {}
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
        Expression::Operator(_) => {
            todo!("operator is not implemented now")
        }
    }
}
