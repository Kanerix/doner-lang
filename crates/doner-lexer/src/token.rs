#[derive(Debug)]
pub enum TokenKind {
    Int(i64),

    Plus,
    Minus,

    LParen,
    RParen,

    SemiColon,

    Print,

    EOF,
}
