use crate::ast::literals::Literal;
use crate::compiler::default::get_local_address;
use crate::processor::processor::ProcessorError;
use crate::processor::type_builder::{Type, TypeTable};

pub struct Bool {}

impl Bool {
    pub fn new() -> Bool {
        Bool {}
    }
    pub fn get_id() -> isize { -2 }
}

impl Type for Bool {
    fn get_id(&self) -> isize {
        Self::get_id()
    }

    fn get_name(&self) -> &str {
        "bool"
    }

    fn get_size(&self, type_table: &TypeTable, path: Option<Vec<isize>>) -> Result<usize, ProcessorError> {
        Ok(8)
    }

    fn instantiate(&self, literal: Option<&Literal>, local_address: isize) -> Result<Vec<String>, ProcessorError> {
        if literal.is_none() { return Ok(vec![]); }
        let Literal::Bool(val) = literal.unwrap() else { panic!() };

        if *val {
            Ok(vec![
                format!("mov qword [{}], 0", get_local_address(local_address))
            ])
        }
        else {
            Ok(vec![
                format!("mov qword [{}], -1", get_local_address(local_address))
            ])
        }
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

    fn instantiate(&self, literal: Option<&Literal>, local_address: isize) -> Result<Vec<String>, ProcessorError> {
        if literal.is_none() { return Ok(vec![]); }
        let Literal::Int(val) = literal.unwrap() else { panic!() };

        Ok(vec![
            format!("mov qword [{}], {}", get_local_address(local_address), *val)
        ])
    }
}