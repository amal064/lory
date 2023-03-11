#[derive(Debug, Clone)]
pub enum TokenKind {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals
    Lit(Literal),

    Identifier { ident: String },

    // Keywords
    And,
    Class,
    Else,
    Fun,
    For,
    If,
    Or,
    Print,
    Return,
    Super,
    This,
    Var,
    While,

    Eof,
}

#[derive(Debug, Clone)]
pub enum Literal {
    True,
    False,
    Nil,
    String { string: String },
    Number(f64),
}
