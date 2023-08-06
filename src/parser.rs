use crate::{
    error::Error,
    expr::{Expr, LiteralValue},
    token::{Token, TokenType},
};

macro_rules! compare_token_types {
    ($t1:expr, $t2:expr) => {
        match ($t1, $t2) {
            (TokenType::Var, TokenType::Var) => true,
            (TokenType::Func, TokenType::Func) => true,
            (TokenType::LeftParen, TokenType::LeftParen) => true,
            (TokenType::RightParen, TokenType::RightParen) => true,
            (TokenType::LeftBrace, TokenType::LeftBrace) => true,
            (TokenType::RightBrace, TokenType::RightBrace) => true,
            (TokenType::If, TokenType::If) => true,
            (TokenType::Else, TokenType::Else) => true,
            (TokenType::ElseIf, TokenType::ElseIf) => true,
            (TokenType::SemiColon, TokenType::SemiColon) => true,
            (TokenType::Plus, TokenType::Plus) => true,
            (TokenType::Minus, TokenType::Minus) => true,
            (TokenType::Equal, TokenType::Equal) => true,
            (TokenType::EqualEqual, TokenType::EqualEqual) => true,
            (TokenType::Greater, TokenType::Greater) => true,
            (TokenType::GreaterEqual, TokenType::GreaterEqual) => true,
            (TokenType::Less, TokenType::Less) => true,
            (TokenType::LessEqual, TokenType::LessEqual) => true,
            (TokenType::Bang, TokenType::Bang) => true,
            (TokenType::BangEqual, TokenType::BangEqual) => true,
            (TokenType::Slash, TokenType::Slash) => true,
            (TokenType::Star, TokenType::Star) => true,
            (TokenType::Dot, TokenType::Dot) => true,
            (TokenType::Comma, TokenType::Comma) => true,
            (TokenType::Identifier, TokenType::Identifier) => true,
            (TokenType::String(_), TokenType::String(_)) => true,
            (TokenType::Number(_), TokenType::Number(_)) => true,
            (TokenType::And, TokenType::And) => true,
            (TokenType::Nil, TokenType::Nil) => true,
            (TokenType::While, TokenType::While) => true,
            (TokenType::For, TokenType::For) => true,
            (TokenType::Or, TokenType::Or) => true,
            (TokenType::Return, TokenType::Return) => true,
            (TokenType::Print, TokenType::Print) => true,
            (TokenType::False, TokenType::False) => true,
            (TokenType::True, TokenType::True) => true,
            (TokenType::EOF, TokenType::EOF) => true,
            _ => false,
        }
    };
}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn from(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Expr, Error> {
        self.expression()
    }

    fn expression(&mut self) -> Result<Expr, Error> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, Error> {
        let mut expr = self.comparison()?;
        while self.match_tokens(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let op = self.previous();
            let right = self.comparison()?;
            expr = Expr::Binary(Box::new(expr), op, Box::new(right));
        }
        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, Error> {
        let mut expr = self.term()?;
        while self.match_tokens(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let op = self.previous();
            let right = self.term()?;
            expr = Expr::Binary(Box::new(expr), op, Box::new(right));
        }
        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, Error> {
        let mut expr = self.factor()?;
        while self.match_tokens(&[TokenType::Minus, TokenType::Plus]) {
            let op = self.previous();
            let right = self.factor()?;
            expr = Expr::Binary(Box::new(expr), op, Box::new(right));
        }
        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, Error> {
        let mut expr = self.unary()?;
        while self.match_tokens(&[TokenType::Slash, TokenType::Star]) {
            let op = self.previous();
            let right = self.unary()?;
            expr = Expr::Binary(Box::new(expr), op, Box::new(right));
        }
        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, Error> {
        if self.match_tokens(&[TokenType::Bang, TokenType::Minus]) {
            let op = self.previous();
            let right = self.unary()?;
            return Ok(Expr::Unary(op, Box::new(right)));
        }
        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, Error> {
        if self.match_tokens(&[TokenType::False]) {
            return Ok(Expr::Literal(LiteralValue::False));
        }
        if self.match_tokens(&[TokenType::True]) {
            return Ok(Expr::Literal(LiteralValue::True));
        }
        if self.match_tokens(&[TokenType::Nil]) {
            return Ok(Expr::Literal(LiteralValue::Nil));
        }
        if self.match_tokens(&[TokenType::Number(0.0)]) {
            if let TokenType::Number(val) = self.previous().token_type {
                return Ok(Expr::Literal(LiteralValue::Number(val)));
            }
        }
        if self.match_tokens(&[TokenType::String(String::new())]) {
            if let TokenType::String(val) = self.previous().token_type {
                return Ok(Expr::Literal(LiteralValue::String(val)));
            }
        }
        if self.match_tokens(&[TokenType::LeftParen]) {
            let expr = self.expression()?;
            self.consume(
                &TokenType::RightParen,
                String::from("Expect ')' after expression."),
            );
            return Ok(Expr::Grouping(Box::new(expr)));
        }
        Err(Error::parsing(String::from("Expected Expression")))
    }

    fn synchronize(&mut self) {
        self.next();
        while !self.is_finished() {
            if matches!(self.previous().token_type, TokenType::SemiColon) {
                return;
            }

            match self.peek().token_type {
                TokenType::Func => return,
                TokenType::Var => return,
                TokenType::For => return,
                TokenType::If => return,
                TokenType::While => return,
                TokenType::Print => return,
                TokenType::Return => return,
                _ => (),
            }
        }
        self.next();
    }

    fn consume(&mut self, token_type: &TokenType, message: String) -> Result<Token, Error> {
        if self.check(token_type) {
            return Ok(self.next());
        }
        Err(Error::parsing(message))
    }

    fn match_tokens(&mut self, token_types: &[TokenType]) -> bool {
        for token_type in token_types {
            if self.check(token_type) {
                self.next();
                return true;
            }
        }
        false
    }

    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_finished() {
            return true;
        }
        compare_token_types!(self.peek().token_type, token_type)
    }

    fn next(&mut self) -> Token {
        if !self.is_finished() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_finished(&self) -> bool {
        matches!(self.peek().token_type, TokenType::EOF)
    }

    fn peek(&self) -> Token {
        self.tokens[self.current].clone()
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }
}
