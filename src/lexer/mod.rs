//! Lexer (Pemindai Leksikal) untuk bahasa Taji.
//!
//! Memecah kode sumber menjadi deretan token yang akan
//! diproses oleh Parser. Setiap token menyimpan metadata
//! posisi (baris dan kolom) untuk pelaporan galat yang akurat.

use crate::token::{Token, TokenType};

#[derive(Clone)]
pub struct Lexer {
    input: Vec<char>,
    position: usize,
    read_position: usize,
    ch: char,
    /// Nomor baris saat ini (1-indexed).
    baris: usize,
    /// Nomor kolom saat ini (1-indexed).
    kolom: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let mut l = Lexer {
            input: input.chars().collect(),
            position: 0,
            read_position: 0,
            ch: '\0',
            baris: 1,
            kolom: 0,
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

        // Pelacakan baris dan kolom
        if self.ch == '\n' {
            self.baris += 1;
            self.kolom = 0;
        } else {
            self.kolom += 1;
        }
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

        // Catat posisi awal token sebelum membaca
        let baris_token = self.baris;
        let kolom_token = self.kolom;

        let tok = match self.ch {
            '=' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::new(TokenType::Eq, "==".to_string(), baris_token, kolom_token)
                } else if self.peek_char() == '>' {
                    self.read_char();
                    Token::new(TokenType::Arrow, "=>".to_string(), baris_token, kolom_token)
                } else {
                    Token::new(TokenType::Assign, "=".to_string(), baris_token, kolom_token)
                }
            }
            '+' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::new(
                        TokenType::PlusEq,
                        "+=".to_string(),
                        baris_token,
                        kolom_token,
                    )
                } else {
                    Token::new(TokenType::Plus, "+".to_string(), baris_token, kolom_token)
                }
            }
            '-' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::new(
                        TokenType::MinusEq,
                        "-=".to_string(),
                        baris_token,
                        kolom_token,
                    )
                } else {
                    Token::new(TokenType::Minus, "-".to_string(), baris_token, kolom_token)
                }
            }
            '!' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::new(TokenType::NotEq, "!=".to_string(), baris_token, kolom_token)
                } else {
                    Token::new(TokenType::Bang, "!".to_string(), baris_token, kolom_token)
                }
            }
            '*' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::new(TokenType::MulEq, "*=".to_string(), baris_token, kolom_token)
                } else {
                    Token::new(
                        TokenType::Asterisk,
                        "*".to_string(),
                        baris_token,
                        kolom_token,
                    )
                }
            }
            '/' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::new(TokenType::DivEq, "/=".to_string(), baris_token, kolom_token)
                } else {
                    Token::new(TokenType::Slash, "/".to_string(), baris_token, kolom_token)
                }
            }
            '%' => Token::new(TokenType::Modulo, "%".to_string(), baris_token, kolom_token),
            '<' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::new(TokenType::LtEq, "<=".to_string(), baris_token, kolom_token)
                } else {
                    Token::new(TokenType::Lt, "<".to_string(), baris_token, kolom_token)
                }
            }
            '>' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::new(TokenType::GtEq, ">=".to_string(), baris_token, kolom_token)
                } else {
                    Token::new(TokenType::Gt, ">".to_string(), baris_token, kolom_token)
                }
            }
            ';' => Token::new(
                TokenType::Semicolon,
                ";".to_string(),
                baris_token,
                kolom_token,
            ),
            ':' => Token::new(TokenType::Colon, ":".to_string(), baris_token, kolom_token),
            '.' => Token::new(TokenType::Dot, ".".to_string(), baris_token, kolom_token),
            ',' => Token::new(TokenType::Comma, ",".to_string(), baris_token, kolom_token),
            '(' => Token::new(TokenType::Lparen, "(".to_string(), baris_token, kolom_token),
            ')' => Token::new(TokenType::Rparen, ")".to_string(), baris_token, kolom_token),
            '{' => Token::new(TokenType::Lbrace, "{".to_string(), baris_token, kolom_token),
            '}' => Token::new(TokenType::Rbrace, "}".to_string(), baris_token, kolom_token),
            '[' => Token::new(
                TokenType::Lbracket,
                "[".to_string(),
                baris_token,
                kolom_token,
            ),
            ']' => Token::new(
                TokenType::Rbracket,
                "]".to_string(),
                baris_token,
                kolom_token,
            ),
            '"' => {
                let literal = self.read_string();
                return Token::new(TokenType::Str, literal, baris_token, kolom_token);
            }
            '\0' => Token::new(TokenType::Eof, "".to_string(), baris_token, kolom_token),
            _ => {
                if is_letter(self.ch) {
                    let literal = self.read_identifier();
                    let type_ = Token::lookup_ident(&literal);
                    return Token::new(type_, literal, baris_token, kolom_token);
                } else if is_digit(self.ch) {
                    return self.read_number_token(baris_token, kolom_token);
                } else {
                    Token::new(
                        TokenType::Illegal,
                        self.ch.to_string(),
                        baris_token,
                        kolom_token,
                    )
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
    fn read_number_token(&mut self, baris: usize, kolom: usize) -> Token {
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
            Token::new(TokenType::Float, literal, baris, kolom)
        } else {
            let literal: String = self.input[position..self.position].iter().collect();
            Token::new(TokenType::Int, literal, baris, kolom)
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
                    'x' => {
                        let hex1 = self.peek_char();
                        if hex1.is_ascii_hexdigit() {
                            self.read_char();
                            let h1 = self.ch;
                            let hex2 = self.peek_char();
                            if hex2.is_ascii_hexdigit() {
                                self.read_char();
                                let h2 = self.ch;
                                if let Ok(byte) = u8::from_str_radix(&format!("{}{}", h1, h2), 16) {
                                    string.push(byte as char);
                                } else {
                                    string.push('x');
                                    string.push(h1);
                                    string.push(h2);
                                }
                            } else {
                                string.push('x');
                                string.push(h1);
                            }
                        } else {
                            string.push('x');
                        }
                    }
                    _ => string.push(self.ch),
                }
            } else {
                string.push(self.ch);
            }
        }
        self.read_char();
        string
    }

    /// Melewati spasi, tab, newline, komentar satu baris (`//`),
    /// dan komentar multi-baris (`/* ... */`).
    fn skip_whitespace(&mut self) {
        loop {
            if self.ch.is_whitespace() {
                self.read_char();
            } else if self.ch == '/' && self.peek_char() == '/' {
                // Komentar satu baris: lewati sampai akhir baris
                while self.ch != '\n' && self.ch != '\0' {
                    self.read_char();
                }
            } else if self.ch == '/' && self.peek_char() == '*' {
                // Komentar multi-baris: /* ... */
                self.read_char(); // lewati '/'
                self.read_char(); // lewati '*'
                loop {
                    if self.ch == '\0' {
                        break; // EOF di dalam komentar, hentikan
                    }
                    if self.ch == '*' && self.peek_char() == '/' {
                        self.read_char(); // lewati '*'
                        self.read_char(); // lewati '/'
                        break;
                    }
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
