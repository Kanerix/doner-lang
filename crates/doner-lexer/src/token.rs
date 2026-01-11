pub type TokenStream = Vec<TokenKind>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
    // Literals
    Int(i64),

    // Operators
    Plus,
    Minus,

    // Parentheses
    LParen,
    RParen,

    // Keywords
    Print,

    SemiColon,
    EOF,
}
