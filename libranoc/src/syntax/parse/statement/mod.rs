use crate::{
    core::ast::{Node, Statement},
    syntax::parse::*,
};

mod declaration;

pub fn parse_statement(s: ParseInput) -> ParseResult<Statement> {
    declaration::parse_declaration_statement(s)
}

pub fn parse_statement_node(s: ParseInput) -> ParseResult<Node> {
    let (s, statement) = parse_statement(s)?;
    Ok((s, Node::Statement(statement)))
}
