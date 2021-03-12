use crate::{codegen::*, core::ast::PostfixOperator};

mod function_call;

impl<'a> Walker<PostfixOperator> for Context<'a> {
    fn walk(&mut self, operator: PostfixOperator) -> Result<(), Error> {
        match operator {
            PostfixOperator::Index(_) => {
                todo!("Postfix operator index is not implemented");
            }
            PostfixOperator::FunctionCall(operator) => self.walk(operator),
        }
    }
}
