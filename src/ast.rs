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
    EmptyList(EmptyListNode),
}

pub trait Ast {
    fn print(&self);
}

impl Ast for ExprAst {
    fn print(&self) {
        match *self {
            ExprAst::Int(ref ast) => ast.print(),
            ExprAst::Str(ref ast) => ast.print(),
            ExprAst::Bool(ref ast) => ast.print(),
            ExprAst::Pair(ref ast) => ast.print(),
            ExprAst::Symbol(ref ast) => ast.print(),
            ExprAst::Char(ref ast) => ast.print(),
            ExprAst::Proc(ref ast) => ast.print(),
            ExprAst::CompProc(ref ast) => ast.print(),
            ExprAst::EmptyList(ref ast) => ast.print(),
        }
    }
}

#[deriving(Clone, PartialEq)]
pub struct IntNode {
    pub value: i32
}

impl IntNode {
    pub fn new(val: i32) -> IntNode {
        IntNode{ value: val}
    }
}

impl Ast for IntNode {
    fn print(&self) {
        println!("IntNode: {}", self.value);
    }
}

#[deriving(Clone, PartialEq)]
pub struct StrNode {
    pub value: String
}

impl Ast for StrNode {
    fn print(&self) {
        println!("StrNode: {}", self.value);
    }
}

#[allow(dead_code)]
impl StrNode {
    pub fn new(val: String) -> StrNode {
        StrNode{ value: val}
    }
}

#[deriving(Clone, PartialEq)]
pub struct BoolNode {
    pub value: bool
}

#[allow(dead_code)]
impl BoolNode {
    pub fn new(val: bool) -> BoolNode {
        BoolNode{ value: val}
    }
}

impl Ast for BoolNode {
    fn print(&self) {
        println!("BoolNode: {}", self.value);
    }
}

#[deriving(Clone, PartialEq)]
pub struct PairNode{
    pub car: Box<ExprAst>,
    pub cdr: Box<ExprAst>
}

#[allow(dead_code)]
impl PairNode {
    pub fn new(_car: Box<ExprAst>, _cdr: Box<ExprAst>) -> PairNode {
        PairNode {
            car: _car,
            cdr: _cdr
        }
    }
}

impl Ast for PairNode {
    fn print(&self) {
        println!("PairNode (");
        self.car.print();
        self.cdr.print();
        println!(")");
    }
}

#[deriving(Clone, PartialEq)]
pub struct SymbolNode {
    pub value: String
}

impl Ast for SymbolNode {
    fn print(&self) {
        println!("SymbolNode: {}", self.value);
    }
}


#[deriving(Clone, PartialEq)]
pub struct CharNode {
    pub value: char
}

impl Ast for CharNode {
    fn print(&self) {
        println!("CharNode: {}", self.value);
    }
}

#[deriving(Clone, PartialEq)]
pub struct ProcNode {
    pub value: String
}

impl Ast for ProcNode {
    fn print(&self) {
        println!("ProcNode: {}", self.value);
    }
}

#[deriving(Clone, PartialEq)]
pub struct EmptyListNode {
    pub value: String
}

impl Ast for EmptyListNode{
    fn print(&self) {
        println!("EmptyListNode: {}", self.value);
    }
}

#[deriving(Clone, PartialEq)]
pub struct CompProcNode {
    pub params: Box<ExprAst>,
    pub body:   Box<ExprAst>,
    pub env:    Box<ExprAst>
}

impl Ast for CompProcNode {
    fn print(&self) {
        println!("CompProcNode: ");
        self.params.print();
        self.body.print();
        self.env.print();
    }
}
