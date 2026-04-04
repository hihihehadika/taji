use crate::token::{Token, TokenType};

/// Lexer (Pemindai Leksikal) untuk bahasa Taji.
pub struct Lexer {
    input: Vec<char>,
    position: usize,
    read_position: usize,
    ch: char,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let mut l = Lexer {
            input: input.chars().collect(),
            position: 0,
            read_position: 0,
            ch: '\0',
        };
        l.read_char();
        l
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn peek_char(&self) -> char {
        if self.read_position >= self.input.len() {
            '\0'
        } else {
            self.input[self.read_position]
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let tok = match self.ch {
            '=' => {
                if self.peek_char() == '=' {
                    let ch = self.ch;
                    self.read_char();
                    Token::new(TokenType::Eq, format!("{}{}", ch, self.ch))
                } else {
                    Token::new(TokenType::Assign, self.ch.to_string())
                }
            }
            '+' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::new(TokenType::PlusEq, "+=".to_string())
                } else {
                    Token::new(TokenType::Plus, self.ch.to_string())
                }
            }
            '-' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::new(TokenType::MinusEq, "-=".to_string())
                } else {
                    Token::new(TokenType::Minus, self.ch.to_string())
                }
            }
            '!' => {
                if self.peek_char() == '=' {
                    let ch = self.ch;
                    self.read_char();
                    Token::new(TokenType::NotEq, format!("{}{}", ch, self.ch))
                } else {
                    Token::new(TokenType::Bang, self.ch.to_string())
                }
            }
            '*' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::new(TokenType::MulEq, "*=".to_string())
                } else {
                    Token::new(TokenType::Asterisk, self.ch.to_string())
                }
            }
            '/' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::new(TokenType::DivEq, "/=".to_string())
                } else {
                    Token::new(TokenType::Slash, self.ch.to_string())
                }
            }
            '%' => Token::new(TokenType::Modulo, self.ch.to_string()),
            '<' => {
                if self.peek_char() == '=' {
                    let ch = self.ch;
                    self.read_char();
                    Token::new(TokenType::LtEq, format!("{}{}", ch, self.ch))
                } else {
                    Token::new(TokenType::Lt, self.ch.to_string())
                }
            }
            '>' => {
                if self.peek_char() == '=' {
                    let ch = self.ch;
                    self.read_char();
                    Token::new(TokenType::GtEq, format!("{}{}", ch, self.ch))
                } else {
                    Token::new(TokenType::Gt, self.ch.to_string())
                }
            }
            ';' => Token::new(TokenType::Semicolon, self.ch.to_string()),
            ':' => Token::new(TokenType::Colon, self.ch.to_string()),
            '.' => Token::new(TokenType::Dot, self.ch.to_string()),
            ',' => Token::new(TokenType::Comma, self.ch.to_string()),
            '(' => Token::new(TokenType::Lparen, self.ch.to_string()),
            ')' => Token::new(TokenType::Rparen, self.ch.to_string()),
            '{' => Token::new(TokenType::Lbrace, self.ch.to_string()),
            '}' => Token::new(TokenType::Rbrace, self.ch.to_string()),
            '[' => Token::new(TokenType::Lbracket, self.ch.to_string()),
            ']' => Token::new(TokenType::Rbracket, self.ch.to_string()),
            '"' => {
                let literal = self.read_string();
                return Token::new(TokenType::Str, literal);
            }
            '\0' => Token::new(TokenType::Eof, "".to_string()),
            _ => {
                if is_letter(self.ch) {
                    let literal = self.read_identifier();
                    let type_ = Token::lookup_ident(&literal);
                    return Token::new(type_, literal);
                } else if is_digit(self.ch) {
                    return self.read_number_token();
                } else {
                    Token::new(TokenType::Illegal, self.ch.to_string())
                }
            }
        };

        self.read_char();
        tok
    }

    // ── Private helpers ─────────────────────────────────

    fn read_identifier(&mut self) -> String {
        let position = self.position;
        while is_letter(self.ch) || is_digit(self.ch) {
            self.read_char();
        }
        self.input[position..self.position].iter().collect()
    }

    /// Membaca angka (bulat atau desimal) dan mengembalikan token.
    fn read_number_token(&mut self) -> Token {
        let position = self.position;
        while is_digit(self.ch) {
            self.read_char();
        }

        // Cek apakah ada titik desimal
        if self.ch == '.' && is_digit(self.peek_char()) {
            self.read_char(); // lewati titik
            while is_digit(self.ch) {
                self.read_char();
            }
            let literal: String = self.input[position..self.position].iter().collect();
            Token::new(TokenType::Float, literal)
        } else {
            let literal: String = self.input[position..self.position].iter().collect();
            Token::new(TokenType::Int, literal)
        }
    }

    fn read_string(&mut self) -> String {
        let position = self.position + 1;
        loop {
            self.read_char();
            if self.ch == '"' || self.ch == '\0' {
                break;
            }
        }
        let s = self.input[position..self.position].iter().collect();
        self.read_char();
        s
    }

    fn skip_whitespace(&mut self) {
        loop {
            if self.ch.is_whitespace() {
                self.read_char();
            } else if self.ch == '/' && self.peek_char() == '/' {
                while self.ch != '\n' && self.ch != '\0' {
                    self.read_char();
                }
            } else {
                break;
            }
        }
    }
}

fn is_letter(ch: char) -> bool {
    ch.is_alphabetic() || ch == '_'
}

fn is_digit(ch: char) -> bool {
    ch.is_ascii_digit()
}
