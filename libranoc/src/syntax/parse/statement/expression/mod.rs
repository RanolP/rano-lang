use crate::{
    core::ast::{Expression, Statement},
    syntax::{parse::*, Token},
};

mod literal;

pub use literal::*;

pub fn parse_expression(s: ParseInput) -> ParseResult<Expression> {
    parse_literal_expression(s)
}

pub fn parse_expression_statement(s: ParseInput) -> ParseResult<Statement> {
    map(
        terminated(parse_expression, tag(Token::PunctuationSemicolon)),
        Statement::Expression,
    )(s)
}
