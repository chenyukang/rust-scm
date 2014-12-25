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

#[allow(dead_code)]
impl ExprAst {
    pub fn as_bool(&self) -> bool {
        match *self {
            ExprAst::Bool(ref ast) => ast.value,
            _ => panic!("error type: expect BoolNode")
        }
    }

    pub fn as_int(&self) -> int {
        match *self {
            ExprAst::Int(ref ast) => ast.value,
            _ => panic!("error type: expect IntNode")
        }
    }

    pub fn as_str(&self) -> String {
        match *self {
            ExprAst::Str(ref ast) => ast.value.clone(),
            _ => panic!("error type: expect StrNode")
        }
    }

    pub fn as_char(&self) -> char {
        match *self {
            ExprAst::Char(ref ast) => ast.value,
            _ => panic!("error type: expect CharNode")
        }
    }

    pub fn car(&self) -> &Box<ExprAst> {
        match *self {
            ExprAst::Pair(ref ast) => &(ast.car),
            _ => panic!("error type: expect PairNode")
        }
    }

    pub fn cdr(&self) -> &Box<ExprAst> {
        match *self {
            ExprAst::Pair(ref ast) => &(ast.cdr),
            _ => panic!("error type: expect PairNode")
        }
    }

    pub fn is_pair(&self) -> bool {
        match *self {
            ExprAst::Pair(_) => true,
            _ => false
        }
    }

    pub fn is_empty_list(&self) -> bool {
        match *self {
            ExprAst::EmptyList(_) => true,
            _ => false
        }
    }

    pub fn is_symbol(&self) -> bool {
        match *self {
            ExprAst::Symbol(_) => true,
            _ => false
        }
    }

    pub fn is_self(&self) -> bool {
        match *self {
            ExprAst::Bool(_) => true,
            ExprAst::Int(_) => true,
            ExprAst::Char(_) => true,
            ExprAst::Str(_) => true,
            _ => false
        }
    }

    pub fn is_quote(&self) -> bool {
        return self.is_tagged(box ExprAst::Symbol(SymbolNode::new("quote".to_string())));
    }

    pub fn is_assign(&self) -> bool {
        return self.is_tagged(box ExprAst::Symbol(SymbolNode::new("set!".to_string())));
    }

    pub fn is_def(&self) -> bool {
        return self.is_tagged(box ExprAst::Symbol(SymbolNode::new("def".to_string())));
    }

    pub fn is_and(&self) -> bool {
        return self.is_tagged(box ExprAst::Symbol(SymbolNode::new("and".to_string())));
    }

    pub fn is_or(&self) -> bool {
        return self.is_tagged(box ExprAst::Symbol(SymbolNode::new("or".to_string())));
    }

    pub fn is_if(&self) -> bool {
        return self.is_tagged(box ExprAst::Symbol(SymbolNode::new("if".to_string())));
    }

    pub fn is_lambda(&self) -> bool {
        return self.is_tagged(box ExprAst::Symbol(SymbolNode::new("lambda".to_string())));
    }

    pub fn is_cond(&self) -> bool {
        return self.is_tagged(box ExprAst::Symbol(SymbolNode::new("cond".to_string())));
    }

    pub fn is_let(&self) -> bool {
        return self.is_tagged(box ExprAst::Symbol(SymbolNode::new("let".to_string())));
    }

    pub fn is_begin(&self) -> bool {
        return self.is_tagged(box ExprAst::Symbol(SymbolNode::new("begin".to_string())));
    }

    fn is_tagged(&self, tag: Box<ExprAst>) -> bool {
        if self.is_pair() {
            let car = self.car();
            return car.is_symbol() && *car == tag;
        }
        return false;
    }
}

#[deriving(Clone, PartialEq, Eq)]
pub struct IntNode {
    pub value: int
}

#[allow(dead_code)]
impl IntNode {
    pub fn new(val: int) -> IntNode {
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
    pub fn new(car: Box<ExprAst>, cdr: Box<ExprAst>) -> PairNode {
        PairNode {
            car: car,
            cdr: cdr
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

#[allow(dead_code)]
impl SymbolNode {
    pub fn new(val: String) -> SymbolNode {
        SymbolNode {
            value: val
        }
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

#[allow(dead_code)]
impl CharNode {
    pub fn new(val: char) -> CharNode {
        CharNode {
            value: val
        }
    }
}

//FIXME, empty struct?
#[deriving(Clone, PartialEq)]
pub struct EmptyListNode {
    pub value: String
}

#[allow(dead_code)]
impl EmptyListNode {
    pub fn new() -> EmptyListNode {
        EmptyListNode {
            value: "EmptyListNode".to_string()
        }
    }
}

impl Ast for EmptyListNode{
    fn print(&self) {
        println!("EmptyListNode");
    }
}

//type AstFunc = |args: ExprAst| -> Option<ExprAst>;
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

#[test]
fn test_ast_int() {
    let int_node = ExprAst::Int(IntNode::new(3));
    assert!(int_node.as_int() == 3);
}

#[test]
#[should_fail]
fn test_ast_int_fail() {
    let str_node = ExprAst::Str(StrNode::new("hello".to_string()));
    assert!(str_node.as_int() == 3);
}

#[test]
fn test_ast_bool() {
    let bool_node = ExprAst::Bool(BoolNode::new(false));
    assert!(bool_node.as_bool() == false);
}

#[test]
#[should_fail]
fn test_ast_bool_fail() {
    let int_node = ExprAst::Int(IntNode::new(3));
    assert!(int_node.as_bool() == false);
}

#[test]
fn test_ast_char() {
    let char_node = ExprAst::Char(CharNode::new('a'));
    assert!(char_node.as_char() == 'a');
}

#[test]
#[should_fail]
fn test_ast_char_fail() {
    let int_node = ExprAst::Int(IntNode::new(1));
    assert!(int_node.as_char() == 'a');
    assert!(int_node.is_self());
}

#[test]
fn test_ast_str() {
    let str_node = ExprAst::Str(StrNode::new("hello".to_string()));
    assert!(str_node.as_str() == "hello".to_string());
    assert!(str_node.is_self());
}

#[test]
#[should_fail]
fn test_ast_str_fail() {
    let int_node = ExprAst::Int(IntNode::new(3));
    assert!(int_node.as_str() == "3".to_string());
    assert!(int_node.is_self());
}

#[test]
fn test_ast_pair() {
    let int_node = ExprAst::Int(IntNode::new(3));
    int_node.print();
    let str_node = ExprAst::Str(StrNode::new("hello".to_string()));
    str_node.print();
    let pair_node = ExprAst::Pair(PairNode::new(box int_node, box str_node));
    let car_node = pair_node.car();
    let cdr_node = pair_node.cdr();
    assert!(car_node.as_int() == 3);
    assert!(cdr_node.as_str() == "hello".to_string());
    assert!(!pair_node.is_self());
}

#[test]
fn test_ast_symbol() {
    let sym_node = ExprAst::Symbol(SymbolNode::new("sym".to_string()));
    assert!(sym_node.is_symbol());
    assert!(!sym_node.is_self());
}

#[test]
#[should_fail]
fn test_ast_pair_fail() {
    let int_node = ExprAst::Int(IntNode::new(3));
    let car_node = int_node.car();
    assert!(car_node.as_int() == 3);
}

#[test]
fn test_ast_emptylist() {
    let empty_node = ExprAst::EmptyList(EmptyListNode::new());
    assert!(empty_node.is_empty_list());
    assert!(!empty_node.is_self());
}

#[test]
#[should_fail]
fn test_ast_emptylist_fail() {
    let empty_node = ExprAst::Int(IntNode::new(3));
    assert!(empty_node.is_empty_list());
    assert!(empty_node.is_quote());
}
