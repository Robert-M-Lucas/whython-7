use crate::processor::preprocess::PreprocessSymbol;
use crate::processor::processor::ProcessorError;
use crate::processor::processor::ProcessorError::TypeNotFound;

use std::collections::HashMap;

use std::path::PathBuf;

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
    attributes: Vec<(String, isize)>,
}

impl UserType {
    pub fn new(name: String, id: isize, attributes: Vec<(String, isize)>) -> UserType {
        UserType {
            name,
            id,
            attributes,
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

    fn get_function(&self) {}
}

pub trait Type {
    fn get_id(&self) -> isize;

    fn get_name(&self) -> &str;

    fn get_function(&self);
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

    pub fn add_builtin(self) -> TypeTable {
        self
    }

    pub fn add_type(&mut self, id: isize, type_: Box<dyn Type>) {
        if self.types.insert(id, type_).is_some() {
            panic!("Attempted to override type")
        }
    }

    pub fn get_id_by_name(&self, name: &str) -> Result<isize, ()> {
        for (id, type_) in &self.types {
            if type_.get_name() == name {
                return Ok(*id);
            }
        }
        Err(())
    }
}

pub fn build_type_table(
    pre_ast: Vec<(PreprocessSymbol, usize)>,
) -> Result<(TypeTable, Vec<(PreprocessSymbol, usize)>), ProcessorError> {
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

            if let Ok(id) = type_table.get_id_by_name(
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
            Box::new(UserType::new(name, type_.id, attributes_processed)),
        )
    }

    Ok((type_table, remaining_pre_ast))
}
