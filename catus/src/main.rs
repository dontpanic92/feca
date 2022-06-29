use crate::interpreter::Interpreter;

mod ast;
mod builtins;
mod interpreter;
mod symtbl;

#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub parser);

fn main() {
    let program = parser::ScriptParser::new().parse("console.log('1');");
    println!("{:?}", program);

    let interpreter = Interpreter::new();
    interpreter.eval(&program.unwrap());
}
