
use ast::ExprAst;
use ast::IntNode;
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
        self.parser.load(code);
        self._eval()
    }

    fn _eval(&mut self) -> ExprAst {
        ExprAst::Int(IntNode::new(0))
    }
}
