use std::collections::{HashMap, HashSet};
use crate::assembler::default::compile_user_function;
use crate::processor::custom_functions::{get_custom_function_implementations, get_custom_function_signatures};
use crate::processor::processor::ProcessorError;
use crate::processor::type_builder::{TypedFunction, TypeTable};



pub enum Line {
    ReturnCall(isize, Vec<(isize, usize)>, isize),
    NoReturnCall(isize, Vec<(isize, usize)>),
    Copy(isize, isize),
    Return(isize),
    InlineAsm(Vec<String>),
}

pub struct UserFunction {
    pub id: isize,
    pub local_variable_count: usize,
    pub arg_count: usize,
    pub lines: Vec<Line>
}

impl Function for UserFunction {
    fn get_asm(&self) -> String {
        compile_user_function(self)
    }

    fn get_id(&self) -> isize {
        self.id
    }
}

pub trait Function {
    fn get_asm(&self) -> String;
    fn get_id(&self) -> isize;
}

pub fn process_functions(mut function_name_map: HashMap<Option<isize>, HashMap<String, isize>>, mut functions: HashMap<isize, Box<dyn TypedFunction>>, type_table: TypeTable) -> Result<Vec<Box<dyn Function>>, ProcessorError> {
    for (t, f) in get_custom_function_signatures() {
        if function_name_map.get(&t).unwrap().contains_key(f.get_name()) {
            continue;
        }
        function_name_map.get_mut(&t).unwrap().insert(f.get_name().to_string(), f.get_id());
        functions.insert(f.get_id(), f);
    }
    let mut processed_functions = get_custom_function_implementations();
    let mut used_functions = HashSet::new();
    used_functions.insert(0);

    for id in functions.keys() {
        let function = functions.get(id).unwrap();
        if function.is_inline() { continue; }

        let local_variable_space = 0;

        let mut lines = Vec::new();
        for line in function.get_contents() {

        }

        processed_functions.push(Box::new(UserFunction {
            id: *id,
            local_variable_count: local_variable_space / 8,
            arg_count: function.get_args().len(),
            lines,
        }));
    }

    let processed_functions = processed_functions.into_iter().filter(|f| used_functions.contains(&f.get_id())).collect();
    Ok(processed_functions)
}