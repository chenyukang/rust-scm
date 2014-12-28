
//use ast::Ast;
use ast::ExprAst;
use ast::StrNode;
use ast::IntNode;
use ast::SymbolNode;
use ast::BoolNode;
use ast::ProcNode;

#[deriving(Clone, PartialEq)]
pub struct Env {
    pub vars: Vec<ExprAst>,
    pub vals: Vec<ExprAst>,
    pub next: Option<Box<Env>>
}

#[allow(dead_code)]
impl Env {
    pub fn new() -> Env {
        let mut res = Env {
            vars: vec![],
            vals: vec![],
            next: None
        };
        res.setup();
        res
    }

    pub fn def_var(&mut self, var: ExprAst, val: ExprAst) {
        self.add_bingding(var, val);
    }

    pub fn add_bingding(&mut self, var: ExprAst, val: ExprAst) {
        assert!(self.vars.len() == self.vals.len());
        self.vars.push(var);
        self.vals.push(val);
    }

    pub fn lookup(&self, var: ExprAst) -> Option<ExprAst> {
        for i in range(0u, self.vars.len()).rev() {
            if self.vars[i] == var {
                return Some(self.vals[i].clone());
            }
        }
        match self.next {
            Some(ref sub) => return sub.lookup(var),
            _ => return None
        };
    }

    fn setup(&mut self) {
        fn is_null(args: ExprAst) -> ExprAst {
            ExprAst::Bool(BoolNode::new(args.car().is_empty()))
        }
        self.def_var(ExprAst::Symbol(SymbolNode::new("null?")),
                     ExprAst::Proc(ProcNode::new(is_null)));
    }
}


#[test]
fn test_env() {
    let mut env = Env::new();
    env.def_var(ExprAst::Str(StrNode::new("hello")),
                ExprAst::Str(StrNode::new("world")));

    let val = env.lookup(ExprAst::Str(StrNode::new("hello")));
    assert!(val.unwrap().as_str() == "world");

    env.def_var(ExprAst::Str(StrNode::new("1")),
                ExprAst::Int(IntNode::new(1)));
    let val = env.lookup(ExprAst::Str(StrNode::new("1")));
    assert!(val.unwrap().as_int() == 1);

    env.def_var(ExprAst::Str(StrNode::new("1")),
                ExprAst::Int(IntNode::new(2)));
    let val = env.lookup(ExprAst::Str(StrNode::new("1")));
    assert!(val.unwrap().as_int() == 2);

    // let val = env.lookup(ExprAst::Str(StrNode::new("init")));
    // assert!(val.unwrap().as_str() == "init_val");

    env.def_var(ExprAst::Symbol(SymbolNode::new("sym")),
                ExprAst::Int(IntNode::new(2)));
    let val = env.lookup(ExprAst::Symbol(SymbolNode::new("sym")));
    assert!(val.unwrap().as_int() == 2);
}
