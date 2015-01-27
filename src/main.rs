#![allow(unstable)]
#[allow(unused_imports)]
extern crate test;

use std::os;
use std::io;
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
    if args.len() < 2 {
        let mut evaler = Evaler::new(io::stdin(), true);
        let res = evaler.eval().unwrap();
        res.print();
        println!("");
    } else if args.len() == 2 {
        let program = args[0].as_slice();
        let code = match io::File::open(&Path::new(args[1].as_slice())) {
            Ok(mut file) => file.read_to_string().unwrap(),
            Err(_) => {
                os::set_exit_status(1);
                return
            }
        };
        println!("code:\n{}", program);
        println!("code:\n{}", code);
        let mut evaler = Evaler::new(io::stdin(), false);
        let res = evaler.eval_from(code).unwrap();
        res.print();
        println!("");
    } else {
        help();
    }

}
