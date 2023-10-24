use crate::error::LoxError;
use crate::token::Token;
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
            tokens: Vec::new(),
        }
    }

    // GETTERS
    fn line(&self) -> usize {
        self.line
    }
    fn current(&self) -> usize {
        self.current
    }

    // SETTERS
    fn set_start(&mut self, index: usize) {
        self.start = index
    }

    fn match_and_advance(&mut self, expected: char) -> bool {
        if !self.is_at_end() || self.source.chars().nth(self.current()) != Some(expected) {
            false
        } else {
            self.current += 1;
            true
        }
    }

    fn update_current_by(&mut self, index: usize) {
        self.current += index;
    }

    fn advance(&mut self) -> Option<char> {
        let result = self.source.chars().nth(self.current);
        self.current += 1;
        result
    }
    fn add_token(&mut self, token_type: TokenType) {
        let token = Token {
            token_type,
            literal: None,
            lexeme: String::from(""),
            line: self.line,
        };
        self.tokens.push(token)
    }

    fn scan_token(&mut self) {
        let next_char = self.advance();
        if let Some(character) = next_char {
            match character {
                '(' => self.add_token(TokenType::LeftParen),
                ')' => self.add_token(TokenType::RightParen),
                '{' => self.add_token(TokenType::LeftBrace),
                '}' => self.add_token(TokenType::RightBrace),
                ',' => self.add_token(TokenType::Comma),
                '.' => self.add_token(TokenType::Dot),
                '-' => self.add_token(TokenType::Minus),
                '+' => self.add_token(TokenType::Plus),
                ';' => self.add_token(TokenType::Semicolon),
                '*' => self.add_token(TokenType::Star),
                _tokens => (),
            }
        }
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
        self.tokens.push(Token::new(
            TokenType::Eof,
            String::from(""),
            None,
            self.line(),
        ));
        dbg!("{}", &self.tokens);
        println!("{}", self.tokens.len());

        Ok(&self.tokens)
    }
}
