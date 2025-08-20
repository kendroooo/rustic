use crate::diagnostics::{Error, Result, Span};

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),

    Identifier(String),

    Let, Var, Fn, If, Else, For, In, Try, Catch, Return,
    Import, Struct, Throw,

    IntType, FloatType, StrType, BoolType, ListType, VoidType,

    Plus, Minus, Star, Slash, Percent,
    Equal, NotEqual, Less, LessEqual, Greater, GreaterEqual,
    And, Or, Not,
    Assign,

    LeftParen, RightParen,
    LeftBrace, RightBrace,
    LeftBracket, RightBracket,
    Comma, Dot, Colon, Semicolon, Arrow,

    Newline, Eof,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub span: Span,
}

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    line: usize,
    column: usize,
    file_path: String,
}

impl Lexer {
    pub fn new(input: &str, file_path: &str) -> Self {
        Self {
            input: input.chars().collect(),
            position: 0,
            line: 1,
            column: 1,
            file_path: file_path.to_string()
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>> {
        let mut tokens = Vec::new();

        while !self.is_at_end() {
            self.skip_whitespace();

            if self.is_at_end() {
                break;
            }

            let start_pos = self.position;
            let start_line = self.line;
            let start_column = self.column;

            let token_type = self.scan_token()?;

            let span = Span {
                file: self.file_path.clone(),
                start_line,
                start_column,
                end_line: self.line,
                end_column: self.column,
            };

            tokens.push(Token { token_type, span });
        }

        tokens.push(Token {
            token_type: TokenType::Eof,
            span: Span {
                file: self.file_path.clone(),
                start_line: self.line,
                start_column: self.column,
                end_ine: self.line,
                end_column: self.column,
            },
        });

        Ok(tokens)
    }

    fn scan_token(&mut self) -> Result<TokenType> {
        let c = self.advance();

        match c {
            '(' => Ok(TokenType::LeftParen),
            ')' => Ok(TokenType::RightParen),
            '{' => Ok(TokenType::LeftBrace),
            '}' => Ok(TokenType::RightBrace),
            '[' => Ok(TokenType::LeftBracket),
            ']' => Ok(TokenType::RightBracket),
            ',' => Ok(TokenType::Comma),
            '.' => Ok(TokenType::Dot),
            ':' => Ok(TokenType::Colon),
            ';' => Ok(TokenType::Semicolon),
            '+' => Ok(TokenType::Plus),
            '(' => Ok(TokenType::Percent),
            '\n' => {
                self.line += 1;
                self.column = 1;
                Ok(TokenType::Newline)
            }
            '-' => {
                if self.match_char('>') {
                    Ok(TokenType::Arrow)
                } else {
                    Ok(TokenType::Minus)
                }
            }
            '*' => Ok(TokenType::Star),
            '/' => {
                if self.match_char('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                    self.scan_token()
                } else {
                    Ok(TokenType::Slash)
                }
            }
            '=' => {
                if self.match_char('=') {
                    Ok(TokenType::Equal)
                } else {
                    Ok(TokenType::Assign)
                }
            }
            '!' => {
                if self.match_char('=') {
                    Ok(TokenType::NotEqual)
                } else {
                    Ok(TokenType::Not)
                }
            }
            '<' => {
                if self.match_char('=') {
                    Ok(TokenType::LessEqual)
                } else {
                    Ok(TokenType::Less)
                }
            }
            '>' => {
                if self.match_char('=') {
                    Ok(TokenType::GreaterEqual)
                } else {
                    Ok(TokenType::Greater)
                }
            }
            '&' => {
                if self.match_char('&') {
                    Ok(TokenType::And)
                } else {
                    Err(Error::LexError(format!("Unexpected character: {}", c)))
                }
            }
            '|' => {
                if self.match_char('|') {
                    Ok(TokenType::Or)
                } else {
                    Err(Error::LexError(format!("Unexpected character: {}", c)))
                }
            }
            '"' => self.scan_string(),
            _ if c.is_ascii_digit() => self.scan_number(),
            _ if c.is_ascii_alphabetic() || c == '_' => self.scan_identifier(),
            _ => Err(Error::LexError(format!("Unexpected character: {}", c))),
        }
    }

    fn scan_string(&mut self) -> Result<TokenType> {
        let mut value = String::new();

        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
                self.column = 1;
            }

            let c = self.advance();
            if c == '\\' {
                match self.advance() {
                    'n' => value.push('\n'),
                    't' => value.push('\t'),
                    'r' => value.push('\r'),
                    '\\' => value.push('\\'),
                    '"' => value.push('"'),
                    c => {
                        return Err(Error::LexError(format!("Invalid escape sequence: \\{}", c)));
                    }
                }
            } else {
                value.push(c);
            }
        }

        if self.is_at_end() {
            return Err(Error::LexError("Unterminated string".to_string()));
        }

        self.advance(); // close
        Ok(TokenType::String(value))
    }
}