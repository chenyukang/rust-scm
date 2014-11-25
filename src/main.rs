use std::io;

fn main() {
    let input = io::stdin().read_line().ok().expect("Failed to read line");

    println!("input: {}", input)
}
