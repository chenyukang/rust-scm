use ast::*;

#[deriving(Clone, PartialEq)]
pub struct Env {
    pub vars: Vec<ExprAst>,
    pub vals: Vec<ExprAst>,
    pub parent: Option<Box<Env>>
}

impl Env {
    pub fn new() -> Env {
        let mut res = Env {
            vars: vec![],
            vals: vec![],
            parent: None
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
        match self.parent {
            Some(ref sub) => return sub.lookup(var),
            _ => return None
        };
    }

    pub fn extend(self, vars: ExprAst, vals: ExprAst) -> Env {
        let mut _vars = vars;
        let mut _vals = vals;
        let mut res = Env::new();
        res.parent = Some(box self);
        loop {
            if _vars.is_last() { break; }
            res.add_bingding(_vars.car(), _vals.car());
            _vars = _vars.cdr();
            _vals = _vals.cdr();
        }
        res.add_bingding(_vars.car(), _vals.car());
        return res.clone();
    }


    fn setup(&mut self) {
        macro_rules! def_proc {
            ($func_name:ident, $raw_func_name:ident) => (
                fn $func_name(args: ExprAst) -> ExprAst {
                    ExprAst::Bool(BoolNode::new(args.car().$raw_func_name()))
                }
                )
        }

        macro_rules! add_proc {
            ($type_str:expr, $func_name:ident) => (
                self.def_var(ExprAst::Symbol(SymbolNode::new($type_str)),
                             ExprAst::Proc(ProcNode::new($func_name)))
                    )
        }

        def_proc!(is_null, is_empty);
        def_proc!(is_boolean, is_bool);
        def_proc!(is_symbol, is_symbol);
        def_proc!(is_string, is_string);
        def_proc!(is_pair, is_pair);
        def_proc!(is_char, is_char);
        def_proc!(is_int, is_int);


        add_proc!("null?", is_null);
        add_proc!("boolean?", is_boolean);
        add_proc!("symbol?", is_symbol);
        add_proc!("string?", is_string);
        add_proc!("char?", is_char);
        add_proc!("integer?", is_int);
        add_proc!("+", add);
        add_proc!("-", sub);
        add_proc!("*", mul);
        add_proc!("/", div);
        add_proc!("eq?", eq);
        add_proc!("<", less);
        add_proc!(">", large);
        add_proc!("pair?", is_pair);
    }
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

#[test]
fn test_env_extend() {
    let mut env = Env::new();
    env.def_var(ExprAst::Str(StrNode::new("hello")),
                ExprAst::Str(StrNode::new("world")));

    let vars = ExprAst::Pair(PairNode::new(
        ExprAst::Str(StrNode::new("var")),
        ExprAst::Nil));
    let vals = ExprAst::Pair(PairNode::new(
        ExprAst::Str(StrNode::new("val")),
        ExprAst::Nil));

    let extend_env = env.extend(vars, vals);
    let val = extend_env.lookup(ExprAst::Str(StrNode::new("var")));
    assert!(val.unwrap().as_str() == "val");

    let val = extend_env.lookup(ExprAst::Str(StrNode::new("hello")));
    assert!(val.unwrap().as_str() == "world");

}
