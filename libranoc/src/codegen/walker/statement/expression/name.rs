use wasm_encoder::Instruction;

use crate::{codegen::*, core::ast::Name};

impl<'a> Walker<Name> for Context<'a> {
    fn walk(&mut self, name: Name) -> Result<(), Error> {
        if let Name::Ident(ident) = name {
            let resolved = self.resolve(&ident.content, ident.span)?;
            self.instructions.push(Instruction::LocalGet(resolved));
        }
        Ok(())
    }
}
