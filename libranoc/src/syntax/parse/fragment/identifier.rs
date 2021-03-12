use crate::syntax::{parse::*, TokenKind};

pub fn parse_identifier(i: ParseInput) -> ParseResult<Token> {
    let (i, token) = any(i)?;
    if let TokenKind::IdentifierIdentifier(_) = &token.kind {
        Ok((i, token))
    } else {
        err_tag(i)
    }
}
