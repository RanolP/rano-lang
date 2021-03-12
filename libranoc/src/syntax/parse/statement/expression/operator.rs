use crate::{
    core::ast::*,
    syntax::{parse::*, TokenKind},
};

pub struct OperatorBindingPowerPrefix {
    pub constructor: Box<dyn FnOnce(Box<Expression>) -> PrefixOperator>,
    pub right_binding_power: u8,
}

pub struct OperatorBindingPowerInfix {
    pub constructor: Box<dyn FnOnce(Box<Expression>, Box<Expression>) -> InfixOperator>,
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
        map(tag(TokenKind::PunctuationsLogicalOr), |_| {
            OperatorBindingPowerInfix {
                constructor: Box::new(|lhs, rhs| InfixOperator::LogicalOr(LogicalOr(lhs, rhs))),
                left_binding_power: 1,
                right_binding_power: 2,
            }
        }),
        map(tag(TokenKind::PunctuationsLogicalAnd), |_| {
            OperatorBindingPowerInfix {
                constructor: Box::new(|lhs, rhs| InfixOperator::LogicalAnd(LogicalAnd(lhs, rhs))),
                left_binding_power: 3,
                right_binding_power: 4,
            }
        }),
        map(tag(TokenKind::PunctuationsEqualTo), |_| {
            OperatorBindingPowerInfix {
                constructor: Box::new(|lhs, rhs| InfixOperator::EqualTo(EqualTo(lhs, rhs))),
                left_binding_power: 5,
                right_binding_power: 6,
            }
        }),
        map(tag(TokenKind::PunctuationsNotEqualTo), |_| {
            OperatorBindingPowerInfix {
                constructor: Box::new(|lhs, rhs| InfixOperator::NotEqualTo(NotEqualTo(lhs, rhs))),
                left_binding_power: 5,
                right_binding_power: 6,
            }
        }),
        map(tag(TokenKind::PunctuationGreaterThanSign), |_| {
            OperatorBindingPowerInfix {
                constructor: Box::new(|lhs, rhs| InfixOperator::GreaterThan(GreaterThan(lhs, rhs))),
                left_binding_power: 7,
                right_binding_power: 8,
            }
        }),
        map(tag(TokenKind::PunctuationLessThanSign), |_| {
            OperatorBindingPowerInfix {
                constructor: Box::new(|lhs, rhs| InfixOperator::LessThan(LessThan(lhs, rhs))),
                left_binding_power: 7,
                right_binding_power: 8,
            }
        }),
        map(tag(TokenKind::PunctuationsGreaterThanOrEqualTo), |_| {
            OperatorBindingPowerInfix {
                constructor: Box::new(|lhs, rhs| {
                    InfixOperator::GreaterThanOrEqualTo(GreaterThanOrEqualTo(lhs, rhs))
                }),
                left_binding_power: 7,
                right_binding_power: 8,
            }
        }),
        map(tag(TokenKind::PunctuationsLessThanOrEqualTo), |_| {
            OperatorBindingPowerInfix {
                constructor: Box::new(|lhs, rhs| {
                    InfixOperator::LessThanOrEqualTo(LessThanOrEqualTo(lhs, rhs))
                }),
                left_binding_power: 7,
                right_binding_power: 8,
            }
        }),
        map(tag(TokenKind::PunctuationPlusSign), |_| {
            OperatorBindingPowerInfix {
                constructor: Box::new(|lhs, rhs| InfixOperator::Add(Add(lhs, rhs))),
                left_binding_power: 9,
                right_binding_power: 10,
            }
        }),
        map(tag(TokenKind::PunctuationHyphenMinus), |_| {
            OperatorBindingPowerInfix {
                constructor: Box::new(|lhs, rhs| InfixOperator::Subtract(Subtract(lhs, rhs))),
                left_binding_power: 9,
                right_binding_power: 10,
            }
        }),
        map(tag(TokenKind::PunctuationAsterisk), |_| {
            OperatorBindingPowerInfix {
                constructor: Box::new(|lhs, rhs| InfixOperator::Multiply(Multiply(lhs, rhs))),
                left_binding_power: 11,
                right_binding_power: 12,
            }
        }),
        map(tag(TokenKind::PunctuationSolidus), |_| {
            OperatorBindingPowerInfix {
                constructor: Box::new(|lhs, rhs| InfixOperator::Divide(Divide(lhs, rhs))),
                left_binding_power: 11,
                right_binding_power: 12,
            }
        }),
        map(tag(TokenKind::PunctuationPercentSign), |_| {
            OperatorBindingPowerInfix {
                constructor: Box::new(|lhs, rhs| InfixOperator::Remainder(Remainder(lhs, rhs))),
                left_binding_power: 11,
                right_binding_power: 12,
            }
        }),
        map(tag(TokenKind::PunctuationFullStop), |_| {
            OperatorBindingPowerInfix {
                constructor: Box::new(|lhs, rhs| InfixOperator::GetField(GetField(lhs, rhs))),
                left_binding_power: 17,
                right_binding_power: 16,
            }
        }),
        map(tag(TokenKind::PunctuationsGetFieldNullable), |_| {
            OperatorBindingPowerInfix {
                constructor: Box::new(|lhs, rhs| {
                    InfixOperator::GetFieldNullable(GetFieldNullable(lhs, rhs))
                }),
                left_binding_power: 17,
                right_binding_power: 16,
            }
        }),
        map(tag(TokenKind::PunctuationsRangeRightExclusive), |_| {
            OperatorBindingPowerInfix {
                constructor: Box::new(|lhs, rhs| {
                    InfixOperator::RangeRightExclusive(RangeRightExclusive(lhs, rhs))
                }),
                left_binding_power: 19,
                right_binding_power: 18,
            }
        }),
        map(tag(TokenKind::PunctuationsRangeRightInclusive), |_| {
            OperatorBindingPowerInfix {
                constructor: Box::new(|lhs, rhs| {
                    InfixOperator::RangeRightInclusive(RangeRightInclusive(lhs, rhs))
                }),
                left_binding_power: 19,
                right_binding_power: 18,
            }
        }),
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
