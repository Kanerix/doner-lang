pub type TokenStream = Vec<TokenKind>;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenKind {
    // Literals
    Int(f64),

    // Operators
    Equals,
    Plus,
    Minus,
    Slash,
    Star,

    // Parentheses
    LParen,
    RParen,

    // Keywords
    Print,

    SemiColon,
    EOF,
}
