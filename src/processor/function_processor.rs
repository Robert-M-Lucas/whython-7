use std::collections::HashMap;
use crate::assembler::default::compile_user_function;
use crate::processor::custom_functions::{get_custom_function_implementations, get_custom_function_signatures};
use crate::processor::processor::ProcessorError;
use crate::processor::type_builder::{TypedFunction, TypeTable};


pub struct UserFunction {

}

impl UserFunction {

}

impl Function for UserFunction {
    fn get_asm(&self) -> String {
        compile_user_function(&self)
    }
}

pub trait Function {
    fn get_asm(&self) -> String;
}

pub fn process_functions(function_name_map: HashMap<Option<isize>, HashMap<String, isize>>, functions: HashMap<isize, TypedFunction>, type_table: TypeTable) -> Result<Vec<Box<dyn Function>>, ProcessorError> {
    let custom_functions = get_custom_function_signatures();
    let mut processed_functions = get_custom_function_implementations();

    Ok(processed_functions)
}