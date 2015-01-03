#![feature(macro_rules)]
#![feature(globs)]

#[allow(unused_imports)]

use std::os;
use std::io;

use ast::Ast;
use eval::Evaler;

mod ast;
mod env;
mod eval;
mod parser;

fn help() {
    println!("rust-scm: prog");
}

#[allow(dead_code)]
fn main() {

    let mut evaler = Evaler::new();
    let res = evaler.eval("(cdr '(1 2))".to_string());
    res.print();
    assert!(res.is_pair());

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
