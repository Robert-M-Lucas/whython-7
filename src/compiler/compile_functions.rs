use std::collections::{HashMap, HashSet};
use std::slice::Iter;
use either::{Either, Left, Right};
use crate::ast::keywords::Keyword;
use crate::ast::literals::Literal;
use crate::ast::operators::Operator;
use crate::basic_ast::punctuation::Punctuation;
use crate::basic_ast::symbol::{BasicSymbol, NameAccessType, NameType};
use crate::compiler::custom_functions::{get_custom_function_implementations, get_custom_function_signatures};
use crate::compiler::default::compile_user_function;
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

pub struct NameHandler<'a> {
    functions: &'a HashMap<isize, Box<dyn TypedFunction>>,
    functions_table: HashMap<Option<isize>, HashMap<String, isize>>,
    type_table: TypeTable,
    args: Vec<(String, isize, isize)>,
    local_variables: Vec<(String, isize, isize)>,
    local_variables_size: usize
}

impl<'a> NameHandler<'a> {
    pub fn new(functions: &'a HashMap<isize, Box<dyn TypedFunction>>,
               functions_table: HashMap<Option<isize>, HashMap<String, isize>>,
               type_table: TypeTable) -> NameHandler {
        NameHandler {
            functions,
            functions_table,
            type_table,
            args: Vec::new(),
            local_variables: Vec::new(),
            local_variables_size: 0,
        }
    }

    pub fn reset(&mut self) {
        self.args.clear();
        self.local_variables.clear();
        self.local_variables_size = 0;
    }

    pub fn type_table(&self) -> &TypeTable {
        &self.type_table
    }

    pub fn get_function(&self, _type: Option<isize>, name: &str) -> Option<&Box<dyn TypedFunction>> {
        self.functions_table.get(&_type).and_then(
            |x| x.get(name).and_then(
                |x| Some(self.functions.get(x).unwrap())
        ))
    }

    pub fn functions(&self) -> &HashMap<isize, Box<dyn TypedFunction>> {
        &self.functions
    }

    pub fn local_variable_space(&self) -> usize {
        self.local_variables_size
    }

    pub fn add_local_variable(&mut self, name: Option<String>, _type: isize) -> isize {
        let size = self.type_table.get_type(_type).unwrap().get_size(&self.type_table, None).unwrap();
        let addr = -(self.local_variables_size as isize) - size as isize;
        if let Some(name) = name {
            self.local_variables.push((name, addr, _type));
        }
        addr
    }

    pub fn name_variable(&mut self, name: String, addr: isize, _type: isize) {
        self.local_variables.push((name, addr, _type));
    }

    pub fn resolve_name<'b>(&self, name: &'b Vec<(String, NameAccessType, NameType)>) -> Result<Either<(isize, isize), (&Box<dyn TypedFunction>, Option<isize>, &'b Vec<Vec<BasicSymbol>>)>, ProcessorError> {
        let mut current_type = None;
        let mut current_variable = None;
        let mut return_func = None;

        for (name, access_type, name_type) in name {
            if return_func.is_some() {
                todo!()
            }

            match name_type {
                NameType::Normal => {
                    if current_type != None || current_variable != None {
                        todo!()
                    }
                    if let Some((_, addr, _type)) = self.local_variables.iter().find(|(n, _, _)| n == name) {
                        current_variable = Some(*addr);
                        current_type = Some(*_type);
                    }
                    else {
                        if let Some(_type) = self.type_table.get_id_by_name(&name) {
                            current_variable = None;
                            current_type = Some(_type);
                        }
                        else {
                            return Err(ProcessorError::NameNotFound(name.clone()));
                        }
                    }
                }
                NameType::Function(contents) => {
                    if let Some(func) = self.functions_table.get(&current_type).unwrap().get(name) {
                        let default_arg = if matches!(access_type, NameAccessType::Normal) {
                            current_type
                        }
                        else {
                            None
                        };
                        return_func = Some((self.functions.get(func).unwrap(), default_arg, contents));
                    }
                }
            }
        }

        if let Some(return_func) = return_func {
            return Ok(Right(return_func));
        }

        Ok(Left((
            current_type.unwrap(),
            current_variable.ok_or(ProcessorError::StandaloneType)?
        )))
    }
}


pub fn compile_functions(mut function_name_map: HashMap<Option<isize>, HashMap<String, isize>>, mut functions: HashMap<isize, Box<dyn TypedFunction>>, type_table: TypeTable) -> Result<Vec<Box<dyn Function>>, ProcessorError> {
    for (t, f) in get_custom_function_signatures() {
        if function_name_map.get(&t).unwrap().contains_key(f.get_name()) {
            continue;
        }
        function_name_map.get_mut(&t).unwrap().insert(f.get_name().to_string(), f.get_id());
        functions.insert(f.get_id(), f);
    }
    let function_ids: Vec<isize> = functions.keys().map(|x| *x).collect();
    let functions = functions;

    let mut processed_functions = get_custom_function_implementations();
    let mut used_functions = HashSet::new();
    used_functions.insert(0);

    for id in function_ids {
        todo!();

        processed_functions.push(Box::new(UserFunction {
            id,
            local_variable_count: name_handler.local_variable_space() / 8,
            arg_count: function.get_args().len(),
            lines,
        }));
    }

    let processed_functions = processed_functions.into_iter().filter(|f| used_functions.contains(&f.get_id())).collect();
    Ok(processed_functions)
}


fn evaluate<'a>(first_symbol: &'a BasicSymbol, symbol_iter: &'a mut Iter<(&'a BasicSymbol, usize)>,
                must_complete: bool, lines: &mut Vec<Line>, name_handler: &mut NameHandler)
                -> Result<Either<(isize, isize), Literal>, ProcessorError> { // addr, type
    todo!()
}

fn evaluate_symbol(symbol: &BasicSymbol, lines: &mut Vec<Line>, name_handler: &mut NameHandler) -> Result<Option<Either<Either<(isize, isize), Literal>, Operator>>, ProcessorError> {
    todo!()
}

fn try_instantiate_literal(literal: Either<(isize, isize), Literal>, lines: &mut Vec<Line>, name_handler: &mut NameHandler
) -> Result<(isize, isize), ProcessorError> {
    match literal {
        Left(r) => Ok(r),
        Right(literal) => instantiate_literal(Left(literal), lines, name_handler)
    }
}

fn instantiate_literal(literal: Either<Literal, isize>, lines: &mut Vec<Line>, name_handler: &mut NameHandler
) -> Result<(isize, isize), ProcessorError> {
    let id = match &literal {
        Left(literal) => literal.get_type_id(),
        Right(id) => *id
    };
    let addr = name_handler.add_local_variable(None, id);
    let _type = name_handler.type_table().get_type(id).unwrap();
    let asm = match literal {
        Left(literal) => _type.instantiate(Some(literal), addr)?,
        Right(_id) => _type.instantiate(None, addr)?
    };
    lines.push(Line::InlineAsm(asm));
    Ok((addr, id))
}

fn evaluate_operation(lhs: Either<(isize, isize), Literal>, op: Operator, rhs: Option<Either<(isize, isize), Literal>>,
                      lines: &mut Vec<Line>, name_handler: &mut NameHandler)
    -> Result<(isize, isize), ProcessorError> {
    todo!()
}