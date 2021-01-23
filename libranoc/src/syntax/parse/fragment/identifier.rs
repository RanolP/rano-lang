use crate::syntax::{parse::*, TokenKind};

pub fn parse_identifier_content(i: ParseInput) -> ParseResult<String> {
    let (i, token) = any(i)?;
    if let TokenKind::IdentifierIdentifier(content) = &token.kind {
        Ok((i, content.clone()))
    } else {
        err_tag(i)
    }
}
