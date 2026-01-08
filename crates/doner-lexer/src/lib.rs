pub mod error;
pub mod token;

pub use crate::{
    error::{Error, Result},
    token::TokenKind,
};

pub fn lex(input: &str) -> Result<Vec<TokenKind>> {
    let mut tokens: Vec<TokenKind> = Vec::new();
    let mut chars = input.chars();

    while let Some(char) = chars.next() {
        if char.is_whitespace() {
            continue;
        }

        match char {
            '+' => tokens.push(TokenKind::Plus),
            '-' => tokens.push(TokenKind::Minus),
            ';' => tokens.push(TokenKind::SemiColon),
            '(' => tokens.push(TokenKind::LParen),
            ')' => tokens.push(TokenKind::RParen),
            c if c.is_alphanumeric() => {
                let mut int_str = String::from(c);
                while let Some(next_char) = chars.next() {
                    match next_char {
                        nc if nc.is_whitespace() => break,
                        nc => int_str.push(nc),
                    };
                }
                let int_val = int_str.parse::<i64>().unwrap();
                tokens.push(TokenKind::Int(int_val))
            }
            c => return Err(Error::UnknownToken(c)),
        };
    }

    tokens.push(TokenKind::EOF);

    Ok(tokens)
}
