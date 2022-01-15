use std::fmt::{Debug, Display};

pub(crate) struct TreeWalk {}

impl TreeWalk {
    pub(crate) fn new() -> Self {
        TreeWalk {}
    }

    pub(crate) fn run(&mut self, source: String) {
        let tokens = Scanner::tokenize(source);
        println!("{:?}", tokens)
    }
}

#[derive(Debug)]
enum TokenType {
    // Single character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Star,
    Slash,
    Semicolon,
    Equal,
    Bang,
    GreaterThan,
    LessThan,

    // Multi char tokens
    DoubleEqual,
    BangEqual,
    GreaterThanEqual,
    LessThanEqual,

    // Literals
    Identifier,
    String,
    Number,

    // Keywords
    And,
    Or,
    If,
    Else,
    True,
    False,
    Nil,
    For,
    While,
    Class,
    Fun,
    Var,
    This,
    Super,
    Print,
    Return,

    // Special
    EOF,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self, f)
    }
}

#[derive(Debug)]
struct Token {
    token_type: TokenType,
    lexeme: String,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Token Type: {}, Lexeme: {}",
            self.token_type, self.lexeme
        )
    }
}

#[derive(Debug)]
struct Scanner<'a> {
    input: &'a str,
    index: usize,
}

impl<'a> Scanner<'a> {
    fn tokenize(source: String) -> Vec<Token> {
        let mut chars = source.char_indices().peekable();
        let mut tokens = Vec::<Token>::new();

        while let Some((idx, ch)) = chars.next() {
            let mut end_idx = idx;
            let token_type = match ch {
                '(' => Some(TokenType::LeftParen),
                ')' => Some(TokenType::RightParen),
                '{' => Some(TokenType::LeftBrace),
                '}' => Some(TokenType::RightBrace),
                ',' => Some(TokenType::Comma),
                '.' => Some(TokenType::Dot),
                '-' => Some(TokenType::Minus),
                '+' => Some(TokenType::Plus),
                '*' => Some(TokenType::Star),
                ';' => Some(TokenType::Semicolon),
                '!' => {
                    if let Some((_, '=')) = chars.peek() {
                        end_idx += 1;
                        chars.next();
                        Some(TokenType::BangEqual)
                    } else {
                        Some(TokenType::Bang)
                    }
                }
                '=' => {
                    if let Some((_, '=')) = chars.peek() {
                        end_idx += 1;
                        chars.next();
                        Some(TokenType::DoubleEqual)
                    } else {
                        Some(TokenType::Equal)
                    }
                }
                '<' => {
                    if let Some((_, '=')) = chars.peek() {
                        end_idx += 1;
                        chars.next();
                        Some(TokenType::LessThanEqual)
                    } else {
                        Some(TokenType::LessThan)
                    }
                }
                '>' => {
                    if let Some((_, '=')) = chars.peek() {
                        end_idx += 1;
                        chars.next();
                        Some(TokenType::GreaterThanEqual)
                    } else {
                        Some(TokenType::GreaterThan)
                    }
                }
                _ => None,
            };

            end_idx += 1;

            if let Some(token_type) = token_type {
                let lexeme = source.get(idx..end_idx).unwrap().to_string();
                tokens.push(Token { token_type, lexeme });
            } else {
                println!("Unexpected character: {}", ch)
            }
        }

        return tokens;
    }
}
