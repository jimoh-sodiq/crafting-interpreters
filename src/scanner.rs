use crate::error::LoxError;
use crate::token::{Object, Token};
use crate::token_types::TokenType;
pub struct Scanner {
    source: String,
    start: usize,
    current: usize,
    line: usize,
    tokens: Vec<Token>,
}
impl Scanner {
    pub fn new(source: String) -> Self {
        Scanner {
            source,
            start: 0,
            current: 0,
            line: 1,
            tokens: vec![Token {
                token_type: TokenType::LeftParen,
                lexeme: String::from("Hello"),
                literal: Some(Object::True),
                line: 0,
            }],
        }
    }

    // GETTERS
    fn line(&self) -> usize {
        self.line
    }
    fn current(&self) -> usize {
        self.current
    }
    fn start(&self) -> usize {
        self.start
    }

    // SETTERS
    fn set_line(&mut self, index: usize) {
        self.line = index
    }
    fn set_current(&mut self, index: usize) {
        self.current = index
    }
    fn set_start(&mut self, index: usize) {
        self.start = index
    }

    fn advance(&mut self) {}

    fn scan_token(&mut self) {
        let c = self.advance();
    }

    fn is_at_end(&self) -> bool {
        return self.current() >= self.source.len();
    }

    pub fn scan_tokens(&mut self) -> Result<&Vec<Token>, LoxError> {
        while !self.is_at_end() {
            // We are at the beginning of the next lexeme.
            self.set_start(self.current());
            self.scan_token();
        }

        Ok(&self.tokens)
    }
}
