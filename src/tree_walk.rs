use std::fmt::{Debug, Display};

pub(crate) struct TreeWalk {}

impl TreeWalk {
    pub(crate) fn new() -> Self {
        TreeWalk {}
    }

    pub(crate) fn run(&mut self, source: String) {
        let tokens = Scanner::tokenize(source);
        println!("{}", TokenFormatter { tokens })
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
    String(String),
    Number(f64),

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
    WhiteSpace,
    NewLine,
    Comment,
    Eof,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self, f)
    }
}

struct TokenFormatter {
    tokens: Vec<Token>,
}

impl Display for TokenFormatter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output: String = Default::default();
        self.tokens.iter().for_each(|token| {
            let fmt = format!("{}\n", token);
            output.push_str(&fmt);
        });
        output = output.trim().to_string();
        write!(f, "{}", output)
    }
}

enum Value {
    String(String),
    Number(f64),
}

impl Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self, f)
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Value::String(ref val) => {
                write!(f, "{}", val)
            }
            Value::Number(ref val) => {
                write!(f, "{:?}", val)
            }
        }
    }
}

#[derive(Debug)]
struct Token {
    token_type: TokenType,
    lexeme: String,
    value: Option<Value>,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.token_type {
            TokenType::LeftParen => {
                write!(f, "LEFT PAREN {} null", self.lexeme)
            }
            TokenType::RightParen => {
                write!(f, "RIGHT PAREN {} null", self.lexeme)
            }
            TokenType::LeftBrace => {
                write!(f, "LEFT BRANCE {} null", self.lexeme)
            }
            TokenType::RightBrace => {
                write!(f, "RIGHT BRANCE {} null", self.lexeme)
            }
            TokenType::Comma => {
                write!(f, "Comma {} null", self.lexeme)
            }
            TokenType::Dot => {
                write!(f, "DOT {} null", self.lexeme)
            }
            TokenType::Minus => {
                write!(f, "MINUS {} null", self.lexeme)
            }
            TokenType::Plus => {
                write!(f, "PLUS {} null", self.lexeme)
            }
            TokenType::Star => {
                write!(f, "STAR {} null", self.lexeme)
            }
            TokenType::Slash => {
                write!(f, "SLASH {} null", self.lexeme)
            }
            TokenType::Semicolon => {
                write!(f, "SEMICOLON {} null", self.lexeme)
            }
            TokenType::Equal => {
                write!(f, "EQUAL {} null", self.lexeme)
            }
            TokenType::Bang => {
                write!(f, "BANG {} null", self.lexeme)
            }
            TokenType::GreaterThan => {
                write!(f, "GREATER THAN {} null", self.lexeme)
            }
            TokenType::LessThan => {
                write!(f, "LESS THAN {} null", self.lexeme)
            }
            TokenType::DoubleEqual => {
                write!(f, "DOUBLE EQUAL {} null", self.lexeme)
            }
            TokenType::BangEqual => {
                write!(f, "BANG EQUAL {} null", self.lexeme)
            }
            TokenType::GreaterThanEqual => {
                write!(f, "GREATER THAN EQUAL {} null", self.lexeme)
            }
            TokenType::LessThanEqual => {
                write!(f, "LESS THAN EQUAL {} null", self.lexeme)
            }
            TokenType::String(_) => {
                let val = self.value.as_ref().unwrap();
                write!(f, "STRING \"{}\" {}", self.lexeme, val)
            }
            TokenType::Number(_) => {
                let val = self.value.as_ref().unwrap();
                write!(f, "NUMBER {} {:?}", self.lexeme, val)
            }
            TokenType::Eof => {
                write!(f, "EOF {} null", self.lexeme)
            }
            _ => write!(f, "UNKNOWN"),
        }
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
                '/' => {
                    if let Some((_, '/')) = chars.peek() {
                        // TODO: chars.next() while we get a newline or None
                        while let Some(&(_, c)) = chars.peek() {
                            if c != '\n' {
                                chars.next();
                            } else {
                                break;
                            }
                        }
                        Some(TokenType::Comment)
                    } else {
                        Some(TokenType::Slash)
                    }
                }
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
                ' ' | '\r' | '\t' => Some(TokenType::WhiteSpace),
                '\n' => Some(TokenType::NewLine),
                '"' => {
                    let mut str_val: String = Default::default();
                    while let Some(&(_, c)) = chars.peek() {
                        if c != '"' {
                            // TODO: handle linenum in multiline strings
                            str_val.push(c);
                            chars.next();
                        } else {
                            break;
                        }
                        end_idx += 1;
                    }
                    if let Some(&(_, '"')) = chars.peek() {
                        chars.next();
                        Some(TokenType::String(str_val))
                    } else {
                        panic!("Unterminated string")
                    }
                }
                '0'..='9' => {
                    let mut num_val: String = Default::default();
                    num_val.push(ch);

                    while let Some(&(_, '0'..='9')) = chars.peek() {
                        if let Some((_, c)) = chars.next() {
                            num_val.push(c);
                        }

                        end_idx += 1;
                        // } else if c == '.' {
                        //     let mut chars_next = chars.clone();
                        //     chars_next.next();
                        //     if let Some(&(_, '0'..='9')) = chars_next.peek() {
                        //         num_val.push(c);
                        //         chars.next();
                        //     } else {
                        //         panic!("Cannot end number at decimal.")
                        //     }
                        // } else {
                        //     break;
                        // }
                    }

                    if let Some(&(_, '.')) = chars.peek() {
                        let mut chars_next = chars.clone();
                        chars_next.next();
                        if let Some(&(_, '0'..='9')) = chars_next.peek() {
                            num_val.push('.');
                            end_idx += 1;
                            chars.next();

                            while let Some(&(_, '0'..='9')) = chars.peek() {
                                if let Some((_, c)) = chars.next() {
                                    num_val.push(c);
                                }

                                end_idx += 1;
                            }
                        }
                    }

                    let num: f64 = num_val.parse().expect("Error parsing value as float");
                    Some(TokenType::Number(num))
                }
                _ => None,
            };

            end_idx += 1;

            if let Some(token_type) = token_type {
                match token_type {
                    TokenType::Comment => {}
                    TokenType::NewLine => {}
                    TokenType::WhiteSpace => {}
                    TokenType::String(ref val) => {
                        let lexeme = val.to_string();
                        let value = Some(Value::String(lexeme.clone()));
                        tokens.push(Token {
                            token_type,
                            lexeme,
                            value,
                        })
                    }
                    TokenType::Number(val) => {
                        let lexeme = val.to_string();
                        let value = Some(Value::Number(val));
                        tokens.push(Token {
                            token_type,
                            lexeme,
                            value,
                        })
                    }
                    _ => {
                        let lexeme = source.get(idx..end_idx).unwrap().to_string();
                        tokens.push(Token {
                            token_type,
                            lexeme,
                            value: None,
                        });
                    }
                }
            } else {
                println!("Unexpected character: {}", ch)
            }
        }

        tokens.push(Token {
            token_type: TokenType::Eof,
            lexeme: "".to_string(),
            value: None,
        });

        tokens
    }
}
