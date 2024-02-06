use crate::ast::literals::Literal;
use crate::processor::processor::ProcessorError;
use crate::processor::type_builder::{Type, TypeTable};

pub struct Bool {}

impl Bool {
    pub fn new() -> Bool {
        Bool {}
    }
}

impl Type for Bool {
    fn get_id(&self) -> isize {
        -2
    }

    fn get_name(&self) -> &str {
        "bool"
    }

    fn get_size(&self, type_table: &TypeTable, path: Option<Vec<isize>>) -> Result<usize, ProcessorError> {
        Ok(8)
    }

    fn instantiate(&self, literal: Option<Literal>, local_address: isize) -> Result<Vec<String>, ProcessorError> {
        todo!()
    }
}

pub struct Int {}

impl Int {
    pub fn new() -> Int {
        Int {}
    }

    pub const fn get_id() -> isize { -1 }
}

impl Type for Int {
    fn get_id(&self) -> isize {
        Int::get_id()
    }

    fn get_name(&self) -> &str {
        "int"
    }

    fn get_size(&self, type_table: &TypeTable, path: Option<Vec<isize>>) -> Result<usize, ProcessorError> {
        Ok(8)
    }

    fn instantiate(&self, literal: Option<Literal>, local_address: isize) -> Result<Vec<String>, ProcessorError> {
        todo!()
    }
}