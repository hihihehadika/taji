//! Pengujian unit untuk Lexer bahasa Taji.

use taji::lexer::Lexer;
use taji::token::TokenType;

#[test]
fn test_next_token() {
    let input = r#"
        misalkan lima = 5;
        misalkan sepuluh = 10;

        misalkan tambah = fungsi(x, y) {
            kembalikan x + y;
        };

        misalkan hasil = tambah(lima, sepuluh);
        !-/*5;
        5 < 10 > 5;

        jika (5 < 10) {
            kembalikan benar;
        } lainnya {
            kembalikan salah;
        }

        10 == 10;
        10 != 9;
    "#;

    let tests = vec![
        (TokenType::Misalkan, "misalkan"),
        (TokenType::Ident, "lima"),
        (TokenType::Assign, "="),
        (TokenType::Int, "5"),
        (TokenType::Semicolon, ";"),
        (TokenType::Misalkan, "misalkan"),
        (TokenType::Ident, "sepuluh"),
        (TokenType::Assign, "="),
        (TokenType::Int, "10"),
        (TokenType::Semicolon, ";"),
        (TokenType::Misalkan, "misalkan"),
        (TokenType::Ident, "tambah"),
        (TokenType::Assign, "="),
        (TokenType::Fungsi, "fungsi"),
        (TokenType::Lparen, "("),
        (TokenType::Ident, "x"),
        (TokenType::Comma, ","),
        (TokenType::Ident, "y"),
        (TokenType::Rparen, ")"),
        (TokenType::Lbrace, "{"),
        (TokenType::Kembalikan, "kembalikan"),
        (TokenType::Ident, "x"),
        (TokenType::Plus, "+"),
        (TokenType::Ident, "y"),
        (TokenType::Semicolon, ";"),
        (TokenType::Rbrace, "}"),
        (TokenType::Semicolon, ";"),
        (TokenType::Misalkan, "misalkan"),
        (TokenType::Ident, "hasil"),
        (TokenType::Assign, "="),
        (TokenType::Ident, "tambah"),
        (TokenType::Lparen, "("),
        (TokenType::Ident, "lima"),
        (TokenType::Comma, ","),
        (TokenType::Ident, "sepuluh"),
        (TokenType::Rparen, ")"),
        (TokenType::Semicolon, ";"),
        (TokenType::Bang, "!"),
        (TokenType::Minus, "-"),
        (TokenType::Slash, "/"),
        (TokenType::Asterisk, "*"),
        (TokenType::Int, "5"),
        (TokenType::Semicolon, ";"),
        (TokenType::Int, "5"),
        (TokenType::Lt, "<"),
        (TokenType::Int, "10"),
        (TokenType::Gt, ">"),
        (TokenType::Int, "5"),
        (TokenType::Semicolon, ";"),
        (TokenType::Jika, "jika"),
        (TokenType::Lparen, "("),
        (TokenType::Int, "5"),
        (TokenType::Lt, "<"),
        (TokenType::Int, "10"),
        (TokenType::Rparen, ")"),
        (TokenType::Lbrace, "{"),
        (TokenType::Kembalikan, "kembalikan"),
        (TokenType::Benar, "benar"),
        (TokenType::Semicolon, ";"),
        (TokenType::Rbrace, "}"),
        (TokenType::Lainnya, "lainnya"),
        (TokenType::Lbrace, "{"),
        (TokenType::Kembalikan, "kembalikan"),
        (TokenType::Salah, "salah"),
        (TokenType::Semicolon, ";"),
        (TokenType::Rbrace, "}"),
        (TokenType::Int, "10"),
        (TokenType::Eq, "=="),
        (TokenType::Int, "10"),
        (TokenType::Semicolon, ";"),
        (TokenType::Int, "10"),
        (TokenType::NotEq, "!="),
        (TokenType::Int, "9"),
        (TokenType::Semicolon, ";"),
        (TokenType::Eof, ""),
    ];

    let mut l = Lexer::new(input);

    for (expected_type, expected_literal) in tests {
        let tok = l.next_token();
        assert_eq!(tok.type_, expected_type);
        assert_eq!(tok.literal, expected_literal);
    }
}

#[test]
fn test_string_token() {
    let input = r#""halo dunia""#;
    let mut l = Lexer::new(input);
    let tok = l.next_token();
    assert_eq!(tok.type_, TokenType::Str);
    assert_eq!(tok.literal, "halo dunia");
}

#[test]
fn test_new_operators() {
    let input = "5 % 3; 10 <= 20; 30 >= 15;";
    let mut l = Lexer::new(input);

    let tests = vec![
        (TokenType::Int, "5"),
        (TokenType::Modulo, "%"),
        (TokenType::Int, "3"),
        (TokenType::Semicolon, ";"),
        (TokenType::Int, "10"),
        (TokenType::LtEq, "<="),
        (TokenType::Int, "20"),
        (TokenType::Semicolon, ";"),
        (TokenType::Int, "30"),
        (TokenType::GtEq, ">="),
        (TokenType::Int, "15"),
        (TokenType::Semicolon, ";"),
        (TokenType::Eof, ""),
    ];

    for (expected_type, expected_literal) in tests {
        let tok = l.next_token();
        assert_eq!(tok.type_, expected_type);
        assert_eq!(tok.literal, expected_literal);
    }
}

#[test]
fn test_new_keywords() {
    let input = "selama dan atau bukan untuk berhenti lanjut masukkan";
    let mut l = Lexer::new(input);

    let tests = vec![
        (TokenType::Selama, "selama"),
        (TokenType::Dan, "dan"),
        (TokenType::Atau, "atau"),
        (TokenType::Bukan, "bukan"),
        (TokenType::Untuk, "untuk"),
        (TokenType::Berhenti, "berhenti"),
        (TokenType::Lanjut, "lanjut"),
        (TokenType::Masukkan, "masukkan"),
        (TokenType::Eof, ""),
    ];

    for (expected_type, expected_literal) in tests {
        let tok = l.next_token();
        assert_eq!(tok.type_, expected_type);
        assert_eq!(tok.literal, expected_literal);
    }
}

#[test]
fn test_brackets_and_colon() {
    let input = "[1, 2]; {\"a\": 1};";
    let mut l = Lexer::new(input);

    let tests = vec![
        (TokenType::Lbracket, "["),
        (TokenType::Int, "1"),
        (TokenType::Comma, ","),
        (TokenType::Int, "2"),
        (TokenType::Rbracket, "]"),
        (TokenType::Semicolon, ";"),
        (TokenType::Lbrace, "{"),
        (TokenType::Str, "a"),
        (TokenType::Colon, ":"),
        (TokenType::Int, "1"),
        (TokenType::Rbrace, "}"),
        (TokenType::Semicolon, ";"),
        (TokenType::Eof, ""),
    ];

    for (expected_type, expected_literal) in tests {
        let tok = l.next_token();
        assert_eq!(tok.type_, expected_type, "literal: {}", tok.literal);
        assert_eq!(tok.literal, expected_literal);
    }
}

#[test]
fn test_comments() {
    let input = r#"
        // Ini adalah komentar, harus diabaikan
        misalkan x = 5; // komentar di akhir baris
        // komentar lagi
        misalkan y = 10;
    "#;

    let mut l = Lexer::new(input);

    let tests = vec![
        (TokenType::Misalkan, "misalkan"),
        (TokenType::Ident, "x"),
        (TokenType::Assign, "="),
        (TokenType::Int, "5"),
        (TokenType::Semicolon, ";"),
        (TokenType::Misalkan, "misalkan"),
        (TokenType::Ident, "y"),
        (TokenType::Assign, "="),
        (TokenType::Int, "10"),
        (TokenType::Semicolon, ";"),
        (TokenType::Eof, ""),
    ];

    for (expected_type, expected_literal) in tests {
        let tok = l.next_token();
        assert_eq!(tok.type_, expected_type, "literal: {}", tok.literal);
        assert_eq!(tok.literal, expected_literal);
    }
}

#[test]
fn test_float_token() {
    let input = "3.14 0.5 100.0";
    let mut l = Lexer::new(input);

    let tests = vec![
        (TokenType::Float, "3.14"),
        (TokenType::Float, "0.5"),
        (TokenType::Float, "100.0"),
        (TokenType::Eof, ""),
    ];

    for (expected_type, expected_literal) in tests {
        let tok = l.next_token();
        assert_eq!(tok.type_, expected_type, "literal: {}", tok.literal);
        assert_eq!(tok.literal, expected_literal);
    }
}

#[test]
fn test_compound_assignment_tokens() {
    let input = "x += 3; y -= 1; z *= 2; w /= 4;";
    let mut l = Lexer::new(input);

    let tests = vec![
        (TokenType::Ident, "x"),
        (TokenType::PlusEq, "+="),
        (TokenType::Int, "3"),
        (TokenType::Semicolon, ";"),
        (TokenType::Ident, "y"),
        (TokenType::MinusEq, "-="),
        (TokenType::Int, "1"),
        (TokenType::Semicolon, ";"),
        (TokenType::Ident, "z"),
        (TokenType::MulEq, "*="),
        (TokenType::Int, "2"),
        (TokenType::Semicolon, ";"),
        (TokenType::Ident, "w"),
        (TokenType::DivEq, "/="),
        (TokenType::Int, "4"),
        (TokenType::Semicolon, ";"),
        (TokenType::Eof, ""),
    ];

    for (expected_type, expected_literal) in tests {
        let tok = l.next_token();
        assert_eq!(tok.type_, expected_type, "literal: {}", tok.literal);
        assert_eq!(tok.literal, expected_literal);
    }
}
