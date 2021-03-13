use wasm_encoder::{EntityType, Instruction};

use crate::{codegen::*, core::ast::*};

impl<'a> Walker<FunctionDeclaration> for Context<'a> {
    fn walk(&mut self, function_declaration: FunctionDeclaration) -> Result<(), Error> {
        let parameters_type: Vec<_> = function_declaration
            .parameters
            .iter()
            .cloned()
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
            self.set_local(function_declaration.name.content.clone(), id);

            let mut param_id = 0;
            for (pat, _ty) in &function_declaration.parameters {
                match pat {
                    Pattern::Slot(name) => match name {
                        Name::Ident(token) => {
                            self.set_local(token.content.clone(), param_id);
                        }
                        Name::Placeholder => {}
                    },
                }
                param_id += 1;
            }

            let mut body = Vec::new();
            std::mem::swap(&mut self.instructions, &mut body);
            if let Some(body) = function_declaration.body.clone() {
                self.walk(body)?;
            }
            std::mem::swap(&mut self.instructions, &mut body);
            if function_declaration
                .body
                .and_then(|block| block.last_expression)
                .is_none()
            {
                if matches!(&function_declaration.return_type, Type::Tuple(v) if v.is_empty()) {
                    body.push(Instruction::I32Const(0));
                } else {
                    return Err(Error::mismatched_type(
                        function_declaration.return_type,
                        Type::Tuple(Vec::new()),
                        function_declaration.name.span,
                    ));
                }
            }
            body.push(Instruction::End);

            dbg!(&body);

            for (pat, _ty) in &function_declaration.parameters {
                match pat {
                    Pattern::Slot(name) => match name {
                        Name::Ident(token) => {
                            self.remove_local(&token.content);
                        }
                        Name::Placeholder => {}
                    },
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
