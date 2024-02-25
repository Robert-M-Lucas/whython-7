mod call_function;
mod evaluate;
mod evaluate_symbol;
mod instantiate_literal;
mod operators;
mod process_lines;
mod reference;

use crate::basic_ast::symbol::{BasicSymbol, NameAccessType, NameType};
use crate::compiler::custom_functions::{
    get_custom_function_implementations, get_custom_function_signatures,
};
use crate::compiler::generate_asm::compile_user_function;
use crate::parser::line_info::LineInfo;
use crate::processor::processor::ProcessorError;
use crate::processor::type_builder::{Type, TypeTable, TypedFunction};
use either::{Either, Left, Right};
use std::collections::{HashMap, HashSet};

pub enum Line {
    ReturnCall(isize, Vec<(isize, usize)>, isize),
    NoReturnCall(isize, Vec<(isize, usize)>),
    Copy(isize, isize, usize),
    DynFromCopy(isize, isize, usize),
    Return(Option<isize>),
    InlineAsm(Vec<String>),
}

pub struct UserFunction {
    pub id: isize,
    pub name: String,
    pub local_variable_count: usize,
    pub arg_count: usize,
    pub lines: Vec<Line>,
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

pub struct FunctionHolder {
    functions: HashMap<isize, Box<dyn TypedFunction>>,
    functions_table: HashMap<Option<isize>, HashMap<String, isize>>,
}

impl FunctionHolder {
    pub fn new(
        functions: HashMap<isize, Box<dyn TypedFunction>>,
        functions_table: HashMap<Option<isize>, HashMap<String, isize>>,
    ) -> FunctionHolder {
        FunctionHolder {
            functions,
            functions_table,
        }
    }

    pub fn get_function(
        &self,
        _type: Option<(isize, usize)>,
        name: &str,
    ) -> Option<&Box<dyn TypedFunction>> {
        self.functions_table
            .get(&_type.and_then(|x| Some(x.0)))
            .and_then(|x| x.get(name).map(|x| self.functions.get(x).unwrap()))
    }

    pub fn functions(&self) -> &HashMap<isize, Box<dyn TypedFunction>> {
        &self.functions
    }

    pub fn functions_table(&self) -> &HashMap<Option<isize>, HashMap<String, isize>> {
        &self.functions_table
    }
}

pub struct NameHandler {
    type_table: TypeTable,
    args: Vec<(String, isize, (isize, usize))>,
    local_variables: Vec<(String, isize, (isize, usize))>,
    local_variables_size: usize,
    used_functions: HashSet<isize>,
    uid: usize,
}

impl NameHandler {
    pub fn new(type_table: TypeTable) -> NameHandler {
        NameHandler {
            type_table,
            args: Vec::new(),
            local_variables: Vec::new(),
            local_variables_size: 0,
            used_functions: HashSet::new(),
            uid: 0,
        }
    }

    pub fn set_args(&mut self, args: Vec<(String, isize, (isize, usize))>) {
        self.args = args
    }

    pub fn reset(&mut self) {
        self.uid = 0;
        self.args.clear();
        self.local_variables.clear();
        self.local_variables_size = 0;
    }

    pub fn get_uid(&mut self) -> usize {
        self.uid += 1;
        self.uid - 1
    }

    pub fn type_table(&self) -> &TypeTable {
        &self.type_table
    }

    pub fn local_variable_space(&self) -> usize {
        self.local_variables_size
    }

    pub fn add_local_variable(
        &mut self,
        name: Option<String>,
        _type: (isize, usize),
    ) -> Result<isize, ProcessorError> {
        let size = self
            .type_table
            .get_type_size(_type)?;
        let addr = -(self.local_variables_size as isize) - size as isize;
        self.local_variables_size += size;
        if let Some(name) = name {
            self.local_variables.push((name, addr, _type));
        }
        Ok(addr)
    }

    pub fn name_variable(&mut self, name: String, addr: isize, _type: (isize, usize)) {
        self.local_variables.push((name, addr, _type));
    }

    pub fn resolve_name<'b>(
        &self,
        function_holder: &'b FunctionHolder,
        name: &'b Vec<(String, NameAccessType, NameType, usize)>,
        line: &LineInfo,
    ) -> Result<
        Either<
            (isize, (isize, usize)),
            (
                &'b Box<dyn TypedFunction>,
                Option<(isize, (isize, usize))>,
                &'b Vec<Vec<(BasicSymbol, LineInfo)>>,
            ),
        >,
        ProcessorError,
    > {
        let mut current_type: Option<(isize, usize)> = None;
        let mut current_variable = None;
        let mut return_func = None;

        for (name, access_type, name_type, indirection) in name {
            if return_func.is_some() {
                // TODO
                return Err(ProcessorError::NotImplemented(line.clone(), "Using '.' or '#' after a function call".to_string()))
            }

            match name_type {
                NameType::Normal => {
                    if current_type.is_some() && current_variable.is_some() {
                        if current_type.unwrap().1 != 0 {
                            println!("{}", line);
                            todo!()
                        }
                        
                        let user_type = self.type_table.get_type(current_type.unwrap().0).unwrap().get_user_type()
                            .ok_or(ProcessorError::AttributeDoesntExist(
                                line.clone(),
                                self.type_table.get_type(current_type.unwrap().0).unwrap().get_name().to_string(),
                                name.clone()
                            ))?;

                        let t = user_type.get_attribute_offset_and_type(name, &self.type_table)?
                            .ok_or(ProcessorError::AttributeDoesntExist(
                                line.clone(),
                                self.type_table.get_type(current_type.unwrap().0).unwrap().get_name().to_string(),
                                name.clone()
                            ))?;

                        current_variable = Some(current_variable.unwrap() + (t.0 as isize));
                        current_type = Some(t.1);
                    }
                    else if current_type.is_some() {
                        return Err(ProcessorError::AttemptedTypeAttribAccess(line.clone()));
                    }
                    else if let Some((_, addr, _type)) = self
                        .local_variables
                        .iter()
                        .rev()
                        .chain(self.args.iter())
                        .find(|(n, _, _)| n == name)
                    {
                        // println!("{}, {}", addr, _type);
                        current_variable = Some(*addr);
                        current_type = Some(*_type);
                    } else if let Some(_type) = self.type_table.get_id_by_name(name) {
                        current_variable = None;
                        current_type = Some((_type, *indirection));
                    } else {
                        return Err(ProcessorError::NameNotFound(line.clone(), name.clone()));
                    }
                }
                NameType::Function(contents) => {
                    if let Some(func) = function_holder
                        .functions_table()
                        .get(&current_type.and_then(|x| Some(x.0)))
                        .unwrap()
                        .get(name)
                    {
                        let default_arg = if matches!(access_type, NameAccessType::Normal) {
                            if current_variable.is_none() {
                                return Err(ProcessorError::TypeNonStaticFunctionCall(line.clone()));
                            }
                            Some((current_variable.unwrap(), current_type.unwrap()))
                        } else {
                            None
                        };
                        return_func = Some((
                            function_holder.functions().get(func).unwrap(),
                            default_arg,
                            contents,
                        ));
                    }
                }
            }
        }

        if let Some(return_func) = return_func {
            return Ok(Right(return_func));
        }

        Ok(Left((
            current_variable.ok_or(ProcessorError::StandaloneType(line.clone()))?,
            current_type.unwrap(),
        )))
    }

    pub fn use_function_id(&mut self, id: isize) {
        self.used_functions.insert(id);
    }

    pub fn use_function(&mut self, func: &Box<dyn TypedFunction>) {
        if !func.is_inline() {
            self.used_functions.insert(func.get_id());
        }
    }

    pub fn used_functions(&self) -> &HashSet<isize> {
        &self.used_functions
    }
}

pub fn compile_functions(
    mut function_name_map: HashMap<Option<isize>, HashMap<String, isize>>,
    mut functions: HashMap<isize, Box<dyn TypedFunction>>,
    type_table: TypeTable,
) -> Result<Vec<Box<dyn Function>>, ProcessorError> {
    let mut function_contents: HashMap<isize, Vec<(BasicSymbol, LineInfo)>> = HashMap::new();
    for (id, func) in &mut functions {
        function_contents.insert(*id, func.take_contents());
    }
    for (t, f) in get_custom_function_signatures() {
        if function_name_map.get_mut(&t).is_none() {
            function_name_map.insert(t, HashMap::new());
        }
        function_name_map
            .get_mut(&t)
            .unwrap()
            .insert(f.get_name().to_string(), f.get_id());
        functions.insert(f.get_id(), f);
    }

    let function_holder = FunctionHolder::new(functions, function_name_map);
    let mut name_handler = NameHandler::new(type_table);
    let mut processed_functions = get_custom_function_implementations();
    name_handler.use_function_id(0);

    for (id, contents) in function_contents {
        let function = function_holder.functions.get(&id).unwrap();
        let name = function.get_name().to_string();
        name_handler.reset();
        name_handler.set_args(function.get_args_positioned(name_handler.type_table()));
        let return_type = function.get_return_type();
        let mut lines = Vec::new();

        let last_return = process_lines::process_lines(
            &contents,
            id,
            return_type,
            &mut lines,
            &mut name_handler,
            &function_holder,
            None,
        )?;

        if return_type.is_some() && !last_return {
            return Err(ProcessorError::NoReturnStatement(function.get_line()));
        }

        processed_functions.push(Box::new(UserFunction {
            id,
            local_variable_count: name_handler.local_variable_space() / 8,
            arg_count: function_holder
                .functions()
                .get(&id)
                .unwrap()
                .get_args()
                .len(),
            lines,
            name
        }));
    }

    let processed_functions = processed_functions
        .into_iter()
        .filter(|f| name_handler.used_functions().contains(&f.get_id()))
        .collect();
    Ok(processed_functions)
}
