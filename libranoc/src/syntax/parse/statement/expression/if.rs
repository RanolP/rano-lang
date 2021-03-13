use crate::{core::ast::*, syntax::parse::*};

fn parse_else(i: ParseInput) -> ParseResult<Else> {
    let (i, else_token) = tag(TokenKind::KeywordElse)(i)?;

    let (i, r#else) = alt((
        map(parse_if, |r#if| {
            Else::If(else_token.clone(), Box::new(r#if))
        }),
        map(parse_block, |block| {
            Else::Block(else_token.clone(), Box::new(block))
        }),
    ))(i)?;

    Ok((i, r#else))
}

pub fn parse_if(i: ParseInput) -> ParseResult<If> {
    let (i, if_token) = tag(TokenKind::KeywordIf)(i)?;
    let original_binding_power = i.binding_power;
    let (i, condition) = parse_expression(i.with_binding_power(0))?;
    let (i, body) = parse_block(i)?;
    let (i, else_part) = opt(parse_else)(i)?;

    Ok((
        i.with_binding_power(original_binding_power),
        If {
            if_token,
            condition: Box::new(condition),
            body: Box::new(body),
            else_part,
        },
    ))
}

pub fn parse_if_expression(i: ParseInput) -> ParseResult<Expression> {
    map(parse_if, Expression::If)(i)
}
