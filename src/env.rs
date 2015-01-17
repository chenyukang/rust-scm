use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;
use ast::*;
use test::Bencher;

#[derive(Clone, PartialEq)]
pub struct Env {
    pub table: HashMap<String, Expr>,
    pub parent: Option<Rc<RefCell<Env>>>
}

#[allow(unreachable_code)]
#[allow(dead_code)]
impl Env {
    pub fn new() -> Env {
        let mut res = Env {
            table: HashMap::new(),
            parent: None
        };
        res.setup();
        res
    }

    pub fn def_var(&mut self, var: String, val: Expr) {
        self.add_binding(var, val);
    }

    pub fn str_def(&mut self, var: &str, val: Expr) {
        self.add_binding(var.to_string(), val);
    }

    pub fn add_binding(&mut self, var: String, val: Expr) {
        self.table.insert(var, val);
    }

    pub fn lookup(&self, var: String) -> Option<Expr> {
        match self.table.get(&var) {
            Some(val) => return Some(val.clone()),
            _ => {}
        }
        match self.parent.clone() {
            Some(p) =>
                return p.borrow().lookup(var),
            _ => {
                return None;
            }
        };
    }

    pub fn str_lookup(&self, var: &str) -> Option<Expr> {
        self.lookup(var.to_string())
    }

    pub fn parent(&self) ->  Option<Rc<RefCell<Env>>> {
        match self.parent.clone() {
            Some(p) => return Some(p.clone()),
            _ => { return None; }
        }
    }

    pub fn extend(&mut self, vars: Expr, vals: Expr) -> Rc<RefCell<Env>> {
        let mut _vars = vars;
        let mut _vals = vals;
        let mut res = Env {
            table: HashMap::new(),
            parent: Some(Rc::new(RefCell::new(self.clone())))
        };
        loop {
            if _vars.is_last() { break; }
            res.add_binding(_vars.car().as_str(), _vals.car());
            _vars = _vars.cdr();
            _vals = _vals.cdr();
        }
        res.add_binding(_vars.car().as_str(), _vals.car());
        return Rc::new(RefCell::new(res));
    }

    fn setup(&mut self) {
        macro_rules! def_proc {
            ($func_name:ident, $raw_func_name:ident) => (
                fn $func_name(args: Expr) -> Expr {
                    Expr::Bool(BoolNode::new(args.car().$raw_func_name()))
                }
                )
        }

        macro_rules! add_proc {
            ($type_str:expr, $func_name:ident) => (
                self.def_var($type_str.to_string(),
                             Expr::Proc(ProcNode::new($func_name)))
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


fn add(args: Expr) -> Expr {
    let mut res = 0is;
    let mut exps = args;
    loop {
        if exps.is_empty() { break; }
        res += exps.car().as_int();
        exps = exps.cdr();
    }
    Expr::Int(IntNode::new(res))
}

fn sub(args: Expr) -> Expr {
    let mut res = args.car().as_int();
    let mut exps = args.cdr();
    loop {
        if exps.is_empty() { break; }
        res -= exps.car().as_int();
        exps = exps.cdr();
    }
    Expr::Int(IntNode::new(res))
}

fn mul(args: Expr) -> Expr {
    let mut res = 1is;
    let mut exps = args;
    loop {
        if exps.is_empty() { break; }
        res *= exps.car().as_int();
        exps = exps.cdr();
    }
    Expr::Int(IntNode::new(res))
}

fn div(args: Expr) -> Expr {
    let mut exps = args;
    let mut res = exps.car().as_int();
    exps = exps.cdr();
    loop {
        if exps.is_empty() { break; }
        let nxt = exps.car().as_int();
        if nxt == 0 {
            return Expr::Symbol(SymbolNode::new("Fail"));
        }
        res /= nxt;
        exps = exps.cdr();
    }
    Expr::Int(IntNode::new(res))
}

fn cons(args: Expr) -> Expr {
    let obj1 = args.car();
    let obj2 = args.c("da");
    Expr::Pair(PairNode::new(obj1, Expr::Pair(PairNode::new(obj2, Expr::Nil))))
}

fn eq(args: Expr) -> Expr {
    let obj1 = args.car();
    let obj2 = args.c("da");
    Expr::Bool(BoolNode::new(obj1 == obj2))
}

fn less(args: Expr) -> Expr {
    let val = args.car().as_int();
    let mut exps = args.cdr();
    loop {
        if exps.is_empty() { break; }
        if val >= exps.car().as_int() {
            return Expr::Bool(BoolNode::new(false));
        }
        exps = exps.cdr();
    }
    Expr::Bool(BoolNode::new(true))
}

fn car(args: Expr) -> Expr {
    assert!(args.car().is_pair());
    args.c("aa")
}

fn cdr(args: Expr) -> Expr {
    assert!(args.car().is_pair());
    args.c("ad")
}


fn large(args: Expr) -> Expr {
    let val = args.car().as_int();
    let mut exps = args.cdr();
    loop {
        if exps.is_empty() { break; }
        if val <= exps.car().as_int() {
            return Expr::Bool(BoolNode::new(false));
        }
        exps = exps.cdr();
    }
    Expr::Bool(BoolNode::new(true))
}


#[test]
fn test_env() {
    let mut env = Env::new();
    env.str_def("hello", Expr::Str(StrNode::new("world")));

    let val = env.str_lookup("hello");
    assert!(val.unwrap().as_str() == "world");

    env.str_def("1", Expr::Int(IntNode::new(1)));
    let val = env.str_lookup("1");
    assert!(val.unwrap().as_int() == 1);

    env.str_def("1", Expr::Int(IntNode::new(2)));
    let val = env.str_lookup("1");
    assert!(val.unwrap().as_int() == 2);

    env.str_def("sym", Expr::Int(IntNode::new(2)));
    let val = env.str_lookup("sym");
    assert!(val.unwrap().as_int() == 2);

    let val = env.str_lookup("null?");
    assert!(val.unwrap().is_proc());
    let val = env.str_lookup("char?");
    assert!(val.unwrap().is_proc());
    let val = env.str_lookup("integer?");
    assert!(val.unwrap().is_proc());

    let val = env.str_lookup(">");
    assert!(val.unwrap().is_proc());

    let val = env.str_lookup("<");
    assert!(val.unwrap().is_proc());

}

#[test]
fn test_env_extend() {
    let mut env = Env::new();
    env.str_def("hello", Expr::Str(StrNode::new("world")));

    let vars = Expr::Pair(PairNode::new( Expr::Str(StrNode::new("var")), Expr::Nil));
    let vals = Expr::Pair(PairNode::new( Expr::Str(StrNode::new("val")), Expr::Nil));

    let extend_env = env.extend(vars, vals);
    let val = extend_env.clone().borrow_mut().str_lookup("var");
    assert!(val.unwrap().as_str() == "val");

    let val = extend_env.clone().borrow_mut().str_lookup("hello");
    assert!(val.unwrap().as_str() == "world");

    let vars = Expr::Pair(PairNode::new( Expr::Str(StrNode::new("var_x")), Expr::Nil));
    let vals = Expr::Pair(PairNode::new( Expr::Str(StrNode::new("val_x")), Expr::Nil));


    let extend_env = extend_env.clone().borrow_mut().extend(vars, vals);
    let val = extend_env.clone().borrow_mut().str_lookup("var_x");
    assert!(val.unwrap().as_str() == "val_x");

    extend_env.clone().borrow_mut().str_def("1", Expr::Str(StrNode::new("1")));

    let val = extend_env.clone().borrow_mut().str_lookup("1");
    assert!(val.unwrap().as_str() == "1");
}

#[test]
fn test_env_parent() {
    let mut env = Env::new();
    env.str_def("hello", Expr::Str(StrNode::new("world")));

    let vars = Expr::Pair(PairNode::new( Expr::Str(StrNode::new("var")), Expr::Nil));
    let vals = Expr::Pair(PairNode::new( Expr::Str(StrNode::new("val")), Expr::Nil));


    let extend_env = env.extend(vars, vals);
    let parent = extend_env.clone().borrow_mut().parent().unwrap();
    let val = parent.clone().borrow_mut().str_lookup("hello");
    assert!(val.unwrap().as_str() == "world");
}

#[bench]
fn env_bench(b: &mut Bencher) {
    fn test_env() {
        let mut env = Env::new();
        for i in 1..10000 {
            let key = i.to_string();
            env.def_var(key.clone(), Expr::Str(StrNode::new("world")));
            let val = env.lookup(key);
            assert!(val.unwrap().as_str() == "world");
        }
    }

    b.iter(|| test_env());
}


#[bench]
fn env_bench_iter(b: &mut Bencher) {
    fn test_env() {
        let mut env = Env::new();
        for i in 1..1000 {
            env.str_def("hello", Expr::Str(StrNode::new("world")));
            let val = env.str_lookup("hello");
            assert!(val.unwrap().as_str() == "world");
        }
    }

    b.iter(|| test_env());
}
