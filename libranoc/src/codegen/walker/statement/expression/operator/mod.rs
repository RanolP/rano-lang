use crate::{codegen::*, core::ast::Operator};

mod postfix;

use postfix::*;

pub fn walk_operator(context: &mut Context, operator: Operator) -> Result<(), Error> {
    match operator {
        Operator::Prefix(kind, expr) => {
            todo!("Prefix operator is not implemented");
        }
        Operator::Infix(lhs, kind, rhs) => {
            todo!("Infix operator is not implemented");
        }
        Operator::Postfix(expr, kind, tails) => walk_postfix_operator(context, expr, kind, tails)
    }
}
