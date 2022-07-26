use crate::ast::Value;
use crate::Expr;

pub fn b<T>(i: T) -> Box<T> {
    Box::new(i)
}

pub struct VM();

#[derive(Debug)]
pub enum ResultTypes {
    Int64(i64),
    Int128(i128),
    String(Box<String>),
    Regex(Box<String>),
    List(Vec<Box<ResultTypes>>),
    None,
}

impl VM {
    pub fn new() -> VM {
        VM()
    }
    pub fn eval(&self, asts: Vec<Box<Expr>>) -> Box<ResultTypes> {
        let mut last = b(ResultTypes::None);
        for ast in asts {
            last = match *ast {
                Expr::ExprWithCodePos { exp, start, end } => self.eval(vec![exp]),
                Expr::List(list) => b(ResultTypes::List(
                    list.into_iter().map(|i| self.eval(vec![i])).collect()
                )),
                Expr::Value(value) => match value {
                    Value::String(string) => b(ResultTypes::String(string)),
                    Value::Int64(int64) => b(ResultTypes::Int64(int64)),
                    Value::Int128(int128) => b(ResultTypes::Int128(int128)),
                    Value::Regex(regex) => b(ResultTypes::Regex(regex)),
                }
                _ => panic!("not impl")
            }
        }
        last
    }
}
