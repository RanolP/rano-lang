use wasm_encoder::{
    CodeSection, Export, ExportSection, Function, FunctionSection, Instruction, Module,
    TypeSection, ValType,
};

use crate::core::ast::Type;

pub struct Context {
    type_section: TypeSection,
    type_section_last_id: u32,
    function_section: FunctionSection,
    code_section: CodeSection,
    export_section: ExportSection,
}

impl Context {
    pub fn new() -> Self {
        Context {
            export_section: ExportSection::new(),
            type_section: TypeSection::new(),
            type_section_last_id: 0,
            function_section: FunctionSection::new(),
            code_section: CodeSection::new(),
        }
    }

    pub fn finish(&self) -> Vec<u8> {
        let mut module = Module::new();
        module.section(&self.type_section);
        module.section(&self.function_section);
        module.section(&self.export_section);
        module.section(&self.code_section);
        module.finish()
    }

    pub fn convert_type(&mut self, _ty: Type) -> Vec<ValType> {
        vec![ValType::I32]
    }

    pub fn define_function(
        &mut self,
        parameters_type: Vec<ValType>,
        return_type: Vec<ValType>,
        body: Vec<Instruction>,
    ) -> u32 {
        self.type_section.function(parameters_type, return_type);
        let result = self.type_section_last_id;
        self.function_section.function(result);
        self.type_section_last_id += 1;
        // TODO: local variables....
        let locals = vec![];
        let mut function = Function::new(locals);
        for instruction in body {
            function.instruction(instruction);
        }
        self.code_section.function(&function);

        result
    }

    pub fn export_function<S: AsRef<str>>(&mut self, name: S, id: u32) {
        self.export_section
            .export(name.as_ref(), Export::Function(id));
    }
}
