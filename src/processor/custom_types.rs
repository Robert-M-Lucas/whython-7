use crate::processor::processor::ProcessorError;
use crate::processor::type_builder::{Type, TypeTable};

pub struct Bool {

}

impl Bool {
    pub fn new() -> Bool {
        Bool {}
    }
}

impl Type for Bool {
    fn get_id(&self) -> isize {
        -1
    }

    fn get_name(&self) -> &str {
        "bool"
    }

    fn get_function(&self) {

    }

    fn get_size(&self, type_table: &TypeTable, path: Option<Vec<isize>>) -> Result<usize, ProcessorError> {
        Ok(1)
    }
}