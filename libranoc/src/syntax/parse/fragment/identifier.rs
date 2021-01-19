use crate::syntax::{parse::*, Token};

pub fn parse_identifier_content(s: ParseInput) -> ParseResult<String> {
    let (s, token) = any(s)?;
    match token {
        Token::IdentifierIdentifier(content) => Ok((s, content.clone())),
        _ => err_tag_closure(s),
    }
}
