mod ast;
mod interpreter;
mod lexer;
mod parser;
mod token;

use std::fs;
use std::path::Path;

pub use ast::{ArithOp, Expr};
pub use interpreter::{Value, eval};
pub use lexer::tokenize;
pub use parser::parse;

pub fn parse_and_eval(source: &str) -> Result<Value, String> {
    let tokens = tokenize(source)?;
    let expr = parse(&tokens)?;
    eval(&expr)
}

pub fn run_file(path: impl AsRef<Path>) -> Result<Value, String> {
    let path = path.as_ref();
    let source = fs::read_to_string(path)
        .map_err(|error| format!("Failed to read {}: {error}", path.display()))?;
    parse_and_eval(&source)
}
