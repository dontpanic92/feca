use std::fs::read_to_string;

use crate::interpreter::Interpreter;

mod ast;
mod builtins;
mod interpreter;
mod parser;
mod symtbl;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("{} file.js", args[0]);
        return;
    }

    let content = read_to_string(&args[1]).unwrap();
    let program = parser::parse(content.as_str());
    // println!("{:?}", program);

    match program {
        Ok(("", script)) => {
            let mut interpreter = Interpreter::new();
            interpreter.eval(&script);
        }
        Ok((rest, _)) => {
            println!("Failed to parse: input left {}", rest);
        }
        Err(e) => println!("Failed to parse: {:?}", e),
    };
}
