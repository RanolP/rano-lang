use crate::{
    core::ast::Expression,
    syntax::{parse::*},
};

pub fn parse_group_tuple_expression(i: ParseInput) -> ParseResult<Expression> {
    let i = i.with_binding_power(0);
    let (i, (mut elements, last_comma)) = delimited(
        tag(TokenKind::PunctuationLeftParenthesis),
        |i| {
            let (i, elements) =
                separated_list1(tag(TokenKind::PunctuationComma), parse_expression)(i)?;
            let (i, last_comma) = opt(tag(TokenKind::PunctuationComma))(i)?;
            Ok((i, (elements, last_comma.is_some())))
        },
        tag(TokenKind::PunctuationRightParenthesis),
    )(i)?;

    let expr = if elements.len() > 1 || last_comma {
        elements.swap_remove(0)
    } else {
        Expression::Tuple(elements)
    };

    Ok((i, expr))
}
