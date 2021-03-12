use wasm_encoder::{EntityType, Instruction};

use crate::{codegen::*, core::ast::*, syntax::Span};

impl<'a> Walker<FunctionDeclaration> for Context<'a> {
    fn walk(&mut self, function_declaration: FunctionDeclaration) -> Result<(), Error> {
        let parameters_type: Vec<_> = function_declaration
            .parameters
            .into_iter()
            .flat_map(|(_, ty)| self.convert_type(ty))
            .collect();
        let return_type = self.convert_type(function_declaration.return_type.clone());
        let id = self.declare_function_type(parameters_type, return_type);
        if function_declaration.is_extern {
            self.declare_extern_type(
                "extern",
                function_declaration.name.clone(),
                EntityType::Function(id),
            )?;
        } else {
            let mut body = Vec::new();
            std::mem::swap(&mut self.instructions, &mut body);
            for statement in function_declaration.body {
                self.walk(statement)?;
            }
            if let Some(last_expression) = function_declaration.last_expression {
                self.walk(last_expression)?;
                self.instructions.push(Instruction::End);
            }
            std::mem::swap(&mut self.instructions, &mut body);
            if !matches!(body.last(), Some(Instruction::End)) {
                if matches!(&function_declaration.return_type, Type::Tuple(v) if v.is_empty()) {
                    body.push(Instruction::I32Const(0));
                    body.push(Instruction::End);
                } else {
                    return Err(Error::mismatched_type(
                        function_declaration.return_type,
                        Type::Tuple(Vec::new()),
                        function_declaration.name.span,
                    ));
                }
            }
            self.implement_function(id, body);
        }
        if function_declaration.is_pub {
            self.export_function(function_declaration.name.content, id)
        }

        Ok(())
    }
}
