use crate::{codegen::*, core::ast::Statement};

mod declaration;
mod expression;

pub use declaration::*;
pub use expression::*;

pub fn walk_statement(context: &mut Context, statement: Statement) {
    match statement {
        Statement::Declaration(declaration) => walk_declaration(context, declaration),
        Statement::Expression(expression) => walk_expression(context, expression),
    }
}
