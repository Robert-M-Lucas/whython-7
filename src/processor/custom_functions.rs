use crate::processor::function_processor::Function;
use crate::processor::type_builder::TypedFunction;

pub fn get_custom_function_signatures() -> Vec<(Option<isize>, TypedFunction)> {
    Vec::new()
}

pub fn get_custom_function_implementations() -> Vec<Box<dyn Function>> {
    Vec::new()
}