#[allow(dead_code)]

#[deriving(Clone, PartialEq)]
pub enum Expr {
    Int(i32),
    Str(String),
    Bool(bool),
    Pair(Box<Expr>, Box<Expr>)
}
