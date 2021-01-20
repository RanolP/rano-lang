use crate::{
    core::ast::{Node, Statement},
    syntax::parse::*,
};

mod declaration;
mod expression;

pub use declaration::*;
pub use expression::*;

pub fn parse_statement(s: ParseInput) -> ParseResult<Statement> {
    alt((
        declaration::parse_declaration_statement,
        parse_expression_statement,
    ))(s)
}

pub fn parse_statement_node(s: ParseInput) -> ParseResult<Node> {
    map(parse_statement, Node::Statement)(s)
}
