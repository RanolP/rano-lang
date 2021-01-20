use crate::{
    core::ast::{Declaration, Statement},
    syntax::parse::*,
};

mod function;

pub use function::*;

pub fn parse_declaration(i: ParseInput) -> ParseResult<Declaration> {
    parse_function_declaration_declaration(i)
}

pub fn parse_declaration_statement(i: ParseInput) -> ParseResult<Statement> {
    map(parse_declaration, Statement::Declaration)(i)
}
