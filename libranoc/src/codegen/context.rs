use std::collections::{HashMap, VecDeque};

use wasm_encoder::{
    CodeSection, DataSection, EntityType, Export, ExportSection, Function, FunctionSection,
    GlobalType, ImportSection, Instruction, Limits, MemoryType, Module, TableType, TypeSection,
    ValType,
};

use crate::{
    core::{ast::Type, Error},
    syntax::{Span, Token},
};

pub struct Context<'a> {
    import_section: ImportSection,
    import_index_function: u32,
    import_index_table: u32,
    import_index_memory: u32,
    import_index_global: u32,
    import_index_instance: u32,
    import_index_module: u32,
    import_extern_type_map: HashMap<String, HashMap<String, (Span, EntityType)>>,

    imports: HashMap<String, u32>,
    locals: HashMap<String, VecDeque<u32>>,

    type_section: TypeSection,
    type_section_last_id: u32,

    function_section: FunctionSection,

    code_section: CodeSection,

    export_section: ExportSection,

    data_section: DataSection,
    data_segment_last_id: u32,
    data_segment_last_offset: i32,

    pub instructions: Vec<Instruction<'a>>,

    compilation_errors: Vec<Error>,
}

impl<'a> Context<'a> {
    pub fn new() -> Self {
        Context {
            import_section: ImportSection::new(),
            import_index_function: 0,
            import_index_table: 0,
            import_index_memory: 0,
            import_index_global: 0,
            import_index_instance: 0,
            import_index_module: 0,
            import_extern_type_map: HashMap::new(),

            imports: HashMap::new(),
            locals: HashMap::new(),

            type_section: TypeSection::new(),
            type_section_last_id: 0,

            function_section: FunctionSection::new(),

            export_section: ExportSection::new(),

            code_section: CodeSection::new(),

            data_section: DataSection::new(),
            data_segment_last_id: 0,
            data_segment_last_offset: 0,

            instructions: Vec::new(),

            compilation_errors: Vec::new(),
        }
    }

    pub fn finish(self) -> (Vec<u8>, Vec<Error>) {
        let mut module = Module::new();
        module.section(&self.type_section);
        module.section(&self.import_section);
        module.section(&self.function_section);
        module.section(&self.export_section);
        module.section(&self.code_section);
        module.section(&self.data_section);
        (module.finish(), self.compilation_errors)
    }

    pub fn convert_type(&mut self, _ty: Type) -> Vec<ValType> {
        vec![ValType::I32]
    }

    pub fn declare_extern_type(
        &mut self,
        module: impl Into<String>,
        name: Token,
        ty: EntityType,
    ) -> Result<(), Error> {
        let module = self
            .import_extern_type_map
            .entry(module.into())
            .or_default();
        if let Some((before_span, _)) = module.get(&name.content) {
            return Err(Error::redefined(
                name.content,
                before_span.clone(),
                name.span,
            ));
        } else {
            module.insert(name.content, (name.span, ty));
        }

        Ok(())
    }

    pub fn declare_function_type(
        &mut self,
        parameters_type: Vec<ValType>,
        return_type: Vec<ValType>,
    ) -> u32 {
        self.type_section.function(parameters_type, return_type);
        let result = self.type_section_last_id;
        self.type_section_last_id += 1;

        result
    }

    pub fn implement_function(&mut self, function_type_id: u32, body: Vec<Instruction>) {
        self.function_section.function(function_type_id);
        // TODO: local variables....
        let locals = vec![];
        let mut function = Function::new(locals);
        for instruction in body {
            function.instruction(instruction);
        }
        self.code_section.function(&function);
    }

    pub fn export_function<S: AsRef<str>>(&mut self, name: S, id: u32) {
        self.export_section
            .export(name.as_ref(), Export::Function(id));
    }

    pub fn set_local(&mut self, name: String, id: u32) {
        self.locals
            .entry(name)
            .or_insert(VecDeque::new())
            .push_back(id);
    }

    pub fn get_local(&mut self, name: &String, span: Span) -> Result<u32, Error> {
        self.locals
            .get(name)
            .and_then(|deque| deque.back())
            .cloned()
            .ok_or_else(|| Error::undefined_symbol(name, span))
    }

    pub fn remove_local(&mut self, name: &String) {
        if let Some(deque) = self.locals.get_mut(name) {
            deque.pop_back();
        }
    }

    pub fn resolve(&mut self, name: &String, span: Span) -> Result<u32, Error> {
        self.get_local(name, span.clone())
            .or(self.import("extern", name, span.clone()))
    }

    pub fn import(&mut self, module: &str, name: &String, span: Span) -> Result<u32, Error> {
        if let Some(id) = self.imports.get(name) {
            return Ok(*id);
        }
        let (_, ty) = self
            .import_extern_type_map
            .get(&module.to_owned())
            .and_then(|module| module.get(name))
            .ok_or_else(|| Error::undefined_symbol(name, span))?;

        let (counter, ty) = match &ty {
            EntityType::Function(id) => (
                &mut self.import_index_function,
                EntityType::Function(id.clone()),
            ),
            EntityType::Table(table) => (
                &mut self.import_index_table,
                EntityType::Table(TableType {
                    element_type: table.element_type.clone(),
                    limits: Limits {
                        min: table.limits.min.clone(),
                        max: table.limits.max.clone(),
                    },
                }),
            ),
            EntityType::Memory(memory) => (
                &mut self.import_index_memory,
                EntityType::Memory(MemoryType {
                    limits: Limits {
                        min: memory.limits.min.clone(),
                        max: memory.limits.max.clone(),
                    },
                }),
            ),
            EntityType::Global(global) => (
                &mut self.import_index_global,
                EntityType::Global(GlobalType {
                    val_type: global.val_type.clone(),
                    mutable: global.mutable.clone(),
                }),
            ),
            EntityType::Instance(id) => (
                &mut self.import_index_instance,
                EntityType::Instance(id.clone()),
            ),
            EntityType::Module(id) => (
                &mut self.import_index_module,
                EntityType::Module(id.clone()),
            ),
        };
        let result = *counter;
        *counter += 1;

        self.import_section.import(module, Some(&name), ty);

        self.imports.insert(name.clone(), result);

        Ok(result)
    }

    pub fn create_data<D>(&mut self, data: D)
    where
        D: IntoIterator<Item = u8>,
        D::IntoIter: ExactSizeIterator,
    {
        let data = data.into_iter();
        let id = self.data_segment_last_id;
        self.data_segment_last_id += 1;
        let offset = Instruction::I32Const(self.data_segment_last_offset);
        self.data_segment_last_offset += data.len() as i32;

        self.data_section.active(id, offset, data);
    }

    pub fn add_compilation_error(&mut self, error: Error) {
        self.compilation_errors.push(error);
    }
}
