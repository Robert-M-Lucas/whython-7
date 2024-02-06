use lazy_static::lazy_static;
use crate::basic_ast::symbol::BasicSymbol;
use crate::compiler::compile_functions::Function;
use crate::compiler::default::Output;
use crate::processor::custom_types::Int;
use crate::processor::type_builder::{Type, TypedFunction};

pub fn get_custom_function_signatures() -> Vec<(Option<isize>, Box<dyn TypedFunction>)> {
    vec![
        (None, Box::new(WindowsExit{}))
    ]
}

pub fn get_custom_function_implementations() -> Vec<Box<dyn Function>> {
    Vec::new()
}

pub struct WindowsExit {}
lazy_static! {
    static ref WINDOWS_EXIT_ARGS: [(String, isize); 1] = [(String::from("exit_code"), Int::get_id())];
}
impl TypedFunction for WindowsExit {
    fn get_id(&self) -> isize {
        -1_000_000
    }

    fn get_name(&self) -> &str {
        "exit"
    }

    fn get_args(&self) -> &[(String, isize)] {
        WINDOWS_EXIT_ARGS.as_ref()
    }

    fn get_return_type(&self) -> Option<isize> {
        None
    }

    fn is_inline(&self) -> bool {
        true
    }

    fn get_contents(&self) -> &Vec<(BasicSymbol, usize)> {
        panic!()
    }

    fn get_inline(&self, args: Vec<isize>) -> String {
        let mut output = Output::new();
        output.push(&format!("mov rcx, [{}]", crate::compiler::default::get_local_address(args[0])));
        output.push("call ExitProcess");
        output.into()
    }
}