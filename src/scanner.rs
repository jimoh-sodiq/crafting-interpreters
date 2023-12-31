use crate::error::LoxError;
use crate::token::{Object, Token};
use crate::token_types::TokenType;
use std::collections::HashMap;
pub struct Scanner {
    source: String,
    start: usize,
    current: usize,
    line: usize,
    tokens: Vec<Token>,
    keywords: HashMap<String, TokenType>,
}
impl Scanner {
    pub fn new(source: String) -> Self {
        Scanner {
            source,
            start: 0,
            current: 0,
            line: 1,
            tokens: Vec::new(),
            keywords: HashMap::from([
                ("and".to_string(), TokenType::And),
                ("class".to_string(), TokenType::Class),
                ("else".to_string(), TokenType::Else),
                ("false".to_string(), TokenType::False),
                ("for".to_string(), TokenType::For),
                ("fun".to_string(), TokenType::Fun),
                ("if".to_string(), TokenType::If),
                ("nil".to_string(), TokenType::Nil),
                ("or".to_string(), TokenType::Or),
                ("print".to_string(), TokenType::Print),
                ("return".to_string(), TokenType::Return),
                ("super".to_string(), TokenType::Super),
                ("this".to_string(), TokenType::This),
                ("true".to_string(), TokenType::True),
                ("var".to_string(), TokenType::Var),
                ("while".to_string(), TokenType::While),
            ]),
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

    fn matches(&mut self, expected: char) -> bool {
        if self.is_at_end() || (self.source.chars().nth(self.current) != Some(expected)) {
            false
        } else {
            self.current += 1;
            true
        }
    }

    fn advance(&mut self) -> Option<char> {
        let result = self.source.chars().nth(self.current);
        self.current += 1;
        result
    }
    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_object(token_type, None)
    }

    fn add_token_object(&mut self, token_type: TokenType, literal: Option<Object>) {
        let text = self
            .source
            .get(self.start.try_into().unwrap()..self.current.try_into().unwrap());
        let token = Token {
            token_type,
            literal,
            lexeme: match text {
                None => String::from(""),
                Some(value) => value.to_string(),
            },
            line: self.line,
        };
        self.tokens.push(token)
    }

    fn match_and_advance(&mut self, expected: char, true_val: TokenType, false_val: TokenType) {
        let matches_expectation = self.matches(expected);
        match matches_expectation {
            true => self.add_token(true_val),
            false => self.add_token(false_val),
        }
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source.chars().nth(self.current).unwrap()
        }
    }

    fn handle_string_literal(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1
            };
            self.advance();
        }

        if self.is_at_end() {
            panic!("Unterminated String");
        }

        // The closing ".
        self.advance();

        // Trim the surrounding quotes.
        let value = self.source.get(self.start + 1..self.current - 1).unwrap();
        self.add_token_object(TokenType::String, Some(Object::Str(value.to_string())));
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            '\0'
        } else {
            self.source.chars().nth(self.current + 1).unwrap()
        }
    }

    fn is_digit(&self, c: char) -> bool {
        c >= '0' && c <= '9'
    }

    fn number(&mut self) {
        while self.is_digit(self.peek()) {
            self.advance();
        }
        // Look for a fractional part.
        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            // Consume the "."
            self.advance();

            while self.is_digit(self.peek()) {
                self.advance();
            }
        }
        let num_val: f64 = self
            .source
            .get(self.start..self.current)
            .unwrap()
            .parse()
            .unwrap();

        self.add_token_object(TokenType::Number, Some(Object::Num(num_val)));
    }

    fn is_alpha(&self, c: char) -> bool {
        c.is_ascii_alphabetic() || c == '_'
    }

    fn is_alpha_numeric(&self, c: char) -> bool {
        c.is_ascii_alphanumeric() || c.is_ascii_digit()
    }

    fn identifier(&mut self) {
        while self.is_alpha_numeric(self.peek()) {
            self.advance();
        }
        let text = self.source.get(self.start..self.current).unwrap();

        let ttype: Option<TokenType> = self.keywords.get(text).cloned();
        match ttype {
            Some(t) => self.add_token(t),
            None => self.add_token(TokenType::Identifier),
        }
    }

    fn handle_multiline_comment(&mut self) {
        loop {
            match self.peek() {
                '*' => {
                    self.advance();
                    if self.matches('/') {
                        return;
                    }
                }
                '/' => {
                    self.advance();
                    if self.matches('*') {
                        self.handle_multiline_comment();
                    }
                }
                '\n' => {
                    self.line += 1;
                    self.advance();
                }
                _ => {
                    self.advance();
                }
            }
            if self.is_at_end() {
                panic!("Failed to close open comment")
            }
        }
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
                '!' => self.match_and_advance('=', TokenType::BangEqual, TokenType::Bang),
                '=' => self.match_and_advance('=', TokenType::EqualEqual, TokenType::Equal),
                '<' => self.match_and_advance('=', TokenType::LessEqual, TokenType::Less),
                '>' => self.match_and_advance('=', TokenType::GreaterEqual, TokenType::Greater),
                '/' => {
                    if self.matches('/') {
                        while (self.peek() != '\n') && (!self.is_at_end()) {
                            self.advance();
                        }
                    } else if self.matches('*') {
                        self.handle_multiline_comment();
                    } else {
                        self.add_token(TokenType::Slash);
                    }
                }
                ' ' | '\r' | '\t' => {}
                '\n' => self.line += 1,
                '"' => self.handle_string_literal(),
                _token => {
                    if self.is_digit(_token) {
                        self.number()
                    } else if self.is_alpha(_token) {
                        self.identifier()
                    } else {
                        unreachable!("token type not matched")
                    }
                }
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
