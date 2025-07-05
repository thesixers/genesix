use crate::parser::ast::Stmt;

#[derive(Clone, Debug)]
pub enum Value {
    String(String),
    Function {
        params: Vec<String>,
        body: Vec<Stmt>,
    },
    Null,
}
