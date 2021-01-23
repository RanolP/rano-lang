use wasm_encoder::Instruction;

use crate::{codegen::*, core::ast::FunctionDeclaration};

pub fn walk_function_declaration(context: &mut Context, function_declaration: FunctionDeclaration) {
    let parameters_type: Vec<_> = function_declaration
        .parameters
        .into_iter()
        .flat_map(|(_, ty)| context.convert_type(ty))
        .collect();
    let return_type = context.convert_type(function_declaration.return_type);
    // TODO: instruction
    let body = vec![Instruction::I32Const(0), Instruction::End];
    let id = context.define_function(parameters_type, return_type, body);
    if function_declaration.is_pub {
        context.export_function(function_declaration.name, id)
    }
}
