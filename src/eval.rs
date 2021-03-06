#[cfg(test)]
use std;
use std::cell::RefCell;
use std::io::Read;
use std::rc::Rc;

use ast::*;
use env::*;
use parser::*;

pub struct Evaler<R> {
    parser: Parser<R>,
    env: Rc<RefCell<Env>>,
    iteractive: bool,
}

#[allow(dead_code)]
impl<R: Read> Evaler<R> {
    pub fn new(inner: R, iteractive: bool) -> Evaler<R> {
        let env = Env::new();
        Evaler {
            parser: Parser::new_from(inner, iteractive),
            env: Rc::new(RefCell::new(env)),
            iteractive: iteractive,
        }
    }

    pub fn eval(&mut self) -> Option<Expr> {
        let mut res = None;
        loop {
            if self.iteractive {
                print!("> ");
            }
            let exp = self.parser.read_exp();
            match exp {
                Some(_exp) => {
                    let r = self.eval_exp(_exp);
                    if self.iteractive {
                        r.print();
                        println!("");
                    }
                    res = Some(r);
                }
                None => break,
            }
        }
        res
    }

    pub fn eval_from(&mut self, code: String) -> Option<Expr> {
        self.parser.load(code);
        let mut res = None;
        loop {
            let exp = self.parser.read_exp();
            match exp {
                Some(_exp) => {
                    let r = self.eval_exp(_exp);
                    res = Some(r);
                }
                None => break,
            }
        }
        res
    }

    fn eval_exp(&mut self, exp: Expr) -> Expr {
        if exp.is_self() {
            return exp;
        }
        if exp.is_sym() {
            return self.env.borrow().lookup(exp.as_str()).unwrap();
        }
        if exp.is_quote() {
            return exp.cdr().car();
        }
        if exp.is_assign() {
            return self.eval_assign(exp);
        }
        if exp.is_def() {
            return self.eval_def(exp);
        }
        if exp.is_begin() {
            return self.eval_begin(exp);
        }
        if exp.is_if() {
            return self.eval_if(exp);
        }
        if exp.is_lambda() {
            return self.eval_lambda(exp);
        }
        if exp.is_and() {
            return self.eval_and(exp);
        }
        if exp.is_or() {
            return self.eval_or(exp);
        }
        if exp.is_cond() {
            return self.eval_cond(exp);
        }
        if exp.is_let() {
            return self.eval_let(exp);
        }
        if exp.is_pair() {
            return self.eval_app(exp);
        }
        Expr::new_sym("OK")
    }

    fn eval_assign(&mut self, exp: Expr) -> Expr {
        let var = exp.c("da");
        let val = self.eval_exp(exp.c("dda"));
        let env = self.env.clone();
        env.borrow_mut().def_var(var.as_str(), val);
        Expr::new_sym("OK")
    }

    fn eval_def(&mut self, exp: Expr) -> Expr {
        let var = exp.def_var();
        let val = exp.def_val();
        let val = self.eval_exp(val);
        let env = self.env.clone();
        env.borrow_mut().def_var(var.as_str(), val);
        Expr::new_sym("OK")
    }

    fn eval_if(&mut self, exp: Expr) -> Expr {
        let pred = exp.c("da");
        let blk_t = exp.c("dda");
        let blk_f = exp.c("ddd");
        let res = self.eval_exp(pred);
        if res.is_true() {
            self.eval_exp(blk_t)
        } else {
            if res.is_empty() {
                Expr::Bool(false)
            } else {
                self.eval_exp(blk_f.car())
            }
        }
    }

    fn eval_and(&mut self, exp: Expr) -> Expr {
        let mut elems = exp.cdr();
        if elems.is_empty() {
            return Expr::Bool(true);
        }
        loop {
            if elems.is_last() {
                break;
            }
            let res = self.eval_exp(elems.car());
            if res.is_false() {
                return Expr::Bool(false);
            }
            elems = elems.cdr();
        }
        self.eval_exp(elems.car())
    }

    fn eval_or(&mut self, exp: Expr) -> Expr {
        let mut elems = exp.cdr();
        if elems.is_empty() {
            return Expr::Bool(true);
        }
        loop {
            if elems.is_last() {
                break;
            }
            let res = self.eval_exp(elems.car());
            if res.is_true() {
                return Expr::Bool(true);
            }
            elems = elems.cdr();
        }
        self.eval_exp(elems.car())
    }

    fn eval_cond(&mut self, exp: Expr) -> Expr {
        let mut elems = exp.cdr().car();
        loop {
            if elems.is_empty() {
                break;
            }
            let cur = elems.car();
            let val = self.eval_exp(cur.clone());
            if val.is_true() || val == Expr::new_sym("else") {
                return self.eval_exp(cur.clone().cdr().car());
            }
            elems = elems.cdr();
        }
        Expr::Bool(true)
    }

    fn eval_let(&mut self, exp: Expr) -> Expr {
        fn bind_params(exp: Expr) -> Expr {
            if exp.is_empty() {
                Expr::Nil
            } else {
                Expr::new_pair(exp.c("aa"), bind_params(exp.cdr()))
            }
        }

        fn bind_args(exp: Expr) -> Expr {
            if exp.is_empty() {
                Expr::Nil
            } else {
                Expr::new_pair(exp.c("ada"), bind_args(exp.cdr()))
            }
        }

        let bindings = exp.c("da");
        let obj = Expr::new_pair(
            bind_params(bindings.clone()).make_lambda(exp.c("dd")),
            bind_args(bindings),
        );
        self.eval_exp(obj)
    }

    fn eval_begin(&mut self, exp: Expr) -> Expr {
        let mut _exp = exp.cdr();
        loop {
            if _exp.is_last() {
                break;
            }
            self.eval_exp(_exp.car());
            _exp = _exp.cdr();
        }
        self.eval_exp(_exp.car())
    }

    fn eval_values(&mut self, exprs: Expr) -> Expr {
        if exprs.is_empty() {
            Expr::Nil
        } else {
            let first = self.eval_exp(exprs.car());
            Expr::new_pair(first, self.eval_values(exprs.cdr()))
        }
    }

    fn eval_app(&mut self, expr: Expr) -> Expr {
        let _proc = self.eval_exp(expr.car());
        let _args = self.eval_values(expr.cdr());
        if _proc.is_proc() {
            let func = _proc.as_proc().func();
            return func(_args);
        } else {
            assert!(_proc.is_cproc());
            let _vars = _proc.params();
            let env = self.env.clone();
            self.env = env.borrow_mut().extend(_vars, _args);
            let begin = Expr::new_sym("begin");
            let res = self.eval_exp(Expr::new_pair(begin, _proc.body()));
            let env = self.env.clone();
            self.env = env.borrow_mut().parent().unwrap();
            res
        }
    }

    fn eval_lambda(&mut self, expr: Expr) -> Expr {
        // vars + body + env
        Expr::new_cproc(expr.c("da"), expr.c("dd"), self.env.clone())
    }
}

#[allow(unused_macros)]
macro_rules! test_case {
    ($test_str:expr, $expect_type:ident, $expect_val:expr) => {{
        let mut evaler = Evaler::new(std::io::stdin(), false);
        let res = evaler.eval_from($test_str.to_string()).unwrap();
        if res.$expect_type() != $expect_val {
            assert!(false);
        }
    }};
}

#[test]
fn test_evaler() {
    test_case!("11", as_int, 11);
    test_case!("'a", as_str, "a");
    test_case!(r#""hello""#, as_str, "hello");
    test_case!("#t", as_bool, true);
    test_case!("(and #t #t)", as_bool, true);

    test_case!("(and #t #f)", as_bool, false);
    test_case!("(or #t #t)", as_bool, true);
    test_case!("(or #f #f)", as_bool, false);
    test_case!("(or #t #f)", as_bool, true);
    test_case!("(set! a 1)", as_str, "OK");
    test_case!("(integer? 1)", as_bool, true);
    test_case!("(boolean? #t)", as_bool, true);
    test_case!("(boolean? #f)", as_bool, true);
    test_case!("(integer? #t)", as_bool, false);
    test_case!("(+ 1 1)", as_int, 2);
    test_case!("(- 1 1)", as_int, 0);
    test_case!("(+ 1 1  1)", as_int, 3);
    test_case!("(+ 1 1 -1 -1)", as_int, 0);
    test_case!("(/ 2 1)", as_int, 2);
    test_case!("(* 2 2)", as_int, 4);
    test_case!("(> 2 1)", as_bool, true);
    test_case!("(> 1 2)", as_bool, false);
    test_case!("(> 1 1)", as_bool, false);
    test_case!("(- (+ 3 (* 8 5)) 1)", as_int, 42);
    test_case!("(eq? 1 1)", as_bool, true);
    test_case!("(eq? 1 2)", as_bool, false);
    test_case!("(eq? 1 #f)", as_bool, false);
    test_case!("(pair? '())", as_bool, true);
    test_case!("(> (+ 1 1) 0)", as_bool, true);
    test_case!("(if (> 1 0) 1 else 2)", as_int, 1);
    test_case!("(begin (set! a 1) a)", as_int, 1);
    test_case!("(let ((a 1)) (+ a 1))", as_int, 2);
    test_case!("(let ((a 1)) (> (+ a 1) 0))", as_bool, true);
    test_case!("(let ((a 1) (b 2)) (> a b))", as_bool, false);
    test_case!("(define (add a b) (+ a b))", as_str, "OK");
    test_case!("(cond ((eq? 1 1) 1) ((> 1 2) 2) ))", as_int, 1);
    test_case!("(begin (set! x 5) (set! x 4) (+ x 1))", as_int, 5);
    test_case!("(car '(1 2))", as_int, 1);
    test_case!("(car (cdr '(1 2)))", as_int, 2);
    test_case!("(car (cons 1 2))", as_int, 1);
    test_case!("(car (cons (pair? '(1)) pair? '()))", as_bool, true);
    test_case!("(car (cdr (cons 1 2)))", as_int, 2);
    test_case!("(car (car (cons (cons 2 3) (cons 1 2))))", as_int, 2);
    test_case!("((lambda (x) x) 1)", as_int, 1);
    test_case!("((lambda (x y) (+ x y )) 1 2)", as_int, 3);
    test_case!(
        "(define add4 (let ((x 4)) (lambda (y) (+ x y))))",
        as_str,
        "OK"
    );
    test_case!("(begin 1 2)", as_int, 2);
    test_case!("((lambda (x) x) 5)", as_int, 5);
    test_case!("(let ((fu (lambda (x) (+ x 1)))) (fu 1))", as_int, 2);
    test_case!(
        "((lambda (x y ) (if ( = y 0) 1 (* y (x x (- y 1)))))
               (lambda (x y ) (if ( = y 0) 1 (* y (x x (- y 1))))) 5)",
        as_int,
        5isize * 4 * 3 * 2
    );
}

// #[bench]
// fn eval_bench(b: &mut Bencher) {
//     b.iter(|| test_evaler());
// }

// #[bench]
// fn lambda_bench(b: &mut Bencher) {
//     b.iter(||
//            test_case!("((lambda (x y ) (if ( = y 0) 1 (* y (x x (- y 1)))))
//                       (lambda (x y ) (if ( = y 0) 1 (* y (x x (- y 1))))) 5)", as_int, 5isize*4*3*2));
// }
