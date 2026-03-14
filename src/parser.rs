use crate::ast::{ArithOp, Expr};
use crate::token::Token;

pub fn parse(tokens: &[Token]) -> Result<Expr, String> {
    let mut parser = Parser::new(tokens);
    let expr = parser
        .parse_expr()?
        .ok_or_else(|| "No input for parsing.".to_string())?;

    if let Some(token) = parser.peek() {
        return Err(format!("Unexpected trailing token: {token:?}"));
    }

    Ok(expr)
}

struct Parser<'a> {
    tokens: &'a [Token],
    index: usize,
}

impl<'a> Parser<'a> {
    fn new(tokens: &'a [Token]) -> Self {
        Self { tokens, index: 0 }
    }

    fn peek(&self) -> Option<&'a Token> {
        self.tokens.get(self.index)
    }

    fn next(&mut self) -> Option<&'a Token> {
        let token = self.tokens.get(self.index);
        if token.is_some() {
            self.index += 1;
        }
        token
    }

    fn parse_expr(&mut self) -> Result<Option<Expr>, String> {
        let next = match self.next().cloned() {
            Some(token) => token,
            None => return Ok(None),
        };

        let expr = match next {
            Token::Number(value) => Expr::Number(value),
            Token::Identifier(name) => Expr::Variable(name),
            Token::Let => {
                let name = self.parse_identifier()?;
                self.consume_assignment()?;
                let value = self
                    .parse_expr()?
                    .ok_or_else(|| "Missing let binding expression.".to_string())?;
                self.consume_in()?;
                let body = self
                    .parse_expr()?
                    .ok_or_else(|| "Missing let body expression.".to_string())?;

                Expr::Let {
                    name,
                    value: Box::new(value),
                    body: Box::new(body),
                }
            }
            Token::If => {
                let test = self
                    .parse_expr()?
                    .ok_or_else(|| "Missing if test expression.".to_string())?;
                self.consume_then()?;
                let then_branch = self
                    .parse_expr()?
                    .ok_or_else(|| "Missing then expression.".to_string())?;
                self.consume_else()?;
                let else_branch = self
                    .parse_expr()?
                    .ok_or_else(|| "Missing else expression.".to_string())?;

                Expr::If {
                    test: Box::new(test),
                    then_branch: Box::new(then_branch),
                    else_branch: Box::new(else_branch),
                }
            }
            Token::Plus | Token::Minus | Token::Times | Token::Div => {
                let op = match next {
                    Token::Plus => ArithOp::Add,
                    Token::Minus => ArithOp::Sub,
                    Token::Times => ArithOp::Mul,
                    Token::Div => ArithOp::Div,
                    _ => unreachable!(),
                };

                self.consume_left_paren()?;
                let left = self
                    .parse_expr()?
                    .ok_or_else(|| "Missing arithmetic left operand.".to_string())?;
                self.consume_comma()?;
                let right = self
                    .parse_expr()?
                    .ok_or_else(|| "Missing arithmetic right operand.".to_string())?;
                self.consume_right_paren()?;

                Expr::Arithmetic {
                    op,
                    left: Box::new(left),
                    right: Box::new(right),
                }
            }
            Token::ZeroPredicate => {
                self.consume_left_paren()?;
                let expr = self
                    .parse_expr()?
                    .ok_or_else(|| "Missing zero? argument.".to_string())?;
                self.consume_right_paren()?;
                Expr::Zero(Box::new(expr))
            }
            Token::LeftParen => {
                let operator = self
                    .parse_expr()?
                    .ok_or_else(|| "Missing call operator.".to_string())?;
                let operand = self
                    .parse_expr()?
                    .ok_or_else(|| "Missing call operand.".to_string())?;
                self.consume_right_paren()?;

                Expr::Call {
                    operator: Box::new(operator),
                    operand: Box::new(operand),
                }
            }
            Token::Procedure => {
                self.consume_left_paren()?;
                let param = self.parse_identifier()?;
                self.consume_right_paren()?;
                let body = self
                    .parse_expr()?
                    .ok_or_else(|| "Missing procedure body.".to_string())?;

                Expr::Proc {
                    param,
                    body: Box::new(body),
                }
            }
            Token::RecursiveLet => {
                let name = self.parse_identifier()?;
                self.consume_left_paren()?;
                let param = self.parse_identifier()?;
                self.consume_right_paren()?;
                self.consume_assignment()?;
                let func_body = self
                    .parse_expr()?
                    .ok_or_else(|| "Missing letrec function body.".to_string())?;
                self.consume_in()?;
                let body = self
                    .parse_expr()?
                    .ok_or_else(|| "Missing letrec body.".to_string())?;

                Expr::LetRec {
                    name,
                    param,
                    func_body: Box::new(func_body),
                    body: Box::new(body),
                }
            }
            unexpected => {
                return Err(format!(
                    "Unexpected token while parsing expression: {unexpected:?}"
                ));
            }
        };

        Ok(Some(expr))
    }

    fn parse_identifier(&mut self) -> Result<String, String> {
        match self.next() {
            Some(Token::Identifier(name)) => Ok(name.clone()),
            other => Err(format!("Expected identifier, found {other:?}")),
        }
    }

    fn consume_left_paren(&mut self) -> Result<(), String> {
        self.consume_fixed(Token::LeftParen, "(")
    }

    fn consume_right_paren(&mut self) -> Result<(), String> {
        self.consume_fixed(Token::RightParen, ")")
    }

    fn consume_comma(&mut self) -> Result<(), String> {
        self.consume_fixed(Token::Comma, ",")
    }

    fn consume_assignment(&mut self) -> Result<(), String> {
        self.consume_fixed(Token::Assignment, "=")
    }

    fn consume_in(&mut self) -> Result<(), String> {
        self.consume_fixed(Token::In, "in")
    }

    fn consume_then(&mut self) -> Result<(), String> {
        self.consume_fixed(Token::Then, "then")
    }

    fn consume_else(&mut self) -> Result<(), String> {
        self.consume_fixed(Token::Else, "else")
    }

    fn consume_fixed(&mut self, expected: Token, label: &str) -> Result<(), String> {
        match self.next() {
            Some(token) if *token == expected => Ok(()),
            other => Err(format!("Expected {label}, found {other:?}")),
        }
    }
}
