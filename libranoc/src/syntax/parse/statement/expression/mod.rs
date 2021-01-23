use crate::{
    core::ast::{Expression, Operator, PostfixOperatorKind, Statement},
    syntax::parse::*,
};

mod group_tuple;
mod literal;
mod name;
mod operator;

pub use group_tuple::*;
pub use literal::*;
pub use name::*;
pub use operator::*;

pub fn parse_simple_expression(i: ParseInput) -> ParseResult<Expression> {
    alt((parse_literal_expression, parse_name_expression))(i)
}
pub fn parse_expression<'a>(i: ParseInput<'a>) -> ParseResult<'a, Expression> {
    let (i, lhs) = alt((
        |i| {
            let (i, operator) = parse_prefix_operator(i)?;
            let original_binding_power = i.binding_power;
            let (i, rhs) = parse_expression(i.with_binding_power(operator.right_binding_power))?;
            Ok((
                i.with_binding_power(original_binding_power),
                Expression::Operator(Operator::Prefix(operator.kind, Box::new(rhs))),
            ))
        },
        parse_group_tuple_expression,
        parse_simple_expression,
    ))(i)?;

    let (i, lhs) = fold_many0(
        alt((
            |i| {
                let (i, operator) = parse_postfix_operator(i)?;
                if operator.left_binding_power < i.binding_power {
                    return err_tag(i);
                }

                if operator.kind == PostfixOperatorKind::Index {
                    let original_binding_power = i.binding_power;
                    let (i, rhs) = parse_expression(i.with_binding_power(0))?;
                    let (i, _) = tag(TokenKind::PunctuationRightSquareBracket)(
                        i.with_binding_power(original_binding_power),
                    )?;

                    Ok((i, AfterLhsOperator::Postfix(operator.kind, vec![rhs])))
                } else {
                    Ok((i, AfterLhsOperator::Postfix(operator.kind, Vec::new())))
                }
            },
            |i| {
                let (i, operator) = parse_infix_operator(i)?;
                if operator.left_binding_power < i.binding_power {
                    return err_tag(i);
                }

                let original_binding_power = i.binding_power;
                let (i, rhs) =
                    parse_expression(i.with_binding_power(operator.right_binding_power))?;

                Ok((
                    i.with_binding_power(original_binding_power),
                    AfterLhsOperator::Infix(operator.kind, Box::new(rhs)),
                ))
            },
            |i: ParseInput<'a>| {
                if 15 < i.binding_power {
                    return err_tag(i);
                }

                let original_binding_power = i.binding_power;
                let (i, rhs) = parse_expression(i.with_binding_power(15))?;

                Ok((
                    i.with_binding_power(original_binding_power),
                    AfterLhsOperator::Postfix(PostfixOperatorKind::FunctionCall, vec![rhs]),
                ))
            },
        )),
        lhs,
        |lhs, curr| match curr {
            AfterLhsOperator::Infix(kind, rhs) => {
                Expression::Operator(Operator::Infix(Box::new(lhs), kind, rhs))
            }
            AfterLhsOperator::Postfix(kind, rhs) => {
                Expression::Operator(Operator::Postfix(Box::new(lhs), kind, rhs))
            }
        },
    )(i)?;

    Ok((i, lhs))
}

pub fn parse_expression_statement(i: ParseInput) -> ParseResult<Statement> {
    map(
        terminated(parse_expression, tag(TokenKind::PunctuationSemicolon)),
        Statement::Expression,
    )(i)
}
