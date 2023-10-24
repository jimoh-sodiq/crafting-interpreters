use crate::token_types::TokenType;
use std::fmt;
#[derive(Debug)]
pub enum Object {
    Num(f64),
    Str(String),
    Nil,
    True,
    False,
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Object::Str(val) => write!(f, "{}", val),
            Object::Num(val) => write!(f, "{}", val),
            Object::Nil => write!(f, "Nil"),
            Object::True => write!(f, "True"),
            Object::False => write!(f, "False"),
        }
    }
}
#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<Object>,
    pub line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: Option<Object>, line: usize) -> Self {
        Self {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?}, {:?} {:?}",
            self.token_type,
            self.lexeme,
            if let Some(value) = &self.literal {
                value.to_string()
            } else {
                "None".to_string()
            }
        )
    }
}
