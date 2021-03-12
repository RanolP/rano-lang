use crate::{codegen::*, core::ast::Declaration};

mod function_declaration;

impl<'a> Walker<Declaration> for Context<'a> {
    fn walk(&mut self, declaration: Declaration) -> Result<(), Error> {
        match declaration {
            Declaration::FunctionDeclaration(function_declaration) => {
                self.walk(function_declaration)
            }
            Declaration::VariableDeclaration => {
                todo!("variable declaration is not implemented now")
            }
            Declaration::StructDeclaration => {
                todo!("struct declaration is not implemented now")
            }
            Declaration::UnionDeclaration => {
                todo!("union declaration is not implemented now")
            }
            Declaration::TypeDeclaration => {
                todo!("type declaration is not implemented now")
            }
            Declaration::TraitDeclaration => {
                todo!("trait declaration is not implemented now")
            }
            Declaration::ImplDeclaration => {
                todo!("impl declaration is not implemented now")
            }
        }
    }
}
