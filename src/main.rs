use std::os;
use std::io;

#[allow(dead_code)]
//use ast::Ast;

mod parser;
mod ast;
mod test;

fn help() {
    println!("rust-scm: prog");
}

#[allow(dead_code)]
fn main() {
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
