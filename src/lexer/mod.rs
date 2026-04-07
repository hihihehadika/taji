//! Lexer (Pemindai Leksikal) untuk bahasa Taji.
//!
//! Memecah kode sumber menjadi deretan token yang akan
//! diproses oleh Parser.

use crate::token::{Token, TokenType};

#[derive(Clone)]
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

    /// Membaca karakter berikutnya dan memajukan posisi.
    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    /// Mengintip karakter berikutnya tanpa memajukan posisi.
    pub fn peek_char(&self) -> char {
        if self.read_position >= self.input.len() {
            '\0'
        } else {
            self.input[self.read_position]
        }
    }

    /// Menghasilkan token berikutnya dari input.
    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let tok = match self.ch {
            '=' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::new(TokenType::Eq, "==".to_string())
                } else if self.peek_char() == '>' {
                    self.read_char();
                    Token::new(TokenType::Arrow, "=>".to_string())
                } else {
                    Token::new(TokenType::Assign, "=".to_string())
                }
            }
            '+' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::new(TokenType::PlusEq, "+=".to_string())
                } else {
                    Token::new(TokenType::Plus, "+".to_string())
                }
            }
            '-' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::new(TokenType::MinusEq, "-=".to_string())
                } else {
                    Token::new(TokenType::Minus, "-".to_string())
                }
            }
            '!' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::new(TokenType::NotEq, "!=".to_string())
                } else {
                    Token::new(TokenType::Bang, "!".to_string())
                }
            }
            '*' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::new(TokenType::MulEq, "*=".to_string())
                } else {
                    Token::new(TokenType::Asterisk, "*".to_string())
                }
            }
            '/' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::new(TokenType::DivEq, "/=".to_string())
                } else {
                    Token::new(TokenType::Slash, "/".to_string())
                }
            }
            '%' => Token::new(TokenType::Modulo, "%".to_string()),
            '<' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::new(TokenType::LtEq, "<=".to_string())
                } else {
                    Token::new(TokenType::Lt, "<".to_string())
                }
            }
            '>' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::new(TokenType::GtEq, ">=".to_string())
                } else {
                    Token::new(TokenType::Gt, ">".to_string())
                }
            }
            ';' => Token::new(TokenType::Semicolon, ";".to_string()),
            ':' => Token::new(TokenType::Colon, ":".to_string()),
            '.' => Token::new(TokenType::Dot, ".".to_string()),
            ',' => Token::new(TokenType::Comma, ",".to_string()),
            '(' => Token::new(TokenType::Lparen, "(".to_string()),
            ')' => Token::new(TokenType::Rparen, ")".to_string()),
            '{' => Token::new(TokenType::Lbrace, "{".to_string()),
            '}' => Token::new(TokenType::Rbrace, "}".to_string()),
            '[' => Token::new(TokenType::Lbracket, "[".to_string()),
            ']' => Token::new(TokenType::Rbracket, "]".to_string()),
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

    // ── Fungsi pembantu internal ─────────────────────────

    /// Membaca pengenal (identifier) sampai bertemu karakter non-huruf/angka.
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

    /// Membaca teks literal di antara tanda kutip ganda.
    fn read_string(&mut self) -> String {
        let mut string = String::new();
        loop {
            self.read_char();
            if self.ch == '"' || self.ch == '\0' {
                break;
            }
            if self.ch == '\\' {
                self.read_char();
                match self.ch {
                    'n' => string.push('\n'),
                    't' => string.push('\t'),
                    'r' => string.push('\r'),
                    '\\' => string.push('\\'),
                    '"' => string.push('"'),
                    _ => string.push(self.ch),
                }
            } else {
                string.push(self.ch);
            }
        }
        self.read_char();
        string
    }

    /// Melewati spasi, tab, newline, dan komentar satu baris (`//`).
    fn skip_whitespace(&mut self) {
        loop {
            if self.ch.is_whitespace() {
                self.read_char();
            } else if self.ch == '/' && self.peek_char() == '/' {
                // Lewati komentar satu baris sampai akhir baris
                while self.ch != '\n' && self.ch != '\0' {
                    self.read_char();
                }
            } else {
                break;
            }
        }
    }
}

/// Cek apakah karakter adalah huruf atau garis bawah.
fn is_letter(ch: char) -> bool {
    ch.is_alphabetic() || ch == '_'
}

/// Cek apakah karakter adalah digit angka.
fn is_digit(ch: char) -> bool {
    ch.is_ascii_digit()
}
