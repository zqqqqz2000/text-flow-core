#[derive(Debug, Eq, PartialEq)]
pub enum Value {
    String(Box<String>),
    Int64(i64),
    Int128(i128),
    Regex(Box<String>),
}

#[derive(Debug, Eq, PartialEq)]
pub enum Op {
    Eq,
    Map,
    AsyncMap,
    Assign,
    Collect,
    Add,
    Sub,
    Mul,
    Div,
    Gt,
    Ge,
    Lt,
    Le,
    Ne,
    If,
    And,
    Or,
    Not,
    BAnd,
    BOr,
    BXor,
    BNot,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Control {
    Ignore,
    Break,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Expr {
    FuncCall { func: Box<Expr>, parameters: Vec<Box<Expr>> },
    Variable(Box<String>),
    FuncDef { parameters: Vec<Box<String>>, body: Box<Expr> },
    Value(Value),
    // [a, b, c]
    List(Vec<Box<Expr>>),
    // abc.xyz
    Get { from: Box<Expr>, key: Box<String> },
    ExprWithCodePos { exp: Box<Expr>, start: usize, end: usize },
    Block(Vec<Box<Expr>>),
    // !x
    Op1 { op: Op, x: Box<Expr> },
    // x || y
    Op2 { op: Op, x: Box<Expr>, y: Box<Expr> },
    // x ? y : z
    Op3 { op: Op, x: Box<Expr>, y: Box<Expr>, z: Box<Expr> },
    Control(Control),
}
