use crate::{core::ast::Path, syntax::parse::*};

pub fn parse_path(i: ParseInput) -> ParseResult<Path> {
    map(
        separated_list1(tag(TokenKind::PunctuationFullStop), parse_identifier),
        Path,
    )(i)
}
