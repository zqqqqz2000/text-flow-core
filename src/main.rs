#[macro_use]
extern crate lalrpop_util;
extern crate core;

use std::sync::{Arc, RwLock};
use crate::ast::Expr;
use tf_vm::vm::VM;
use crate::tf_vm::env::Env;
lalrpop_mod!(pub text_flow);
mod ast;
mod utils;
mod tf_vm;
mod test;

fn main() {
    let parser = text_flow::ExprsParser::new();
    let vm = VM::new();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let ast: Result<Vec<Box<Expr>>, _> = parser.parse(input.as_str());
    let global = Arc::new(RwLock::new(Env::new(None, None)));
    println!("{:#?}", vm.eval(global, ast.unwrap()))
}
