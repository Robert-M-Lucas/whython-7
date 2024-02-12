use crate::ast::keywords::Keyword;
use crate::ast::literals::Literal;
use crate::ast::operators::Operator;
use crate::basic_ast::punctuation::Punctuation;
use crate::basic_ast::symbol::{BasicSymbol, NameAccessType, NameType};
use crate::compiler::custom_functions::{
    get_custom_function_implementations, get_custom_function_signatures,
};
use crate::compiler::default::{compile_user_function, get_function_sublabel, get_local_address};
use crate::parser::line_info::LineInfo;
use crate::processor::custom_types::Bool;
use crate::processor::processor::ProcessorError;
use crate::processor::type_builder::{Type, TypeTable, TypedFunction};
use either::{Either, Left, Right};
use std::collections::{HashMap, HashSet};


pub enum Line {
    ReturnCall(isize, Vec<(isize, usize)>, isize),
    NoReturnCall(isize, Vec<(isize, usize)>),
    Copy(isize, isize),
    Return(Option<isize>),
    InlineAsm(Vec<String>),
}

pub struct UserFunction {
    pub id: isize,
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
        _type: Option<isize>,
        name: &str,
    ) -> Option<&Box<dyn TypedFunction>> {
        self.functions_table.get(&_type).and_then(|x| {
            x.get(name).map(|x| self.functions.get(x).unwrap())
        })
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

    pub fn set_args(&mut self, args: Vec<(String, isize, isize)>) {
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

    pub fn add_local_variable(&mut self, name: Option<String>, _type: isize) -> Result<isize, ProcessorError> {
        let size = self
            .type_table
            .get_type(_type)
            .unwrap()
            .get_size(&self.type_table, None)?;
        let addr = -(self.local_variables_size as isize) - size as isize;
        self.local_variables_size += size;
        if let Some(name) = name {
            self.local_variables.push((name, addr, _type));
        }
        Ok(addr)
    }

    pub fn name_variable(&mut self, name: String, addr: isize, _type: isize) {
        self.local_variables.push((name, addr, _type));
    }

    pub fn resolve_name<'b>(
        &self,
        function_holder: &'b FunctionHolder,
        name: &'b Vec<(String, NameAccessType, NameType)>,
        line: &LineInfo,
    ) -> Result<
        Either<
            (isize, isize),
            (
                &'b Box<dyn TypedFunction>,
                Option<(isize, isize)>,
                &'b Vec<Vec<(BasicSymbol, LineInfo)>>,
            ),
        >,
        ProcessorError,
    > {
        let mut current_type = None;
        let mut current_variable = None;
        let mut return_func = None;

        for (name, access_type, name_type) in name {
            if return_func.is_some() {
                todo!()
            }

            match name_type {
                NameType::Normal => {
                    if current_type.is_some() || current_variable.is_some() {
                        todo!()
                    }
                    if let Some((_, addr, _type)) = self
                        .local_variables
                        .iter()
                        .chain(self.args.iter())
                        .find(|(n, _, _)| n == name)
                    {
                        // println!("{}, {}", addr, _type);
                        current_variable = Some(*addr);
                        current_type = Some(*_type);
                    } else if let Some(_type) = self.type_table.get_id_by_name(name) {
                        current_variable = None;
                        current_type = Some(_type);
                    } else {
                        return Err(ProcessorError::NameNotFound(line.clone(), name.clone()));
                    }
                }
                NameType::Function(contents) => {
                    if let Some(func) = function_holder
                        .functions_table()
                        .get(&current_type)
                        .unwrap()
                        .get(name)
                    {
                        let default_arg = if matches!(access_type, NameAccessType::Normal) {
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
        name_handler.reset();
        name_handler.set_args(function.get_args_positioned(name_handler.type_table()));
        let return_type = function.get_return_type();
        let mut lines = Vec::new();

        let last_return = process_lines(
            &contents,
            id,
            return_type,
            &mut lines,
            &mut name_handler,
            &function_holder,
            None
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
        }));
    }

    let processed_functions = processed_functions
        .into_iter()
        .filter(|f| name_handler.used_functions().contains(&f.get_id()))
        .collect();
    Ok(processed_functions)
}

fn process_lines(
    section: &[(BasicSymbol, LineInfo)],
    current_id: isize,
    return_type: Option<isize>,
    lines: &mut Vec<Line>,
    name_handler: &mut NameHandler,
    function_holder: &FunctionHolder,
    mut break_label: Option<String>
) -> Result<bool, ProcessorError> {
    let mut last_return = false;

    for line in section.split(|x| matches!(x.0, BasicSymbol::Punctuation(Punctuation::Semicolon))) {
        if line.is_empty() {
            continue;
        }
        last_return = false;

        if line.len() > 1 {
            match &line[1].0 {
                BasicSymbol::Assigner(assigner) => {
                    let name = match &line[0].0 {
                        BasicSymbol::Name(name) => name,
                        _ => return Err(ProcessorError::NonNameAssignment(line[0].1.clone())),
                    };
                    let Left(variable) =
                        name_handler.resolve_name(function_holder, name, &line[0].1)?
                    else {
                        return Err(ProcessorError::AssignToNonVariable(line[0].1.clone()));
                    };
                    if line.len() < 3 {
                        return Err(ProcessorError::NoAssignmentRHS(line[1].1.clone()));
                    }
                    if let Some(assigner) = assigner {
                        let result =
                            evaluate(&line[2..], lines, name_handler, function_holder, None)?
                                .ok_or(ProcessorError::DoesntEvaluate(line[2].1.clone()))?;
                        evaluate_operation(
                            variable,
                            (assigner, &line[1].1),
                            Some(result),
                            lines,
                            name_handler,
                            function_holder,
                            Some(variable),
                        )?;
                    } else {
                        evaluate(
                            &line[2..],
                            lines,
                            name_handler,
                            function_holder,
                            Some(variable),
                        )?;
                    }

                    continue;
                }
                _ => {}
            }
        }

        match &line[0].0 {
            BasicSymbol::Keyword(Keyword::Return) => {
                last_return = true;
                if line.len() == 1 {
                    if return_type.is_none() {
                        lines.push(Line::Return(None));
                        continue;
                    } else {
                        return Err(ProcessorError::NoneReturnOnTypedFunction(line[0].1.clone()));
                    }
                } else if return_type.is_none() {
                    return Err(ProcessorError::TypeReturnOnVoidFunction(line[1].1.clone()));
                }
                let return_type = return_type.unwrap();

                let return_into = name_handler.add_local_variable(None, return_type)?;
                let return_value = evaluate(
                    &line[1..],
                    lines,
                    name_handler,
                    function_holder,
                    Some((return_into, return_type)),
                )?;
                if return_value.is_none() {
                    return Err(ProcessorError::DoesntEvaluate(line[1].1.clone()));
                }
                let return_value = return_value.unwrap();
                if return_type != return_value.1 {
                    return Err(ProcessorError::BadReturnType(
                        line[1].1.clone(),
                        name_handler
                            .type_table()
                            .get_type(return_type)
                            .unwrap()
                            .get_name()
                            .to_string(),
                        name_handler
                            .type_table()
                            .get_type(return_value.1)
                            .unwrap()
                            .get_name()
                            .to_string(),
                    ));
                }
                lines.push(Line::Return(Some(return_value.0)));
            }
            BasicSymbol::Keyword(Keyword::Let) => {
                if line.len() < 2 {
                    return Err(ProcessorError::LetNoName(line[0].1.clone()));
                }
                let BasicSymbol::Name(name) = &line[1].0 else {
                    return Err(ProcessorError::LetNoName(line[1].1.clone()));
                };

                if name.len() > 1 {
                    return Err(ProcessorError::MultipartNameDef(line[1].1.clone()));
                }
                let name = &name[0];

                if !matches!(&name.2, NameType::Normal) {
                    return Err(ProcessorError::LetNoName(line[0].1.clone()));
                }
                let name = &name.0;

                if line.len() < 4 {
                    return Err(ProcessorError::NameTypeNotDefined(line[1].1.clone()));
                }
                if !matches!(&line[2].0, BasicSymbol::Punctuation(Punctuation::Colon)) {
                    return Err(ProcessorError::NameTypeNotDefined(line[2].1.clone()));
                }

                let BasicSymbol::Name(type_name) = &line[3].0 else {
                    return Err(ProcessorError::NameTypeNotDefined(line[3].1.clone()));
                };

                if type_name.len() > 1 {
                    return Err(ProcessorError::MultipartTypeName(line[3].1.clone()));
                }
                let type_name = &type_name[0];

                if !matches!(&type_name.2, NameType::Normal) {
                    return Err(ProcessorError::NameTypeNotDefined(line[3].1.clone()));
                }

                let type_id = name_handler
                    .type_table()
                    .get_id_by_name(&type_name.0)
                    .ok_or(ProcessorError::TypeNotFound(
                        line[3].1.clone(),
                        type_name.0.clone(),
                    ))?;
                let addr = name_handler.add_local_variable(Some(name.clone()), type_id)?;

                if line.len() < 6 {
                    return Err(ProcessorError::LetNoValue(line[3].1.clone()));
                }
                if !matches!(&line[4].0, BasicSymbol::Assigner(None)) {
                    return Err(ProcessorError::LetNoValue(line[4].1.clone()));
                }

                evaluate(
                    &line[5..],
                    lines,
                    name_handler,
                    function_holder,
                    Some((addr, type_id)),
                )?;
            }
            BasicSymbol::Keyword(Keyword::While) => {
                if line.len() < 2 {
                    return Err(ProcessorError::WhileNoBrackets(line[0].1.clone()));
                }

                let BasicSymbol::BracketedSection(expr) = &line[1].0 else {
                    return Err(ProcessorError::WhileNoBrackets(line[1].1.clone()));
                };
                let start_label =
                    get_function_sublabel(current_id, &name_handler.get_uid().to_string());
                let end_label =
                    get_function_sublabel(current_id, &name_handler.get_uid().to_string());
                break_label = Some(end_label.clone());

                lines.push(Line::InlineAsm(vec![format!("{}:", start_label)]));
                let evaluated = evaluate(expr, lines, name_handler, function_holder, None)?
                    .ok_or(ProcessorError::DoesntEvaluate(line[1].1.clone()))?;
                lines.push(Line::InlineAsm(vec![
                    format!("mov rax, [{}]", get_local_address(evaluated.0)),
                    "cmp rax, 0".to_string(),
                    format!("jnz {}", end_label),
                ]));

                if evaluated.1 != Bool::new().get_id() {
                    return Err(ProcessorError::BadConditionType(
                        line[1].1.clone(),
                        name_handler
                            .type_table()
                            .get_type(evaluated.1)
                            .unwrap()
                            .get_name()
                            .to_string(),
                    ));
                }
                if line.len() < 3 {
                    return Err(ProcessorError::WhileNoBraces(line[1].1.clone()));
                }
                let BasicSymbol::BracedSection(inner) = &line[2].0 else {
                    return Err(ProcessorError::WhileNoBraces(line[2].1.clone()));
                };
                process_lines(
                    inner,
                    current_id,
                    return_type,
                    lines,
                    name_handler,
                    function_holder,
                    break_label.clone()
                )?;
                lines.push(Line::InlineAsm(vec![
                    format!("jmp {}", start_label),
                    format!("{}:", end_label),
                ]));

                if line.len() > 3 {
                    return Err(ProcessorError::WhileMoreAfterBraces(line[3].1.clone()));
                }
            }
            BasicSymbol::Keyword(Keyword::If) => {
                if line.len() < 2 {
                    return Err(ProcessorError::IfElifNoBrackets(line[0].1.clone()));
                }

                let BasicSymbol::BracketedSection(expr) = &line[1].0 else {
                    return Err(ProcessorError::IfElifNoBrackets(line[1].1.clone()));
                };
                let mut next_label =
                    get_function_sublabel(current_id, &name_handler.get_uid().to_string());
                let end_label =
                    get_function_sublabel(current_id, &name_handler.get_uid().to_string());

                let evaluated = evaluate(expr, lines, name_handler, function_holder, None)?
                    .ok_or(ProcessorError::DoesntEvaluate(line[1].1.clone()))?;
                if evaluated.1 != Bool::new().get_id() {
                    return Err(ProcessorError::BadConditionType(
                        line[1].1.clone(),
                        name_handler
                            .type_table()
                            .get_type(evaluated.1)
                            .unwrap()
                            .get_name()
                            .to_string(),
                    ));
                }
                lines.push(Line::InlineAsm(vec![
                    format!("mov rax, [{}]", get_local_address(evaluated.0)),
                    "cmp rax, 0".to_string(),
                    format!("jnz {}", next_label),
                ]));
                if line.len() < 3 {
                    return Err(ProcessorError::IfElifElseNoBraces(line[1].1.clone()));
                }
                let BasicSymbol::BracedSection(inner) = &line[2].0 else {
                    return Err(ProcessorError::IfElifElseNoBraces(line[2].1.clone()));
                };
                process_lines(
                    inner,
                    current_id,
                    return_type,
                    lines,
                    name_handler,
                    function_holder,
                    break_label.clone()
                )?;

                let mut i = 3;
                let mut ended = false;
                while line.len() >= i + 1 {
                    lines.push(Line::InlineAsm(vec![
                        format!("jmp {}", end_label),
                        format!("{}:", next_label),
                    ]));

                    next_label =
                        get_function_sublabel(current_id, &name_handler.get_uid().to_string());

                    match &line[i].0 {
                        BasicSymbol::Keyword(Keyword::Elif) => {
                            if ended {
                                return Err(ProcessorError::IfElifAfterElse(line[i].1.clone()));
                            }
                            i += 1;
                            if line.len() <= i {
                                return Err(ProcessorError::IfElifNoBrackets(line[i - 1].1.clone()));
                            }

                            let BasicSymbol::BracketedSection(expr) = &line[i].0 else {
                                return Err(ProcessorError::IfElifNoBrackets(line[i].1.clone()));
                            };

                            let evaluated = evaluate(expr, lines, name_handler, function_holder, None)?
                                .ok_or(ProcessorError::DoesntEvaluate(line[i].1.clone()))?;
                            if evaluated.1 != Bool::new().get_id() {
                                return Err(ProcessorError::BadConditionType(
                                    line[i].1.clone(),
                                    name_handler
                                        .type_table()
                                        .get_type(evaluated.1)
                                        .unwrap()
                                        .get_name()
                                        .to_string(),
                                ));
                            }
                            lines.push(Line::InlineAsm(vec![
                                format!("mov rax, [{}]", get_local_address(evaluated.0)),
                                "cmp rax, 0".to_string(),
                                format!("jnz {}", next_label),
                            ]));

                            i += 1;
                            if line.len() <= i  {
                                return Err(ProcessorError::IfElifElseNoBraces(line[i-1].1.clone()));
                            }
                            let BasicSymbol::BracedSection(inner) = &line[i].0 else {
                                return Err(ProcessorError::IfElifElseNoBraces(line[i].1.clone()));
                            };
                            process_lines(
                                inner,
                                current_id,
                                return_type,
                                lines,
                                name_handler,
                                function_holder,
                                break_label.clone()
                            )?;
                            i += 1;
                        }
                        BasicSymbol::Keyword(Keyword::Else) => {
                            ended = true;
                            i += 1;
                            if line.len() <= i  {
                                return Err(ProcessorError::IfElifElseNoBraces(line[i-1].1.clone()));
                            }
                            let BasicSymbol::BracedSection(inner) = &line[i].0 else {
                                return Err(ProcessorError::IfElifElseNoBraces(line[i].1.clone()));
                            };
                            process_lines(
                                inner,
                                current_id,
                                return_type,
                                lines,
                                name_handler,
                                function_holder,
                                break_label.clone()
                            )?;
                            i += 1;
                        }
                        _ => {
                            return Err(ProcessorError::ElseMoreAfterBraces(line[i].1.clone()))
                        }
                    }
                }

                lines.push(Line::InlineAsm(vec![
                    format!("{}:", next_label),
                    format!("{}:", end_label)
                ]));
            }
            BasicSymbol::Keyword(Keyword::Elif | Keyword::Else) => {
                return Err(ProcessorError::RawElifElse(line[0].1.clone()))
            }
            BasicSymbol::Keyword(Keyword::Break) => {
                if line.len() > 1 {
                    return Err(ProcessorError::BreakLineNotEmpty(line[1].1.clone()))
                }
                let Some(break_label) = &break_label else { return Err(ProcessorError::NothingToBreak(line[0].1.clone())); };
                lines.push(Line::InlineAsm(vec![format!("jmp {}", break_label)]));
            }
            _ => {
                evaluate(line, lines, name_handler, function_holder, None)?;
            }
        };
    }

    Ok(last_return)
}

fn evaluate<'a>(
    section: &[(BasicSymbol, LineInfo)],
    lines: &mut Vec<Line>,
    name_handler: &mut NameHandler,
    function_holder: &FunctionHolder,
    return_into: Option<(isize, isize)>,
) -> Result<Option<(isize, isize)>, ProcessorError> {
    // addr, type
    Ok(if section.len() == 1 {
        evaluate_symbol(
            &section[0],
            lines,
            name_handler,
            function_holder,
            return_into,
        )?
    } else if section.len() == 2 {
        let op = evaluate_operator(&section[0])?;
        let Some(value) = evaluate_symbol(&section[1], lines, name_handler, function_holder, None)?
        else {
            return Err(ProcessorError::DoesntEvaluate(section[1].1.clone()));
        };
        evaluate_operation(
            value,
            (op, &section[1].1),
            None,
            lines,
            name_handler,
            function_holder,
            return_into,
        )?
    } else if section.len() == 3 {
        let Some(lhs) = evaluate_symbol(&section[0], lines, name_handler, function_holder, None)?
        else {
            return Err(ProcessorError::DoesntEvaluate(section[0].1.clone()));
        };
        let op = evaluate_operator(&section[1])?;
        let Some(rhs) = evaluate_symbol(&section[2], lines, name_handler, function_holder, None)?
        else {
            return Err(ProcessorError::DoesntEvaluate(section[2].1.clone()));
        };
        evaluate_operation(
            lhs,
            (op, &section[1].1),
            Some(rhs),
            lines,
            name_handler,
            function_holder,
            return_into,
        )?
    } else {
        return Err(ProcessorError::BadEvaluableLayout(section[3].1.clone()));
    })
}

fn evaluate_symbol(
    symbol: &(BasicSymbol, LineInfo),
    lines: &mut Vec<Line>,
    name_handler: &mut NameHandler,
    function_holder: &FunctionHolder,
    return_into: Option<(isize, isize)>,
) -> Result<Option<(isize, isize)>, ProcessorError> {
    // println!("{:?}", symbol);
    Ok(match &symbol.0 {
        BasicSymbol::AbstractSyntaxTree(_) => panic!(),
        BasicSymbol::Operator(_) => {
            return Err(ProcessorError::BadEvaluableLayout(symbol.1.clone()))
        }
        BasicSymbol::Literal(literal) => Some(instantiate_literal(
            Left(literal),
            lines,
            name_handler,
            function_holder,
            return_into,
        )?),
        BasicSymbol::BracketedSection(inner) => {
            evaluate(inner, lines, name_handler, function_holder, return_into)?
        }
        BasicSymbol::Name(name) => {
            // println!("{:?}", name);
            match name_handler.resolve_name(function_holder, name, &symbol.1)? {
                Left(variable) => {
                    // println!("{:?}", variable);
                    Some(variable)
                }
                Right((function, default_args, args)) => call_function(
                    function,
                    &symbol.1,
                    default_args,
                    args,
                    lines,
                    name_handler,
                    function_holder,
                    return_into,
                )?,
            }
        }
        _other => return Err(ProcessorError::BadEvaluableLayout(symbol.1.clone())),
    })
}

fn call_function(
    function: &Box<dyn TypedFunction>,
    start_line: &LineInfo,
    default_arg: Option<(isize, isize)>,
    args: &Vec<Vec<(BasicSymbol, LineInfo)>>,
    lines: &mut Vec<Line>,
    name_handler: &mut NameHandler,
    function_holder: &FunctionHolder,
    return_into: Option<(isize, isize)>,
) -> Result<Option<(isize, isize)>, ProcessorError> {
    name_handler.use_function(function);
    let target_args = function.get_args();
    let mut args_len = args.len();
    if default_arg.is_some() { args_len += 1; }

    if args_len > target_args.len() {
        return Err(ProcessorError::BadArgCount( // TODO: Bad line location
            args[target_args.len() - (args_len - args.len())][0].1.clone(),
            target_args.len(),
            args_len,
            function.get_line()
        ));
    }
    if args_len < target_args.len() {
        if args.is_empty() {
            return Err(ProcessorError::BadArgCount( // TODO: Bad line location
                start_line.clone(),
                target_args.len(),
                args_len,
                function.get_line()
            ));
        } else {
            return Err(ProcessorError::BadArgCount( // TODO: Bad line location
                args[args.len()-1].last().unwrap().1.clone(),
                target_args.len(),
                args_len,
                function.get_line()
            ));
        }
    }

    let mut call_args = Vec::new();
    if let Some(default_arg) = default_arg {
        if default_arg.1 != target_args[0].1 {
            return Err(ProcessorError::Placeholder2); // TODO:
        }
        call_args.push((
            default_arg.0,
            name_handler
                .type_table()
                .get_type_size(default_arg.1)
                .unwrap(),
        ));
    }
    for arg in args {
        let evaluated = evaluate(arg, lines, name_handler, function_holder, None)?;
        // println!("{:?}", evaluated);
        if evaluated.is_none() {
            return Err(ProcessorError::DoesntEvaluate(arg[0].1.clone()));
        }
        let evaluated = evaluated.unwrap();
        if evaluated.1 != target_args[call_args.len()].1 {
            return Err(ProcessorError::BadArgType(
                arg[0].1.clone(),
                name_handler
                    .type_table()
                    .get_type(target_args[call_args.len()].1)
                    .unwrap()
                    .get_name()
                    .to_string(),
                name_handler
                    .type_table()
                    .get_type(evaluated.1)
                    .unwrap()
                    .get_name()
                    .to_string(),
                function.get_line()
            ));
        }
        call_args.push((
            evaluated.0,
            name_handler
                .type_table()
                .get_type_size(evaluated.1)
                .unwrap(),
        ));
    }

    Ok(if let Some(return_type) = function.get_return_type() {
        if return_into.is_some() && return_into.unwrap().1 != return_type {
            return Err(ProcessorError::BadEvaluatedType(
                start_line.clone(),
                name_handler
                    .type_table()
                    .get_type(return_into.unwrap().1)
                    .unwrap()
                    .get_name()
                    .to_string(),
                name_handler
                    .type_table()
                    .get_type(return_type)
                    .unwrap()
                    .get_name()
                    .to_string(),
            ));
        }
        let return_into = if let Some(return_into) = return_into {
            (
                return_into.0,
                name_handler
                    .type_table()
                    .get_type_size(return_type)
                    .unwrap(),
            )
        } else {
            (
                name_handler.add_local_variable(None, return_type)?,
                name_handler.type_table.get_type_size(return_type)?,
            )
        };

        if function.is_inline() {
            let mut inline_args: Vec<_> = call_args.into_iter().map(|x| x.0).collect();
            inline_args.push(return_into.0);
            lines.push(Line::InlineAsm(function.get_inline(inline_args)));
        } else {
            lines.push(Line::ReturnCall(
                function.get_id(),
                call_args,
                return_into.0,
            ))
        }

        Some((return_into.0, return_type))
    } else {
        if return_into.is_some() {
            return Err(ProcessorError::BadEvaluatedType(
                start_line.clone(),
                name_handler
                    .type_table()
                    .get_type(return_into.unwrap().1)
                    .unwrap()
                    .get_name()
                    .to_string(),
                "None".to_string(),
            ));
        }

        if function.is_inline() {
            let inline_args: Vec<_> = call_args.into_iter().map(|x| x.0).collect();
            lines.push(Line::InlineAsm(function.get_inline(inline_args)));
        } else {
            lines.push(Line::NoReturnCall(function.get_id(), call_args))
        }

        None
    })
}

fn evaluate_operator(
    symbol: &(BasicSymbol, LineInfo),
) -> Result<&Operator, ProcessorError> {
    match &symbol.0 {
        BasicSymbol::Operator(operator) => Ok(operator),
        _ => Err(ProcessorError::BadEvaluableLayout(symbol.1.clone())),
    }
}

fn try_instantiate_literal(
    literal: Either<(isize, isize), &Literal>,
    lines: &mut Vec<Line>,
    name_handler: &mut NameHandler,
    function_holder: &FunctionHolder,
    return_into: Option<(isize, isize)>,
) -> Result<(isize, isize), ProcessorError> {
    match literal {
        Left(r) => Ok(r),
        Right(literal) => instantiate_literal(
            Left(literal),
            lines,
            name_handler,
            function_holder,
            return_into,
        ),
    }
}

fn instantiate_literal(
    literal: Either<&Literal, isize>,
    lines: &mut Vec<Line>,
    name_handler: &mut NameHandler,
    _function_holder: &FunctionHolder,
    return_into: Option<(isize, isize)>,
) -> Result<(isize, isize), ProcessorError> {
    let (addr, id) = if let Some((addr, id)) = return_into {
        (addr, id)
    } else {
        let id = match &literal {
            Left(literal) => literal.get_type_id(),
            Right(id) => *id,
        };
        (name_handler.add_local_variable(None, id)?, id)
    };
    let _type = name_handler.type_table().get_type(id).unwrap();
    let asm = match literal {
        Left(literal) => _type.instantiate(Some(literal), addr)?,
        Right(_id) => _type.instantiate(None, addr)?,
    };
    lines.push(Line::InlineAsm(asm));
    Ok((addr, id))
}

fn evaluate_operation(
    lhs: (isize, isize),
    op: (&Operator, &LineInfo),
    rhs: Option<(isize, isize)>,
    lines: &mut Vec<Line>,
    name_handler: &mut NameHandler,
    function_holder: &FunctionHolder,
    return_into: Option<(isize, isize)>,
) -> Result<Option<(isize, isize)>, ProcessorError> {
    Ok(Some(match &op.0 {
        Operator::Not => {
            let func = function_holder.get_function(Some(lhs.1), "not").ok_or(
                ProcessorError::SingleOpFunctionNotFound(
                    op.1.clone(),
                    "not".to_string(),
                    name_handler
                        .type_table
                        .get_type(lhs.1)
                        .unwrap()
                        .get_name()
                        .to_string(),
                ),
            )?;
            name_handler.use_function(func);
            let func_args = func.get_args();
            let func_id = func.get_id();
            if func_args.len() != 1 {
                return Err(ProcessorError::SingleOpFunctionNotFound(
                    op.1.clone(),
                    "not".to_string(),
                    name_handler
                        .type_table
                        .get_type(lhs.1)
                        .unwrap()
                        .get_name()
                        .to_string(),
                ));
            }
            let output = if let Some(return_into) = return_into {
                return_into
            } else {
                instantiate_literal(
                    Right(
                        func.get_return_type()
                            .ok_or(ProcessorError::SingleOpFunctionNotFound(
                                op.1.clone(),
                                "not".to_string(),
                                name_handler
                                    .type_table
                                    .get_type(lhs.1)
                                    .unwrap()
                                    .get_name()
                                    .to_string(),
                            ))?,
                    ),
                    lines,
                    name_handler,
                    function_holder,
                    None,
                )?
            };
            let func = function_holder.functions().get(&func_id).unwrap();
            if func.is_inline() {
                lines.push(Line::InlineAsm(func.get_inline(vec![lhs.0, output.0])));
            } else {
                lines.push(Line::ReturnCall(
                    func.get_id(),
                    vec![(lhs.0, name_handler.type_table().get_type_size(lhs.1)?)],
                    output.0,
                ));
            }
            output
        }
        op_ => {
            let rhs = rhs.ok_or(ProcessorError::BadOperatorPosition(
                op.1.clone(),
                op.0.clone(),
            ))?;
            let func_name = match op_ {
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
                Operator::Not => panic!(),
            };

            let func = function_holder.get_function(Some(lhs.1), func_name).ok_or(
                ProcessorError::OpFunctionNotFound(
                    op.1.clone(),
                    func_name.to_string(),
                    name_handler
                        .type_table
                        .get_type(lhs.1)
                        .unwrap()
                        .get_name()
                        .to_string(),
                    name_handler
                        .type_table
                        .get_type(rhs.1)
                        .unwrap()
                        .get_name()
                        .to_string(),
                ),
            )?;
            name_handler.use_function(func);
            let func_args = func.get_args();
            let func_id = func.get_id();

            if func_args.len() != 2 {
                return Err(ProcessorError::OpFunctionNotFound(
                    op.1.clone(),
                    func_name.to_string(),
                    name_handler
                        .type_table
                        .get_type(lhs.1)
                        .unwrap()
                        .get_name()
                        .to_string(),
                    name_handler
                        .type_table
                        .get_type(rhs.1)
                        .unwrap()
                        .get_name()
                        .to_string(),
                ));
            }

            let output = if let Some(return_into) = return_into {
                return_into
            } else {
                instantiate_literal(
                    Right(
                        func.get_return_type()
                            .ok_or(ProcessorError::OpFunctionNotFound(
                                op.1.clone(),
                                func_name.to_string(),
                                name_handler
                                    .type_table
                                    .get_type(lhs.1)
                                    .unwrap()
                                    .get_name()
                                    .to_string(),
                                name_handler
                                    .type_table
                                    .get_type(rhs.1)
                                    .unwrap()
                                    .get_name()
                                    .to_string(),
                            ))?,
                    ),
                    lines,
                    name_handler,
                    function_holder,
                    None,
                )?
            };

            let func = function_holder.functions().get(&func_id).unwrap();
            if func.is_inline() {
                lines.push(Line::InlineAsm(
                    func.get_inline(vec![lhs.0, rhs.0, output.0]),
                ));
            } else {
                lines.push(Line::ReturnCall(
                    func.get_id(),
                    vec![
                        (lhs.0, name_handler.type_table().get_type_size(lhs.1)?),
                        (rhs.0, name_handler.type_table().get_type_size(rhs.1)?),
                    ],
                    output.0,
                ));
            }
            output
        }
    }))
}
