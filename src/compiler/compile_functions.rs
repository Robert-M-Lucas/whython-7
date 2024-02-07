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

pub struct NameHandler {
    functions: HashMap<isize, Box<dyn TypedFunction>>,
    functions_table: HashMap<Option<isize>, HashMap<String, isize>>,
    type_table: TypeTable,
    args: Vec<(String, isize, isize)>,
    local_variables: Vec<(String, isize, isize)>,
    local_variables_size: usize
}

impl NameHandler {
    pub fn new(functions: HashMap<isize, Box<dyn TypedFunction>>,
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

    pub fn resolve_name(&self, name: &Vec<(String, NameAccessType, NameType)>) -> Result<Either<(isize, isize), (&Box<dyn TypedFunction>, Option<isize>, &Vec<Vec<BasicSymbol>>)>, ProcessorError> {
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
                    if let Some(func) = self.functions_table.get(&current_type).unwrap().get(&name) {
                        let default_arg = if access_type == NameAccessType::Normal {
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
    let mut name_handler = NameHandler::new(functions, function_name_map, type_table);
    let mut processed_functions = get_custom_function_implementations();
    let mut used_functions = HashSet::new();
    used_functions.insert(0);

    for id in functions.keys() {
        name_handler.reset();
        let function = functions.get(id).unwrap();
        if function.is_inline() { continue; }

        let mut lines = Vec::new();
        let mut symbols = function.get_contents().iter();

        let mut next_saved = None;

        loop {
            let next = if next_saved.is_some() {
                next_saved.take().unwrap()
            }
            else {
                if let Some(s) = symbols.next() {
                    &s.0
                }
                else {
                    break
                }
            };

            match next {
                BasicSymbol::Keyword(Keyword::Let) => {
                    panic!()
                }
                BasicSymbol::Keyword(Keyword::If) => {
                    panic!()
                }
                BasicSymbol::Keyword(Keyword::While) => {
                    panic!()
                }
                BasicSymbol::Keyword(_) => {
                    return Err(ProcessorError::BadKeyword);
                },
                other_symbol => {
                    evaluate(other_symbol, &mut symbols, false, &mut lines, &mut name_handler)?;
                }
            }
        }

        processed_functions.push(Box::new(UserFunction {
            id: *id,
            local_variable_count: name_handler.local_variable_space() / 8,
            arg_count: function.get_args().len(),
            lines,
        }));
    }

    let processed_functions = processed_functions.into_iter().filter(|f| used_functions.contains(&f.get_id())).collect();
    Ok(processed_functions)
}


fn evaluate<'a>(first_symbol: &'a BasicSymbol, symbol_iter: &'a mut Iter<(BasicSymbol, usize)>,
                must_complete: bool, lines: &mut Vec<Line>, name_handler: &mut NameHandler)
    -> Result<Either<(isize, isize), Literal>, ProcessorError> { // addr, type
    let mut op = None;
    let mut lhs = None;

    match evaluate_symbol(first_symbol, lines, name_handler)? {
        Left(_lhs) => { lhs = Some(_lhs) }
        Right(_op) => { op = Some(_op) }
    }

    let second_symbol = if let Some(symbol) = symbol_iter.next() {
        &symbol.0
    }
    else if lhs.is_some() {
        return Ok(lhs.unwrap());
    }
    else {
        return Err(ProcessorError::BadItemInEvaluation);
    };

    if matches!(second_symbol, BasicSymbol::Punctuation(Punctuation::Semicolon)) {
        if lhs.is_some() && !must_complete {
            return Ok(lhs.unwrap());
        }
        return Err(ProcessorError::BadItemInEvaluation);
    }

    match evaluate_symbol(first_symbol, lines, name_handler)? {
        Some(Left(_lhs)) => { lhs = Some(_lhs) }
        Some(Right(_op)) => { op = Some(_op) }
        _ => {}
    }

    if lhs.is_none() || op.is_none() {
        return Err(ProcessorError::ExpectedOperatorOperand)
    }

    let (op, lhs) = (op.unwrap(), lhs.unwrap());

    if matches!(op, Operator::Not) {
        return Ok(Left(evaluate_operation(lhs, op, None, lines, name_handler)?))
    }

    let third_symbol = if let Some(symbol) = symbol_iter.next() {
        &symbol.0
    }
    else {
        return Err(ProcessorError::BadItemInEvaluation);
    };

    if matches!(second_symbol, BasicSymbol::Punctuation(Punctuation::Semicolon)) {
        return Err(ProcessorError::BadItemInEvaluation);
    }

    let rhs = match evaluate_symbol(third_symbol, lines, name_handler)? {
        None => return Err(ProcessorError::DoesntEvaluate),
        Some(Left(rhs)) => { rhs }
        Some(Right(_)) => return Err(ProcessorError::BadItemInEvaluation)
    };

    return Ok(Left(evaluate_operation(lhs, op, Some(rhs), lines, name_handler)?))
}

fn evaluate_symbol(symbol: &BasicSymbol, lines: &mut Vec<Line>, name_handler: &mut NameHandler) -> Result<Option<Either<Either<(isize, isize), Literal>, Operator>>, ProcessorError> {
    Ok(Some(match symbol {
        BasicSymbol::BracketedSection(inner) => {
            let mut iter = inner.into_iter();
            let next = if let Some(next) = iter.next() {
                &next.0
            }
            else { return Err(ProcessorError::EmptyBrackets); };
            let res = evaluate(next, &mut iter, true, lines, name_handler)?;
            Left(res)
        }
        BasicSymbol::Literal(literal) => { Left(Right(literal.clone())) }
        BasicSymbol::Operator(operator) => {
            if !matches!(operator, Operator::Not) {
                return Err(ProcessorError::BadItemInEvaluation);
            }
            Right(operator.clone())
        }
        BasicSymbol::Name(name) => { match name_handler.resolve_name(name)? {
            Left(_) => { todo!() }
            Right((function, default_arg, args)) => {
                if default_arg.is_some() || !args.is_empty() || !function.get_args().is_empty() { todo!() }
                if function.is_inline() {
                    if let Some(_type) = function.get_return_type() {
                        todo!()
                        // name_handler.type_table().get_type(_type).unwrap().instantiate()
                    }
                    else {
                        lines.push(Line::InlineAsm(function.get_inline(Vec::new())));
                        return Ok(None);
                    }
                }
                else {
                    if let Some(_type) = function.get_return_type() {
                        todo!()
                        // name_handler.type_table().get_type(_type).unwrap().instantiate()
                    }
                    else {
                        lines.push(Line::NoReturnCall(function.get_id(), Vec::new()));
                        return Ok(None);
                    }
                }
            }
        }}
        _ => {
            return Err(ProcessorError::BadItemInEvaluation);
        }
    }))
}

fn instantiate_literal(literal: Either<Literal, isize>, lines: &mut Vec<Line>, name_handler: &mut NameHandler
) -> Result<(isize, isize), ProcessorError> {
    let id = match &literal {
        Left(literal) => literal.get_type_id(),
        Right(id) => *id
    };
    let _type = name_handler.type_table().get_type(id).unwrap();
    let addr = name_handler.add_local_variable(None, id);
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
    let lhs = match lhs {
        Left(addr) => { addr }
        Right(literal) => { instantiate_literal(Left(literal), lines, name_handler)? }
    };

    let rhs = if let Some(rhs) = rhs {
        Some(match rhs {
            Left(addr) => { addr }
            Right(literal) => { instantiate_literal(Left(literal), lines, name_handler)? }
        })
    }
    else { None };

     Ok(match op {
        Operator::Not => {
            let func = name_handler.get_function(Some(lhs.1), "not").ok_or(ProcessorError::BadOperatorFunction)?;
            let func_args = func.get_args();
            if func_args.len() != 2 {
                return Err(ProcessorError::BadOperatorFunction);
            }
            let output = instantiate_literal(
                Right(func.get_return_type().ok_or(ProcessorError::BadOperatorFunction)?),
                lines, name_handler
            )?;
            if func.is_inline() {
                lines.push(Line::InlineAsm(func.get_inline(vec![lhs.0, output.0])));
            }
            else {
                lines.push(Line::ReturnCall(func.get_id(), vec![(lhs.0, name_handler.type_table().get_type_size(lhs.1)?)], output.0));
            }
            output
        },
        op => {
            let rhs = rhs.ok_or(ProcessorError::BadOperatorPosition)?;
            let func_name = match op {
                Operator::Add => "add",
                Operator::Subtract => "sub",
                Operator::Product => "mul",
                Operator::Divide => "div",
                Operator::Greater => "gt",
                Operator::Less => "lt",
                Operator::GreaterEqual => "ge",
                Operator::LessEqual => "le",
                Operator::Equal => "eq",
                Operator::NotEqual => "ne",
                Operator::Or => "or",
                Operator::And => "and",
                Operator::Not => panic!()
            };

            let func = name_handler.get_function(Some(lhs.1), func_name).ok_or(ProcessorError::BadOperatorFunction)?;
            let func_args = func.get_args();
            if func_args.len() != 3 {
                return Err(ProcessorError::BadOperatorFunction);
            }
            let output = instantiate_literal(
                Right(func.get_return_type().ok_or(ProcessorError::BadOperatorFunction)?),
                lines, name_handler
            )?;
            if func.is_inline() {
                lines.push(Line::InlineAsm(func.get_inline(vec![lhs.0, rhs.0, output.0])));
            }
            else {
                lines.push(Line::ReturnCall(func.get_id(), vec![(lhs.0, name_handler.type_table().get_type_size(lhs.1)?), (rhs.0, name_handler.type_table().get_type_size(rhs.1)?)], output.0));
            }
            output
        }
    })
}