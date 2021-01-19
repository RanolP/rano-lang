use crate::{core::ast::Pattern, syntax::parse::*};

pub fn parse_pattern_slot(s: ParseInput) -> ParseResult<Pattern> {
    let (s, name) = parse_name(s)?;
    Ok((s, Pattern::Slot(name)))
}

pub fn parse_pattern(s: ParseInput) -> ParseResult<Pattern> {
    parse_pattern_slot(s)
}
