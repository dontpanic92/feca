mod ast;
mod interpreter;
mod symtbl;

#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub parser);

fn main() {
    println!(
        "{:?}",
        parser::ScriptParser::new().parse("console.log('1');")
    );
}
