use crate::token::Token;

pub fn tokenize(source: &str) -> Result<Vec<Token>, String> {
    split_to_token_text(source)?
        .into_iter()
        .map(|text| match text.as_str() {
            "(" => Ok(Token::LeftParen),
            ")" => Ok(Token::RightParen),
            "," => Ok(Token::Comma),
            "=" => Ok(Token::Assignment),
            "let" => Ok(Token::Let),
            "in" => Ok(Token::In),
            "if" => Ok(Token::If),
            "then" => Ok(Token::Then),
            "else" => Ok(Token::Else),
            "zero?" => Ok(Token::ZeroPredicate),
            "proc" => Ok(Token::Procedure),
            "letrec" => Ok(Token::RecursiveLet),
            "-" => Ok(Token::Minus),
            "+" => Ok(Token::Plus),
            "*" => Ok(Token::Times),
            "/" => Ok(Token::Div),
            _ if text.starts_with(char::is_numeric) => text
                .parse::<i32>()
                .map(Token::Number)
                .map_err(|_| format!("Invalid number literal: {text}")),
            _ => Ok(Token::Identifier(text)),
        })
        .collect()
}

fn split_to_token_text(source: &str) -> Result<Vec<String>, String> {
    let mut raw_tokens = Vec::new();
    let mut current = String::new();

    let flush = |buffer: &mut String, output: &mut Vec<String>| {
        if !buffer.is_empty() {
            output.push(std::mem::take(buffer));
        }
    };

    for ch in source.chars() {
        match ch {
            c if c.is_whitespace() => flush(&mut current, &mut raw_tokens),
            '(' | ')' | ',' | '=' => {
                flush(&mut current, &mut raw_tokens);
                raw_tokens.push(ch.to_string());
            }
            c if c.is_control() => {
                return Err(format!("Unsupported control character: {c:?}"));
            }
            _ => current.push(ch),
        }
    }

    flush(&mut current, &mut raw_tokens);
    Ok(raw_tokens)
}
