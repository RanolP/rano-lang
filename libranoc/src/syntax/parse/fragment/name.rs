use crate::{core::ast::Name, syntax::parse::*};

pub fn parse_name_ident(i: ParseInput) -> ParseResult<Name> {
    map(parse_identifier_content, Name::Ident)(i)
}

pub fn parse_name_placeholder(i: ParseInput) -> ParseResult<Name> {
    map(tag(Token::KeywordPlaceholderName), |_| Name::Placeholder)(i)
}

pub fn parse_name(i: ParseInput) -> ParseResult<Name> {
    alt((parse_name_ident, parse_name_placeholder))(i)
}
