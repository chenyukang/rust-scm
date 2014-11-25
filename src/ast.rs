#[allow(dead_code)]

#[deriving(Clone, PartialEq)]
pub enum ExprAst {
    Int(IntNode),
    Str(StrNode),
    Bool(BoolNode),
    Pair(PairNode),
    Symbol(SymbolNode),
    Char(CharNode),
    Proc(ProcNode),
    CompProc(CompProcNode),
    EmptyList(EmptyListNode)
}

pub trait Ast {
    fn eval(self) -> ExprAst;
}

#[deriving(Clone, PartialEq)]
pub struct IntNode {
    pub value: i32
}

#[deriving(Clone, PartialEq)]
pub struct StrNode {
    pub value: String
}

#[deriving(Clone, PartialEq)]
pub struct BoolNode {
    pub value: bool
}

#[deriving(Clone, PartialEq)]
pub struct PairNode {
    pub car: Box<ExprAst>,
    pub cdr: Box<ExprAst>
}

#[deriving(Clone, PartialEq)]
pub struct SymbolNode {
    pub value: String
}

#[deriving(Clone, PartialEq)]
pub struct CharNode {
    pub value: char
}

#[deriving(Clone, PartialEq)]
pub struct ProcNode;

#[deriving(Clone, PartialEq)]
pub struct EmptyListNode;

#[deriving(Clone, PartialEq)]
pub struct CompProcNode {
    pub params: Box<ExprAst>,
    pub body:   Box<ExprAst>,
    pub env:    Box<ExprAst>
}
