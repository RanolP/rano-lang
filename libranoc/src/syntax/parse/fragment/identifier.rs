use crate::syntax::{parse::*, Token};

pub fn parse_identifier_content(i: ParseInput) -> ParseResult<String> {
    let (i, token) = any(i)?;
    if let Token::IdentifierIdentifier(content) = token {
        Ok((i, content.clone()))
    } else {
        err_tag(i)
    }
}
