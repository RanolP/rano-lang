use crate::{
    core::ast::{Declaration, Statement},
    syntax::parse::*,
};

mod function;

pub use function::*;

pub fn parse_declaration(s: ParseInput) -> ParseResult<Declaration> {
    parse_function_declaration_declaration(s)
}

pub fn parse_declaration_statement(s: ParseInput) -> ParseResult<Statement> {
    let (s, declaration) = parse_declaration(s)?;
    Ok((s, Statement::Declaration(declaration)))
}
