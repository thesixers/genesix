use crate::parser::ast::{Stmt, Expr};
use crate::runtime::values::Value;

use std::collections::HashMap;

pub struct Evaluator {
    globals: HashMap<String, Value>,
}

type Env = HashMap<String, Value>;

impl Evaluator {
    pub fn new() -> Self {
        Self {
            globals: HashMap::new(),
        }
    }

    pub fn evaluate(&mut self, program: Vec<Stmt>) {
        let mut env = self.globals.clone();

        for stmt in program {
            self.execute(stmt, &mut env);
        }
    }

    fn execute(&mut self, stmt: Stmt, env: &mut Env) {
        match stmt {
            Stmt::Init { name, params, body } => {
                env.insert(name.clone(), Value::Function { params, body });
            }

            Stmt::Log { value } => {
                let result = self.eval_expr(value, env);
                println!("{}", self.stringify(result));
            }

            Stmt::ExprStmt(expr) => {
                self.eval_expr(expr, env);
            }
        }
    }

    fn eval_expr(&mut self, expr: Expr, env: &mut Env) -> Value {
        match expr {
            Expr::Literal(val) => Value::String(val),
            Expr::Template(template) => {
                let interpolated = self.interpolate_template(template, env);
                Value::String(interpolated)
            }

            Expr::Variable(name) => {
                env.get(&name)
                    .cloned()
                    .unwrap_or(Value::String(format!("<undefined {}>", name)))
            }

            Expr::Call { callee, arguments } => {
                let function = env.get(&callee).cloned();
                match function {
                    Some(Value::Function { params, body }) => {
                        let mut new_env = self.globals.clone();

                        for (i, param_name) in params.iter().enumerate() {
                            if let Some(arg) = arguments.get(i) {
                                let val = self.eval_expr(arg.clone(), env);
                                new_env.insert(param_name.clone(), val);
                            }
                        }

                        for stmt in body {
                            self.execute(stmt.clone(), &mut new_env);
                        }

                        Value::Null
                    }
                    Some(_) => panic!("{} is not a function", callee),
                    None => panic!("Function not found: {}", callee),
                }
            }
        }
    }

    fn interpolate_template(&self, template: String, env: &Env) -> String {
        let mut result = String::new();
        let mut chars = template.chars().peekable();

        while let Some(c) = chars.next() {
            if c == '$' && chars.peek() == Some(&'{') {
                chars.next(); // consume {
                let mut name = String::new();

                while let Some(&next) = chars.peek() {
                    if next == '}' {
                        chars.next();
                        break;
                    }
                    name.push(next);
                    chars.next();
                }

                let value = env
                    .get(&name)
                    .map(|v| self.stringify(v.clone()))
                    .unwrap_or("".to_string());
                result.push_str(&value);
            } else {
                result.push(c);
            }
        }

        result
    }

    fn stringify(&self, value: Value) -> String {
        match value {
            Value::String(s) => s,
            Value::Null => "null".to_string(),
            Value::Function { .. } => "<function>".to_string(),
        }
    }
}
