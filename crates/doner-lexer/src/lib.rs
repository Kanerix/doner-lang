pub mod error;
pub mod token;

pub use crate::{
    error::{Error, Result},
    token::TokenKind,
};

pub fn lex(input: &str) -> Result<Vec<TokenKind>> {
    let mut tokens: Vec<TokenKind> = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(char) = chars.next() {
        if char.is_whitespace() {
            continue;
        }

        match char {
            '+' => tokens.push(TokenKind::Plus),
            '-' => tokens.push(TokenKind::Minus),
            '(' => tokens.push(TokenKind::LParen),
            ')' => tokens.push(TokenKind::RParen),
            ';' => tokens.push(TokenKind::SemiColon),
            c if c.is_ascii_digit() => {
                let mut int_str = String::from(c);
                while let Some(&next_char) = chars.peek() {
                    if next_char.is_ascii_digit() {
                        chars.next();
                        int_str.push(next_char);
                    } else {
                        break;
                    }
                }

                let int_val = int_str
                    .parse::<i64>()
                    .map_err(|_| Error::InvalidIntLiteral(int_str.clone()))?;
                tokens.push(TokenKind::Int(int_val));
            }
            c if c.is_ascii_alphabetic() => {
                let mut ident = String::from(c);
                while let Some(&next_char) = chars.peek() {
                    if next_char.is_ascii_alphabetic() {
                        chars.next();
                        ident.push(next_char);
                    } else {
                        break;
                    }
                }

                let token = match ident.as_ref() {
                    "serve" => TokenKind::Print,
                    _ => return Err(Error::UnknownTokenSequence(ident)),
                };

                tokens.push(token);
            }
            c => return Err(Error::UnknownToken(c)),
        };
    }

    tokens.push(TokenKind::EOF);

    Ok(tokens)
}
