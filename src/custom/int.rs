use lazy_static::lazy_static;
use unique_type_id::UniqueTypeId;
use crate::compiler::default::get_local_address;
use crate::parser::line_info::LineInfo;
use crate::processor::custom_types::{Bool, Int};
use crate::processor::type_builder::TypedFunction;

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
pub struct IntLT {}
lazy_static! {
    static ref INT_LT_ARGS: [(String, isize); 2] = [
        (String::from("lhs"), Int::get_id()),
        (String::from("rhs"), Int::get_id())
    ];
}
impl TypedFunction for IntLT {
    fn get_id(&self) -> isize {
        -(Self::id().0 as isize)
    }

    fn get_name(&self) -> &str {
        "lt"
    }

    fn get_args(&self) -> &[(String, isize)] {
        INT_LT_ARGS.as_ref()
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
            format!("setle [{}]", get_local_address(args[2])),
        ]
    }
}

#[derive(UniqueTypeId)]
#[UniqueTypeIdType = "u16"]
pub struct IntGT {}
lazy_static! {
    static ref INT_GT_ARGS: [(String, isize); 2] = [
        (String::from("lhs"), Int::get_id()),
        (String::from("rhs"), Int::get_id())
    ];
}
impl TypedFunction for IntGT {
    fn get_id(&self) -> isize {
        -(Self::id().0 as isize)
    }

    fn get_name(&self) -> &str {
        "gt"
    }

    fn get_args(&self) -> &[(String, isize)] {
        INT_GT_ARGS.as_ref()
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
            "cmp rax, rcx".to_string(),
            format!("setle [{}]", get_local_address(args[2])),
        ]
    }
}

#[derive(UniqueTypeId)]
#[UniqueTypeIdType = "u16"]
pub struct IntLE {}
lazy_static! {
    static ref INT_LE_ARGS: [(String, isize); 2] = [
        (String::from("lhs"), Int::get_id()),
        (String::from("rhs"), Int::get_id())
    ];
}
impl TypedFunction for IntLE {
    fn get_id(&self) -> isize {
        -(Self::id().0 as isize)
    }

    fn get_name(&self) -> &str {
        "le"
    }

    fn get_args(&self) -> &[(String, isize)] {
        INT_LE_ARGS.as_ref()
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
            "cmp rax, rcx".to_string(),
            format!("setnle [{}]", get_local_address(args[2])),
        ]
    }
}

#[derive(UniqueTypeId)]
#[UniqueTypeIdType = "u16"]
pub struct IntGE {}
lazy_static! {
    static ref INT_GE_ARGS: [(String, isize); 2] = [
        (String::from("lhs"), Int::get_id()),
        (String::from("rhs"), Int::get_id())
    ];
}
impl TypedFunction for IntGE {
    fn get_id(&self) -> isize {
        -(Self::id().0 as isize)
    }

    fn get_name(&self) -> &str {
        "ge"
    }

    fn get_args(&self) -> &[(String, isize)] {
        INT_GE_ARGS.as_ref()
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
            format!("setnle [{}]", get_local_address(args[2])),
        ]
    }
}

#[derive(UniqueTypeId)]
#[UniqueTypeIdType = "u16"]
pub struct IntEQ {}
lazy_static! {
    static ref INT_EQ_ARGS: [(String, isize); 2] = [
        (String::from("lhs"), Int::get_id()),
        (String::from("rhs"), Int::get_id())
    ];
}
impl TypedFunction for IntEQ {
    fn get_id(&self) -> isize {
        -(Self::id().0 as isize)
    }

    fn get_name(&self) -> &str {
        "eq"
    }

    fn get_args(&self) -> &[(String, isize)] {
        INT_EQ_ARGS.as_ref()
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

#[derive(UniqueTypeId)]
#[UniqueTypeIdType = "u16"]
pub struct IntNE {}
lazy_static! {
    static ref INT_NE_ARGS: [(String, isize); 2] = [
        (String::from("lhs"), Int::get_id()),
        (String::from("rhs"), Int::get_id())
    ];
}
impl TypedFunction for IntNE {
    fn get_id(&self) -> isize {
        -(Self::id().0 as isize)
    }

    fn get_name(&self) -> &str {
        "ne"
    }

    fn get_args(&self) -> &[(String, isize)] {
        INT_NE_ARGS.as_ref()
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
