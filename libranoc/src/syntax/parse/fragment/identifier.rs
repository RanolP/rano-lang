use crate::{
    core::ast::Identifier,
    syntax::{parse::*, TokenKind},
};

pub fn parse_identifier(i: ParseInput) -> ParseResult<Identifier> {
    let (i, token) = any(i)?;
    if let TokenKind::IdentifierIdentifier(content) = &token.kind {
        Ok((i, Identifier(token.clone(), content.clone())))
    } else {
        err_tag(i)
    }
}
