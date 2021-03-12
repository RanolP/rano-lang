use crate::{
    core::ast::*,
    syntax::parse::*,
};

pub fn parse_literal_string(i: ParseInput) -> ParseResult<Literal> {
    let (i, token) = any(i)?;
    if let TokenKind::LiteralString(v) = &token.kind {
        Ok((i, Literal::String(v.clone())))
    } else {
        err_tag(i)
    }
}

pub fn parse_literal_character(i: ParseInput) -> ParseResult<Literal> {
    let (i, token) = any(i)?;
    if let TokenKind::LiteralCharacter(v) = &token.kind {
        Ok((i, Literal::Character(v.clone())))
    } else {
        err_tag(i)
    }
}

pub fn parse_literal_boolean(i: ParseInput) -> ParseResult<Literal> {
    let (i, token) = any(i)?;
    if let TokenKind::LiteralBoolean(v) = &token.kind {
        Ok((i, Literal::Boolean(v.clone())))
    } else {
        err_tag(i)
    }
}

pub fn parse_literal_integer(i: ParseInput) -> ParseResult<Literal> {
    let (i, token) = any(i)?;
    if let TokenKind::LiteralNumberIntegral(v) = &token.kind {
        Ok((i, Literal::Integer(Integer(v.clone()))))
    } else {
        err_tag(i)
    }
}
pub fn parse_literal_decimal(i: ParseInput) -> ParseResult<Literal> {
    let (i, token) = any(i)?;
    if let TokenKind::LiteralNumberDecimal(v) = &token.kind {
        Ok((i, Literal::Decimal(v.clone())))
    } else {
        err_tag(i)
    }
}

pub fn parse_literal(i: ParseInput) -> ParseResult<Literal> {
    alt((
        parse_literal_string,
        parse_literal_character,
        parse_literal_boolean,
        parse_literal_integer,
        parse_literal_decimal,
    ))(i)
}

pub fn parse_literal_expression(i: ParseInput) -> ParseResult<Expression> {
    map(parse_literal, Expression::Literal)(i)
}
