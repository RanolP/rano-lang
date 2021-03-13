use wasm_encoder::{BlockType, Instruction, ValType};

use crate::{codegen::*, core::ast::*};

impl<'a> Walker<Else> for Context<'a> {
    fn walk(&mut self, r#else: Else) -> Result<(), Error> {
        self.instructions.push(Instruction::Else);
        match r#else {
            Else::If(_, r#if) => self.walk(r#if),
            Else::Block(_, block) => self.walk(block),
        }
    }
}

impl<'a> Walker<If> for Context<'a> {
    fn walk(&mut self, r#if: If) -> Result<(), Error> {
        self.walk(r#if.condition)?;
        // TODO
        // we all pretend to be the i32 on the good side.
        // but what if we're not the i32 on the other?
        let expression_type = ValType::I32;
        self.instructions
            .push(Instruction::If(BlockType::Result(expression_type)));
        self.walk(r#if.body)?;
        if let Some(r#else) = r#if.else_part {
            self.walk(r#else)?;
        }
        self.instructions.push(Instruction::End);

        Ok(())
    }
}
