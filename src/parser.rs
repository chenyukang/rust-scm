
#[deriving(Clone, Show)]
pub struct Parser {
    code: String,
    cur: uint,
    col: uint,
    line: uint
}

pub fn new() -> Parser {
    Parser{
        code: "".to_string(),
        cur: 0,
        col: 0,
        line: 0
    }
}

impl Parser {
    fn is_delimiter(&self, ch: char) -> bool {
        if ch.is_whitespace() ||
            ch == '\"' || ch == '(' || ch == ')' ||  ch == ';' ||
            ch as u32 == 0 {
                true
            } else {
                false
            }
    }

    fn is_initial(&self, ch: char) -> bool {
        if ch.is_alphabetic() ||
            ch == '*' || ch == '/' ||
            ch == '+' || ch == '-' ||
            ch == '>' || ch == '<' ||
            ch == '=' || ch == '?' ||
            ch == '!' {
                true
            } else {
                false
            }
    }

    fn skip_white(&mut self) {
        while self.peekc().is_whitespace() {
            if self.peekc() == '\n' {
                self.line += 1;
            }
            self.cur += 1;
        }
    }

    fn peekc(&self) -> char {
        if self.cur < self.code.len() {
            self.code.char_at(self.cur)
        } else {
            0 as char
        }
    }


}
