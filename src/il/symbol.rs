use crate::ast::builtin::Builtin;
use crate::ast::operators::Operator;

pub struct VariableId {
    global: bool,
    path: Vec<usize>,
}

pub struct Type {
    name: String,
    builtin_id: Option<usize>,
    subtypes: Vec<usize>,
}

pub struct TypeTable {
    inner: Vec<Type>,
}

impl TypeTable {
    pub fn new() -> TypeTable {
        TypeTable { inner: Vec::new() }
    }

    pub fn add_type(&mut self, t: Type) -> usize {
        self.inner.push(t);
        self.inner.len() - 1
    }
}

type Function = (Vec<(usize, usize)>, Vec<Line>);

#[derive(PartialEq, Clone, strum_macros::Display, Debug)]
pub enum Line {
    IfTree(
        (usize, Vec<Line>),
        Vec<(usize, Vec<Line>)>,
        Option<Vec<Line>>,
    ), // If, elifs, else
    While(usize, Vec<Line>),
    Call(isize, Vec<usize>, usize), // Func id, args, return
    Operation(usize, Operator, Option<usize>, usize), // left, operator, right, output
    Copy(usize, usize),             // from, to
    Builtin(Builtin, Vec<usize>),
    Return(usize),
}
