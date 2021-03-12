use crate::{core::ast::*, syntax::parse::*};

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
pub fn parse_expression(i: ParseInput) -> ParseResult<Expression> {
    let (i, lhs) = alt((
        |i| {
            let (i, operator) = parse_prefix_operator(i)?;
            let original_binding_power = i.binding_power;
            let (i, rhs) = parse_expression(i.with_binding_power(operator.right_binding_power))?;
            Ok((
                i.with_binding_power(original_binding_power),
                Expression::Operator(Operator::Prefix((operator.constructor)(Box::new(rhs)))),
            ))
        },
        parse_group_tuple_expression,
        parse_simple_expression,
    ))(i)?;

    type Transformer = Box<dyn FnOnce(Box<Expression>) -> Expression>;
    let (i, lhs) = fold_many0(
        alt((
            |i| {
                let (i, operator) = parse_postfix_operator(i)?;
                if operator.left_binding_power < i.binding_power {
                    return err_tag(i);
                }

                let original_binding_power = i.binding_power;
                let (i, tails) = (operator.tails)(i.with_binding_power(0))?;
                let (i, _) = (operator.close)(i.with_binding_power(original_binding_power))?;
                let constructor = operator.constructor;

                let transformer: Transformer = Box::new(move |lhs| {
                    Expression::Operator(Operator::Postfix(constructor(lhs, tails)))
                });
                Ok((i, transformer))
            },
            |i| {
                let (i, operator) = parse_infix_operator(i)?;
                if operator.left_binding_power < i.binding_power {
                    return err_tag(i);
                }

                let original_binding_power = i.binding_power;
                let (i, rhs) =
                    parse_expression(i.with_binding_power(operator.right_binding_power))?;

                let transformer: Transformer = Box::new(move |lhs| {
                    Expression::Operator(Operator::Infix((operator.constructor)(
                        lhs,
                        Box::new(rhs),
                    )))
                });
                Ok((i.with_binding_power(original_binding_power), transformer))
            },
            |i: ParseInput| {
                if 15 < i.binding_power {
                    return err_tag(i);
                }

                let original_binding_power = i.binding_power;
                let (i, rhs) = parse_expression(i.with_binding_power(15))?;

                let transformer: Transformer = Box::new(move |lhs| {
                    Expression::Operator(Operator::Postfix(PostfixOperator::FunctionCall(
                        FunctionCall(lhs, vec![rhs]),
                    )))
                });
                Ok((i.with_binding_power(original_binding_power), transformer))
            },
        )),
        lhs,
        |lhs, transformer| transformer(Box::new(lhs)),
    )(i)?;

    Ok((i, lhs))
}

pub fn parse_expression_statement(i: ParseInput) -> ParseResult<Statement> {
    map(
        terminated(parse_expression, tag(TokenKind::PunctuationSemicolon)),
        Statement::Expression,
    )(i)
}
