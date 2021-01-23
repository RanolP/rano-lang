use crate::{codegen::*, core::ast::Declaration};

mod function_declaration;

use function_declaration::*;

pub fn walk_declaration(context: &mut Context, declaration: Declaration) {
    match declaration {
        Declaration::FunctionDeclaration(function_declaration) => {
            walk_function_declaration(context, function_declaration);
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
