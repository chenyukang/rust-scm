use ast::*;

#[deriving(Clone, Show)]
pub struct Parser {
    code: String,
    cur: uint,
    col: uint,
    line: uint
}

#[allow(dead_code)]
pub struct ParserError {
    line: uint,
    col: uint,
    desc: String
}

#[allow(dead_code)]
impl ParserError {
    pub fn new(line: uint, col: uint, desc: String) -> ParserError {
        ParserError {
            line: line,
            col: col,
            desc: desc
        }
    }
}

#[allow(dead_code)]
pub type ParseResult<T> = Result<T, ParserError>;

impl Parser {
    pub fn new() -> Parser {
        Parser{
            code: "".to_string(),
            line: 0, cur: 0, col: 0
        }
    }

    pub fn load(&mut self, code: String) -> ExprAst {
        self.code = code;
        self.line = 1;
        self.cur = 0;
        self.col = 0;
        self.skip_space();
        self.read_exp()
    }

    //============= private methods =================
    fn read_exp(&mut self) -> ExprAst {
        let mut cur = self.readc();
        if cur == '#' {
            let next = self.readc();
            match next {
                't' => return ExprAst::Bool(BoolNode::new(true)),
                'f' => return ExprAst::Bool(BoolNode::new(false)),
                '\\' => return self.read_char(),
                _ => panic!("error")
            }
        } else if UnicodeChar::is_numeric(cur) ||
            (cur == '-' && (UnicodeChar::is_numeric(self.peekc()))) {
                let mut sign = 1i;
                let mut num = 0i;
                if cur == '-' {
                    sign = -1;
                } else {
                    self.unread();
                }
                loop {
                    cur = self.readc();
                    if !UnicodeChar::is_numeric(cur) {
                        break;
                    }
                    num = (num * 10i) + (cur as int - '0' as int);
                }
                num *= sign;
                if self.is_delimiter(cur) {
                    self.unread();
                    return ExprAst::Int(IntNode::new(num));
                } else {
                    panic!("number not followed by delimiter");
                }
            } else if cur == '\"' {
                let mut buf = String::new();
                loop {
                    cur = self.readc();
                    if cur == '\"' {
                        break;
                    }
                    buf.push(cur);
                }
                return ExprAst::Str(StrNode::new(buf.as_slice()));
            } else if cur == '(' {
                return self.read_pair();
            } else if self.is_initial(cur) {
                let mut buf = String::new();
                buf.push(cur);
                loop {
                    cur = self.readc();
                    if !(self.is_initial(cur) || UnicodeChar::is_numeric(cur)) {
                        break;
                    }
                    buf.push(cur);
                }
                if self.is_delimiter(cur) {
                    self.unread();
                    return ExprAst::Symbol(SymbolNode::new(buf.as_slice()));
                }
            } else if cur == '\'' {
                let quote_sym = ExprAst::Symbol(SymbolNode::new("quote"));
                let quote_exp = ExprAst::Pair(PairNode::new(self.read_exp(),
                                                            ExprAst::Nil));
                return ExprAst::Pair(PairNode::new(quote_sym, quote_exp));
            }
        ExprAst::Int(IntNode::new(0))
    }

    fn read_pair(&mut self) -> ExprAst {
        self.skip_space();
        let mut cur = self.readc();
        // rust-mode bug here
        if cur != '(' && cur == ')' {
            return ExprAst::Nil;
        }
        self.unread();
        let car_obj = self.read_exp();
        self.skip_space();
        cur = self.readc();
        if cur != '.' {
            self.unread();
            let cdr_obj = self.read_pair();
            return ExprAst::Pair(PairNode::new(car_obj, cdr_obj));
        } else {
            return ExprAst::Nil;
        }
    }

    fn is_delimiter(&self, ch: char) -> bool {
        ch.is_whitespace() ||
            ch == '\"' || ch == '(' || ch == ')' ||  ch == ';' ||
            (ch as u32 == 0)
    }

    fn is_initial(&self, ch: char) -> bool {
        ch.is_alphabetic() ||
            ch == '*' || ch == '/' || ch == '+' || ch == '-' ||
            ch == '>' || ch == '<' || ch == '=' || ch == '?' ||
            ch == '!'
    }

    fn skip_space(&mut self) {
        while self.peekc().is_whitespace() {
            self.readc();
        }
    }

    fn peekc(&self) -> char {
        if self.cur >= self.code.len() {
            panic!("invalid position");
        }
        self.code.char_at(self.cur)
    }

    fn prevc(&self) -> char {
        if self.cur <= 0 {
            panic!("invalid position");
        }
        self.code.char_at(self.cur - 1)
    }

    fn readc(&mut self) -> char {
        if self.cur < self.code.len() {
            let res = self.peekc();
            if res == '\n' {
                self.line += 1;
            }
            self.cur += 1;
            res
        } else {
            0 as char
        }
    }

    fn unread(&mut self) {
        if self.cur == 0 {
            panic!("error current position");
        }
        if self.prevc() == '\n' {
            assert!(self.line > 1);
            self.line -= 1;
        }
        self.cur -= 1;
    }

    fn read_char(&mut self) -> ExprAst {
        ExprAst::Char(CharNode::new('a'))
    }
}

#[test]
fn test_parser() {
    let mut parser = Parser::new();
    let res = parser.load("11".to_string());
    assert!(res.as_int() == 11);
    res.print();

    let res = parser.load("-11".to_string());
    assert!(res.as_int() == -11);
    res.print();

    let res = parser.load(r#""hello""#.to_string());
    assert!(res.as_str() == "hello");
    res.print();

    let res = parser.load("()".to_string());
    assert!(res.is_empty());
    res.print();

    let res = parser.load("(1 2)".to_string());
    assert!(res.is_pair());
    assert!(res.car().as_int() == 1);
    assert!(res.cdr().car().as_int() == 2);
    assert!(res.cdr().cdr().is_empty());

    let res = parser.load("(+ 1 2)".to_string());
    assert!(res.is_pair());
    assert!(res.car().is_symbol());
    assert!(res.cdr().car().as_int() == 1);
    assert!(res.cdr().cdr().car().as_int() == 2);
}
