//#![feature(test)]
//extern crate test;

use std::io::Read;
use std::path::Path;
use std::fs::File;
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
    let mut args = std::env::args();
    if args.len() < 2 {
        let mut evaler = Evaler::new(std::io::stdin(), true);
        let res = evaler.eval().unwrap();
        res.print();
        println!("");
    } else if args.len() == 2 {
        let program = args.nth(0).unwrap();
        let mut code = String::new();
        match args.nth(1) {
            Some(arg) => {
                let path = Path::new(arg.trim());
                let mut f = File::open(&path).unwrap();
                f.read_to_string(&mut code).unwrap();
            },
            None => {
                //std::env::set_exit_status(1);
                return
            }
        }
        println!("code:\n{}", program);
        println!("code:\n{}", code);
        let mut evaler = Evaler::new(std::io::stdin(), false);
        let res = evaler.eval_from(code).unwrap();
        res.print();
        println!("");
    } else {
        help();
    }

}
