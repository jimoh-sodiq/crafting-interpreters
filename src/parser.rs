use crate::error::*;
use crate::expr::*;
use crate::token::*;
use crate::token_types::*;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    fn expression(&mut self) -> Result<Expr, LoxError> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.comparison()?;
        while self.is_match(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = *self.previous();
            let right = self.comparison()?;
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }
        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.term()?;

        while self.is_match(vec![
            TokenType::Greater,
            TokenType::Less,
            TokenType::GreaterEqual,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous();
            let right = self.term()?;
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }
        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.factor()?;

        while self.is_match(vec![TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous();
            let right = self.factor()?;
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }
        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.unary()?;

        while self.is_match(vec![TokenType::Slash, TokenType::Star]) {
            let operator = self.previous();
            let right = self.unary()?;
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }
        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, LoxError> {
        if self.is_match(vec![TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.unary()?;
            let expr = Expr::Unary(UnaryExpr {
                operator,
                right: Box::new(right),
            });
            return Ok(expr);
        }

        return self.primary();
    }

    fn primary(&mut self) -> Result<Expr, LoxError> {
        if self.is_match(vec![TokenType::False]) {
            let expr = Expr::Literal(LiteralExpr {
                value: Some(Object::False)
            });
            return Ok(expr);
        }
        if self.is_match(vec![TokenType::True]) {
            let expr = Expr::Literal(LiteralExpr {
                value: Some(Object::True)
            });
            return Ok(expr);
        }
        if self.is_match(vec![TokenType::Nil]) {
            let expr = Expr::Literal(LiteralExpr {
                value: Some(Object::Nil)
            });
            return Ok(expr);
        }
        if self.is_match(vec![TokenType::Number, TokenType::String]) {
            let expr = Expr::Literal(LiteralExpr {
                value: self.previous().literal
            });
            return Ok(expr);
        }

        if self.is_match(vec![TokenType::LeftParen]) {
            let expr = self.expression()?;
            self.consume(TokenType::RightParen, "Expect ')' after expression.".to_string());
            return Ok(Expr::Grouping(GroupingExpr {
                expression:  Box::new(expr),
            }))
        }

        return Err(LoxError::new())
    }

    fn consume(&mut self, ttype: TokenType, message: String) -> Result<&Token, LoxError> {
        if self.check(ttype){
            return Ok(self.advance());
        }
        return Err(LoxError::new())
    }

    fn is_match(&self, token_types: Vec<TokenType>) -> bool {
        for ttype in token_types {
            if self.check(ttype.clone()) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, ttype: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        };
        self.peek().unwrap().token_type == ttype
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().unwrap().token_type == TokenType::Eof
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    fn previous(&self) -> &Token {
        self.tokens.get(self.current - 1).unwrap()
    }
}
