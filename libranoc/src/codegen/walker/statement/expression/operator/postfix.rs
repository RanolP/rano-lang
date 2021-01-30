use crate::{
    codegen::*,
    core::ast::{Expression, Operator, PostfixOperatorKind},
};

pub fn walk_postfix_operator(
    context: &mut Context,
    expr: Box<Expression>,
    kind: PostfixOperatorKind,
    tails: Vec<Expression>,
) -> Result<(), Error> {
    match kind {
        PostfixOperatorKind::Index => {
            todo!("Postfix operator index is not implemented");
        }
        PostfixOperatorKind::FunctionCall => {
            todo!("Postfix operator function call is not implemented");
        }
    }
}
