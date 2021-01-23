use crate::{core::ast::Path, syntax::parse::*};

use super::parse_identifier_content;

pub fn parse_path(i: ParseInput) -> ParseResult<Path> {
    map(
        separated_list1(
            tag(TokenKind::PunctuationFullStop),
            parse_identifier_content,
        ),
        Path,
    )(i)
}
