use std::borrow::Cow;
use crate::processor::preprocess::{PreProcessFunction, PreprocessSymbol};
use crate::processor::processor::ProcessorError;
use crate::processor::processor::ProcessorError::TypeNotFound;

use std::collections::{HashMap, HashSet};

use std::path::PathBuf;
use unique_type_id::UniqueTypeId;
use crate::ast::literals::Literal;
use crate::basic_ast::symbol::BasicSymbol;
use crate::processor::custom_types::{Bool, Int};

struct UninitialisedType {
    pub path: PathBuf,
    pub line: usize,
    pub id: isize,
    pub attributes: Vec<(String, Result<isize, (String, usize)>)>,
}

impl UninitialisedType {
    pub fn new(
        path: PathBuf,
        line: usize,
        id: isize,
        attributes: Vec<(String, usize, String, usize)>,
    ) -> UninitialisedType {
        let mut attributes_processed = Vec::new();
        for (attr_name, _attr_line, attr_type, attr_type_line) in attributes {
            attributes_processed.push((attr_name, Err((attr_type, attr_type_line))));
        }

        UninitialisedType {
            path,
            line,
            id,
            attributes: attributes_processed,
        }
    }

    // pub fn to_initialised(self) -> Result<UserType, ProcessorError> {
    //     let mut attributes_processed = Vec::new();
    //     for (attr_name, attr_type) in self.attributes {
    //         if attr_type.is_err() {
    //             let (attr_type, attr_type_line) = attr_type.unwrap_err();
    //             return Err(TypeNotFoundError(self.path, attr_type_line, attr_type));
    //         }
    //
    //         attributes_processed.push((attr_name, attr_type.unwrap()))
    //     }
    //
    //     Ok(UserType::new(self.name, self.id, attributes_processed))
    // }
}

pub struct UserType {
    name: String,
    id: isize,
    path: PathBuf,
    line: usize,
    attributes: Vec<(String, isize)>
}

impl UserType {
    pub fn new(name: String, id: isize, path: PathBuf, line: usize, attributes: Vec<(String, isize)>) -> UserType {
        UserType {
            name,
            id,
            path,
            line,
            attributes
        }
    }
}

impl Type for UserType {
    fn get_id(&self) -> isize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_size(&self, type_table: &TypeTable, mut path: Option<Vec<isize>>) -> Result<usize, ProcessorError> {
        if path.is_none() {
            path = Some(vec![self.get_id()])
        }
        else {
            for id in & **path.as_ref().unwrap() {
                if *id == self.get_id() {
                    return Err(ProcessorError::CircularType(self.path.clone(), self.line, self.name.clone()));
                }
            }

            path.as_mut().unwrap().push(self.get_id());
        };

        let mut size = 0;

        for (name, id) in &self.attributes {
            size += type_table.get_type(*id).unwrap().get_size(type_table, path.clone())?;
        }

        Ok(size)
    }

    fn instantiate(&self, literal: Option<&Literal>, local_address: isize) -> Result<Vec<String>, ProcessorError> {
        todo!()
    }
}

pub trait Type {
    fn get_id(&self) -> isize;

    fn get_name(&self) -> &str;

    fn get_size(&self, type_table: &TypeTable,  path: Option<Vec<isize>>) -> Result<usize, ProcessorError>;

    fn instantiate(&self, literal: Option<&Literal>, local_address: isize) -> Result<Vec<String>, ProcessorError>;
}

pub struct TypeTable {
    types: HashMap<isize, Box<dyn Type>>,
}

impl TypeTable {
    pub fn new() -> TypeTable {
        TypeTable {
            types: HashMap::new(),
        }
    }

    pub fn add_builtin(mut self) -> TypeTable {
        self.add_type(Int::new().get_id(), Box::new(Int::new()));
        self.add_type(Bool::new().get_id(), Box::new(Bool::new()));
        self
    }

    pub fn add_type(&mut self, id: isize, type_: Box<dyn Type>) {
        if self.types.insert(id, type_).is_some() {
            panic!("Attempted to override type")
        }
    }

    pub fn get_id_by_name(&self, name: &str) -> Option<isize> {
        for (id, type_) in &self.types {
            if type_.get_name() == name {
                return Some(*id);
            }
        }
        None
    }

    pub fn get_type(&self, id: isize) -> Option<&Box<dyn Type>> {
        self.types.get(&id)
    }

    pub fn get_type_size(&self, id: isize) -> Result<usize, ProcessorError> {
        self.types.get(&id).unwrap().get_size(&self, None)
    }
}

#[derive(Debug)]
pub struct UserTypedFunction {
    pub id: isize,
    pub name: String,
    pub args: Vec<(String, isize)>,
    pub return_type: Option<isize>,
    pub contents: Option<Vec<BasicSymbol>>
}

impl TypedFunction for UserTypedFunction {
    fn get_id(&self) -> isize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_args(&self) -> &[(String, isize)] {
        &self.args
    }

    fn get_return_type(&self) -> Option<isize> {
        self.return_type
    }

    fn is_inline(&self) -> bool {
        false
    }

    fn contents(&self) -> &Vec<BasicSymbol> {
        self.contents.as_ref().unwrap()
    }

    fn take_contents(&mut self) -> Vec<BasicSymbol> {
        self.contents.take().unwrap()
    }

    fn get_inline(&self, args: Vec<isize>) -> Vec<String> {
        panic!()
    }
}

pub trait TypedFunction {
    fn get_id(&self) -> isize;
    fn get_name(&self) -> &str;
    fn get_args(&self) -> &[(String, isize)];
    fn get_args_positioned(&self, type_table: &TypeTable) -> Vec<(String, isize, isize)> {
        let mut offset = 16isize;
        let mut output = Vec::new();

        for (name, _type) in self.get_args() {
            output.push((name.clone(), offset, *_type));
            offset += type_table.get_type_size(*_type).unwrap() as isize;
        }

        output
    }
    fn get_return_type(&self) -> Option<isize>;
    fn is_inline(&self) -> bool;
    fn contents(&self) -> &Vec<BasicSymbol>;
    fn take_contents(&mut self) -> Vec<BasicSymbol>;
    fn get_inline(&self, args: Vec<isize>) -> Vec<String>;
}

// #[derive(Debug)]
// pub enum TypedImplsFns {
//     Impl(isize, Vec<TypedFunction>),
//     Fn(TypedFunction)
// }

pub fn build_types(
    pre_ast: Vec<(PreprocessSymbol, usize)>,
) -> Result<(TypeTable, HashMap<Option<isize>, HashMap<String, isize>>, HashMap<isize, Box<dyn TypedFunction>>), ProcessorError> {
    let mut remaining_pre_ast = Vec::new();

    let mut uninitialised_types: HashMap<String, UninitialisedType> = HashMap::new();
    let mut type_counter = 0isize;

    let mut type_table = TypeTable::new().add_builtin();

    for (symbol, line) in pre_ast {
        match symbol {
            PreprocessSymbol::Struct(path, name, args) => {
                if let Some(existing) = uninitialised_types.insert(
                    name.clone(),
                    UninitialisedType::new(path.clone(), line, type_counter, args),
                ) {
                    return Err(ProcessorError::TypeRedefinition(
                        path,
                        line,
                        name,
                        existing.path,
                        existing.line,
                    ));
                }
                type_counter += 1;
            }
            other => remaining_pre_ast.push((other, line)),
        }
    }

    let mut uninitialised_types: Vec<_> = uninitialised_types.into_iter().collect();

    for i in 0..uninitialised_types.len() {
        'attr_loop: for a in 0..uninitialised_types[i].1.attributes.len() {
            for j in 0..uninitialised_types.len() {
                if uninitialised_types[i].1.attributes[a]
                    .1
                    .as_ref()
                    .unwrap_err()
                    .0
                    == uninitialised_types[j].0
                {
                    uninitialised_types[i].1.attributes[a].1 = Ok(uninitialised_types[j].1.id);
                    continue 'attr_loop;
                }
            }

            if let Some(id) = type_table.get_id_by_name(
                &uninitialised_types[i].1.attributes[a]
                    .1
                    .as_ref()
                    .unwrap_err()
                    .0,
            ) {
                uninitialised_types[i].1.attributes[a].1 = Ok(id);
                continue 'attr_loop;
            }

            let type_ = uninitialised_types.remove(i).1;
            let (path, mut attributes) = (type_.path, type_.attributes);
            let (type_name, line) = attributes.remove(a).1.unwrap_err();
            return Err(TypeNotFound(path, line, type_name));
        }
    }

    for (name, type_) in uninitialised_types {
        let (_id, path, attributes) = (type_.id, type_.path, type_.attributes);

        let mut attributes_processed = Vec::new();
        for (attr_name, attr_type) in attributes {
            if attr_type.is_err() {
                let (attr_type, attr_type_line) = attr_type.unwrap_err();
                return Err(TypeNotFound(path, attr_type_line, attr_type));
            }

            attributes_processed.push((attr_name, attr_type.unwrap()))
        }

        type_table.add_type(
            type_.id,
            Box::new(UserType::new(name, type_.id, path, type_.line, attributes_processed)),
        )
    }

    let mut typed_fns = HashMap::new();
    let mut fn_name_map = HashMap::new();
    fn_name_map.insert(None, HashMap::new());
    let mut id_counter: isize = 1;
    for (symbol, line) in remaining_pre_ast {
        match symbol {
            PreprocessSymbol::Impl(path, type_name, functions) => {
                let type_id = type_table.get_id_by_name(&type_name).ok_or(ProcessorError::BadImplType(path))?;
                if !fn_name_map.contains_key(&Some(type_id)) {
                    fn_name_map.insert(Some(type_id), HashMap::new());
                }
                for (function, line) in functions {
                    if let Some(existing) = fn_name_map.get_mut(&Some(type_id)).unwrap().insert(function.0.clone(), id_counter) {
                        return Err(ProcessorError::FunctionRedefinition);
                    }
                    typed_fns.insert(id_counter, process_function(function, &type_table, id_counter, Some(type_id))?);
                    id_counter += 1;
                }
            }
            PreprocessSymbol::Fn(path, function) => {
                let id = if &function.0 == "main" {
                    0
                }
                else {
                    id_counter += 1;
                    id_counter - 1
                };
                if let Some(existing) = fn_name_map.get_mut(&None).unwrap().insert(function.0.clone(), id) {
                    return Err(ProcessorError::FunctionRedefinition);
                }
                typed_fns.insert(id, process_function(function, &type_table, id, None)?);
                id_counter += 1;
            }
            _ => panic!("Expected Impl of Functions")
        }
    }

    if let Some(main) = typed_fns.get(&0) {
        if main.get_args().len() != 0 {
            return Err(ProcessorError::BadMainFunction("Main function cannot have arguments".to_string()))
        }
        if main.get_return_type() != Some(-1) {
            return Err(ProcessorError::BadMainFunction("Main must return an 'int'".to_string()))
        }
    }
    else {
        return Err(ProcessorError::NoMainFunction)
    }

    Ok((type_table, fn_name_map, typed_fns))
}

fn process_function(function: PreProcessFunction, type_table: &TypeTable, id: isize, impl_type: Option<isize>) -> Result<Box<dyn TypedFunction>, ProcessorError> {
    let (name, args, return_type, contents) = function;

    let mut args_processed = Vec::new();

    for (arg_name, arg_line, type_name, type_line) in args {
        for (existing_arg, _) in &args_processed {
            if &arg_name == existing_arg {
                return Err(ProcessorError::VariableAlreadyDefined(arg_name));
            }
        }
        args_processed.push((arg_name, type_table.get_id_by_name(&type_name).ok_or(TypeNotFound(PathBuf::from("TODO"), type_line, type_name))?));
    }

    let return_type = if let Some(type_name) = return_type {
        Some(type_table.get_id_by_name(&type_name).ok_or(TypeNotFound(PathBuf::from("TODO"), 999999, type_name))?)
    }
    else {
        None
    };

    Ok(Box::new(UserTypedFunction {
        id,
        name,
        args: args_processed,
        return_type,
        contents: Some(contents),
    }))
}