use crate::parser::tokens::Token;

#[derive(Debug, Clone)]
pub enum Stmt {
    Init {
        name: String,
        params: Vec<String>,
        body: Vec<Stmt>,
    },
    Log {
        value: Expr,
    },
    ExprStmt(Expr),
}

#[derive(Debug, Clone)]
pub enum Expr {
    Literal(String),
    Variable(String),
    Call {
        callee: String,
        arguments: Vec<Expr>,
    },
    Template(String),
}
