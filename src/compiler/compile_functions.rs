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

pub struct FunctionHolder {
    functions: HashMap<isize, Box<dyn TypedFunction>>,
    functions_table: HashMap<Option<isize>, HashMap<String, isize>>,
}

impl FunctionHolder {
    pub fn new(functions: HashMap<isize, Box<dyn TypedFunction>>,
               functions_table: HashMap<Option<isize>, HashMap<String, isize>>,) -> FunctionHolder {
        FunctionHolder {
            functions,
            functions_table
        }
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

    pub fn functions_table(&self) -> &HashMap<Option<isize>, HashMap<String, isize>> {
        &self.functions_table
    }
}

pub struct NameHandler {
    type_table: TypeTable,
    args: Vec<(String, isize, isize)>,
    local_variables: Vec<(String, isize, isize)>,
    local_variables_size: usize
}

impl NameHandler {
    pub fn new(type_table: TypeTable) -> NameHandler {
        NameHandler {
            type_table,
            args: Vec::new(),
            local_variables: Vec::new(),
            local_variables_size: 0,
        }
    }

    pub fn set_args(&mut self, args: Vec<(String, isize, isize)>) {
        self.args = args
    }

    pub fn reset(&mut self) {
        self.args.clear();
        self.local_variables.clear();
        self.local_variables_size = 0;
    }

    pub fn type_table(&self) -> &TypeTable {
        &self.type_table
    }

    pub fn local_variable_space(&self) -> usize {
        self.local_variables_size
    }

    pub fn add_local_variable(&mut self, name: Option<String>, _type: isize) -> isize {
        let size = self.type_table.get_type(_type).unwrap().get_size(&self.type_table, None).unwrap();
        let addr = -(self.local_variables_size as isize) - size as isize;
        self.local_variables_size += size;
        if let Some(name) = name {
            self.local_variables.push((name, addr, _type));
        }
        addr
    }

    pub fn name_variable(&mut self, name: String, addr: isize, _type: isize) {
        self.local_variables.push((name, addr, _type));
    }

    pub fn resolve_name<'b>(&self, function_holder: &'b FunctionHolder, name: &'b Vec<(String, NameAccessType, NameType)>) -> Result<Either<(isize, isize), (&'b Box<dyn TypedFunction>, Option<(isize, isize)>, &'b Vec<Vec<BasicSymbol>>)>, ProcessorError> {
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
                    if let Some((_, addr, _type)) = self.local_variables.iter().chain(self.args.iter()).find(|(n, _, _)| n == name) {
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
                    if let Some(func) = function_holder.functions_table().get(&current_type).unwrap().get(name) {
                        let default_arg = if matches!(access_type, NameAccessType::Normal) {
                            Some((current_variable.unwrap(), current_type.unwrap()))
                        }
                        else {
                            None
                        };
                        return_func = Some((function_holder.functions().get(func).unwrap(), default_arg, contents));
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
    let mut function_contents: HashMap<isize, Vec<BasicSymbol>> = HashMap::new();
    for (id, func) in &mut functions {
        function_contents.insert(*id, func.take_contents());
    }
    for (t, f) in get_custom_function_signatures() {
        if function_name_map.get(&t).unwrap().contains_key(f.get_name()) {
            continue;
        }
        function_name_map.get_mut(&t).unwrap().insert(f.get_name().to_string(), f.get_id());
        functions.insert(f.get_id(), f);
    }

    let function_holder = FunctionHolder::new(functions, function_name_map);
    let mut name_handler = NameHandler::new(type_table);
    let mut processed_functions = get_custom_function_implementations();
    let mut used_functions = HashSet::new();
    used_functions.insert(0);

    for (id, contents) in function_contents {
        name_handler.reset();
        name_handler.set_args(function_holder.functions.get(&id).unwrap().get_args_positioned(name_handler.type_table()));
        let mut lines = Vec::new();

        for line in contents.split(|x| matches!(x, BasicSymbol::Punctuation(Punctuation::Semicolon))) {
            if line.len() == 0 { continue; }
            evaluate(line, &mut lines, &mut name_handler, &function_holder, None)?;
        }

        processed_functions.push(Box::new(UserFunction {
            id,
            local_variable_count: name_handler.local_variable_space() / 8,
            arg_count: function_holder.functions().get(&id).unwrap().get_args().len(),
            lines,
        }));
    }

    let processed_functions = processed_functions.into_iter().filter(|f| used_functions.contains(&f.get_id())).collect();
    Ok(processed_functions)
}


fn evaluate<'a>(section: &[BasicSymbol], lines: &mut Vec<Line>, name_handler: &mut NameHandler, function_holder: &FunctionHolder,
    return_into: Option<(isize, isize)>
    )-> Result<Option<(isize, isize)>, ProcessorError> { // addr, type
    Ok(if section.len() == 1 {
        evaluate_symbol(&section[0], lines, name_handler, function_holder, return_into)?
    }
    else if section.len() == 2 {
        let op = evaluate_operator(&section[0])?;
        let Some(value) = evaluate_symbol(&section[1], lines, name_handler, function_holder, None)?
        else { return Err(ProcessorError::BadEvaluableLayout); };
        evaluate_operation(value, op, None, lines, name_handler, function_holder, return_into)?
    }
    else if section.len() == 3 {
        let Some(lhs) = evaluate_symbol(&section[0], lines, name_handler, function_holder, None)?
            else { return Err(ProcessorError::BadEvaluableLayout); };
        let op = evaluate_operator(&section[1])?;
        let Some(rhs) = evaluate_symbol(&section[2], lines, name_handler, function_holder, None)?
            else { return Err(ProcessorError::BadEvaluableLayout); };
        evaluate_operation(lhs, op, Some(rhs), lines, name_handler, function_holder, return_into)?
    }
    else {
        return Err(ProcessorError::BadEvaluableLayout);
    })
}

fn evaluate_symbol(symbol: &BasicSymbol, lines: &mut Vec<Line>, name_handler: &mut NameHandler, function_holder: &FunctionHolder,
    return_into: Option<(isize, isize)>) -> Result<Option<(isize, isize)>, ProcessorError> {
    Ok(match symbol {
        BasicSymbol::AbstractSyntaxTree(_) => panic!(),
        BasicSymbol::Operator(_) => return Err(ProcessorError::BadOperatorPosition),
        BasicSymbol::Literal(literal) => {
            Some(instantiate_literal(Left(literal), lines, name_handler, function_holder, return_into)?)
        }
        BasicSymbol::BracketedSection(inner) => {
            evaluate(inner, lines, name_handler, function_holder, return_into)?
        }
        BasicSymbol::Name(name) => {
            match name_handler.resolve_name(function_holder, name)? {
                Left(variable) => {
                    Some(variable)
                }
                Right((function, default_args, args)) => {
                    call_function(function, default_args, args, lines, name_handler, function_holder, return_into)?
                }
            }
        }
        other => return Err(ProcessorError::UnexpectedSymbol(other.clone()))
    })
}

fn call_function(function: &Box<dyn TypedFunction>, default_arg: Option<(isize, isize)>, args: &Vec<Vec<BasicSymbol>>,
    lines: &mut Vec<Line>, name_handler: &mut NameHandler, function_holder: &FunctionHolder, return_into: Option<(isize, isize)>)
    -> Result<Option<(isize, isize)>, ProcessorError> {
    let target_args = function.get_args();
    if args.len() != target_args.len() {
        return Err(ProcessorError::BadArgCount);
    }
    let mut call_args = Vec::new();
    if let Some(default_arg) = default_arg {
        if default_arg.1 != target_args[0].1 {
            return Err(ProcessorError::Placeholder);
        }
        call_args.push((default_arg.0, name_handler.type_table().get_type_size(default_arg.1).unwrap()));
    }
    for arg in args {
        let evaluated = evaluate(arg, lines, name_handler, function_holder, None)?;
        if evaluated.is_none() { return Err(ProcessorError::DoesntEvaluate) }
        let evaluated = evaluated.unwrap();
        if evaluated.1 != target_args[call_args.len()].1 {
            return Err(ProcessorError::BadArgType);
        }
        call_args.push((evaluated.0, name_handler.type_table().get_type_size(evaluated.1).unwrap()));
    }

    Ok(if let Some(return_type) = function.get_return_type() {
        if return_into.is_some() && return_into.unwrap().1 != return_type {
            return Err(ProcessorError::Placeholder);
        }
        let return_into = if let Some(return_into) = return_into {
            (return_into.0, name_handler.type_table().get_type_size(return_type).unwrap())
        }
        else {
            (name_handler.add_local_variable(None, return_type), name_handler.type_table.get_type_size(return_type).unwrap())
        };

        if function.is_inline() {
            let mut inline_args: Vec<_> = call_args.into_iter().map(|x| x.0).collect();
            inline_args.push(return_into.0);
            lines.push(Line::InlineAsm(function.get_inline(inline_args)));
        }
        else {
            lines.push(Line::ReturnCall(function.get_id(), call_args, return_type))
        }

        Some((return_into.0, return_type))
    }
    else {
        if return_into.is_some() {
            return Err(ProcessorError::Placeholder);
        }

        if function.is_inline() {
            let inline_args: Vec<_> = call_args.into_iter().map(|x| x.0).collect();
            lines.push(Line::InlineAsm(function.get_inline(inline_args)));
        }
        else {
            lines.push(Line::NoReturnCall(function.get_id(), call_args))
        }

        None
    })
}

fn evaluate_operator<'a>(symbol: &'a BasicSymbol) -> Result<&'a Operator, ProcessorError> {
    match symbol {
        BasicSymbol::Operator(operator) => Ok(operator),
        _ => Err(ProcessorError::BadEvaluableLayout)
    }
}

fn try_instantiate_literal(literal: Either<(isize, isize), &Literal>, lines: &mut Vec<Line>, name_handler: &mut NameHandler,
   function_holder: &FunctionHolder, return_into: Option<(isize, isize)>) -> Result<(isize, isize), ProcessorError> {
    match literal {
        Left(r) => Ok(r),
        Right(literal) => instantiate_literal(Left(literal), lines, name_handler, function_holder, return_into)
    }
}

fn instantiate_literal(literal: Either<&Literal, isize>, lines: &mut Vec<Line>, name_handler: &mut NameHandler,
   _function_holder: &FunctionHolder, return_into: Option<(isize, isize)>) -> Result<(isize, isize), ProcessorError> {
    let (addr, id) = if let Some((addr, id)) = return_into {
        (addr, id)
    }
    else {
        let id = match &literal {
            Left(literal) => literal.get_type_id(),
            Right(id) => *id
        };
        (name_handler.add_local_variable(None, id), id)
    };
    let _type = name_handler.type_table().get_type(id).unwrap();
    let asm = match literal {
        Left(literal) => _type.instantiate(Some(literal), addr)?,
        Right(_id) => _type.instantiate(None, addr)?
    };
    lines.push(Line::InlineAsm(asm));
    Ok((addr, id))
}

fn evaluate_operation(lhs: (isize, isize), op: &Operator, rhs: Option<(isize, isize)>,
    lines: &mut Vec<Line>, name_handler: &mut NameHandler, function_holder: &FunctionHolder, return_into: Option<(isize, isize)>)
    -> Result<Option<(isize, isize)>, ProcessorError> {
    todo!()
}