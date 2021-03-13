use wasm_encoder::Instruction;

use crate::{codegen::*, core::ast::InfixOperator};

impl<'a> Walker<InfixOperator> for Context<'a> {
    fn walk(&mut self, operator: InfixOperator) -> Result<(), Error> {
        match &operator {
            InfixOperator::LogicalOr(lhs, operator_span, rhs)
            | InfixOperator::LogicalAnd(lhs, operator_span, rhs)
            | InfixOperator::Add(lhs, operator_span, rhs)
            | InfixOperator::Subtract(lhs, operator_span, rhs)
            | InfixOperator::Multiply(lhs, operator_span, rhs)
            | InfixOperator::Divide(lhs, operator_span, rhs)
            | InfixOperator::Remainder(lhs, operator_span, rhs)
            | InfixOperator::RangeRightExclusive(lhs, operator_span, rhs)
            | InfixOperator::RangeRightInclusive(lhs, operator_span, rhs) => {
                let trait_name = operator.trait_name();
                let lhs_type = "i32";
                let rhs_type = "i32";
                let function_id = self.import(
                    "extern",
                    format!("{}__{}_{}", trait_name, lhs_type, rhs_type),
                    operator_span.clone(),
                )?;
                self.walk(lhs)?;
                self.walk(rhs)?;
                self.instructions.push(Instruction::Call(function_id));
                Ok(())
            }
            InfixOperator::EqualTo(lhs, operator_span, rhs)
            | InfixOperator::NotEqualTo(lhs, operator_span, rhs) => {
                let to_negate = matches!(operator, InfixOperator::NotEqualTo(..));
                let lhs_type = "i32";
                let rhs_type = "i32";
                let function_id = self.import(
                    "extern",
                    format!("PartialEq__{}_{}", lhs_type, rhs_type),
                    operator_span.clone(),
                )?;
                self.walk(lhs)?;
                self.walk(rhs)?;
                self.instructions.push(Instruction::Call(function_id));
                if to_negate {
                    todo!()
                }
                Ok(())
            }
            InfixOperator::GreaterThan(lhs, operator_span, rhs) => {
                todo!()
            }
            InfixOperator::LessThan(lhs, operator_span, rhs) => {
                todo!()
            }
            InfixOperator::GreaterThanOrEqualTo(lhs, operator_span, rhs) => {
                todo!()
            }
            InfixOperator::LessThanOrEqualTo(lhs, operator_span, rhs) => {
                todo!()
            }
            InfixOperator::GetField(_) => {
                todo!()
            }
            InfixOperator::GetFieldNullable(_) => {
                todo!()
            }
        }
    }
}
