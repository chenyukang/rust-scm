use std::cell::RefCell;
use std::rc::Rc;
use env;

#[derive(Clone, PartialEq)]
pub enum ExprAst {
    Int(IntNode),
    Str(StrNode),
    Bool(BoolNode),
    Pair(PairNode),
    Symbol(SymbolNode),
    Char(CharNode),
    Proc(ProcNode),
    CompProc(CompProcNode),
    Nil
}

pub trait Ast {
    fn print(&self);
}

impl Ast for ExprAst {
    fn print(&self) {
        match *self {
            ExprAst::Int(ref ast) =>  ast.print(),
            ExprAst::Str(ref ast) =>  ast.print(),
            ExprAst::Bool(ref ast) => ast.print(),
            ExprAst::Symbol(ref ast) => ast.print(),
            ExprAst::Char(ref ast) => ast.print(),
            ExprAst::Pair(_) => {
                print!("(");
                let exps = self.collect();
                for i in 0..exps.len() {
                    exps[i].print();
                    if i != exps.len()-1 {print!(" ");}
                }
                print!(")");
            }
            ExprAst::Proc(ref ast) => ast.print(),
            ExprAst::CompProc(ref ast) => ast.print(),
            ExprAst::Nil => print!("Nil")
        }
    }
}

macro_rules! is_ast_type {
    ($func_name:ident, $type_name:ident) => (impl ExprAst {
        pub fn $func_name(&self) -> bool {
            match *self {
                ExprAst::$type_name(_) => true,
                _ => false
            }
        }})
}

is_ast_type!(is_char, Char);
is_ast_type!(is_int, Int);
is_ast_type!(is_symbol, Symbol);
is_ast_type!(is_string, Str);
is_ast_type!(is_proc, Proc);
is_ast_type!(is_bool, Bool);
is_ast_type!(is_cproc, CompProc);

macro_rules! is_type {
    ($func_name:ident, $type_str:expr) => (impl ExprAst {
        pub fn $func_name(&self) -> bool {
            return self.is_tagged(ExprAst::Symbol(SymbolNode::new($type_str)))
        }
    })
}

is_type!(is_quote, "quote");
is_type!(is_def, "define");
is_type!(is_and, "and");
is_type!(is_or, "or");
is_type!(is_if, "if");
is_type!(is_assign, "set!");
is_type!(is_lambda, "lambda");
is_type!(is_cond, "cond");
is_type!(is_let, "let");
is_type!(is_begin, "begin");

impl ExprAst {
    pub fn is_true(&self) -> bool {
        return self.as_bool();
    }

    pub fn is_false(&self) -> bool {
        return !self.as_bool();
    }

    pub fn is_pair(&self) -> bool {
        match *self {
            ExprAst::Pair(_) => true,
            ExprAst::Nil => true,
            _ => false
        }
    }

    pub fn is_empty(&self) -> bool {
        match *self {
            ExprAst::Nil => true,
            _ => false
        }
    }

    pub fn as_bool(&self) -> bool {
        match *self {
            ExprAst::Bool(ref ast) => ast.value,
            _ => panic!("error type: expect BoolNode")
        }
    }

    pub fn as_int(&self) -> isize {
        match *self {
            ExprAst::Int(ref ast) => ast.value,
            _ => panic!("error type: expect IntNode")
        }
    }

    pub fn as_str(&self) -> String {
        match *self {
            ExprAst::Str(ref ast) => ast.value.clone(),
            ExprAst::Symbol(ref ast) => ast.value.clone(),
            _ => panic!("error type: expect StrNode")
        }
    }

    pub fn as_char(&self) -> char {
        match *self {
            ExprAst::Char(ref ast) => ast.value,
            _ => panic!("error type: expect CharNode")
        }
    }

    pub fn as_proc(&self) -> ProcFunc {
        match *self {
            ExprAst::Proc(ref ast) => ast.func.clone(),
            _ => panic!("error type: expct ProcNode")
        }
    }

    pub fn car(&self) -> ExprAst {
        match *self {
            ExprAst::Pair(ref ast) => ast.pair[0].clone(),
            _ => {
                self.print();
                panic!("error type: expect PairNode");
            }
        }
    }

    pub fn cdr(&self) -> ExprAst {
        match *self {
            ExprAst::Pair(ref ast) => ast.pair[1].clone(),
            _ => panic!("error type: expect PairNode")
        }
    }

    pub fn c(&self, s: &str) -> ExprAst {
        assert!(self.is_pair());
        let mut r = self.clone();
        for c in s.to_string().chars() {
            if c == 'a' { r = r.car(); }
            else { r = r.cdr(); }
        }
        return r;
    }

    pub fn is_last(&self) -> bool {
        assert!(self.is_pair());
        return self.cdr().is_empty();
    }

    pub fn is_self(&self) -> bool {
        match *self {
            ExprAst::Bool(_) | ExprAst::Int(_) |
            ExprAst::Char(_) | ExprAst::Str(_)
                => true ,
            _ => false
        }
    }

    pub fn def_var(&self) -> ExprAst {
        assert!(self.is_def());
        if self.cdr().car().is_symbol() {
            self.c("da")
        } else {
            self.c("daa")
        }
    }

    pub fn def_val(&self) -> ExprAst {
        assert!(self.is_def());
        if self.c("da").is_symbol() {
            self.c("dda")
        } else {
            //proc
            return self.c("dad").make_lambda(self.c("dd"));
        }
    }

    pub fn make_lambda(&self, body: ExprAst) -> ExprAst {
        let lambda = ExprAst::Symbol(SymbolNode::new("lambda"));
        return ExprAst::Pair(PairNode::new(lambda,
                                           ExprAst::Pair(PairNode::new(
                                               (*self).clone(), body))));
    }

    pub fn params(&self) -> ExprAst {
        match *self {
            ExprAst::CompProc(ref ast) => ast.pair[0].clone(),
            _ => panic!("error type: expect CompProc")
        }
    }

    pub fn body(&self) -> ExprAst {
        match *self {
            ExprAst::CompProc(ref ast) => ast.pair[1].clone(),
            _ => panic!("error type: expect CompProc")
        }
    }

    fn is_tagged(&self, tag: ExprAst) -> bool {
        if self.is_pair() {
            let car = self.car();
            return car.is_symbol() && car == tag;
        }
        return false;
    }

    fn collect(&self) -> Vec<ExprAst> {
        let mut res: Vec<ExprAst> = vec![];
        let mut _exp = self.clone();
        loop {
            let f = _exp.car();
            if f.is_self() {
                res.push(f);
            } else if !f.is_empty() {
                res.push_all(f.collect().as_slice());
            }
            _exp = _exp.cdr();
            if !_exp.is_pair() { break; }
            if _exp.is_empty() { break; }
        }
        res
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct IntNode {
    value: isize
}

impl IntNode {
    pub fn new(val: isize) -> IntNode {
        IntNode{ value: val}
    }
    fn print(&self) {
        print!("{}", self.value);
    }
}


#[derive(Clone, PartialEq)]
pub struct StrNode {
    value: String
}

impl StrNode {
    pub fn new(val: &str) -> StrNode {
        StrNode{ value: val.to_string()}
    }

    fn print(&self) {
        print!("{}", self.value);
    }
}

#[derive(Clone, PartialEq)]
pub struct BoolNode {
    value: bool
}

impl BoolNode {
    pub fn new(val: bool) -> BoolNode {
        BoolNode{ value: val}
    }

    fn print(&self) {
        print!("{}", self.value);
    }
}

#[derive(Clone, PartialEq)]
pub struct PairNode{
    pair: Vec<ExprAst>,
}

impl PairNode {
    pub fn new(car: ExprAst, cdr: ExprAst) -> PairNode {
        PairNode {
            pair: vec![car, cdr]
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct SymbolNode {
    value: String
}

impl SymbolNode {
    pub fn new(val: &str) -> SymbolNode {
        SymbolNode { value: val.to_string() }
    }

    fn print(&self) {
        print!("{}", self.value);
    }
}

#[derive(Clone, PartialEq)]
pub struct CharNode {
    value: char
}

impl CharNode {
    pub fn new(val: char) -> CharNode {
        CharNode { value: val}
    }

    fn print(&self) {
        print!("{}", self.value);
    }
}

#[derive(Clone)]
pub struct ProcFunc(fn(ExprAst) -> ExprAst);
impl PartialEq for ProcFunc {
    fn eq(&self, o: &ProcFunc) -> bool {
        let _o: *const() = unsafe { ::std::mem::transmute(o)};
        let _s: *const() = unsafe { ::std::mem::transmute(self)};
        _s == _o
    }
    fn ne(&self, o: &ProcFunc) -> bool {
        !self.eq(o)
    }
}

impl ProcFunc {
    pub fn func(&self) -> (fn(ExprAst) -> ExprAst) {
        match *self {
            ProcFunc(fun) => fun
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct ProcNode {
    value: String,
    func: ProcFunc
}

impl ProcNode {
    pub fn new(obj: fn(ExprAst)-> ExprAst) -> ProcNode {
        ProcNode { value: "proc".to_string(), func: ProcFunc(obj) }
    }

    fn print(&self) {
        print!("{}", self.value);
    }
}


#[derive(Clone, PartialEq)]
pub struct CompProcNode {
    pub pair:   Vec<ExprAst>,
    pub env:    Rc<RefCell<env::Env>>
}

impl CompProcNode {
    pub fn new(params: ExprAst, body: ExprAst,
               env: Rc<RefCell<env::Env>>) -> CompProcNode {
        CompProcNode { pair: vec![params, body],  env: env }
    }

    fn print(&self) {
        print!("CompProcNode: ");
    }
}

#[test]
fn test_ast() {
    let int_node = ExprAst::Int(IntNode::new(3));
    assert!(int_node.as_int() == 3);

    let char_node = ExprAst::Char(CharNode::new('a'));
    assert!(char_node.as_char() == 'a');

    let bool_node = ExprAst::Bool(BoolNode::new(false));
    assert!(bool_node.as_bool() == false);

    let str_node = ExprAst::Str(StrNode::new("hello"));
    assert!(str_node.as_str() == "hello");
    assert!(str_node.is_self());

    let int_node = ExprAst::Int(IntNode::new(3));
    let str_node = ExprAst::Str(StrNode::new("hello"));
    let pair_node = ExprAst::Pair(PairNode::new(int_node, str_node));
    let car_node = pair_node.car();
    let cdr_node = pair_node.cdr();
    assert!(car_node.as_int() == 3);
    assert!(cdr_node.as_str() == "hello");
    assert!(!pair_node.is_self());

    let sym_node = ExprAst::Symbol(SymbolNode::new("sym"));
    assert!(sym_node.is_symbol());
    assert!(!sym_node.is_self());

    let empty_node = ExprAst::Nil;
    assert!(empty_node.is_empty());
    assert!(!empty_node.is_self());
}

#[test]
fn test_ast_is_set() {
    macro_rules! test_case {
        ($str_name:expr) => {
            {let node = ExprAst::Pair(PairNode::new(
                ExprAst::Symbol(SymbolNode::new($str_name)),
                ExprAst::Int(IntNode::new(3))));
            node}
        }
    }
    assert!(test_case!("let").is_let());
    assert!(test_case!("if").is_if());
    assert!(test_case!("lambda").is_lambda());
    assert!(test_case!("cond").is_cond());
    assert!(test_case!("set!").is_assign());
    assert!(test_case!("begin").is_begin());
    assert!(!test_case!("begin").is_assign());
}

#[test]
fn test_symbol_eq() {
    let aa = ExprAst::Symbol(SymbolNode::new("else"));
    let bb = ExprAst::Symbol(SymbolNode::new("else"));
    assert!(aa == bb);
}

#[test]
fn test_proc() {
    fn _proc(obj: ExprAst) -> ExprAst {
        return ExprAst::Symbol(SymbolNode::new("ok"));
    }

    let proc_node = ExprAst::Proc(ProcNode::new(_proc));
    assert!(proc_node.is_proc());
    assert!(!proc_node.is_cproc());
}
