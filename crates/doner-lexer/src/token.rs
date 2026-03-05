pub type TokenStream = Vec<TokenKind>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    // Literals
    Int(i64),

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
