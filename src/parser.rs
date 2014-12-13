use ast::ExprAst;
use ast::BoolNode;
use ast::CharNode;
use ast::IntNode;
use ast::StrNode;

#[allow(dead_code)]

#[deriving(Clone, Show)]
pub struct Parser {
    code: String,
    cur: uint,
    col: uint,
    line: uint
}

pub struct ParserError {
    line: uint,
    col: uint,
    desc: String
}

impl ParserError {
    pub fn new(line: uint, col: uint, desc: String) -> ParserError {
        ParserError {
            line: line,
            col: col,
            desc: desc
        }
    }
}

pub type ParseResult<T> = Result<T, ParserError>;

#[allow(dead_code)]
impl Parser {
    pub fn new() -> Parser {
        Parser{
            code: "".to_string(),
            line: 0,
            cur: 0,
            col: 0
        }
    }

    pub fn load(&mut self, code: String) -> ExprAst {
        self.code = code;
        self.line = 1;
        self.cur = 0;
        self.col = 0;
        self.skip_space();

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
            if cur == '-' {
                sign = -1;
            } else {
                self.unread();
            }
            let mut num = 0i;
            loop {
                cur = self.readc();
                if !UnicodeChar::is_numeric(cur) {
                    break;
                }
                num = (num * 10i) + (cur as int - 0i);
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
            return ExprAst::Str(StrNode::new(buf));
        }
        ExprAst::Char(CharNode::new('a'))
    }

    //============= private methods =================
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
        if self.peekc() == '\n' {
            assert!(self.line > 1);
            self.line -= 1;
        }
        self.cur -= 1;
    }

    fn read_char(&mut self) -> ExprAst {
        ExprAst::Char(CharNode::new('a'))
    }

}
