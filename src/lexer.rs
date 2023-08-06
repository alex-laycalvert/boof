use std::collections::HashMap;

use crate::token::{Token, TokenType};
use crate::Error;

pub struct Lexer {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    keywords: HashMap<&'static str, TokenType>,
}

impl Lexer {
    pub fn from(source: String) -> Self {
        let keywords = HashMap::from([
            ("boof", TokenType::Var),
            ("boofer", TokenType::Func),
            ("and", TokenType::And),
            ("for", TokenType::For),
            ("while", TokenType::While),
            ("nil", TokenType::Nil),
            ("or", TokenType::Or),
            ("if", TokenType::If),
            ("else", TokenType::Else),
            ("elseif", TokenType::ElseIf),
            ("return", TokenType::Return),
            ("true", TokenType::True),
            ("false", TokenType::False),
            ("print", TokenType::Print),
        ]);
        Lexer {
            source,
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
            keywords,
        }
    }
    pub fn scan(&mut self) -> Result<Vec<Token>, Error> {
        while !self.is_finished() {
            self.start = self.current;
            self.scan_token()?;
        }
        self.tokens
            .push(Token::from(TokenType::EOF, "\0".to_string(), self.line));
        Ok(self.tokens.clone())
    }

    fn scan_token(&mut self) -> Result<(), Error> {
        let c = self.next();
        match c {
            '(' => Ok(self.add_token(TokenType::LeftParen)),
            ')' => Ok(self.add_token(TokenType::RightParen)),
            '{' => Ok(self.add_token(TokenType::LeftBrace)),
            '}' => Ok(self.add_token(TokenType::RightBrace)),
            ';' => Ok(self.add_token(TokenType::SemiColon)),
            '+' => Ok(self.add_token(TokenType::Plus)),
            '-' => Ok(self.add_token(TokenType::Minus)),
            '*' => Ok(self.add_token(TokenType::Star)),
            '.' => Ok(self.add_token(TokenType::Dot)),
            ',' => Ok(self.add_token(TokenType::Comma)),
            '!' => {
                if self.match_char('=') {
                    Ok(self.add_token(TokenType::BangEqual))
                } else {
                    Ok(self.add_token(TokenType::Bang))
                }
            }
            '=' => {
                if self.match_char('=') {
                    Ok(self.add_token(TokenType::EqualEqual))
                } else {
                    Ok(self.add_token(TokenType::Equal))
                }
            }
            '>' => {
                if self.match_char('=') {
                    Ok(self.add_token(TokenType::GreaterEqual))
                } else {
                    Ok(self.add_token(TokenType::Greater))
                }
            }
            '<' => {
                if self.match_char('=') {
                    Ok(self.add_token(TokenType::LessEqual))
                } else {
                    Ok(self.add_token(TokenType::Less))
                }
            }
            '/' => {
                if self.match_char('/') {
                    while self.peek() != '\n' && !self.is_finished() {
                        self.next();
                    }
                    Ok(())
                } else {
                    Ok(self.add_token(TokenType::Slash))
                }
            }
            ' ' | '\r' | '\t' => Ok(()),
            '\n' => Ok(self.line += 1),
            '"' => self.process_string(),
            c => {
                if c.is_numeric() {
                    self.process_number()
                } else if c.is_ascii_alphabetic() || c == '_' {
                    self.process_identifier()
                } else {
                    Err(Error::lexing(
                        format!("Unexpected Character \"{c}\""),
                        self.line,
                    ))
                }
            }
        }
    }

    fn process_identifier(&mut self) -> Result<(), Error> {
        while self.peek().is_ascii_alphanumeric() || self.peek() == '_' {
            self.next();
        }
        let text = &self.source[self.start..self.current];
        let token_type = self.keywords.get(&text).unwrap_or(&TokenType::Identifier);
        Ok(self.add_token(token_type.clone()))
    }

    fn process_number(&mut self) -> Result<(), Error> {
        while self.peek().is_numeric() {
            self.next();
        }
        if self.peek() == '.' && self.peek_next().is_numeric() {
            self.next();
            while self.peek().is_numeric() {
                self.next();
            }
        }
        match self.source[self.start..self.current].parse() {
            Ok(val) => Ok(self.add_token(TokenType::Number(val))),
            Err(e) => Err(Error::lexing(e.to_string(), self.line)),
        }
    }

    fn process_string(&mut self) -> Result<(), Error> {
        while self.peek() != '"' && !self.is_finished() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.next();
        }
        if self.is_finished() {
            return Err(Error::lexing("Unterminated String".to_string(), self.line));
        }

        self.next();

        let value = &self.source[(self.start + 1)..(self.current - 1)];
        Ok(self.add_token(TokenType::String(value.to_string())))
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_finished() {
            return false;
        }
        if self.source.chars().nth(self.current) == Some(expected) {
            self.current += 1;
            return true;
        }
        false
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.tokens.push(Token::from(
            token_type,
            self.source[self.start..self.current].to_string(),
            self.line,
        ));
    }

    fn is_finished(&self) -> bool {
        self.current >= self.source.len()
    }

    fn next(&mut self) -> char {
        let ch = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        ch
    }

    fn peek(&self) -> char {
        if self.is_finished() {
            return '\0';
        }
        self.source.chars().nth(self.current).unwrap()
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        self.source.chars().nth(self.current + 1).unwrap()
    }
}
