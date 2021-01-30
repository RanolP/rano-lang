use crate::core::ast::Module;

mod fragment;
mod nom;
mod statement;

pub(super) use self::nom::*;
pub(super) use fragment::*;
pub(super) use statement::*;

pub use crate::syntax::{
    parse::nom::{Error, ParseResult},
    Token, TokenKind,
};

pub fn parse<'a>(tokens: &'a [Token]) -> crate::core::Result<Module<'a>> {
    let i = ParseInput::new(tokens);
    let (_, nodes) = all_consuming(many0(parse_statement_node))(i)?;
    Ok(Module { nodes })
}
