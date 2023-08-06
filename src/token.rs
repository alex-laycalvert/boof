#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Var,
    Func,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    If,
    Else,
    ElseIf,
    SemiColon,
    Plus,
    Minus,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Bang,
    BangEqual,
    Slash,
    Star,
    Dot,
    Comma,
    Identifier,
    String(String),
    Number(f32),
    And,
    Nil,
    While,
    For,
    Or,
    Return,
    Print,
    False,
    True,
    EOF,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    line: usize,
}

impl Token {
    pub fn from(token_type: TokenType, lexeme: String, line: usize) -> Self {
        Token {
            token_type,
            line,
            lexeme,
        }
    }

    pub fn to_string(&self) -> String {
        format!("{}", self.lexeme)
    }
}
