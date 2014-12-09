

#[deriving(PartialEq, Eq, Show)]
pub enum Token {
    LParen,
    RParen,
    Plus,
    Minus,
    Int(uint)
}
