use lazy_static::lazy_static;
use unique_type_id::UniqueTypeId;

use crate::root::basic_ast::symbol::BasicSymbol;
use crate::root::compiler::compile_functions::{Function, Line, UserFunction};
use crate::root::compiler::generate_asm::{compile_user_function, get_function_sublabel};
use crate::root::custom::types::bool::Bool;
use crate::root::custom::types::float::Float;
use crate::root::custom::types::int::Int;
use crate::root::parser::line_info::LineInfo;
use crate::root::processor::type_builder::TypedFunction;

pub fn add_function_signatures(existing: &mut Vec<(Option<isize>, Box<dyn TypedFunction>)>) {
    let signatures: [(Option<isize>, Box<dyn TypedFunction>); 3] = [
        (None, Box::new(PrintI {})),
        (None, Box::new(PrintB {})),
        (None, Box::new(PrintF {})),
    ];

    for s in signatures {
        existing.push(s);
    }
}

pub fn add_function_implementations(existing: &mut Vec<Box<dyn Function>>) {
    let functions: [Box<dyn Function>; 3] = [
        Box::new(PrintI {}),
        Box::new(PrintB {}),
        Box::new(PrintF{})
    ];

    for s in functions {
        existing.push(s);
    }
}


#[derive(UniqueTypeId)]
#[UniqueTypeIdType = "u16"]
pub struct PrintI {}
lazy_static! {
    static ref PRINT_I_ARGS: [(String, (isize, usize)); 1] =
        [(String::from("integer"), (Int::get_id(), 0))];
}
impl TypedFunction for PrintI {
    fn get_id(&self) -> isize {
        -(Self::id().0 as isize) - 1
    }

    fn get_name(&self) -> &str {
        "printi"
    }

    fn get_args(&self) -> &[(String, (isize, usize))] {
        PRINT_I_ARGS.as_ref()
    }

    fn get_line(&self) -> LineInfo {
        LineInfo::builtin()
    }

    fn get_return_type(&self) -> Option<(isize, usize)> {
        None
    }

    fn is_inline(&self) -> bool {
        false
    }

    fn contents(&self) -> &Vec<(BasicSymbol, LineInfo)> {
        panic!()
    }

    fn take_contents(&mut self) -> Vec<(BasicSymbol, LineInfo)> {
        panic!()
    }

    fn get_inline(&self, _args: Vec<isize>) -> Vec<String> {
        panic!()
    }
}

impl Function for PrintI {
    fn get_asm(&self) -> String {
        compile_user_function(&UserFunction {
            id: TypedFunction::get_id(self),
            local_variable_size: 48,
            arg_count: 1,
            lines: vec![Line::InlineAsm(vec![
                "sub rsp, 32".to_string(),
                "mov rcx, rbp".to_string(),
                "dec rcx".to_string(),
                "mov rax, qword [rbp+16]".to_string(),
                "mov qword [rbp-24], \"\"".to_string(),
                "mov qword [rbp-16], \"\"".to_string(),
                "mov dword [rbp-8], \"\"".to_string(),
                "mov dword [rbp-4], `\\0\\0\\0\\n`".to_string(),
                "cmp rax, 0".to_string(),
                format!(
                    "jge {}",
                    get_function_sublabel(TypedFunction::get_id(self), "positive")
                ),
                "mov dword [rbp-20], \"-\"".to_string(),
                "mov r8, rax".to_string(),
                "mov rax, 0".to_string(),
                "sub rax, r8".to_string(),
                format!(
                    "{}:",
                    get_function_sublabel(TypedFunction::get_id(self), "positive")
                ),
                "mov rbx, 10".to_string(),
                format!(
                    "{}:",
                    get_function_sublabel(TypedFunction::get_id(self), "loop")
                ),
                "xor rdx, rdx".to_string(),
                "div rbx".to_string(),
                "dec rcx".to_string(),
                "add rdx, '0'".to_string(),
                "mov [rcx], dl".to_string(),
                "test.txt rax, rax".to_string(),
                format!(
                    "jnz {}",
                    get_function_sublabel(TypedFunction::get_id(self), "loop")
                ),
                "mov ecx, -11".to_string(), // Get std handle (32 bit arg)
                "call GetStdHandle".to_string(), // Get
                //; You have to reserve space for these despite not being on the stack!
                "mov rcx, rax".to_string(),            // ; STD Handle
                "mov rdx, rbp".to_string(),            // ; Data pointer
                "sub rdx, 24".to_string(),             // ; cont.
                "mov r8, 24".to_string(),              // ; Bytes to write
                "mov qword [rbp - 40], 0".to_string(), // ; optional out bytes written
                "mov r9, rbp".to_string(),             //
                "sub r9, 24".to_string(),              // ; contd.
                "mov qword [rbp - 48], 0".to_string(), // ; optional lpOverlapped
                "call WriteFile".to_string(),
            ])],
            name: "printi".to_string(),
        })
    }

    fn get_id(&self) -> isize {
        TypedFunction::get_id(self)
    }
}

#[derive(UniqueTypeId)]
#[UniqueTypeIdType = "u16"]
pub struct PrintB {}
lazy_static! {
    static ref PRINT_B_ARGS: [(String, (isize, usize)); 1] =
        [(String::from("bool"), (Bool::get_id(), 0))];
}
impl TypedFunction for PrintB {
    fn get_id(&self) -> isize {
        -(Self::id().0 as isize) - 1
    }

    fn get_name(&self) -> &str {
        "printb"
    }

    fn get_args(&self) -> &[(String, (isize, usize))] {
        PRINT_B_ARGS.as_ref()
    }

    fn get_line(&self) -> LineInfo {
        LineInfo::builtin()
    }

    fn get_return_type(&self) -> Option<(isize, usize)> {
        None
    }

    fn is_inline(&self) -> bool {
        false
    }
}

impl Function for PrintB {
    fn get_asm(&self) -> String {
        compile_user_function(&UserFunction {
            id: TypedFunction::get_id(self),
            local_variable_size: 32,
            arg_count: 1,
            lines: vec![Line::InlineAsm(vec![
                "sub rsp, 32".to_string(),
                "mov qword [rbp-16], \"true\"".to_string(),
                "mov qword [rbp-8], `\\r\\n`".to_string(),
                "mov rax, qword [rbp+16]".to_string(),
                "cmp rax, 0".to_string(),
                format!(
                    "jz {}",
                    get_function_sublabel(TypedFunction::get_id(self), "true")
                ),
                "mov qword [rbp-16], \"fals\"".to_string(),
                "mov qword [rbp-8], `e\\r\\n`".to_string(),
                format!(
                    "{}:",
                    get_function_sublabel(TypedFunction::get_id(self), "true")
                ),
                "mov ecx, -11".to_string(), // Get std handle (32 bit arg)
                "call GetStdHandle".to_string(), // Get
                //; You have to reserve space for these despite not being on the stack!
                "mov rcx, rax".to_string(),            // ; STD Handle
                "mov rdx, rbp".to_string(),            // ; Data pointer
                "sub rdx, 16".to_string(),             // ; cont.
                "mov r8, 16".to_string(),              // ; Bytes to write
                "mov qword [rbp - 24], 0".to_string(), // ; optional out bytes written
                "mov r9, rbp".to_string(),             //
                "sub r9, 24".to_string(),              // ; contd.
                "mov qword [rbp - 32], 0".to_string(), // ; optional lpOverlapped
                "call WriteFile".to_string(),
            ])],
            name: "printb".to_string(),
        })
    }

    fn get_id(&self) -> isize {
        TypedFunction::get_id(self)
    }
}

#[derive(UniqueTypeId)]
#[UniqueTypeIdType = "u16"]
pub struct PrintF {}
lazy_static! {
    static ref PRINT_F_ARGS: [(String, (isize, usize)); 1] =
        [(String::from("float"), (Float::get_id(), 0))];
}
impl TypedFunction for PrintF {
    fn get_id(&self) -> isize {
        -(Self::id().0 as isize) - 1
    }

    fn get_name(&self) -> &str {
        "printf"
    }

    fn get_args(&self) -> &[(String, (isize, usize))] {
        PRINT_F_ARGS.as_ref()
    }

    fn get_line(&self) -> LineInfo {
        LineInfo::builtin()
    }

    fn get_return_type(&self) -> Option<(isize, usize)> {
        None
    }

    fn is_inline(&self) -> bool {
        false
    }
}

impl Function for PrintF {
    fn get_asm(&self) -> String {
        compile_user_function(&UserFunction {
            id: TypedFunction::get_id(self),
            local_variable_size: 32,
            arg_count: 1,
            lines: vec![Line::InlineAsm(vec![
                "mov dword [rbp-4], 0x00".to_string(),
                "mov dword [rbp-8], 0x0a666C25".to_string(),
                "mov rcx, rbp".to_string(),
                "sub rcx, 8".to_string(),
                "movq xmm1, qword [rbp+16]".to_string(),
                "movq rdx, xmm1".to_string(),
                "sub rsp, 40".to_string(),
                "call printf".to_string(),
                "add rsp, 40".to_string()
            ])],
            name: "printf".to_string(),
        })
    }

    fn get_id(&self) -> isize {
        TypedFunction::get_id(self)
    }
}
