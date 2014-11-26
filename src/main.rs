use std::os;
use std::io;

#[allow(dead_code)]
use ast::Ast;

mod parser;
mod ast;

fn help() {
    println!("rust-scm: prog");
}


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
    let parser = parser::new();

    println!("parser: {}", parser);
    println!("code: {}", code);

    let int_node = ast::IntNode::new(3);
    int_node.print();
}
