use crate::{codegen::*, core::ast::Expression};

impl<'a> Walker<Vec<Expression>> for Context<'a> {
    fn walk(&mut self, mut tuple: Vec<Expression>) -> Result<(), Error> {
        let len = tuple.len();
        match len {
            0 => {}
            1 => {
                self.walk(tuple.remove(0))?;
            }
            _ => {
                todo!("tuple is not implemented now");
            }
        }
        Ok(())
    }
}
