use std::os;
use std::io;

#[allow(dead_code)]

use ast::Ast;
use parser::Parser;

use ast::ExprAst;
use ast::SymbolNode;

use eval::Evaler;
use std::collections::HashMap;

mod ast;
mod parser;
mod env;
mod eval;
mod test;


fn help() {
    println!("rust-scm: prog");
}

fn add(val: int) -> int {
    return val + 1;
}

#[allow(dead_code)]
fn main() {
    let mut env = HashMap::new();
    env.insert("Haha", add);
    println!("env: {}", env.capacity());

    let mut evaler = Evaler::new();
    let res = evaler.eval("11".to_string());
    res.print();

    let mut parser = Parser::new();
    let res = parser.load("11".to_string());
    //assert!(res.as_int() == 11);
    res.print();

    let args = os::args();

    if args.len() < 2 {
        help();
        os::set_exit_status(1);
        return;
    }

    let program = args[0].as_slice();
    println!("args: {} program: {}", args, program);
    let code = match io::File::open(&Path::new(args[1].as_slice())) {
        Ok(mut file) => file.read_to_string().unwrap(),
        Err(_) => {
            os::set_exit_status(1);
            return
        }
    };
    let parser = parser::Parser::new();
    let res = UnicodeChar::is_numeric('a');
    println!("res: {}", res);
    println!("parser: {}", parser);
    println!("code: {}", code);
}
