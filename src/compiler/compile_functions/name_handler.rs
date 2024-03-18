use crate::basic_ast::symbol::{BasicSymbol, NameAccessType, NameType};
use crate::compiler::compile_functions::{FunctionHolder, Line};
use crate::parser::line_info::LineInfo;
use crate::processor::custom_types::Int;
use crate::processor::processor::ProcessorError;
use crate::processor::type_builder::{Type, TypeTable, TypedFunction};
use either::{Either, Left, Right};
use std::collections::HashSet;
use crate::utils::align;

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
        self.args = args;
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
        lines: &mut Vec<Line>,
    ) -> Result<isize, ProcessorError> {
        let size = align(self.type_table.get_type_size(_type)?, 8);
        let addr = -(self.local_variables_size as isize) - size as isize;
        self.local_variables_size += size;
        // lines.push(Line::InlineAsm(vec![format!("sub rsp, {}", size)]));
        if let Some(name) = name {
            self.local_variables.push((name, addr, _type));
        }
        Ok(addr)
    }

    pub fn destroy_local_variables(&mut self, lines: &mut Vec<Line>) -> Result<(), ProcessorError> {
        // return Ok(())

        for (name, addr, (type_id, indirection)) in self.local_variables.clone() {
            if indirection != 0 {
                continue;
            }
            let t = self.type_table.get_type(type_id).unwrap();
            if let Some(destructor) = t.get_destructor() {
                let ref_ = self.add_local_variable(None, (type_id, 1), lines)?;

                lines.push(Line::InlineAsm(Int::instantiate_local_ref(addr, ref_)));

                lines.push(Line::NoReturnCall(
                    destructor,
                    -(self.local_variable_space() as isize),
                    vec![(ref_, self.type_table.get_type_size((type_id, 1))?)],
                    0
                ));

                self.use_function_id(destructor);
            }
        }

        Ok(())
    }

    pub fn name_variable(&mut self, name: String, addr: isize, _type: (isize, usize)) {
        self.local_variables.push((name, addr, _type));
    }

    pub fn resolve_name<'b>(
        &mut self,
        function_holder: &'b FunctionHolder,
        name: &'b Vec<(String, NameAccessType, NameType, usize)>,
        line: &LineInfo,
        lines: &mut Vec<Line>,
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
                return Err(ProcessorError::NotImplemented(
                    line.clone(),
                    "Using '.' or '#' after a function call".to_string(),
                ));
            }

            match name_type {
                NameType::Normal => {
                    if current_type.is_some() && current_variable.is_some() {
                        let user_type = self
                            .type_table
                            .get_type(current_type.unwrap().0)
                            .unwrap()
                            .get_user_type()
                            .ok_or(ProcessorError::AttributeDoesntExist(
                                line.clone(),
                                self.type_table
                                    .get_type(current_type.unwrap().0)
                                    .unwrap()
                                    .get_name()
                                    .to_string(),
                                name.clone(),
                            ))?;

                        let t = user_type
                            .get_attribute_offset_and_type(name, &self.type_table)?
                            .ok_or(ProcessorError::AttributeDoesntExist(
                                line.clone(),
                                self.type_table
                                    .get_type(current_type.unwrap().0)
                                    .unwrap()
                                    .get_name()
                                    .to_string(),
                                name.clone(),
                            ))?;

                        if current_type.unwrap().1 > 0 {
                            let ref_addr = self
                                .add_local_variable(None, (t.1 .0, current_type.unwrap().1), lines)
                                .unwrap();
                            lines.push(Line::InlineAsm(Int::instantiate_ref(
                                current_variable.unwrap(),
                                t.0 as isize,
                                ref_addr,
                            )));
                            current_variable = Some(ref_addr);
                            current_type = Some((t.1 .0, current_type.unwrap().1 + t.1 .1));
                        } else {
                            current_variable = Some(current_variable.unwrap() + (t.0 as isize));
                            current_type = Some(t.1);
                        }
                    } else if current_type.is_some() {
                        return Err(ProcessorError::AttemptedTypeAttribAccess(line.clone()));
                    } else if let Some((_, addr, _type)) = self
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
                        .get(&current_type.map(|x| x.0))
                        .unwrap()
                        .get(name)
                    {
                        let default_arg = if matches!(access_type, NameAccessType::Normal) {
                            if current_variable.is_none() {
                                return Err(ProcessorError::TypeNonStaticFunctionCall(
                                    line.clone(),
                                ));
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
