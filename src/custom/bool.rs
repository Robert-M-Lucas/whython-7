use crate::compiler::generate_asm::get_local_address;
use crate::parser::line_info::LineInfo;
use crate::processor::custom_types::Bool;
use crate::processor::type_builder::TypedFunction;
use lazy_static::lazy_static;
use unique_type_id::UniqueTypeId;

#[derive(UniqueTypeId)]
#[UniqueTypeIdType = "u16"]
pub struct BoolNot {}
lazy_static! {
    static ref BOOL_NOT_ARGS: [(String, isize); 1] = [(String::from("lhs"), Bool::get_id())];
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

    fn get_inline(&self, args: Vec<isize>) -> Vec<String> {
        vec![
            format!("mov rax, [{}]", get_local_address(args[0])),
            "cmp rax, 0".to_string(),
            "setz rax".to_string(),
            format!("mov [{}], rax", get_local_address(args[1])),
        ]
    }
}

#[derive(UniqueTypeId)]
#[UniqueTypeIdType = "u16"]
pub struct BoolEQ {}
lazy_static! {
    static ref BOOL_EQ_ARGS: [(String, isize); 2] = [
        (String::from("lhs"), Bool::get_id()),
        (String::from("rhs"), Bool::get_id())
    ];
}
impl TypedFunction for BoolEQ {
    fn get_id(&self) -> isize {
        -(Self::id().0 as isize)
    }

    fn get_name(&self) -> &str {
        "eq"
    }

    fn get_args(&self) -> &[(String, isize)] {
        BOOL_EQ_ARGS.as_ref()
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

    fn get_inline(&self, args: Vec<isize>) -> Vec<String> {
        vec![
            format!("mov rax, [{}]", get_local_address(args[0])),
            format!("mov rcx, [{}]", get_local_address(args[1])),
            "cmp rcx, rax".to_string(),
            format!("setnz [{}]", get_local_address(args[2])),
        ]
    }
}

#[derive(UniqueTypeId)]
#[UniqueTypeIdType = "u16"]
pub struct BoolNE {}
lazy_static! {
    static ref BOOL_NE_ARGS: [(String, isize); 2] = [
        (String::from("lhs"), Bool::get_id()),
        (String::from("rhs"), Bool::get_id())
    ];
}
impl TypedFunction for BoolNE {
    fn get_id(&self) -> isize {
        -(Self::id().0 as isize)
    }

    fn get_name(&self) -> &str {
        "ne"
    }

    fn get_args(&self) -> &[(String, isize)] {
        BOOL_NE_ARGS.as_ref()
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

    fn get_inline(&self, args: Vec<isize>) -> Vec<String> {
        vec![
            format!("mov rax, [{}]", get_local_address(args[0])),
            format!("mov rcx, [{}]", get_local_address(args[1])),
            "cmp rcx, rax".to_string(),
            format!("setz [{}]", get_local_address(args[2])),
        ]
    }
}
