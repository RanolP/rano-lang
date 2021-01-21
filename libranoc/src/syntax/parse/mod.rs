use crate::core::ast::{Module, Node};

mod fragment;
mod nom;
mod statement;

pub(super) use self::nom::*;
pub(super) use fragment::*;
pub(super) use statement::*;

pub use crate::syntax::{parse::nom::ParseResult, Token};

pub fn parse(tokens: &[Token]) -> ParseResultStd<Module> {
    let i = ParseInput::new(tokens);
    let (_, nodes) = all_consuming(many0(statement::parse_statement_node))(i)?;
    Ok(Module { nodes })
}
