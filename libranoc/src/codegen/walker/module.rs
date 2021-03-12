use crate::{
    codegen::*,
    core::ast::{Module, Node},
};

impl<'a> Walker<Module> for Context<'a> {
    fn walk(&mut self, module: Module) -> Result<(), Error> {
        for node in module.nodes {
            match node {
                Node::Directive => {
                    todo!("directive is not implemented now")
                }
                Node::Statement(statement) => {
                    if let Err(error) = self.walk(statement) {
                        self.add_compilation_error(error);
                    }
                }
            }
        }
        Ok(())
    }
}
