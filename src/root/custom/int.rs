use crate::root::compiler::generate_asm::get_local_address;
use crate::root::parser::line_info::LineInfo;
use crate::root::processor::type_builder::{Type, TypedFunction, TypeTable};
use lazy_static::lazy_static;
use unique_type_id::UniqueTypeId;
use crate::root::ast::literals::Literal;
use crate::root::custom::bool::Bool;
use crate::root::processor::processor::ProcessorError;

#[derive(UniqueTypeId)]
#[UniqueTypeIdType = "u16"]
pub struct Int {}

impl Int {
    pub fn new() -> Int {
        Int {}
    }

    pub fn get_id() -> isize {
        -(Self::id().0 as isize) - 1
    }
}

impl Int {
    pub fn instantiate_local_ref(offset: isize, local_address: isize) -> Vec<String> {
        vec![
            "mov rax, rbp".to_string(),
            format!("add rax, {offset}"),
            format!("mov qword [{}], rax", get_local_address(local_address),),
        ]
    }

    pub fn instantiate_ref(base_variable: isize, offset: isize, ref_address: isize) -> Vec<String> {
        vec![
            format!("mov rax, qword [{}]", get_local_address(base_variable)),
            format!("add rax, {offset}"),
            format!("mov qword [{}], rax", get_local_address(ref_address),),
        ]
    }
}

impl Type for Int {
    fn get_id(&self) -> isize {
        Self::get_id()
    }

    fn get_name(&self) -> &str {
        "int"
    }

    fn get_size(
        &self,
        _type_table: &TypeTable,
        _path: Option<Vec<isize>>,
    ) -> Result<usize, ProcessorError> {
        Ok(8)
    }

    fn instantiate(
        &self,
        literal: Option<&Literal>,
        local_address: isize,
    ) -> Result<Vec<String>, ProcessorError> {
        if literal.is_none() {
            return Ok(vec![]);
        }
        let Literal::Int(val) = literal.unwrap() else {
            panic!()
        };

        // ? Hacky workaround as NASM doesn't appear to support 64-bit literals
        let hex_str = format!("{:016x}", *val as i64);
        let upper = &hex_str[..8];
        let lower = &hex_str[8..];
        
        Ok(vec![
            format!("mov dword [{}], 0x{}", get_local_address(local_address), lower),
            format!("mov dword [{}], 0x{}", get_local_address(local_address + 4), upper),
        ])
    }
}


#[derive(UniqueTypeId)]
#[UniqueTypeIdType = "u16"]
pub struct IntAdd {}
lazy_static! {
    static ref INT_ADD_ARGS: [(String, (isize, usize)); 2] = [
        (String::from("lhs"), (Int::get_id(), 0)),
        (String::from("rhs"), (Int::get_id(), 0))
    ];
}
impl TypedFunction for IntAdd {
    fn get_id(&self) -> isize {
        -(Self::id().0 as isize) - 1
    }

    fn get_name(&self) -> &str {
        "add"
    }

    fn get_args(&self) -> &[(String, (isize, usize))] {
        INT_ADD_ARGS.as_ref()
    }

    fn get_line(&self) -> LineInfo {
        LineInfo::builtin()
    }

    fn get_return_type(&self) -> Option<(isize, usize)> {
        Some((Int::get_id(), 0))
    }

    fn is_inline(&self) -> bool {
        true
    }

    fn get_inline(&self, args: Vec<isize>) -> Vec<String> {
        vec![
            format!("mov rax, qword [{}]", get_local_address(args[0])),
            format!("add rax, [{}]", get_local_address(args[1])),
            format!("mov qword [{}], rax", get_local_address(args[2])),
        ]
    }
}

#[derive(UniqueTypeId)]
#[UniqueTypeIdType = "u16"]
pub struct IntSub {}
lazy_static! {
    static ref INT_SUB_ARGS: [(String, (isize, usize)); 2] = [
        (String::from("lhs"), (Int::get_id(), 0)),
        (String::from("rhs"), (Int::get_id(), 0))
    ];
}
impl TypedFunction for IntSub {
    fn get_id(&self) -> isize {
        -(Self::id().0 as isize) - 1
    }

    fn get_name(&self) -> &str {
        "sub"
    }

    fn get_args(&self) -> &[(String, (isize, usize))] {
        INT_SUB_ARGS.as_ref()
    }

    fn get_line(&self) -> LineInfo {
        LineInfo::builtin()
    }

    fn get_return_type(&self) -> Option<(isize, usize)> {
        Some((Int::get_id(), 0))
    }

    fn is_inline(&self) -> bool {
        true
    }

    fn get_inline(&self, args: Vec<isize>) -> Vec<String> {
        vec![
            format!("mov rax, qword [{}]", get_local_address(args[0])),
            format!("sub rax, [{}]", get_local_address(args[1])),
            format!("mov [{}], rax", get_local_address(args[2])),
        ]
    }
}

#[derive(UniqueTypeId)]
#[UniqueTypeIdType = "u16"]
pub struct IntMul {}
lazy_static! {
    static ref INT_MUL_ARGS: [(String, (isize, usize)); 2] = [
        (String::from("lhs"), (Int::get_id(), 0)),
        (String::from("rhs"), (Int::get_id(), 0))
    ];
}
impl TypedFunction for IntMul {
    fn get_id(&self) -> isize {
        -(Self::id().0 as isize) - 1
    }

    fn get_name(&self) -> &str {
        "mul"
    }

    fn get_args(&self) -> &[(String, (isize, usize))] {
        INT_SUB_ARGS.as_ref()
    }

    fn get_line(&self) -> LineInfo {
        LineInfo::builtin()
    }

    fn get_return_type(&self) -> Option<(isize, usize)> {
        Some((Int::get_id(), 0))
    }

    fn is_inline(&self) -> bool {
        true
    }

    fn get_inline(&self, args: Vec<isize>) -> Vec<String> {
        vec![
            format!("mov rax, qword [{}]", get_local_address(args[0])),
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
    static ref INT_DIV_ARGS: [(String, (isize, usize)); 2] = [
        (String::from("lhs"), (Int::get_id(), 0)),
        (String::from("rhs"), (Int::get_id(), 0))
    ];
}
impl TypedFunction for IntDiv {
    fn get_id(&self) -> isize {
        -(Self::id().0 as isize) - 1
    }

    fn get_name(&self) -> &str {
        "div"
    }

    fn get_args(&self) -> &[(String, (isize, usize))] {
        INT_SUB_ARGS.as_ref()
    }

    fn get_line(&self) -> LineInfo {
        LineInfo::builtin()
    }

    fn get_return_type(&self) -> Option<(isize, usize)> {
        Some((Int::get_id(), 0))
    }

    fn is_inline(&self) -> bool {
        true
    }

    fn get_inline(&self, args: Vec<isize>) -> Vec<String> {
        vec![
            format!("mov rax, qword [{}]", get_local_address(args[0])),
            format!("mov rcx, [{}]", get_local_address(args[1])),
            "cqo".to_string(),
            "idiv rcx".to_string(),
            format!("mov [{}], rax", get_local_address(args[2])),
        ]
    }
}

#[derive(UniqueTypeId)]
#[UniqueTypeIdType = "u16"]
pub struct IntMod {}
lazy_static! {
    static ref INT_MOD_ARGS: [(String, (isize, usize)); 2] = [
        (String::from("lhs"), (Int::get_id(), 0)),
        (String::from("rhs"), (Int::get_id(), 0))
    ];
}
impl TypedFunction for IntMod {
    fn get_id(&self) -> isize {
        -(Self::id().0 as isize) - 1
    }

    fn get_name(&self) -> &str {
        "mod"
    }

    fn get_args(&self) -> &[(String, (isize, usize))] {
        INT_MOD_ARGS.as_ref()
    }

    fn get_line(&self) -> LineInfo {
        LineInfo::builtin()
    }

    fn get_return_type(&self) -> Option<(isize, usize)> {
        Some((Int::get_id(), 0))
    }

    fn is_inline(&self) -> bool {
        true
    }

    fn get_inline(&self, args: Vec<isize>) -> Vec<String> {
        vec![
            format!("mov rax, qword [{}]", get_local_address(args[0])),
            format!("mov rcx, [{}]", get_local_address(args[1])),
            "cqo".to_string(),
            "idiv rcx".to_string(),
            format!("mov [{}], rdx", get_local_address(args[2])),
        ]
    }
}

#[derive(UniqueTypeId)]
#[UniqueTypeIdType = "u16"]
pub struct IntLT {}
lazy_static! {
    static ref INT_LT_ARGS: [(String, (isize, usize)); 2] = [
        (String::from("lhs"), (Int::get_id(), 0)),
        (String::from("rhs"), (Int::get_id(), 0))
    ];
}
impl TypedFunction for IntLT {
    fn get_id(&self) -> isize {
        -(Self::id().0 as isize) - 1
    }

    fn get_name(&self) -> &str {
        "lt"
    }

    fn get_args(&self) -> &[(String, (isize, usize))] {
        INT_LT_ARGS.as_ref()
    }

    fn get_line(&self) -> LineInfo {
        LineInfo::builtin()
    }

    fn get_return_type(&self) -> Option<(isize, usize)> {
        Some((Bool::get_id(), 0))
    }

    fn is_inline(&self) -> bool {
        true
    }

    fn get_inline(&self, args: Vec<isize>) -> Vec<String> {
        vec![
            format!("mov rax, qword [{}]", get_local_address(args[0])),
            format!("mov rcx, [{}]", get_local_address(args[1])),
            "cmp rcx, rax".to_string(),
            format!("mov qword [{}], 0", get_local_address(args[2])),
            format!("setle [{}]", get_local_address(args[2])),
        ]
    }
}

#[derive(UniqueTypeId)]
#[UniqueTypeIdType = "u16"]
pub struct IntGT {}
lazy_static! {
    static ref INT_GT_ARGS: [(String, (isize, usize)); 2] = [
        (String::from("lhs"), (Int::get_id(), 0)),
        (String::from("rhs"), (Int::get_id(), 0))
    ];
}
impl TypedFunction for IntGT {
    fn get_id(&self) -> isize {
        -(Self::id().0 as isize) - 1
    }

    fn get_name(&self) -> &str {
        "gt"
    }

    fn get_args(&self) -> &[(String, (isize, usize))] {
        INT_GT_ARGS.as_ref()
    }

    fn get_line(&self) -> LineInfo {
        LineInfo::builtin()
    }

    fn get_return_type(&self) -> Option<(isize, usize)> {
        Some((Bool::get_id(), 0))
    }

    fn is_inline(&self) -> bool {
        true
    }

    fn get_inline(&self, args: Vec<isize>) -> Vec<String> {
        vec![
            format!("mov rax, qword [{}]", get_local_address(args[0])),
            format!("mov rcx, [{}]", get_local_address(args[1])),
            "cmp rax, rcx".to_string(),
            format!("mov qword [{}], 0", get_local_address(args[2])),
            format!("setle [{}]", get_local_address(args[2])),
        ]
    }
}

#[derive(UniqueTypeId)]
#[UniqueTypeIdType = "u16"]
pub struct IntLE {}
lazy_static! {
    static ref INT_LE_ARGS: [(String, (isize, usize)); 2] = [
        (String::from("lhs"), (Int::get_id(), 0)),
        (String::from("rhs"), (Int::get_id(), 0))
    ];
}
impl TypedFunction for IntLE {
    fn get_id(&self) -> isize {
        -(Self::id().0 as isize) - 1
    }

    fn get_name(&self) -> &str {
        "le"
    }

    fn get_args(&self) -> &[(String, (isize, usize))] {
        INT_LE_ARGS.as_ref()
    }

    fn get_line(&self) -> LineInfo {
        LineInfo::builtin()
    }

    fn get_return_type(&self) -> Option<(isize, usize)> {
        Some((Bool::get_id(), 0))
    }

    fn is_inline(&self) -> bool {
        true
    }

    fn get_inline(&self, args: Vec<isize>) -> Vec<String> {
        vec![
            format!("mov rax, qword [{}]", get_local_address(args[0])),
            format!("mov rcx, [{}]", get_local_address(args[1])),
            "cmp rax, rcx".to_string(),
            format!("mov qword [{}], 0", get_local_address(args[2])),
            format!("setnle [{}]", get_local_address(args[2])),
        ]
    }
}

#[derive(UniqueTypeId)]
#[UniqueTypeIdType = "u16"]
pub struct IntGE {}
lazy_static! {
    static ref INT_GE_ARGS: [(String, (isize, usize)); 2] = [
        (String::from("lhs"), (Int::get_id(), 0)),
        (String::from("rhs"), (Int::get_id(), 0))
    ];
}
impl TypedFunction for IntGE {
    fn get_id(&self) -> isize {
        -(Self::id().0 as isize) - 1
    }

    fn get_name(&self) -> &str {
        "ge"
    }

    fn get_args(&self) -> &[(String, (isize, usize))] {
        INT_GE_ARGS.as_ref()
    }

    fn get_line(&self) -> LineInfo {
        LineInfo::builtin()
    }

    fn get_return_type(&self) -> Option<(isize, usize)> {
        Some((Bool::get_id(), 0))
    }

    fn is_inline(&self) -> bool {
        true
    }

    fn get_inline(&self, args: Vec<isize>) -> Vec<String> {
        vec![
            format!("mov rax, qword [{}]", get_local_address(args[0])),
            format!("mov rcx, [{}]", get_local_address(args[1])),
            "cmp rcx, rax".to_string(),
            format!("mov qword [{}], 0", get_local_address(args[2])),
            format!("setnle [{}]", get_local_address(args[2])),
        ]
    }
}

#[derive(UniqueTypeId)]
#[UniqueTypeIdType = "u16"]
pub struct IntEQ {}
lazy_static! {
    static ref INT_EQ_ARGS: [(String, (isize, usize)); 2] = [
        (String::from("lhs"), (Int::get_id(), 0)),
        (String::from("rhs"), (Int::get_id(), 0))
    ];
}
impl TypedFunction for IntEQ {
    fn get_id(&self) -> isize {
        -(Self::id().0 as isize) - 1
    }

    fn get_name(&self) -> &str {
        "eq"
    }

    fn get_args(&self) -> &[(String, (isize, usize))] {
        INT_EQ_ARGS.as_ref()
    }

    fn get_line(&self) -> LineInfo {
        LineInfo::builtin()
    }

    fn get_return_type(&self) -> Option<(isize, usize)> {
        Some((Bool::get_id(), 0))
    }

    fn is_inline(&self) -> bool {
        true
    }

    fn get_inline(&self, args: Vec<isize>) -> Vec<String> {
        vec![
            format!("mov rax, qword [{}]", get_local_address(args[0])),
            format!("mov rcx, [{}]", get_local_address(args[1])),
            "cmp rcx, rax".to_string(),
            format!("mov qword [{}], 0", get_local_address(args[2])),
            format!("setnz [{}]", get_local_address(args[2])),
        ]
    }
}

#[derive(UniqueTypeId)]
#[UniqueTypeIdType = "u16"]
pub struct IntNE {}
lazy_static! {
    static ref INT_NE_ARGS: [(String, (isize, usize)); 2] = [
        (String::from("lhs"), (Int::get_id(), 0)),
        (String::from("rhs"), (Int::get_id(), 0))
    ];
}
impl TypedFunction for IntNE {
    fn get_id(&self) -> isize {
        -(Self::id().0 as isize) - 1
    }

    fn get_name(&self) -> &str {
        "ne"
    }

    fn get_args(&self) -> &[(String, (isize, usize))] {
        INT_NE_ARGS.as_ref()
    }

    fn get_line(&self) -> LineInfo {
        LineInfo::builtin()
    }

    fn get_return_type(&self) -> Option<(isize, usize)> {
        Some((Bool::get_id(), 0))
    }

    fn is_inline(&self) -> bool {
        true
    }

    fn get_inline(&self, args: Vec<isize>) -> Vec<String> {
        vec![
            format!("mov rax, qword [{}]", get_local_address(args[0])),
            format!("mov rcx, [{}]", get_local_address(args[1])),
            "cmp rcx, rax".to_string(),
            format!("mov qword [{}], 0", get_local_address(args[2])),
            format!("setz [{}]", get_local_address(args[2])),
        ]
    }
}
