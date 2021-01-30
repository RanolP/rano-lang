use wasm_encoder::{EntityType, Instruction};

use crate::{
    codegen::*,
    core::ast::FunctionDeclaration,
    syntax::{tokenize, Token, TokenKind},
};

pub fn walk_function_declaration(
    context: &mut Context,
    function_declaration: FunctionDeclaration,
) -> Result<(), Error> {
    let parameters_type: Vec<_> = function_declaration
        .parameters
        .into_iter()
        .flat_map(|(_, ty)| context.convert_type(ty))
        .collect();
    let return_type = context.convert_type(function_declaration.return_type);
    let id = context.declare_function_type(parameters_type, return_type);
    if function_declaration.is_extern {
        context.declare_extern_type(
            "extern",
            function_declaration.name.0.clone(),
            EntityType::Function(id),
        )?;
    } else {
        // TODO: instruction
        let add_id = context.import("extern", &tokenize("add")[0])?;
        let show_id = context.import("extern", &tokenize("show")[0])?;
        //  let id = context.create_data(b"Hello, world!".iter().copied());
        let body = vec![
            Instruction::I32Const(40),
            Instruction::I32Const(2),
            Instruction::Call(add_id),
            Instruction::Call(show_id),
            Instruction::Drop,
            Instruction::I32Const(0),
            Instruction::End,
        ];
        context.implement_function(id, body);
    }
    if function_declaration.is_pub {
        context.export_function(function_declaration.name.1, id)
    }

    Ok(())
}
