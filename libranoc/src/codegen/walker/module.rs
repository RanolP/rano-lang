use crate::{
    codegen::*,
    core::ast::{Module, Node},
};

pub fn walk_module(context: &mut Context, module: Module) {
    for node in module.nodes {
        match node {
            Node::Directive => {
                todo!("directive is not implemented now")
            }
            Node::Statement(statement) => {
                walk_statement(context, statement);
            }
        }
    }
}
