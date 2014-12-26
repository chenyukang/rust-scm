
//use ast::Ast;
use ast::ExprAst;
use ast::StrNode;
use ast::IntNode;

#[deriving(Clone, PartialEq)]
pub struct Env {
    pub vars: Vec<ExprAst>,
    pub vals: Vec<ExprAst>,
    pub next: Option<Box<Env>>
}


#[allow(dead_code)]
impl Env {
    pub fn new() -> Env {
        Env {
            vars: vec![],
            vals: vec![],
            next: None
        }
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

}


#[test]
fn test_env() {
    let mut env = Env::new();
    env.def_var(ExprAst::Str(StrNode::new("hello".to_string())),
                ExprAst::Str(StrNode::new("world".to_string())));

    let val = env.lookup(ExprAst::Str(StrNode::new("hello".to_string())));
    assert!(val.unwrap().as_str() == "world".to_string());

    env.def_var(ExprAst::Str(StrNode::new("1".to_string())),
                ExprAst::Int(IntNode::new(1)));
    let val = env.lookup(ExprAst::Str(StrNode::new("1".to_string())));
    assert!(val.unwrap().as_int() == 1);

    env.def_var(ExprAst::Str(StrNode::new("1".to_string())),
                ExprAst::Int(IntNode::new(2)));
    let val = env.lookup(ExprAst::Str(StrNode::new("1".to_string())));
    assert!(val.unwrap().as_int() == 2);
}
