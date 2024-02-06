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


pub fn compile_functions(mut function_name_map: HashMap<Option<isize>, HashMap<String, isize>>, mut functions: HashMap<isize, Box<dyn TypedFunction>>, type_table: TypeTable) -> Result<Vec<Box<dyn Function>>, ProcessorError> {
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
                    evaluate(other_symbol, &mut symbols, &mut local_variable_space)?;
                }
            }

            if let Some(next) = symbols.next() {
                if !matches!(next.0, BasicSymbol::Punctuation(Punctuation::Semicolon)) {
                    return Err(ProcessorError::ExpectedSemicolon);
                }
            }
            else { return Err(ProcessorError::ExpectedSemicolon); }
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


fn evaluate<'a>(first_symbol: &BasicSymbol, symbol_iter: &'a mut Iter<(BasicSymbol, usize)>, local_variable_space: &mut usize, must_complete: bool) -> Result<Either<(isize, isize), &'a Literal>, ProcessorError> { // addr, type
    let mut op = None;
    let mut lhs = None;

    match evaluate_symbol(first_symbol, local_variable_space)? {
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

    match evaluate_symbol(first_symbol, local_variable_space)? {
        Left(_lhs) => { lhs = Some(_lhs) }
        Right(_op) => { op = Some(_op) }
    }

    if lhs.is_none() || op.is_none() {
        return Err(ProcessorError::ExpectedOperatorOperand)
    }

    let (op, lhs) = (op.unwrap(), lhs.unwrap());

    if matches!(op, Operator::Not) {
        return Ok(Left(evaluate_operation(lhs, op, None, local_variable_space)?))
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

    let rhs = match evaluate_symbol(first_symbol, local_variable_space)? {
        Left(rhs) => { rhs }
        Right(_) => return Err(ProcessorError::BadItemInEvaluation)
    };

    return Ok(Left(evaluate_operation(lhs, op, Some(rhs), local_variable_space)?))
}

fn evaluate_symbol<'a>(symbol: &'a BasicSymbol, local_variable_space: &mut usize) -> Result<Either<Either<(isize, isize), &'a Literal>, &'a Operator>, ProcessorError> {
    Ok(match symbol {
        BasicSymbol::BracketedSection(inner) => {
            let mut iter = inner.iter();
            let next = if let Some(next) = iter.next() {
                &next.0
            }
            else { return Err(ProcessorError::EmptyBrackets); };
            Left(evaluate(next, &mut iter, local_variable_space, true)?)
        }
        BasicSymbol::Literal(literal) => { Left(Right(literal)) }
        BasicSymbol::Operator(operator) => {
            if !matches!(operator, Operator::Not) {
                return Err(ProcessorError::BadItemInEvaluation);
            }
            Right(operator)
        }
        BasicSymbol::Name(_) => { todo!() }
        _ => {
            return Err(ProcessorError::BadItemInEvaluation);
        }
    })
}

fn evaluate_operation(lhs: Either<(isize, isize), &Literal>, op: &Operator, rhs: Option<Either<(isize, isize), &Literal>>, local_variable_space: &mut usize)
    -> Result<(isize, isize), ProcessorError> {
    todo!()
}