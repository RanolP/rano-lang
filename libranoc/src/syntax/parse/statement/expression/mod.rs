use crate::{
    core::ast::{Expression, Operator, PostfixOperatorKind, Statement},
    syntax::{parse::*, Token},
};

mod group_tuple;
mod literal;
mod operator;

pub use group_tuple::*;
pub use literal::*;
pub use operator::*;

pub fn parse_simple_expression(i: ParseInput) -> ParseResult<Expression> {
    parse_literal_expression(i)
}
pub fn parse_expression(i: ParseInput) -> ParseResult<Expression> {
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
                    let (i, _) = tag(Token::PunctuationRightSquareBracket)(
                        i.with_binding_power(original_binding_power),
                    )?;

                    Ok((
                        i,
                        AfterLhsOperator::Postfix(operator.kind, Some(Box::new(rhs))),
                    ))
                } else {
                    Ok((i, AfterLhsOperator::Postfix(operator.kind, None)))
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
        terminated(parse_expression, tag(Token::PunctuationSemicolon)),
        Statement::Expression,
    )(i)
}
