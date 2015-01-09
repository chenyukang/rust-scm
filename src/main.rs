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
    let res = evaler.eval("((lambda (x y) (if (= y 0) 1 (+ y (x x (- y 1)))))
                            (lambda (x y) (if (= y 0) 1 (+ y (x x (- y 1))))) 30)".to_string());
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
    println!("parser: {}", parser);
    println!("code: {}", code);

}
