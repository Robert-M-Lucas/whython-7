use std::collections::{HashMap, HashSet};
use std::slice::Iter;
use either::{Either, Left, Right};
use crate::ast::keywords::Keyword;
use crate::ast::literals::Literal;
use crate::ast::operators::Operator;
use crate::basic_ast::punctuation::Punctuation;
use crate::basic_ast::symbol::BasicSymbol;
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
    functions: HashMap<Option<isize>, HashMap<String, isize>>,
    type_table: TypeTable,
    args: Vec<(String, isize)>,
    local_variables: Vec<(String, isize)>,
    local_variables_size: usize
}

impl NameHandler {
    pub fn new(functions: HashMap<Option<isize>, HashMap<String, isize>>,
               type_table: TypeTable,
               args: Vec<(String, isize)>,
               local_variables: Vec<(String, isize)>,
               local_variables_size: usize) -> NameHandler {
        NameHandler {
            functions,
            type_table,
            args,
            local_variables,
            local_variables_size,
        }
    }

    pub fn resolve_name(&self, name: Vec<String>) -> Either<(isize, isize), Box<dyn TypedFunction>> {
        todo!()
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
    let functions = functions;
    let function_name_map = function_name_map;
    let mut processed_functions = get_custom_function_implementations();
    let mut used_functions = HashSet::new();
    used_functions.insert(0);

    for id in functions.keys() {
        let function = functions.get(id).unwrap();
        if function.is_inline() { continue; }

        let mut local_variable_space = 0;
        let mut local_variables: Vec<(String, isize)> = Vec::new();

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
                }
                other_symbol => {
                    evaluate(other_symbol, &mut symbols, &mut local_variable_space, false, &function_name_map, &type_table, &mut lines, &functions)?;
                }
            }
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


fn evaluate<'a>(first_symbol: &'a BasicSymbol, symbol_iter: &'a mut Iter<(BasicSymbol, usize)>,
                local_variable_space: &mut usize, must_complete: bool, function_name_map: &HashMap<Option<isize>,
        HashMap<String, isize>>, type_table: &TypeTable, lines: &mut Vec<Line>, functions: &HashMap<isize, Box<dyn TypedFunction>>)
    -> Result<Either<(isize, isize), Literal>, ProcessorError> { // addr, type
    let mut op = None;
    let mut lhs = None;

    match evaluate_symbol(first_symbol, local_variable_space, function_name_map, type_table, lines, functions)? {
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

    match evaluate_symbol(first_symbol, local_variable_space, function_name_map, type_table, lines, functions)? {
        Left(_lhs) => { lhs = Some(_lhs) }
        Right(_op) => { op = Some(_op) }
    }

    if lhs.is_none() || op.is_none() {
        return Err(ProcessorError::ExpectedOperatorOperand)
    }

    let (op, lhs) = (op.unwrap(), lhs.unwrap());

    if matches!(op, Operator::Not) {
        return Ok(Left(evaluate_operation(lhs, op, None, local_variable_space, function_name_map, type_table, lines, functions)?))
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

    let rhs = match evaluate_symbol(third_symbol, local_variable_space, function_name_map, type_table, lines, functions)? {
        Left(rhs) => { rhs }
        Right(_) => return Err(ProcessorError::BadItemInEvaluation)
    };

    return Ok(Left(evaluate_operation(lhs, op, Some(rhs), local_variable_space, function_name_map, type_table, lines, functions)?))
}

fn evaluate_symbol(symbol: &BasicSymbol, local_variable_space: &mut usize, function_name_map: &HashMap<Option<isize>,
    HashMap<String, isize>>, type_table: &TypeTable, lines: &mut Vec<Line>, functions: &HashMap<isize, Box<dyn TypedFunction>>) -> Result<Either<Either<(isize, isize), Literal>, Operator>, ProcessorError> {
    Ok(match symbol {
        BasicSymbol::BracketedSection(inner) => {
            let mut iter = inner.into_iter();
            let next = if let Some(next) = iter.next() {
                &next.0
            }
            else { return Err(ProcessorError::EmptyBrackets); };
            let res = evaluate(next, &mut iter, local_variable_space, true, function_name_map, type_table, lines, functions)?;
            Left(res)
        }
        BasicSymbol::Literal(literal) => { Left(Right(literal.clone())) }
        BasicSymbol::Operator(operator) => {
            if !matches!(operator, Operator::Not) {
                return Err(ProcessorError::BadItemInEvaluation);
            }
            Right(operator.clone())
        }
        BasicSymbol::Name(_) => { todo!() }
        _ => {
            return Err(ProcessorError::BadItemInEvaluation);
        }
    })
}

fn instantiate_literal(literal: Either<Literal, isize>, local_variable_space: &mut usize, function_name_map: &HashMap<Option<isize>,
    HashMap<String, isize>>, type_table: &TypeTable, lines: &mut Vec<Line>, functions: &HashMap<isize, Box<dyn TypedFunction>>) -> Result<(isize, isize), ProcessorError> {
    let id = match &literal {
        Left(literal) => literal.get_type_id(),
        Right(id) => *id
    };
    let _type = type_table.get_type(id).unwrap();
    let size = _type.get_size(type_table, None)?;
    let addr = -(*local_variable_space as isize) - size as isize;
    let asm = match literal {
        Left(literal) => _type.instantiate(Some(literal), addr)?,
        Right(_id) => _type.instantiate(None, addr)?
    };
    lines.push(Line::InlineAsm(asm));
    Ok((addr, id))
}

fn evaluate_operation(lhs: Either<(isize, isize), Literal>, op: Operator, rhs: Option<Either<(isize, isize), Literal>>,
                      local_variable_space: &mut usize, function_name_map: &HashMap<Option<isize>,
                HashMap<String, isize>>, type_table: &TypeTable, lines: &mut Vec<Line>, functions: &HashMap<isize, Box<dyn TypedFunction>>)
    -> Result<(isize, isize), ProcessorError> {
    let lhs = match lhs {
        Left(addr) => { addr }
        Right(literal) => { instantiate_literal(Left(literal), local_variable_space, function_name_map, type_table, lines, functions)? }
    };

    let rhs = if let Some(rhs) = rhs {
        Some(match rhs {
            Left(addr) => { addr }
            Right(literal) => { instantiate_literal(Left(literal), local_variable_space, function_name_map, type_table, lines, functions)? }
        })
    }
    else { None };

     Ok(match op {
        Operator::Not => {
            let func = function_name_map.get(&Some(lhs.1)).unwrap().get("not").ok_or(ProcessorError::BadOperatorFunction)?;
            let func = functions.get(func).unwrap();
            let func_args = func.get_args();
            if func_args.len() != 2 {
                return Err(ProcessorError::BadOperatorFunction);
            }
            let output = instantiate_literal(
                Right(func.get_return_type().ok_or(ProcessorError::BadOperatorFunction)?),
                local_variable_space, function_name_map, type_table, lines, functions
            )?;
            if func.is_inline() {
                lines.push(Line::InlineAsm(func.get_inline(vec![lhs.0, output.0])));
            }
            else {
                lines.push(Line::ReturnCall(func.get_id(), vec![(lhs.0, type_table.get_type_size(lhs.1)?)], output.0));
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

            let func = function_name_map.get(&Some(lhs.1)).unwrap().get(func_name).ok_or(ProcessorError::BadOperatorFunction)?;
            let func = functions.get(func).unwrap();
            let func_args = func.get_args();
            if func_args.len() != 3 {
                return Err(ProcessorError::BadOperatorFunction);
            }
            let output = instantiate_literal(
                Right(func.get_return_type().ok_or(ProcessorError::BadOperatorFunction)?),
                local_variable_space, function_name_map, type_table, lines, functions
            )?;
            if func.is_inline() {
                lines.push(Line::InlineAsm(func.get_inline(vec![lhs.0, rhs.0, output.0])));
            }
            else {
                lines.push(Line::ReturnCall(func.get_id(), vec![(lhs.0, type_table.get_type_size(lhs.1)?), (rhs.0, type_table.get_type_size(rhs.1)?)], output.0));
            }
            output
        }
    })
}