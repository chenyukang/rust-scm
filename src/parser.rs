use ast::*;
use std::io::Reader;
use std::io;

#[derive(Clone, Show)]
pub struct Parser<R> {
    code: String,
    cur: usize,
    col: usize,
    line: usize,
    iteractive: bool,
    inner: R
}

impl <R: Reader> Parser<R> {
    pub fn new_from(inner: R, iteractive: bool) -> Parser<R> {
        Parser{
            code: "".to_string(),
            line: 0, cur: 0, col: 0,
            iteractive: iteractive,
            inner: inner
        }
    }

    pub fn load(&mut self, code: String) {
        self.code = code;
        self.line = 1;
        self.cur = 0;
        self.col = 0;
    }

    //============= private methods =================
    pub fn read_exp(&mut self) -> Option<Expr> {
        if self.eof() { return None }
        self.skip_space();
        let mut cur = self.readc();
        if cur == '#' {
            let next = self.readc();
            match next {
                't' => return Some(Expr::Bool(BoolNode::new(true))),
                'f' => return Some(Expr::Bool(BoolNode::new(false))),
                '\\' => return self.read_char(),
                _ => panic!("error")
            }
        } else if cur.is_numeric() ||
            (cur == '-' && (self.peekc().is_numeric())) {
                let mut sign = 1is;
                let mut num = 0is;
                if cur == '-' {
                    sign = -1;
                } else {
                    self.unread();
                }
                loop {
                    cur = self.readc();
                    if !cur.is_numeric() {
                        break;
                    }
                    num = (num * 10is) + (cur as isize - '0' as isize);
                }
                num *= sign;
                if self.is_delimiter(cur) {
                    self.unread();
                }
                return Some(Expr::Int(IntNode::new(num)));
            } else if cur == '\"' {
                let mut buf = String::new();
                loop {
                    cur = self.readc();
                    if cur == '\"' {
                        break;
                    }
                    buf.push(cur);
                }
                return Some(Expr::Str(StrNode::new(buf.as_slice())));
            } else if cur == '(' && cur != ')' {
                return self.read_pair();
            } else if self.is_initial(cur) {
                let mut buf = String::new();
                buf.push(cur);
                loop {
                    cur = self.readc();
                    if !(self.is_initial(cur) || cur.is_numeric()) {
                        break;
                    }
                    buf.push(cur);
                }
                if self.is_delimiter(cur) {
                    self.unread();
                }
                return Some(Expr::Symbol(SymbolNode::new(buf.as_slice())));
            } else if cur == '\'' {
                let quote_sym = Expr::Symbol(SymbolNode::new("quote"));
                let quote_exp = Expr::Pair(PairNode::new(self.read_exp().unwrap(),
                                                            Expr::Nil));
                return Some(Expr::Pair(PairNode::new(quote_sym, quote_exp)));
            }
        None
    }

    fn read_pair(&mut self) -> Option<Expr> {
        self.skip_space();
        let mut cur = self.readc();
        // rust-mode bug here
        if cur != '(' && cur == ')' {
            return Some(Expr::Nil);
        }
        self.unread();
        let car_obj = self.read_exp();
        self.skip_space();
        cur = self.readc();
        if cur != '.' {
            self.unread();
            let cdr_obj = self.read_pair();
            return Some(Expr::Pair(PairNode::new(car_obj.unwrap(),
                                                    cdr_obj.unwrap())));
        } else {
            return Some(Expr::Nil);
        }
    }

    fn is_delimiter(&self, ch: char) -> bool {
        ch.is_whitespace() ||
            ch == '\"' || ch == '(' || ch == ')' ||  ch == ';'
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

    fn eof(&mut self) -> bool {
        if self.iteractive {
            let mut vec: Vec<u8> = Vec::new();
            match self.inner.push(1us, &mut vec) {
                Ok(len) => {
                    for i in vec.into_iter() {
                        self.code.push(i as char);
                    }
                },
                Err(_) => { return true; }
            }
        }
        self.cur >= self.code.len()
    }

    fn peekc(&mut self) -> char {
        if self.cur >= self.code.len() {
            if self.iteractive {
                let mut vec: Vec<u8> = Vec::new();
                match self.inner.push(1us, &mut vec) {
                    Ok(len) => {
                        for i in vec.into_iter() {
                            self.code.push(i as char);
                        }
                    },
                    Err(_) => { return 0 as char; }
                }
            }
        }
        if self.cur >= self.code.len() {
            return 0 as char;
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
            assert!(self.line >= 1);
            self.line -= 1;
        }
        self.cur -= 1;
    }

    fn read_char(&mut self) -> Option<Expr> {
        Some(Expr::Char(CharNode::new('a')))
    }
}

#[test]
fn test_parser() {
    macro_rules! test_case {
        ($test_str:expr, $expect_type:ident, $expect_val:expr) => { {
            let mut parser = Parser::new_from(io::stdin(), false);
            parser.load($test_str.to_string());
            let res = parser.read_exp().unwrap();
            if res.$expect_type() != $expect_val {
                assert!(false);
            }
        }}
    }

    macro_rules! test_res {
        ($test_str:expr) => { {
            let mut parser = Parser::new_from(io::stdin(), false);
            parser.load($test_str.to_string());
            parser.read_exp().unwrap()
        }}
    }

    test_case!("11", as_int, 11);
    test_case!("-11", as_int, -11);
    test_case!(r#""hello""#, as_str, "hello");

    let res = test_res!("()");
    assert!(res.is_empty());

    let res = test_res!("(1 2)");
    assert!(res.is_pair());
    assert!(res.car().as_int() == 1);
    assert!(res.cdr().car().as_int() == 2);
    assert!(res.cdr().cdr().is_empty());

    let res = test_res!("(+ 1 2)");
    assert!(res.is_pair());
    assert!(res.car().is_symbol());
    assert!(res.cdr().car().as_int() == 1);
    assert!(res.cdr().cdr().car().as_int() == 2);
}
