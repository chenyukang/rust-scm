
use ast::ExprAst;
use ast::IntNode;
use env::Env;

use ast::SymbolNode;
// use ast::BoolNode;
// use ast::CharNode;
// use ast::StrNode;
// use ast::PairNode;
// use ast::SymbolNode;
// use ast::EmptyListNode;

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

    fn _eval(&mut self, ast: ExprAst, env: &mut Env) -> ExprAst {
        if ast.is_self() {
            return ast;
        } else if ast.is_symbol() {
            return env.lookup(ast).unwrap();
        } else if ast.is_quote() {
            return ast.cdr().car();
        } else if ast.is_assign() {
            return self._eval_assign(ast, env);
        }
        ExprAst::Int(IntNode::new(0))
    }

    fn _eval_assign(&mut self, exp: ExprAst, env: &mut Env) -> ExprAst {
        let var = exp.cdr().car();
        let val = self._eval(exp.cdr().cdr().car(), env);
        env.def_var(var, val);
        return ExprAst::Symbol(SymbolNode::new("OK"));
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
    assert!(res.as_bool() == true);

}
