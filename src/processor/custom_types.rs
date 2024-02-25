use crate::ast::literals::Literal;
use crate::compiler::generate_asm::get_local_address;
use crate::parser::line_info::LineInfo;
use crate::processor::processor::ProcessorError;
use crate::processor::type_builder::{Type, TypeTable};

pub struct Bool {}

impl Bool {
    pub fn new() -> Bool {
        Bool {}
    }
    pub fn get_id() -> isize {
        -2
    }
}

impl Type for Bool {
    fn get_id(&self) -> isize {
        Self::get_id()
    }

    fn get_name(&self) -> &str {
        "bool"
    }

    fn get_size(
        &self,
        _type_table: &TypeTable,
        _path: Option<Vec<isize>>,
    ) -> Result<usize, ProcessorError> {
        Ok(8)
    }

    fn instantiate(
        &self,
        literal: Option<&Literal>,
        local_address: isize,
    ) -> Result<Vec<String>, ProcessorError> {
        if literal.is_none() {
            return Ok(vec![]);
        }
        let Literal::Bool(val) = literal.unwrap() else {
            panic!()
        };

        if *val {
            Ok(vec![format!(
                "mov qword [{}], 0",
                get_local_address(local_address)
            )])
        } else {
            Ok(vec![format!(
                "mov qword [{}], 1",
                get_local_address(local_address)
            )])
        }
    }
}

pub struct Int {}

impl Int {
    pub fn new() -> Int {
        Int {}
    }

    pub const fn get_id() -> isize {
        -1
    }
}

impl Int {
    pub fn instantiate_ref(
        offset: isize,
        local_address: isize,
    ) -> Vec<String> {
        
        vec![
            "mov rax, rbp".to_string(),
            format!("add rax, {offset}"),
            format!(
                "mov qword [{}], rax",
                get_local_address(local_address),
        )]
    }
}

impl Type for Int {
    fn get_id(&self) -> isize {
        Int::get_id()
    }

    fn get_name(&self) -> &str {
        "int"
    }

    fn get_size(
        &self,
        _type_table: &TypeTable,
        _path: Option<Vec<isize>>,
    ) -> Result<usize, ProcessorError> {
        Ok(8)
    }

    fn instantiate(
        &self,
        literal: Option<&Literal>,
        local_address: isize,
    ) -> Result<Vec<String>, ProcessorError> {
        if literal.is_none() {
            return Ok(vec![]);
        }
        let Literal::Int(val) = literal.unwrap() else {
            panic!()
        };

        Ok(vec![
            format!("mov rax, qword {}", *val),
            format!(
            "mov qword [{}], rax",
            get_local_address(local_address),
        )])
    }
}
