use crate::{core::ast::Pattern, syntax::parse::*};

pub fn parse_pattern_slot(i: ParseInput) -> ParseResult<Pattern> {
    map(parse_name, Pattern::Slot)(i)
}

pub fn parse_pattern(i: ParseInput) -> ParseResult<Pattern> {
    parse_pattern_slot(i)
}
