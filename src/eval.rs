
use ast::*;
use env::*;

//use ast::Ast;
use parser::Parser;

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
        self._eval(ast, &mut Env::new())
    }

    fn _eval(&mut self, exp: ExprAst, env: &mut Env) -> ExprAst {
        if exp.is_self() {
            return exp;
        } else if exp.is_symbol() {
            return env.lookup(exp).unwrap();
        } else if exp.is_quote() {
            return exp.cdr().car();
        } else if exp.is_assign() {
            return self._eval_assign(exp, env);
        } else if exp.is_def() {
            return self._eval_def(exp, env);
        } else if exp.is_begin() {
            return self._eval_begin(exp, env);
        } else if exp.is_if() {
            return self._eval_if(exp, env);
        } else if exp.is_and() {
            return self._eval_and(exp, env);
        } else if exp.is_or() {
            return self._eval_or(exp, env);
        } else if exp.is_cond() {
            return self._eval_cond(exp, env);
        } else if exp.is_pair() { //app
            return self._eval_app(exp, env);
        }
        ExprAst::Int(IntNode::new(0))
    }

    fn _eval_assign(&mut self, exp: ExprAst, env: &mut Env) -> ExprAst {
        let var = exp.cdr().car();
        let val = self._eval(exp.cdr().cdr().car(), env);
        env.def_var(var, val);
        return ExprAst::Symbol(SymbolNode::new("OK"));
    }

    fn _eval_def(&mut self, exp: ExprAst, env: &mut Env) -> ExprAst {
        let var = exp.def_var();
        let val = exp.def_val();
        let val = self._eval(val, env);
        env.def_var(var, val);
        return ExprAst::Symbol(SymbolNode::new("OK"));
    }

    fn _eval_if(&mut self, exp: ExprAst, env: &mut Env) -> ExprAst {
        let pred = exp.cdr().car();
        let t_blk = exp.cdr().cdr().car();
        let f_blk = exp.cdr().cdr().cdr();
        let res = self._eval(pred, env);
        if res.is_true() {
            return self._eval(t_blk, env);
        } else {
            if res.is_empty() {
                return ExprAst::Bool(BoolNode::new(false));
            } else {
                return self._eval(f_blk.car(), env);
            }
        }
    }

    fn _eval_and(&mut self, exp: ExprAst, env: &mut Env) -> ExprAst {
        let mut elems = exp.cdr();
        if elems.is_empty() {
            return ExprAst::Bool(BoolNode::new(true));
        }
        loop {
            if elems.is_last() { break; }
            let res = self._eval(elems.car(), env);
            if res.is_false() {
                return ExprAst::Bool(BoolNode::new(false));
            }
            elems = elems.cdr();
        }
        return self._eval(elems.car(), env);
    }

    fn _eval_or(&mut self, exp: ExprAst, env: &mut Env) -> ExprAst {
        let mut elems = exp.cdr();
        if elems.is_empty() {
            return ExprAst::Bool(BoolNode::new(true));
        }
        loop {
            if elems.is_last() { break; }
            let res = self._eval(elems.car(), env);
            if res.is_true() {
                return ExprAst::Bool(BoolNode::new(true));
            }
            elems = elems.cdr();
        }
        return self._eval(elems.car(), env);
    }

    fn _eval_cond(&mut self, exp: ExprAst, env: &mut Env) -> ExprAst {
        let mut elems = exp.cdr().car();
        loop {
            if elems.is_empty() { break; }
            let cur = elems.car();
            let val = self._eval(cur.clone(), env);
            if val.is_true() || val == ExprAst::Symbol(SymbolNode::new("else")) {
                return self._eval(cur.clone().cdr().car(), env);
            }
            elems = elems.cdr();
        }
        return ExprAst::Bool(BoolNode::new(true));
    }

    fn _eval_begin(&mut self, exp: ExprAst, env: &mut Env) -> ExprAst {
        let mut _exp = exp.cdr();
        loop {
            if _exp.is_last() { break; }
            self._eval(_exp.car(), env);
            _exp = _exp.cdr();
        }
        return self._eval(_exp.car(), env);
    }

    fn _eval_values(&mut self, exprs: ExprAst, env: &mut Env) -> ExprAst {
        if exprs.is_empty() {
            return ExprAst::Nil;
        } else {
            let first = self._eval(exprs.car(), env);
            return ExprAst::Pair(PairNode::new(first,
                                               self._eval_values(exprs.cdr(), env)));
        }
    }

    fn _eval_app(&mut self, expr: ExprAst, env: &mut Env) -> ExprAst {
        let _proc = self._eval(expr.car(), env);
        if _proc.is_proc() {
            let _args = self._eval_values(expr.cdr(), env);
            let func = _proc.as_proc().func();
            return func(_args);
        } else {
            return ExprAst::Bool(BoolNode::new(true));
        }
    }

}

#[test]
fn test_evaler() {
    let mut evaler = Evaler::new();
    let res = evaler.eval("11".to_string());
    assert!(res.as_int() == 11);

    let res = evaler.eval(r#""hello""#.to_string());
    assert!(res.as_str() == "hello");

    let res = evaler.eval("#t".to_string());
    assert!(res.as_bool());

    let res = evaler.eval("(and #t #t)".to_string());
    assert!(res.as_bool());

    let res = evaler.eval("(and #t #f)".to_string());
    assert!(res.as_bool() == false);

    let res = evaler.eval("(or #t #t)".to_string());
    assert!(res.as_bool());

    let res = evaler.eval("(or #f #f)".to_string());
    assert!(res.as_bool() == false);

    let res = evaler.eval("(or #t #f)".to_string());
    assert!(res.as_bool());

    let res = evaler.eval("(set! a 1)".to_string());
    assert!(res.as_str() == "OK");

    let res = evaler.eval("(+ 1 1)".to_string());
    assert!(res.as_int() == 2);

    let res = evaler.eval("(+ 1 1 1)".to_string());
    assert!(res.as_int() == 3);

    let res = evaler.eval("(+ 1 1 -1 -1)".to_string());
    assert!(res.as_int() == 0);
}
