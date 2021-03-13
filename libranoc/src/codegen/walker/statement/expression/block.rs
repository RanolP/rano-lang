use crate::{codegen::*, core::ast::Block};

impl<'a> Walker<Block> for Context<'a> {
    fn walk(&mut self, block: Block) -> Result<(), Error> {
        for statement in block.body {
            self.walk(statement)?;
        }
        if let Some(last_expression) = block.last_expression {
            self.walk(last_expression)?;
        }
        Ok(())
    }
}
