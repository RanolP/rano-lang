use crate::{core::ast::Name, syntax::parse::*};

pub fn parse_name_ident(s: ParseInput) -> ParseResult<Name> {
    let (s, token) = parse_identifier_content(s)?;
    Ok((s, Name::Ident(token)))
}

pub fn parse_name_placeholder(s: ParseInput) -> ParseResult<Name> {
    let (s, _) = tag(Token::KeywordPlaceholderName)(s)?;
    Ok((s, Name::Placeholder))
}

pub fn parse_name(s: ParseInput) -> ParseResult<Name> {
    alt((parse_name_ident, parse_name_placeholder))(s)
}
