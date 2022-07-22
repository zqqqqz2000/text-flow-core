#[macro_use] extern crate lalrpop_util;

use crate::ast::Expr;
lalrpop_mod!(pub text_flow);
mod ast;
mod utils;

#[test]
fn text_flow() {
}

fn main() {
    let parser = text_flow::ExprsParser::new();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let res: Result<Vec<Box<Expr>>, _> = parser.parse(input.as_str());
    println!("{:#?}", res.map_err(|e|e))
}
