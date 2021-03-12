use crate::{codegen::*, core::ast::Literal};

mod integer;

impl<'a> Walker<Literal> for Context<'a> {
    fn walk(&mut self, literal: Literal) -> Result<(), Error> {
        match literal {
            Literal::String(_) => {
                todo!()
            }
            Literal::Character(_) => {
                todo!()
            }
            Literal::Integer(integer) => self.walk(integer),
            Literal::Decimal(_) => {
                todo!()
            }
            Literal::Boolean(_) => {
                todo!()
            }
        }
    }
}
