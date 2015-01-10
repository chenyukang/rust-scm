use ast::*;

#[derive(Clone, PartialEq)]
pub struct Env {
    pub vars: Vec<ExprAst>,
    pub vals: Vec<ExprAst>,
    pub parent: Option<Box<Env>>
}

#[allow(unreachable_code)]
#[allow(dead_code)]
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
        for i in range(0us, self.vars.len()).rev() {
            if self.vars[i] == var {
                return Some(self.vals[i].clone());
            }
        }
        match self.parent {
            Some(ref sub) => return sub.lookup(var),
            _ => {
                var.print();
                panic!("Not found:");
                return None;
            }
        };
    }

    pub fn extend(&mut self, vars: ExprAst, vals: ExprAst) -> Env {
        let mut _vars = vars;
        let mut _vals = vals;
        // FIXME: remove clone
        let mut res = Env {
            vars: vec![],
            vals: vec![],
            parent: Some(Box::new(self.clone()))
        };
        loop {
            if _vars.is_last() { break; }
            res.add_bingding(_vars.car(), _vals.car());
            _vars = _vars.cdr();
            _vals = _vals.cdr();
        }
        res.add_bingding(_vars.car(), _vals.car());
        return res;
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
        add_proc!("=", eq);
        add_proc!("<", less);
        add_proc!(">", large);
        add_proc!("pair?", is_pair);
        add_proc!("car", car);
        add_proc!("cdr", cdr);
        add_proc!("cons", cons);
    }
}


fn add(args: ExprAst) -> ExprAst {
    let mut res = 0is;
    let mut exps = args;
    loop {
        if exps.is_empty() { break; }
        res += exps.car().as_int();
        exps = exps.cdr();
    }
    ExprAst::Int(IntNode::new(res))
}

fn sub(args: ExprAst) -> ExprAst {
    let mut res = args.car().as_int();
    let mut exps = args.cdr();
    loop {
        if exps.is_empty() { break; }
        res -= exps.car().as_int();
        exps = exps.cdr();
    }
    ExprAst::Int(IntNode::new(res))
}



fn mul(args: ExprAst) -> ExprAst {
    let mut res = 1is;
    let mut exps = args;
    loop {
        if exps.is_empty() { break; }
        res *= exps.car().as_int();
        exps = exps.cdr();
    }
    ExprAst::Int(IntNode::new(res))
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
    ExprAst::Int(IntNode::new(res))
}

fn cons(args: ExprAst) -> ExprAst {
    let obj1 = args.car();
    let obj2 = args.c("da");
    ExprAst::Pair(PairNode::new(obj1,
                                ExprAst::Pair(PairNode::new(obj2, ExprAst::Nil))))
}

fn eq(args: ExprAst) -> ExprAst {
    let obj1 = args.car();
    let obj2 = args.c("da");
    ExprAst::Bool(BoolNode::new(obj1 == obj2))
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
    ExprAst::Bool(BoolNode::new(true))
}

fn car(args: ExprAst) -> ExprAst {
    assert!(args.car().is_pair());
    args.c("aa")
}

fn cdr(args: ExprAst) -> ExprAst {
    assert!(args.car().is_pair());
    args.c("ad")
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
    ExprAst::Bool(BoolNode::new(true))
}

#[test]
fn test_env() {
    let mut env = Env::new();
    env.def_var(ExprAst::Str(StrNode::new("hello")),
                ExprAst::Str(StrNode::new("world")));

    let val = env.lookup(ExprAst::Str(StrNode::new("hello")));
    assert!(val.unwrap().as_str() == "world");

    env.def_var(ExprAst::Str(StrNode::new("1")), ExprAst::Int(IntNode::new(1)));
    let val = env.lookup(ExprAst::Str(StrNode::new("1")));
    assert!(val.unwrap().as_int() == 1);

    env.def_var(ExprAst::Str(StrNode::new("1")), ExprAst::Int(IntNode::new(2)));
    let val = env.lookup(ExprAst::Str(StrNode::new("1")));
    assert!(val.unwrap().as_int() == 2);

    env.def_var(ExprAst::Symbol(SymbolNode::new("sym")), ExprAst::Int(IntNode::new(2)));
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

    let vars = ExprAst::Pair(PairNode::new( ExprAst::Str(StrNode::new("var")),
                                            ExprAst::Nil));

    let vals = ExprAst::Pair(PairNode::new( ExprAst::Str(StrNode::new("val")),
                                            ExprAst::Nil));

    let mut extend_env = env.extend(vars, vals);
    let val = extend_env.lookup(ExprAst::Str(StrNode::new("var")));
    assert!(val.unwrap().as_str() == "val");

    let val = extend_env.lookup(ExprAst::Str(StrNode::new("hello")));
    assert!(val.unwrap().as_str() == "world");

    let vars = ExprAst::Pair(PairNode::new( ExprAst::Str(StrNode::new("var_x")),
                                            ExprAst::Nil));

    let vals = ExprAst::Pair(PairNode::new( ExprAst::Str(StrNode::new("val_x")),
                                            ExprAst::Nil));

    let mut extend_env = extend_env.extend(vars, vals);
    let val = extend_env.lookup(ExprAst::Str(StrNode::new("var_x")));
    assert!(val.unwrap().as_str() == "val_x");

    extend_env.def_var(ExprAst::Str(StrNode::new("1")),
                       ExprAst::Str(StrNode::new("1")));

    let val = extend_env.lookup(ExprAst::Str(StrNode::new("1")));
    assert!(val.unwrap().as_str() == "1");
}
