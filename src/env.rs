
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


// macro_rules! def_proc {
//     ($type_str:expr, $func_name:ident) => (

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

        fn is_boolean(args: ExprAst) -> ExprAst {
            ExprAst::Bool(BoolNode::new(args.car().is_bool()))
        }
        fn is_symbol(args: ExprAst) -> ExprAst {
            ExprAst::Bool(BoolNode::new(args.car().is_symbol()))
        }

        fn is_string(args: ExprAst) -> ExprAst {
            ExprAst::Bool(BoolNode::new(args.car().is_string()))
        }

        fn is_pair(args: ExprAst) -> ExprAst {
            ExprAst::Bool(BoolNode::new(args.car().is_pair()))
        }

        fn is_char(args: ExprAst) -> ExprAst {
            ExprAst::Bool(BoolNode::new(args.car().is_char()))
        }

        fn is_int(args: ExprAst) -> ExprAst {
            ExprAst::Bool(BoolNode::new(args.car().is_int()))
        }

        fn add(args: ExprAst) -> ExprAst {
            let mut res = 0i;
            let mut exps = args;
            loop {
                if exps.is_empty() { break; }
                res += exps.car().as_int();
                exps = exps.cdr();
            }
            return ExprAst::Int(IntNode::new(res));
        }

        fn sub(args: ExprAst) -> ExprAst {
            let mut res = 0i;
            let mut exps = args;
            loop {
                if exps.is_empty() { break; }
                res -= exps.car().as_int();
                exps = exps.cdr();
            }
            return ExprAst::Int(IntNode::new(res));
        }

        fn mul(args: ExprAst) -> ExprAst {
            let mut res = 1i;
            let mut exps = args;
            loop {
                if exps.is_empty() { break; }
                res *= exps.car().as_int();
                exps = exps.cdr();
            }
            return ExprAst::Int(IntNode::new(res));
        }

        fn div(args: ExprAst) -> ExprAst {
            let mut exps = args;
            let mut res = exps.car().as_int();
            exps = exps.cdr();
            loop {
                if exps.is_empty() { break; }
                let nxt = exps.car().as_int();
                if nxt == 0 {
                    return ExprAst::Symbol(SymbolNode::new("Fail"));
                }
                res /= nxt;
                exps = exps.cdr();
            }
            return ExprAst::Int(IntNode::new(res));
        }

        fn eq(args: ExprAst) -> ExprAst {
            let obj1 = args.car();
            let obj2 = args.cdr().car();
            return ExprAst::Bool(BoolNode::new(obj1 == obj2));
        }

        fn less(args: ExprAst) -> ExprAst {
            let val = args.car().as_int();
            let mut exps = args.cdr();
            loop {
                if exps.is_empty() { break; }
                if val >= exps.car().as_int() {
                    return ExprAst::Bool(BoolNode::new(false));
                }
                exps = exps.cdr();
            }
            return ExprAst::Bool(BoolNode::new(true));
        }

        fn large(args: ExprAst) -> ExprAst {
            let val = args.car().as_int();
            let mut exps = args.cdr();
            loop {
                if exps.is_empty() { break; }
                if val <= exps.car().as_int() {
                    return ExprAst::Bool(BoolNode::new(false));
                }
                exps = exps.cdr();
            }
            return ExprAst::Bool(BoolNode::new(true));
        }

        self.def_var(ExprAst::Symbol(SymbolNode::new("null?")),
                     ExprAst::Proc(ProcNode::new(is_null)));
        self.def_var(ExprAst::Symbol(SymbolNode::new("boolean?")),
                     ExprAst::Proc(ProcNode::new(is_boolean)));
        self.def_var(ExprAst::Symbol(SymbolNode::new("symbol?")),
                     ExprAst::Proc(ProcNode::new(is_symbol)));
        self.def_var(ExprAst::Symbol(SymbolNode::new("string?")),
                     ExprAst::Proc(ProcNode::new(is_string)));
        self.def_var(ExprAst::Symbol(SymbolNode::new("char?")),
                     ExprAst::Proc(ProcNode::new(is_char)));
        self.def_var(ExprAst::Symbol(SymbolNode::new("integer?")),
                     ExprAst::Proc(ProcNode::new(is_int)));
        self.def_var(ExprAst::Symbol(SymbolNode::new("+")),
                     ExprAst::Proc(ProcNode::new(add)));
        self.def_var(ExprAst::Symbol(SymbolNode::new("-")),
                     ExprAst::Proc(ProcNode::new(sub)));
        self.def_var(ExprAst::Symbol(SymbolNode::new("<")),
                     ExprAst::Proc(ProcNode::new(less)));
        self.def_var(ExprAst::Symbol(SymbolNode::new(">")),
                     ExprAst::Proc(ProcNode::new(large)));
        self.def_var(ExprAst::Symbol(SymbolNode::new("*")),
                     ExprAst::Proc(ProcNode::new(mul)));
        self.def_var(ExprAst::Symbol(SymbolNode::new("/")),
                     ExprAst::Proc(ProcNode::new(div)));
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

    env.def_var(ExprAst::Symbol(SymbolNode::new("sym")),
                ExprAst::Int(IntNode::new(2)));
    let val = env.lookup(ExprAst::Symbol(SymbolNode::new("sym")));
    assert!(val.unwrap().as_int() == 2);

    let val = env.lookup(ExprAst::Symbol(SymbolNode::new("null?")));
    assert!(val.unwrap().is_proc());
    let val = env.lookup(ExprAst::Symbol(SymbolNode::new("char?")));
    assert!(val.unwrap().is_proc());
    let val = env.lookup(ExprAst::Symbol(SymbolNode::new("integer?")));
    assert!(val.unwrap().is_proc());

    let val = env.lookup(ExprAst::Symbol(SymbolNode::new(">")));
    assert!(val.unwrap().is_proc());

    let val = env.lookup(ExprAst::Symbol(SymbolNode::new("<")));
    assert!(val.unwrap().is_proc());
}
