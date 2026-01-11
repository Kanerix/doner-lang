pub mod error;
pub mod token;

use std::{iter::Peekable, str::Chars};

pub use crate::{error::*, token::*};

pub type CharStream<'a> = Peekable<Chars<'a>>;

pub fn lex(input: &str) -> Result<TokenStream> {
    let mut tokens: Vec<TokenKind> = Vec::new();
    let mut chars: CharStream = input.chars().peekable();

    while let Some(char) = chars.peek() {
        if char.is_whitespace() {
            let _ = chars.next();
            continue;
        }

        let token = match char {
            '+' => {
                let _ = chars.next();
                TokenKind::Plus
            }
            '-' => {
                let _ = chars.next();
                TokenKind::Minus
            }
            '(' => {
                let _ = chars.next();
                TokenKind::LParen
            }
            ')' => {
                let _ = chars.next();
                TokenKind::RParen
            }
            ';' => {
                let _ = chars.next();
                TokenKind::SemiColon
            }
            c if c.is_ascii_digit() => lex_int(&mut chars)?,
            c if c.is_ascii_alphabetic() => lex_ident(&mut chars)?,
            _ => return Err(Error::UnknownToken(chars.next().unwrap_or('?'))),
        };

        tokens.push(token);
    }

    tokens.push(TokenKind::EOF);

    Ok(tokens)
}

pub fn lex_int(chars: &mut CharStream) -> Result<TokenKind> {
    let c = chars.next().ok_or(Error::UnexpectedEoF)?;
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

    Ok(TokenKind::Int(int_val))
}

pub fn lex_ident(chars: &mut CharStream) -> Result<TokenKind> {
    let c = chars.next().ok_or(Error::UnexpectedEoF)?;
    let mut ident = String::from(c);
    while let Some(&next_char) = chars.peek() {
        if next_char.is_ascii_alphabetic() {
            chars.next();
            ident.push(next_char);
        } else {
            break;
        }
    }

    match ident.as_ref() {
        "serve" => Ok(TokenKind::Print),
        _ => return Err(Error::UnknownTokenSequence(ident)),
    }
}
