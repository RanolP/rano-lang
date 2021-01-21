use wasm_encoder::{FunctionSection, TypeSection, ValType};

use crate::{codegen::*, core::ast::Declaration};

pub fn walk_declaration(context: &mut Context, declaration: Declaration) {
    match declaration {
        Declaration::FunctionDeclaration(function_declaration) => {
            /*
            Section을 여기서 만들면 안될 것 같음
            let mut type_section = TypeSection::new();
            let parameters_type: Vec<ValType> = function_declaration
                .parameters
                .iter()
                .map(|(_, ty)| {
                    todo!("params type conversion in function declaration is not implemented now");
                })
                .collect();
            let return_type = {
                // TODO: return type conversion
                function_declaration.return_type;

                vec![ValType::I32]
            };
            type_section.function(parameters_type, return_type);
            let function_section = FunctionSection::new();
            function_section.function(0);
            */
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
