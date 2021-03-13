use crate::{core::ast::Block, syntax::parse::*};

pub fn parse_block(i: ParseInput) -> ParseResult<Block> {
    let (i, curly_bracket_open_token) = tag(TokenKind::PunctuationLeftCurlyBracket)(i)?;
    let original_binding_power = i.binding_power;
    let (i, (body, last_expression)) =
        cut(tuple((many0(parse_statement), opt(parse_expression))))(i.with_binding_power(0))?;
    let (i, curly_bracket_close_token) =
        tag(TokenKind::PunctuationRightCurlyBracket)(i.with_binding_power(original_binding_power))?;

    Ok((
        i,
        Block {
            curly_bracket_open_token,
            body,
            last_expression,
            curly_bracket_close_token,
        },
    ))
}
