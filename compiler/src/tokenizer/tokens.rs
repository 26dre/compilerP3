pub enum Token {
    Number(isize),
    Identifier(String),
    Plus,
    Minus,
    Times,
    Divide,
    Assignment,
    Equal,        // ==
    NotEqual,     // !=
    Less,         // <
    LessEqual,    // <=
    Greater,      // >
    GreaterEqual, // >=
    Semicolon,
    Comma,
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    If,
    Fi,
    Then,
    Else,
    While,
    Do,
    Od,
    Function,
    FunctionCall,
    Return,
    Variable,
    Let,
    Main,
    Void,
    EOF,
}
