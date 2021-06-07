use env;
use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

#[derive(Clone, PartialEq, Debug)]
pub enum Expr {
    Int(isize),
    Str(String),
    Sym(String),
    Bool(bool),
    Char(char),
    Pair(Vec<Expr>),
    Proc(ProcFunc),
    CompProc(Vec<Expr>, Rc<RefCell<env::Env>>),
    Nil,
}

#[derive(Clone)]
pub struct ProcFunc(fn(Expr) -> Expr);

impl PartialEq for ProcFunc {
    fn eq(&self, o: &ProcFunc) -> bool {
        let _o: *const () = unsafe { ::std::mem::transmute(o) };
        let _s: *const () = unsafe { ::std::mem::transmute(self) };
        _s == _o
    }
    fn ne(&self, o: &ProcFunc) -> bool {
        !self.eq(o)
    }
}

impl ProcFunc {
    pub fn func(&self) -> fn(Expr) -> Expr {
        match *self {
            ProcFunc(fun) => fun,
        }
    }
}

impl fmt::Debug for ProcFunc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "proc")
    }
}

impl Expr {
    pub fn new_pair(car: Expr, cdr: Expr) -> Expr {
        Expr::Pair(vec![car, cdr])
    }

    pub fn new_str(val: &str) -> Expr {
        Expr::Str(val.to_string())
    }

    pub fn new_sym(val: &str) -> Expr {
        Expr::Sym(val.to_string())
    }

    pub fn new_proc(func: fn(Expr) -> Expr) -> Expr {
        Expr::Proc(ProcFunc(func))
    }

    pub fn new_cproc(params: Expr, body: Expr, env: Rc<RefCell<env::Env>>) -> Expr {
        Expr::CompProc(vec![params, body], env)
    }

    pub fn is_true(&self) -> bool {
        self.as_bool()
    }

    pub fn is_false(&self) -> bool {
        !self.as_bool()
    }

    pub fn is_pair(&self) -> bool {
        match *self {
            Expr::Pair(_) | Expr::Nil => true,
            _ => false,
        }
    }

    pub fn is_empty(&self) -> bool {
        match *self {
            Expr::Nil => true,
            _ => false,
        }
    }

    pub fn is_cproc(&self) -> bool {
        match *self {
            Expr::CompProc(_, _) => true,
            _ => false,
        }
    }

    pub fn is_last(&self) -> bool {
        assert!(self.is_pair());
        self.cdr().is_empty()
    }

    pub fn is_self(&self) -> bool {
        match *self {
            Expr::Bool(_) | Expr::Int(_) | Expr::Char(_) | Expr::Str(_) => true,
            _ => false,
        }
    }

    pub fn is_tagged(&self, tag: Expr) -> bool {
        if self.is_pair() {
            let car = self.car();
            return car.is_sym() && car == tag;
        }
        false
    }

    pub fn as_int(&self) -> isize {
        match *self {
            Expr::Int(ref val) => return *val,
            _ => panic!("expect Int"),
        }
    }

    pub fn as_bool(&self) -> bool {
        match *self {
            Expr::Bool(ref val) => return *val,
            _ => panic!("expect Bool"),
        }
    }

    #[cfg(test)]
    pub fn as_char(&self) -> char {
        match *self {
            Expr::Char(ref val) => return *val,
            _ => panic!("expect Char"),
        }
    }

    pub fn as_proc(&self) -> ProcFunc {
        match *self {
            Expr::Proc(ref val) => val.clone(),
            _ => panic!("expect Proc"),
        }
    }

    pub fn as_str(&self) -> String {
        match *self {
            Expr::Str(ref val) => return val.clone(),
            Expr::Sym(ref val) => return val.clone(),
            _ => panic!("expect Str"),
        }
    }

    pub fn car(&self) -> Expr {
        match *self {
            Expr::Pair(ref vec) => {
                return vec[0].clone();
            }
            _ => panic!("expect Pair"),
        }
    }

    pub fn cdr(&self) -> Expr {
        match *self {
            Expr::Pair(ref vec) => {
                return vec[1].clone();
            }
            _ => panic!("expect Pair"),
        }
    }

    pub fn def_var(&self) -> Expr {
        assert!(self.is_def());
        if self.cdr().car().is_sym() {
            self.c("da")
        } else {
            self.c("daa")
        }
    }

    pub fn def_val(&self) -> Expr {
        assert!(self.is_def());
        if self.c("da").is_sym() {
            self.c("dda")
        } else {
            //proc
            self.c("dad").make_lambda(self.c("dd"))
        }
    }

    pub fn make_lambda(&self, body: Expr) -> Expr {
        let lambda = Expr::new_sym("lambda");
        Expr::new_pair(lambda, Expr::new_pair((*self).clone(), body))
    }

    pub fn params(&self) -> Expr {
        match *self {
            Expr::CompProc(ref val, _) => val[0].clone(),
            _ => panic!("expect CompProc"),
        }
    }

    pub fn body(&self) -> Expr {
        match *self {
            Expr::CompProc(ref val, _) => val[1].clone(),
            _ => panic!("expect CompProc"),
        }
    }

    pub fn c(&self, s: &str) -> Expr {
        assert!(self.is_pair());
        let mut r = self.clone();
        for c in s.to_string().chars() {
            if c == 'a' {
                r = r.car();
            } else {
                r = r.cdr();
            }
        }
        r
    }

    fn collect(&self) -> Vec<Expr> {
        let mut res: Vec<Expr> = vec![];
        let mut _exp = self.clone();
        loop {
            let f = _exp.car();
            if f.is_self() {
                res.push(f);
            } else if !f.is_empty() {
                for e in f.collect() {
                    res.push(e);
                }
                //res.push_all(f.collect().as_slice());
            }
            _exp = _exp.cdr();
            if !_exp.is_pair() {
                break;
            }
            if _exp.is_empty() {
                break;
            }
        }
        res
    }

    pub fn print(&self) {
        match *self {
            Expr::Int(ref ast) => println!("{:?}", ast),
            Expr::Str(ref ast) => println!("{:?}", ast),
            Expr::Bool(ref ast) => println!("{:?}", ast),
            Expr::Sym(ref ast) => println!("{:?}", ast),
            Expr::Char(ref ast) => println!("{:?}", ast),
            Expr::Pair(_) => {
                print!("(");
                let exps = self.collect();
                for i in 0..exps.len() {
                    exps[i].print();
                    if i != exps.len() - 1 {
                        print!(" ");
                    }
                }
                print!(")");
            }
            Expr::Proc(ref ast) => println!("{:?}", ast),
            Expr::CompProc(ref ast, _) => println!("{:?}", ast),
            Expr::Nil => print!("Nil"),
        }
    }
}

macro_rules! is_ast_type {
    ($func_name:ident, $type_name:ident) => {
        impl Expr {
            pub fn $func_name(&self) -> bool {
                match *self {
                    Expr::$type_name(_) => true,
                    _ => false,
                }
            }
        }
    };
}

is_ast_type!(is_char, Char);
is_ast_type!(is_int, Int);
is_ast_type!(is_sym, Sym);
is_ast_type!(is_str, Str);
is_ast_type!(is_proc, Proc);
is_ast_type!(is_bool, Bool);

macro_rules! is_type {
    ($func_name:ident, $type_str:expr) => {
        impl Expr {
            pub fn $func_name(&self) -> bool {
                return self.is_tagged(Expr::Sym($type_str.to_string()));
            }
        }
    };
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_ast() {
        let int_node = Expr::Int(3);
        assert!(int_node.as_int() == 3);

        let char_node = Expr::Char('a');
        assert!(char_node.as_char() == 'a');

        let bool_node = Expr::Bool(false);
        assert!(bool_node.as_bool() == false);

        let str_node = Expr::new_str("hello");
        assert!(str_node.as_str() == "hello");
        assert!(str_node.is_self());

        let int_node = Expr::Int(3);
        let str_node = Expr::new_str("hello");
        let pair_node = Expr::new_pair(int_node, str_node);
        let car_node = pair_node.car();
        let cdr_node = pair_node.cdr();
        assert!(car_node.as_int() == 3);
        assert!(cdr_node.as_str() == "hello");
        assert!(!pair_node.is_self());

        let sym_node = Expr::new_sym("sym");
        assert!(sym_node.is_sym());
        assert!(!sym_node.is_self());

        let empty_node = Expr::Nil;
        assert!(empty_node.is_empty());
        assert!(!empty_node.is_self());
    }

    #[test]
    fn test_ast_is_set() {
        macro_rules! test_case {
            ($str_name:expr) => {{
                Expr::new_pair(Expr::new_sym($str_name), Expr::Int(3))
            }};
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
        let aa = Expr::new_sym("else");
        let bb = Expr::new_sym("else");
        assert!(aa == bb);
    }

    #[test]
    fn test_proc() {
        fn _proc(obj: Expr) -> Expr {
            obj.print();
            return Expr::new_sym("ok");
        }

        let proc_node = Expr::new_proc(_proc);
        assert!(proc_node.is_proc());
        assert!(!proc_node.is_cproc());
    }
}
