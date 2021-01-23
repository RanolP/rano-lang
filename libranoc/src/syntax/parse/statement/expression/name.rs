use crate::{core::ast::Expression, syntax::parse::*};

pub fn parse_name_expression(i: ParseInput) -> ParseResult<Expression> {
    map(parse_name, Expression::Name)(i)
}
