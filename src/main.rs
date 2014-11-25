use std::os;
use std::io;

#[allow(dead_code)]
mod parser;
mod ast;

fn help() {
    println!("rust-scm: prog");
}

fn main() {
    let s = "demo".to_string();
    println!("now: {} {}", s.char_at(0), s.len());

    if 0 < s.len() {
        println!("yes");
    }

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
}
