use crate::{
    core::ast::Path,
    syntax::{parse::*, Token},
};

use super::parse_identifier_content;

pub fn parse_path(s: ParseInput) -> ParseResult<Path> {
    let (s, segments) =
        separated_list1(tag(Token::PunctuationFullStop), parse_identifier_content)(s)?;

    Ok((s, Path(segments)))
}
