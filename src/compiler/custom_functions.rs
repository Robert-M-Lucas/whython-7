use crate::basic_ast::symbol::BasicSymbol;
use crate::compiler::compile_functions::{Function, Line, UserFunction};
use crate::compiler::default::{
    compile_user_function, get_function_sublabel, get_local_address, Output,
};
use crate::parser::line_info::LineInfo;
use crate::processor::custom_types::{Bool, Int};
use crate::processor::type_builder::{Type, TypedFunction};
use lazy_static::lazy_static;
use unique_type_id::UniqueTypeId;

pub fn get_custom_function_signatures() -> Vec<(Option<isize>, Box<dyn TypedFunction>)> {
    vec![
        (None, Box::new(WindowsExit {})),
        (None, Box::new(PrintI {})),
        (None, Box::new(PrintB {})),
        (Some(-1), Box::new(IntAdd {})),
        (Some(-1), Box::new(IntSub {})),
        (Some(-1), Box::new(IntMul {})),
        (Some(-1), Box::new(IntDiv {})),
        (Some(-2), Box::new(BoolNot {})),
    ]
}

pub fn get_custom_function_implementations() -> Vec<Box<dyn Function>> {
    vec![Box::new(PrintI {}), Box::new(PrintB {})]
}

#[derive(UniqueTypeId)]
#[UniqueTypeIdType = "u16"]
pub struct WindowsExit {}
lazy_static! {
    static ref WINDOWS_EXIT_ARGS: [(String, isize); 1] =
        [(String::from("exit_code"), Int::get_id())];
}
impl TypedFunction for WindowsExit {
    fn get_id(&self) -> isize {
        -(Self::id().0 as isize)
    }

    fn get_name(&self) -> &str {
        "exit"
    }

    fn get_args(&self) -> &[(String, isize)] {
        WINDOWS_EXIT_ARGS.as_ref()
    }

    fn get_line(&self) -> LineInfo {
        LineInfo::builtin()
    }

    fn get_return_type(&self) -> Option<isize> {
        None
    }

    fn is_inline(&self) -> bool {
        true
    }

    fn contents(&self) -> &Vec<(BasicSymbol, LineInfo)> {
        panic!()
    }

    fn take_contents(&mut self) -> Vec<(BasicSymbol, LineInfo)> {
        panic!()
    }

    fn get_inline(&self, args: Vec<isize>) -> Vec<String> {
        vec![
            format!(
                "mov rcx, [{}]",
                crate::compiler::default::get_local_address(args[0])
            ),
            "call ExitProcess".to_string(),
        ]
    }
}

#[derive(UniqueTypeId)]
#[UniqueTypeIdType = "u16"]
pub struct PrintI {}
lazy_static! {
    static ref PRINT_I_ARGS: [(String, isize); 1] = [(String::from("integer"), Int::get_id())];
}
impl TypedFunction for PrintI {
    fn get_id(&self) -> isize {
        -(Self::id().0 as isize)
    }

    fn get_name(&self) -> &str {
        "printi"
    }

    fn get_args(&self) -> &[(String, isize)] {
        PRINT_I_ARGS.as_ref()
    }

    fn get_line(&self) -> LineInfo {
        LineInfo::builtin()
    }

    fn get_return_type(&self) -> Option<isize> {
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

    fn get_inline(&self, args: Vec<isize>) -> Vec<String> {
        panic!()
    }
}

impl Function for PrintI {
    fn get_asm(&self) -> String {
        compile_user_function(&UserFunction {
            id: TypedFunction::get_id(self),
            local_variable_count: 2,
            arg_count: 1,
            lines: vec![Line::InlineAsm(vec![
                "mov rcx, rbp".to_string(),
                "dec rcx".to_string(),
                "dec rcx".to_string(),
                "mov qword [rbp-16], 0h".to_string(),
                "mov qword [rbp-8], 0000000000000D0Ah".to_string(),
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
                "test rax, rax".to_string(),
                format!(
                    "jnz {}",
                    get_function_sublabel(TypedFunction::get_id(self), "loop")
                ),
                "sub rsp, 48".to_string(),
                "mov ecx, -11".to_string(),
                "call GetStdHandle".to_string(),
                "mov rcx, rax".to_string(),
                "mov rdx, rbp ".to_string(),
                "sub rdx, 16".to_string(),
                "mov qword [rsp + 40], 10h".to_string(),
                "mov r8, [rsp + 40]".to_string(),
                "mov r9, dword 00h".to_string(),
                "mov qword [rsp + 32], 00h".to_string(),
                "call WriteFile".to_string(),
                "add rsp, 48".to_string(),
            ])],
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
    static ref PRINT_B_ARGS: [(String, isize); 1] = [(String::from("bool"), Bool {}.get_id())];
}
impl TypedFunction for PrintB {
    fn get_id(&self) -> isize {
        -(Self::id().0 as isize)
    }

    fn get_name(&self) -> &str {
        "printb"
    }

    fn get_args(&self) -> &[(String, isize)] {
        crate::compiler::custom_functions::PRINT_B_ARGS.as_ref()
    }

    fn get_line(&self) -> LineInfo {
        LineInfo::builtin()
    }

    fn get_return_type(&self) -> Option<isize> {
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

    fn get_inline(&self, args: Vec<isize>) -> Vec<String> {
        panic!()
    }
}

impl Function for PrintB {
    fn get_asm(&self) -> String {
        compile_user_function(&UserFunction {
            id: TypedFunction::get_id(self),
            local_variable_count: 2,
            arg_count: 1,
            lines: vec![Line::InlineAsm(vec![
                "mov rcx, rbp".to_string(),
                "sub rcx, 8".to_string(),
                "mov qword [rbp-16], rcx".to_string(),
                "mov qword [rbp-8], 0000657572740A0Dh".to_string(),
                "mov rax, [rbp+16]".to_string(),
                "cmp rax, 0".to_string(),
                format!(
                    "jz {}",
                    get_function_sublabel(TypedFunction::get_id(self), "true")
                ),
                "mov qword [rbp-8], 0065736C61660A0Dh".to_string(),
                format!(
                    "{}:",
                    get_function_sublabel(TypedFunction::get_id(self), "true")
                ),
                "sub rsp, 48".to_string(),
                "mov ecx, -11".to_string(),
                "call GetStdHandle".to_string(),
                "mov rcx, rax".to_string(),
                "mov rdx, [rbp-16]".to_string(),
                // "sub rdx, 8".to_string(),
                "mov qword [rsp + 40], 08h".to_string(),
                "mov r8, [rsp + 40]".to_string(),
                "mov r9, dword 00h".to_string(),
                "mov qword [rsp + 32], 00h".to_string(),
                "call WriteFile".to_string(),
                "add rsp, 48".to_string(),
            ])],
        })
    }

    fn get_id(&self) -> isize {
        TypedFunction::get_id(self)
    }
}

#[derive(UniqueTypeId)]
#[UniqueTypeIdType = "u16"]
pub struct IntAdd {}
lazy_static! {
    static ref INT_ADD_ARGS: [(String, isize); 2] = [
        (String::from("lhs"), Int::get_id()),
        (String::from("rhs"), Int::get_id())
    ];
}
impl TypedFunction for IntAdd {
    fn get_id(&self) -> isize {
        -(Self::id().0 as isize)
    }

    fn get_name(&self) -> &str {
        "add"
    }

    fn get_args(&self) -> &[(String, isize)] {
        INT_ADD_ARGS.as_ref()
    }

    fn get_line(&self) -> LineInfo {
        LineInfo::builtin()
    }

    fn get_return_type(&self) -> Option<isize> {
        Some(Int::get_id())
    }

    fn is_inline(&self) -> bool {
        true
    }

    fn contents(&self) -> &Vec<(BasicSymbol, LineInfo)> {
        panic!()
    }

    fn take_contents(&mut self) -> Vec<(BasicSymbol, LineInfo)> {
        panic!()
    }

    fn get_inline(&self, args: Vec<isize>) -> Vec<String> {
        vec![
            format!("mov rax, [{}]", get_local_address(args[0])),
            format!("add rax, [{}]", get_local_address(args[1])),
            format!("mov [{}], rax", get_local_address(args[2])),
        ]
    }
}

#[derive(UniqueTypeId)]
#[UniqueTypeIdType = "u16"]
pub struct IntSub {}
lazy_static! {
    static ref INT_SUB_ARGS: [(String, isize); 2] = [
        (String::from("lhs"), Int::get_id()),
        (String::from("rhs"), Int::get_id())
    ];
}
impl TypedFunction for IntSub {
    fn get_id(&self) -> isize {
        -(Self::id().0 as isize)
    }

    fn get_name(&self) -> &str {
        "sub"
    }

    fn get_args(&self) -> &[(String, isize)] {
        INT_SUB_ARGS.as_ref()
    }

    fn get_line(&self) -> LineInfo {
        LineInfo::builtin()
    }

    fn get_return_type(&self) -> Option<isize> {
        Some(Int::get_id())
    }

    fn is_inline(&self) -> bool {
        true
    }

    fn contents(&self) -> &Vec<(BasicSymbol, LineInfo)> {
        panic!()
    }

    fn take_contents(&mut self) -> Vec<(BasicSymbol, LineInfo)> {
        panic!()
    }

    fn get_inline(&self, args: Vec<isize>) -> Vec<String> {
        vec![
            format!("mov rax, [{}]", get_local_address(args[0])),
            format!("sub rax, [{}]", get_local_address(args[1])),
            format!("mov [{}], rax", get_local_address(args[2])),
        ]
    }
}

#[derive(UniqueTypeId)]
#[UniqueTypeIdType = "u16"]
pub struct IntMul {}
lazy_static! {
    static ref INT_MUL_ARGS: [(String, isize); 2] = [
        (String::from("lhs"), Int::get_id()),
        (String::from("rhs"), Int::get_id())
    ];
}
impl TypedFunction for IntMul {
    fn get_id(&self) -> isize {
        -(Self::id().0 as isize)
    }

    fn get_name(&self) -> &str {
        "mul"
    }

    fn get_args(&self) -> &[(String, isize)] {
        crate::compiler::custom_functions::INT_SUB_ARGS.as_ref()
    }

    fn get_line(&self) -> LineInfo {
        LineInfo::builtin()
    }

    fn get_return_type(&self) -> Option<isize> {
        Some(Int::get_id())
    }

    fn is_inline(&self) -> bool {
        true
    }

    fn contents(&self) -> &Vec<(BasicSymbol, LineInfo)> {
        panic!()
    }

    fn take_contents(&mut self) -> Vec<(BasicSymbol, LineInfo)> {
        panic!()
    }

    fn get_inline(&self, args: Vec<isize>) -> Vec<String> {
        vec![
            format!("mov rax, [{}]", get_local_address(args[0])),
            format!("mov rcx, [{}]", get_local_address(args[1])),
            "mul rcx".to_string(),
            format!("mov [{}], rax", get_local_address(args[2])),
        ]
    }
}

#[derive(UniqueTypeId)]
#[UniqueTypeIdType = "u16"]
pub struct IntDiv {}
lazy_static! {
    static ref INT_DIV_ARGS: [(String, isize); 2] = [
        (String::from("lhs"), Int::get_id()),
        (String::from("rhs"), Int::get_id())
    ];
}
impl TypedFunction for IntDiv {
    fn get_id(&self) -> isize {
        -(Self::id().0 as isize)
    }

    fn get_name(&self) -> &str {
        "div"
    }

    fn get_args(&self) -> &[(String, isize)] {
        crate::compiler::custom_functions::INT_SUB_ARGS.as_ref()
    }

    fn get_line(&self) -> LineInfo {
        LineInfo::builtin()
    }

    fn get_return_type(&self) -> Option<isize> {
        Some(Int::get_id())
    }

    fn is_inline(&self) -> bool {
        true
    }

    fn contents(&self) -> &Vec<(BasicSymbol, LineInfo)> {
        panic!()
    }

    fn take_contents(&mut self) -> Vec<(BasicSymbol, LineInfo)> {
        panic!()
    }

    fn get_inline(&self, args: Vec<isize>) -> Vec<String> {
        vec![
            format!("mov rax, [{}]", get_local_address(args[0])),
            format!("mov rcx, [{}]", get_local_address(args[1])),
            "div rcx".to_string(),
            format!("mov [{}], rax", get_local_address(args[2])),
        ]
    }
}

#[derive(UniqueTypeId)]
#[UniqueTypeIdType = "u16"]
pub struct BoolNot {}
lazy_static! {
    static ref BOOL_NOT_ARGS: [(String, isize); 1] = [(String::from("lhs"), Bool {}.get_id())];
}
impl TypedFunction for BoolNot {
    fn get_id(&self) -> isize {
        -(Self::id().0 as isize)
    }

    fn get_name(&self) -> &str {
        "not"
    }

    fn get_args(&self) -> &[(String, isize)] {
        BOOL_NOT_ARGS.as_ref()
    }

    fn get_line(&self) -> LineInfo {
        LineInfo::builtin()
    }

    fn get_return_type(&self) -> Option<isize> {
        Some(Bool::get_id())
    }

    fn is_inline(&self) -> bool {
        true
    }

    fn contents(&self) -> &Vec<(BasicSymbol, LineInfo)> {
        panic!()
    }

    fn take_contents(&mut self) -> Vec<(BasicSymbol, LineInfo)> {
        panic!()
    }

    fn get_inline(&self, args: Vec<isize>) -> Vec<String> {
        vec![
            format!("mov rax, [{}]", get_local_address(args[0])),
            "not rax".to_string(),
            format!("mov [{}], rax", get_local_address(args[1])),
        ]
    }
}

#[derive(UniqueTypeId)]
#[UniqueTypeIdType = "u16"]
pub struct IntLT {}
lazy_static! {
    static ref INT_LT_ARGS: [(String, isize); 2] = [
        (String::from("lhs"), Int {}.get_id()),
        (String::from("rhs"), Int {}.get_id())
    ];
}
// impl TypedFunction for IntLT {
//     fn get_id(&self) -> isize {
//         -(Self::id().0 as isize)
//     }
//
//     fn get_name(&self) -> &str {
//         "lt"
//     }
//
//     fn get_args(&self) -> &[(String, isize)] {
//         crate::compiler::custom_functions::BOOL_NOT_ARGS.as_ref()
//     }
//
//     fn get_return_type(&self) -> Option<isize> {
//         Some(Bool::get_id())
//     }
//
//     fn is_inline(&self) -> bool {
//         true
//     }
//
//     fn contents(&self) -> &Vec<BasicSymbol> {
//         panic!()
//     }
//
//     fn take_contents(&mut self) -> Vec<BasicSymbol> {
//         panic!()
//     }
//
//     fn get_inline(&self, args: Vec<isize>) -> Vec<String> {
//         vec![
//             format!("mov rax, [{}]", get_local_address(args[0])),
//             format!("mov rcx, [{}]", get_local_address(args[1])),
//             "cmp rcx".to_string(),
//             format!("mov [{}], rax", get_local_address(args[1])),
//         ]
//     }
// }
