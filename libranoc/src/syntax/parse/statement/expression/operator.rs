use crate::{
    core::ast::*,
    syntax::{parse::*, Span, TokenKind},
};

pub struct OperatorBindingPowerPrefix {
    pub constructor: Box<dyn FnOnce(Box<Expression>) -> PrefixOperator>,
    pub right_binding_power: u8,
}

pub struct OperatorBindingPowerInfix {
    pub operator: Token,
    pub constructor: Box<dyn FnOnce(Box<Expression>, Span, Box<Expression>) -> InfixOperator>,
    pub left_binding_power: u8,
    pub right_binding_power: u8,
}

pub struct OperatorBindingPowerPostfix {
    pub constructor: Box<dyn FnOnce(Box<Expression>, Vec<Expression>) -> PostfixOperator>,
    pub left_binding_power: u8,
    pub tails: Box<dyn FnOnce(ParseInput) -> ParseResult<Vec<Expression>>>,
    pub close: Box<dyn FnOnce(ParseInput) -> ParseResult<()>>,
}

pub fn parse_prefix_operator(i: ParseInput) -> ParseResult<OperatorBindingPowerPrefix> {
    alt((
        map(tag(TokenKind::PunctuationExclamationMark), |_| {
            OperatorBindingPowerPrefix {
                constructor: Box::new(|expr| PrefixOperator::Not(Not(expr))),
                right_binding_power: 13,
            }
        }),
        map(tag(TokenKind::PunctuationPlusSign), |_| {
            OperatorBindingPowerPrefix {
                constructor: Box::new(|expr| PrefixOperator::UnaryPlus(UnaryPlus(expr))),
                right_binding_power: 13,
            }
        }),
        map(tag(TokenKind::PunctuationHyphenMinus), |_| {
            OperatorBindingPowerPrefix {
                constructor: Box::new(|expr| PrefixOperator::UnaryMinus(UnaryMinus(expr))),
                right_binding_power: 13,
            }
        }),
    ))(i)
}

pub fn parse_infix_operator(i: ParseInput) -> ParseResult<OperatorBindingPowerInfix> {
    alt((
        map(tag(TokenKind::PunctuationsLogicalOr), |operator| {
            OperatorBindingPowerInfix {
                operator,
                constructor: Box::new(|lhs, span, rhs| InfixOperator::LogicalOr(lhs, span, rhs)),
                left_binding_power: 1,
                right_binding_power: 2,
            }
        }),
        map(tag(TokenKind::PunctuationsLogicalAnd), |operator| {
            OperatorBindingPowerInfix {
                operator,
                constructor: Box::new(|lhs, span, rhs| InfixOperator::LogicalAnd(lhs, span, rhs)),
                left_binding_power: 3,
                right_binding_power: 4,
            }
        }),
        map(tag(TokenKind::PunctuationsEqualTo), |operator| {
            OperatorBindingPowerInfix {
                operator,
                constructor: Box::new(|lhs, span, rhs| InfixOperator::EqualTo(lhs, span, rhs)),
                left_binding_power: 5,
                right_binding_power: 6,
            }
        }),
        map(tag(TokenKind::PunctuationsNotEqualTo), |operator| {
            OperatorBindingPowerInfix {
                operator,
                constructor: Box::new(|lhs, span, rhs| InfixOperator::NotEqualTo(lhs, span, rhs)),
                left_binding_power: 5,
                right_binding_power: 6,
            }
        }),
        map(tag(TokenKind::PunctuationGreaterThanSign), |operator| {
            OperatorBindingPowerInfix {
                operator,
                constructor: Box::new(|lhs, span, rhs| InfixOperator::GreaterThan(lhs, span, rhs)),
                left_binding_power: 7,
                right_binding_power: 8,
            }
        }),
        map(tag(TokenKind::PunctuationLessThanSign), |operator| {
            OperatorBindingPowerInfix {
                operator,
                constructor: Box::new(|lhs, span, rhs| InfixOperator::LessThan(lhs, span, rhs)),
                left_binding_power: 7,
                right_binding_power: 8,
            }
        }),
        map(
            tag(TokenKind::PunctuationsGreaterThanOrEqualTo),
            |operator| OperatorBindingPowerInfix {
                operator,
                constructor: Box::new(|lhs, span, rhs| {
                    InfixOperator::GreaterThanOrEqualTo(lhs, span, rhs)
                }),
                left_binding_power: 7,
                right_binding_power: 8,
            },
        ),
        map(tag(TokenKind::PunctuationsLessThanOrEqualTo), |operator| {
            OperatorBindingPowerInfix {
                operator,
                constructor: Box::new(|lhs, span, rhs| {
                    InfixOperator::LessThanOrEqualTo(lhs, span, rhs)
                }),
                left_binding_power: 7,
                right_binding_power: 8,
            }
        }),
        map(tag(TokenKind::PunctuationPlusSign), |operator| {
            OperatorBindingPowerInfix {
                operator,
                constructor: Box::new(|lhs, span, rhs| InfixOperator::Add(lhs, span, rhs)),
                left_binding_power: 9,
                right_binding_power: 10,
            }
        }),
        map(tag(TokenKind::PunctuationHyphenMinus), |operator| {
            OperatorBindingPowerInfix {
                operator,
                constructor: Box::new(|lhs, span, rhs| InfixOperator::Subtract(lhs, span, rhs)),
                left_binding_power: 9,
                right_binding_power: 10,
            }
        }),
        map(tag(TokenKind::PunctuationAsterisk), |operator| {
            OperatorBindingPowerInfix {
                operator,
                constructor: Box::new(|lhs, span, rhs| InfixOperator::Multiply(lhs, span, rhs)),
                left_binding_power: 11,
                right_binding_power: 12,
            }
        }),
        map(tag(TokenKind::PunctuationSolidus), |operator| {
            OperatorBindingPowerInfix {
                operator,
                constructor: Box::new(|lhs, span, rhs| InfixOperator::Divide(lhs, span, rhs)),
                left_binding_power: 11,
                right_binding_power: 12,
            }
        }),
        map(tag(TokenKind::PunctuationPercentSign), |operator| {
            OperatorBindingPowerInfix {
                operator,
                constructor: Box::new(|lhs, operator, rhs| {
                    InfixOperator::Remainder(lhs, operator, rhs)
                }),
                left_binding_power: 11,
                right_binding_power: 12,
            }
        }),
        map(tag(TokenKind::PunctuationFullStop), |operator| {
            OperatorBindingPowerInfix {
                operator,
                constructor: Box::new(|lhs, span, rhs| InfixOperator::GetField(GetField(lhs, rhs))),
                left_binding_power: 17,
                right_binding_power: 16,
            }
        }),
        map(tag(TokenKind::PunctuationsGetFieldNullable), |operator| {
            OperatorBindingPowerInfix {
                operator,
                constructor: Box::new(|lhs, span, rhs| {
                    InfixOperator::GetFieldNullable(GetFieldNullable(lhs, rhs))
                }),
                left_binding_power: 17,
                right_binding_power: 16,
            }
        }),
        map(
            tag(TokenKind::PunctuationsRangeRightExclusive),
            |operator| OperatorBindingPowerInfix {
                operator,
                constructor: Box::new(|lhs, span, rhs| {
                    InfixOperator::RangeRightExclusive(lhs, span, rhs)
                }),
                left_binding_power: 19,
                right_binding_power: 18,
            },
        ),
        map(
            tag(TokenKind::PunctuationsRangeRightInclusive),
            |operator| OperatorBindingPowerInfix {
                operator,
                constructor: Box::new(|lhs, span, rhs| {
                    InfixOperator::RangeRightInclusive(lhs, span, rhs)
                }),
                left_binding_power: 19,
                right_binding_power: 18,
            },
        ),
    ))(i)
}

pub fn parse_postfix_operator(i: ParseInput) -> ParseResult<OperatorBindingPowerPostfix> {
    map(tag(TokenKind::PunctuationLeftSquareBracket), |_| {
        OperatorBindingPowerPostfix {
            constructor: Box::new(|expr, tails| PostfixOperator::Index(Index(expr, tails))),
            left_binding_power: 14,
            tails: Box::new(map(parse_expression, |expr| vec![expr])),
            close: Box::new(map(tag(TokenKind::PunctuationRightSquareBracket), |_| ())),
        }
    })(i)
}
