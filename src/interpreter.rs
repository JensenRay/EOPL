use std::fmt;
use std::rc::Rc;

use crate::ast::{ArithOp, Expr};

pub fn eval(expr: &Expr) -> Result<Value, String> {
    let env = Rc::new(Env::Empty);
    eval_in_env(expr, env)
}

#[derive(Debug, Clone)]
pub enum Value {
    Number(i32),
    Bool(bool),
    Procedure(Rc<Procedure>),
}

#[derive(Debug, Clone)]
pub struct Procedure {
    param: String,
    body: Expr,
    env: Rc<Env>,
}

#[derive(Debug, Clone)]
enum Env {
    Empty,
    Extend {
        name: String,
        value: Value,
        saved: Rc<Env>,
    },
    ExtendRec {
        name: String,
        param: String,
        body: Expr,
        saved: Rc<Env>,
    },
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Number(left), Self::Number(right)) => left == right,
            (Self::Bool(left), Self::Bool(right)) => left == right,
            _ => false,
        }
    }
}

impl Eq for Value {}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Number(value) => write!(f, "{value}"),
            Self::Bool(value) => write!(f, "{value}"),
            Self::Procedure(_) => write!(f, "<procedure>"),
        }
    }
}

fn eval_in_env(expr: &Expr, env: Rc<Env>) -> Result<Value, String> {
    match expr {
        Expr::Number(value) => Ok(Value::Number(*value)),
        Expr::Variable(name) => env.lookup(name),
        Expr::Let { name, value, body } => {
            let bound = eval_in_env(value, env.clone())?;
            let next_env = Rc::new(Env::Extend {
                name: name.clone(),
                value: bound,
                saved: env,
            });
            eval_in_env(body, next_env)
        }
        Expr::If {
            test,
            then_branch,
            else_branch,
        } => match eval_in_env(test, env.clone())? {
            Value::Bool(true) => eval_in_env(then_branch, env),
            Value::Bool(false) => eval_in_env(else_branch, env),
            other => Err(format!("if test must be boolean, found {other:?}")),
        },
        Expr::Arithmetic { op, left, right } => {
            let left = eval_in_env(left, env.clone())?;
            let right = eval_in_env(right, env)?;
            let (left, right) = match (left, right) {
                (Value::Number(left), Value::Number(right)) => (left, right),
                (left, right) => {
                    return Err(format!(
                        "Arithmetic operands must be numbers, found {left:?} and {right:?}"
                    ));
                }
            };

            let value = match op {
                ArithOp::Add => left + right,
                ArithOp::Sub => left - right,
                ArithOp::Mul => left * right,
                ArithOp::Div => {
                    if right == 0 {
                        return Err("Division by zero.".to_string());
                    }
                    left / right
                }
            };

            Ok(Value::Number(value))
        }
        Expr::Zero(expr) => match eval_in_env(expr, env)? {
            Value::Number(value) => Ok(Value::Bool(value == 0)),
            other => Err(format!("zero? expects a number, found {other:?}")),
        },
        Expr::Proc { param, body } => Ok(Value::Procedure(Rc::new(Procedure {
            param: param.clone(),
            body: body.as_ref().clone(),
            env,
        }))),
        Expr::Call { operator, operand } => {
            let procedure = eval_in_env(operator, env.clone())?;
            let argument = eval_in_env(operand, env)?;

            match procedure {
                Value::Procedure(proc) => {
                    let next_env = Rc::new(Env::Extend {
                        name: proc.param.clone(),
                        value: argument,
                        saved: proc.env.clone(),
                    });
                    eval_in_env(&proc.body, next_env)
                }
                other => Err(format!(
                    "Call operator must be a procedure, found {other:?}"
                )),
            }
        }
        Expr::LetRec {
            name,
            param,
            func_body,
            body,
        } => {
            let next_env = Rc::new(Env::ExtendRec {
                name: name.clone(),
                param: param.clone(),
                body: func_body.as_ref().clone(),
                saved: env,
            });
            eval_in_env(body, next_env)
        }
    }
}

impl Env {
    fn lookup(self: &Rc<Self>, name: &str) -> Result<Value, String> {
        match self.as_ref() {
            Env::Empty => Err(format!("Unbound variable: {name}")),
            Env::Extend {
                name: bound_name,
                value,
                saved,
            } => {
                if bound_name == name {
                    Ok(value.clone())
                } else {
                    saved.lookup(name)
                }
            }
            Env::ExtendRec {
                name: func_name,
                param,
                body,
                saved,
            } => {
                if func_name == name {
                    Ok(Value::Procedure(Rc::new(Procedure {
                        param: param.clone(),
                        body: body.clone(),
                        env: self.clone(),
                    })))
                } else {
                    saved.lookup(name)
                }
            }
        }
    }
}
