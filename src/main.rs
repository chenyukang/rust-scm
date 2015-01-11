#[allow(unused_imports)]
#[allow(unused_variables)]
#[allow(unstable)]

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
    let args = os::args();

    let mut evaler = Evaler::new();
    let res = evaler.eval("(define (caaaar x) (car (car (car (car x)))))".to_string()).unwrap();
    res.print();

    if args.len() < 2 {
        help();
        os::set_exit_status(1);
        return;
    }

    let program = args[0].as_slice();
    let code = match io::File::open(&Path::new(args[1].as_slice())) {
        Ok(mut file) => file.read_to_string().unwrap(),
        Err(_) => {
            os::set_exit_status(1);
            return
        }
    };
    let parser = parser::Parser::new();
    println!("code:\n{}", code);
    let mut evaler = Evaler::new();
    let res = evaler.eval(code).unwrap();
    res.print();
}
