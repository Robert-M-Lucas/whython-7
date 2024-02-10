use lazy_static::lazy_static;
use crate::basic_ast::symbol::BasicSymbol;
use crate::compiler::compile_functions::{Function, Line, UserFunction};
use crate::compiler::default::{compile_user_function, get_function_sublabel, get_local_address, Output};
use crate::processor::custom_types::Int;
use crate::processor::type_builder::{Type, TypedFunction};

pub fn get_custom_function_signatures() -> Vec<(Option<isize>, Box<dyn TypedFunction>)> {
    vec![
        (None, Box::new(WindowsExit{})),
        (None, Box::new(PrintI{}))
    ]
}

pub fn get_custom_function_implementations() -> Vec<Box<dyn Function>> {
    vec![
        Box::new(PrintI{})
    ]
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

    fn contents(&self) -> &Vec<BasicSymbol> {
        panic!()
    }

    fn take_contents(&mut self) -> Vec<BasicSymbol> {
        panic!()
    }

    fn get_inline(&self, args: Vec<isize>) -> Vec<String> {
        let mut output = Vec::new();
        output.push(format!("mov rcx, [{}]", crate::compiler::default::get_local_address(args[0])));
        output.push("call ExitProcess".to_string());
        output
    }
}

pub struct PrintI {}
lazy_static! {
    static ref PRINT_I_ARGS: [(String, isize); 1] = [(String::from("integer"), Int::get_id())];
}
impl TypedFunction for PrintI {
    fn get_id(&self) -> isize {
        -1
    }

    fn get_name(&self) -> &str {
        "printi"
    }

    fn get_args(&self) -> &[(String, isize)] {
        PRINT_I_ARGS.as_ref()
    }

    fn get_return_type(&self) -> Option<isize> {
        None
    }

    fn is_inline(&self) -> bool {
        false
    }

    fn contents(&self) -> &Vec<BasicSymbol> {
        panic!()
    }

    fn take_contents(&mut self) -> Vec<BasicSymbol> {
        panic!()
    }

    fn get_inline(&self, args: Vec<isize>) -> Vec<String> {
        panic!()
    }
}

impl Function for PrintI {
    fn get_asm(&self) -> String {
        compile_user_function(
            &UserFunction {
                id: -1,
                local_variable_count: 1,
                arg_count: 1,
                lines: vec![
                    Line::InlineAsm(vec![
                        // "mov rcx, rsp".to_string(),
                        // format!("mov rax, [{}]", get_local_address(0)),
                        // "mov rbx, 8".to_string(),
                        "mov byte [rbp-4], 4Eh".to_string(),
                        // format!("{}:", get_function_sublabel(TypedFunction::get_id(self), "loop")),
                        // "xor rdx, rdx".to_string(),
                        // "div rbx".to_string(),
                        // "dec rcx".to_string(),
                        // "add rdx, '0'".to_string(),
                        // "mov [rcx], dl".to_string(),
                        // "test rax, rax".to_string(),
                        // format!("jnz {}", get_function_sublabel(TypedFunction::get_id(self), "loop")),
                        "sub     rsp, 56".to_string(),
                        "mov     qword [rsp + 40], 0eh ".to_string(),
                        "mov     ecx, -11".to_string(),
                        "call    GetStdHandle".to_string(),
                        "mov     rcx, rax".to_string(),
                        "mov     rdx, [rbp - 8]".to_string(),
                        "mov     r8, [rsp + 40]".to_string(),
                        "mov     r9, 01h".to_string(),
                        "mov     qword [rsp + 32], 00h ".to_string(),
                        "call    WriteFile".to_string(),
                        "add     rsp, 56".to_string(),
                    ])
                ],
            }
        )
    }

    fn get_id(&self) -> isize {
        TypedFunction::get_id(self)
    }
}