use ast::*;
use env::*;
use parser::*;

pub struct Evaler {
    parser: Parser
}

#[allow(dead_code)]
impl Evaler {
    pub fn new() -> Evaler {
        Evaler {
            parser: Parser::new()
        }
    }

    pub fn eval(&mut self, code: String) -> ExprAst {
        let ast = self.parser.load(code);
        self.eval_exp(ast, &mut Env::new())
    }

    fn eval_exp(&self, exp: ExprAst, env: &mut Env) -> ExprAst {
        if exp.is_self() {
            return exp;
        } else if exp.is_symbol() {
            return env.lookup(exp).unwrap();
        } else if exp.is_quote() {
            return exp.cdr().car();
        } else if exp.is_assign() {
            return self.eval_assign(exp, env);
        } else if exp.is_def() {
            return self.eval_def(exp, env);
        } else if exp.is_begin() {
            return self.eval_begin(exp, env);
        } else if exp.is_if() {
            return self.eval_if(exp, env);
        } else if exp.is_lambda() {
            return self.eval_lambda(exp, env);
        } else if exp.is_and() {
            return self.eval_and(exp, env);
        } else if exp.is_or() {
            return self.eval_or(exp, env);
        } else if exp.is_cond() {
            return self.eval_cond(exp, env);
        } else if exp.is_let() {
            return self.eval_let(exp, env);
        } else if exp.is_pair() { //app
            return self.eval_app(exp, env);
        }
        return ExprAst::Symbol(SymbolNode::new("OK"));
    }

    fn eval_assign(&self, exp: ExprAst, env: &mut Env) -> ExprAst {
        let var = exp.cdr().car();
        let val = self.eval_exp(exp.cdr().cdr().car(), env);
        env.def_var(var, val);
        return ExprAst::Symbol(SymbolNode::new("OK"));
    }

    fn eval_def(&self, exp: ExprAst, env: &mut Env) -> ExprAst {
        let var = exp.def_var();
        let val = exp.def_val();
        let val = self.eval_exp(val, env);
        env.def_var(var, val);
        return ExprAst::Symbol(SymbolNode::new("OK"));
    }

    fn eval_if(&self, exp: ExprAst, env: &mut Env) -> ExprAst {
        let pred = exp.cdr().car();
        let t_blk = exp.cdr().cdr().car();
        let f_blk = exp.cdr().cdr().cdr();
        let res = self.eval_exp(pred, env);
        if res.is_true() {
            return self.eval_exp(t_blk, env);
        } else {
            if res.is_empty() {
                return ExprAst::Bool(BoolNode::new(false));
            } else {
                return self.eval_exp(f_blk.car(), env);
            }
        }
    }

    fn eval_and(&self, exp: ExprAst, env: &mut Env) -> ExprAst {
        let mut elems = exp.cdr();
        if elems.is_empty() {
            return ExprAst::Bool(BoolNode::new(true));
        }
        loop {
            if elems.is_last() { break; }
            let res = self.eval_exp(elems.car(), env);
            if res.is_false() {
                return ExprAst::Bool(BoolNode::new(false));
            }
            elems = elems.cdr();
        }
        return self.eval_exp(elems.car(), env);
    }

    fn eval_or(&self, exp: ExprAst, env: &mut Env) -> ExprAst {
        let mut elems = exp.cdr();
        if elems.is_empty() {
            return ExprAst::Bool(BoolNode::new(true));
        }
        loop {
            if elems.is_last() { break; }
            let res = self.eval_exp(elems.car(), env);
            if res.is_true() {
                return ExprAst::Bool(BoolNode::new(true));
            }
            elems = elems.cdr();
        }
        return self.eval_exp(elems.car(), env);
    }

    fn eval_cond(&self, exp: ExprAst, env: &mut Env) -> ExprAst {
        let mut elems = exp.cdr().car();
        loop {
            if elems.is_empty() { break; }
            let cur = elems.car();
            let val = self.eval_exp(cur.clone(), env);
            if val.is_true() || val == ExprAst::Symbol(SymbolNode::new("else")) {
                return self.eval_exp(cur.clone().cdr().car(), env);
            }
            elems = elems.cdr();
        }
        return ExprAst::Bool(BoolNode::new(true));
    }

    fn eval_let(&self, exp: ExprAst, env: &mut Env) -> ExprAst {
        fn bind_params(exp: ExprAst) -> ExprAst {
            if exp.is_empty() {
                ExprAst::Nil
            } else {
                return ExprAst::Pair(PairNode::new(exp.car().car(),
                                                   bind_params(exp.cdr())));
            }
        }

        fn bind_argus(exp: ExprAst) -> ExprAst {
            if exp.is_empty() {
                ExprAst::Nil
            } else {
                return ExprAst::Pair(PairNode::new(exp.car().cdr().car(),
                                                   bind_argus(exp.cdr())));
            }
        }

        //FIXME : remove clone?
        let bindings = exp.cdr().car();
        bindings.print();
        let obj = ExprAst::Pair(PairNode::new(
            bind_params(bindings.clone()).make_lambda(exp.cdr().cdr()),
            bind_argus(bindings)));
        return self.eval_exp(obj, env);
    }

    fn eval_begin(&self, exp: ExprAst, env: &mut Env) -> ExprAst {
        let mut _exp = exp.cdr();
        loop {
            if _exp.is_last() { break; }
            self.eval_exp(_exp.car(), env);
            _exp = _exp.cdr();
        }
        return self.eval_exp(_exp.car(), env);
    }

    fn eval_values(&self, exprs: ExprAst, env: &mut Env) -> ExprAst {
        if exprs.is_empty() {
            return ExprAst::Nil;
        } else {
            let first = self.eval_exp(exprs.car(), env);
            return ExprAst::Pair(PairNode::new(first,
                                               self.eval_values(exprs.cdr(), env)));
        }
    }

    fn _make_begin(&self, exprs: ExprAst) -> ExprAst {
        let begin = ExprAst::Symbol(SymbolNode::new("begin"));
        return ExprAst::Pair(PairNode::new(begin, exprs));
    }


    fn eval_app(&self, expr: ExprAst, env: &mut Env) -> ExprAst {
        let _proc = self.eval_exp(expr.car(), env);
        let _args = self.eval_values(expr.cdr(), env);
        if _proc.is_proc() {
            let func = _proc.as_proc().func();
            return func(_args);
        } else {
            assert!(_proc.is_cproc());
            let _vars = _proc.params();
            let res = self.eval_exp(self._make_begin(_proc.body()),
                                 &mut env.extend(_vars, _args));
            return res;
        }
    }

    fn eval_lambda(&self, expr: ExprAst, env: &Env) -> ExprAst {
        //FIXME: remove clone
        return ExprAst::CompProc(CompProcNode::new(expr.cdr().car(), //vars
                                                   expr.cdr().cdr(), //body
                                                   box env.clone()));

    }
}

#[test]
fn test_evaler() {
    macro_rules! test_case {
        ($test_str:expr, $expect_type:ident, $expect_val:expr) => { {
            let mut evaler = Evaler::new();
            let res = evaler.eval($test_str.to_string());
            if res.$expect_type() != $expect_val {
                assert!(false);
            }
        }}
    }
    test_case!("11", as_int, 11);
    //test_case!("'a", as_char, 'a');
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
    test_case!("(+ 1 1  1)", as_int, 3);
    test_case!("(+ 1 1 -1 -1)", as_int, 0);
    test_case!("(/ 2 1)", as_int, 2);
    test_case!("(* 2 2)", as_int, 4);
    test_case!("(> 2 1)", as_bool, true);
    test_case!("(> 1 2)", as_bool, false);
    test_case!("(> 1 1)", as_bool, false);
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
    test_case!("(car (cdr (cons 1 2)))", as_int, 2);
    test_case!("(car (car (cons (cons 2 3) (cons 1 2))))", as_int, 2);
    test_case!("((lambda (x) x) 1)", as_int, 1);
    test_case!("((lambda (x y) (+ x y )) 1 2)", as_int, 3);
    test_case!("(define add4 (let ((x 4)) (lambda (y) (+ x y))))", as_str, "OK");
}
