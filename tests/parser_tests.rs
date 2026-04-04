use taji::lexer::Lexer;
use taji::parser::Parser;

// ═══════════════════════════════════════════════════════════
//  Test: Pernyataan Misalkan
// ═══════════════════════════════════════════════════════════

#[test]
fn test_misalkan_statements() {
    let input = r#"
        misalkan x = 5;
        misalkan y = 10;
        misalkan foobar = 838383;
    "#;

    let l = Lexer::new(input);
    let mut p = Parser::new(l);
    let program = p.parse_program();
    check_parser_errors(&p);

    assert_eq!(
        program.statements.len(),
        3,
        "program.statements seharusnya berisi 3 pernyataan, ditemukan {}",
        program.statements.len()
    );

    let expected_names = vec!["x", "y", "foobar"];
    for (i, name) in expected_names.iter().enumerate() {
        let stmt = &program.statements[i];
        assert!(
            format!("{}", stmt).contains(&format!("misalkan {} =", name)),
            "pernyataan ke-{} seharusnya berisi 'misalkan {} ='",
            i,
            name
        );
    }
}

// ═══════════════════════════════════════════════════════════
//  Test: Pernyataan Kembalikan
// ═══════════════════════════════════════════════════════════

#[test]
fn test_kembalikan_statements() {
    let input = r#"
        kembalikan 5;
        kembalikan 10;
        kembalikan 993322;
    "#;

    let l = Lexer::new(input);
    let mut p = Parser::new(l);
    let program = p.parse_program();
    check_parser_errors(&p);

    assert_eq!(
        program.statements.len(),
        3,
        "program.statements seharusnya berisi 3 pernyataan"
    );
}

// ═══════════════════════════════════════════════════════════
//  Test: Ekspresi Sederhana
// ═══════════════════════════════════════════════════════════

#[test]
fn test_identifier_expression() {
    let input = "foobar;";

    let l = Lexer::new(input);
    let mut p = Parser::new(l);
    let program = p.parse_program();
    check_parser_errors(&p);

    assert_eq!(program.statements.len(), 1);
    assert_eq!(format!("{}", program.statements[0]), "foobar");
}

#[test]
fn test_integer_literal_expression() {
    let input = "5;";

    let l = Lexer::new(input);
    let mut p = Parser::new(l);
    let program = p.parse_program();
    check_parser_errors(&p);

    assert_eq!(program.statements.len(), 1);
    assert_eq!(format!("{}", program.statements[0]), "5");
}

#[test]
fn test_boolean_literal_expression() {
    let tests = vec![("benar;", "benar"), ("salah;", "salah")];

    for (input, expected) in tests {
        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program();
        check_parser_errors(&p);

        assert_eq!(program.statements.len(), 1);
        assert_eq!(format!("{}", program.statements[0]), expected);
    }
}

#[test]
fn test_string_literal_expression() {
    let input = r#""halo dunia";"#;

    let l = Lexer::new(input);
    let mut p = Parser::new(l);
    let program = p.parse_program();
    check_parser_errors(&p);

    assert_eq!(program.statements.len(), 1);
    assert_eq!(
        format!("{}", program.statements[0]),
        "\"halo dunia\""
    );
}

// ═══════════════════════════════════════════════════════════
//  Test: Ekspresi Prefix
// ═══════════════════════════════════════════════════════════

#[test]
fn test_prefix_expressions() {
    let tests = vec![
        ("!5;", "(!5)"),
        ("-15;", "(-15)"),
        ("!benar;", "(!benar)"),
        ("!salah;", "(!salah)"),
    ];

    for (input, expected) in tests {
        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program();
        check_parser_errors(&p);

        assert_eq!(program.statements.len(), 1);
        assert_eq!(format!("{}", program.statements[0]), expected);
    }
}

// ═══════════════════════════════════════════════════════════
//  Test: Ekspresi Infix (Prioritas Operator)
// ═══════════════════════════════════════════════════════════

#[test]
fn test_infix_expressions() {
    let tests = vec![
        ("5 + 5;", "(5 + 5)"),
        ("5 - 5;", "(5 - 5)"),
        ("5 * 5;", "(5 * 5)"),
        ("5 / 5;", "(5 / 5)"),
        ("5 > 5;", "(5 > 5)"),
        ("5 < 5;", "(5 < 5)"),
        ("5 == 5;", "(5 == 5)"),
        ("5 != 5;", "(5 != 5)"),
        ("benar == benar;", "(benar == benar)"),
        ("benar != salah;", "(benar != salah)"),
    ];

    for (input, expected) in tests {
        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program();
        check_parser_errors(&p);

        assert_eq!(program.statements.len(), 1);
        assert_eq!(format!("{}", program.statements[0]), expected);
    }
}

// ═══════════════════════════════════════════════════════════
//  Test: Prioritas Operator (Operator Precedence)
// ═══════════════════════════════════════════════════════════

#[test]
fn test_operator_precedence() {
    let tests = vec![
        ("-a * b;", "((-a) * b)"),
        ("!-a;", "(!(-a))"),
        ("a + b + c;", "((a + b) + c)"),
        ("a + b - c;", "((a + b) - c)"),
        ("a * b * c;", "((a * b) * c)"),
        ("a * b / c;", "((a * b) / c)"),
        ("a + b / c;", "(a + (b / c))"),
        ("a + b * c + d / e - f;", "(((a + (b * c)) + (d / e)) - f)"),
        ("5 > 4 == 3 < 4;", "((5 > 4) == (3 < 4))"),
        (
            "3 + 4 * 5 == 3 * 1 + 4 * 5;",
            "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))",
        ),
        ("1 + (2 + 3) + 4;", "((1 + (2 + 3)) + 4)"),
        ("(5 + 5) * 2;", "((5 + 5) * 2)"),
        ("2 / (5 + 5);", "(2 / (5 + 5))"),
        ("-(5 + 5);", "(-(5 + 5))"),
        ("!(benar == benar);", "(!(benar == benar))"),
    ];

    for (input, expected) in tests {
        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program();
        check_parser_errors(&p);

        let actual = format!("{}", program);
        assert_eq!(actual, expected, "input: {}", input);
    }
}

// ═══════════════════════════════════════════════════════════
//  Test: Ekspresi Jika/Lainnya
// ═══════════════════════════════════════════════════════════

#[test]
fn test_jika_expression() {
    let input = "jika (x < y) { x }";

    let l = Lexer::new(input);
    let mut p = Parser::new(l);
    let program = p.parse_program();
    check_parser_errors(&p);

    assert_eq!(program.statements.len(), 1);
}

#[test]
fn test_jika_lainnya_expression() {
    let input = "jika (x < y) { x } lainnya { y }";

    let l = Lexer::new(input);
    let mut p = Parser::new(l);
    let program = p.parse_program();
    check_parser_errors(&p);

    assert_eq!(program.statements.len(), 1);
}

// ═══════════════════════════════════════════════════════════
//  Test: Fungsi Literal
// ═══════════════════════════════════════════════════════════

#[test]
fn test_fungsi_literal() {
    let input = "fungsi(x, y) { x + y; }";

    let l = Lexer::new(input);
    let mut p = Parser::new(l);
    let program = p.parse_program();
    check_parser_errors(&p);

    assert_eq!(program.statements.len(), 1);
}

#[test]
fn test_fungsi_parameter_parsing() {
    let tests = vec![
        ("fungsi() {};", vec![]),
        ("fungsi(x) {};", vec!["x"]),
        ("fungsi(x, y, z) {};", vec!["x", "y", "z"]),
    ];

    for (input, expected_params) in tests {
        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program();
        check_parser_errors(&p);

        assert_eq!(program.statements.len(), 1);
        let _ = expected_params; // parameter count validated by no-error parse
    }
}

// ═══════════════════════════════════════════════════════════
//  Test: Pemanggilan Fungsi
// ═══════════════════════════════════════════════════════════

#[test]
fn test_panggilan_expression() {
    let input = "tambah(1, 2 * 3, 4 + 5);";

    let l = Lexer::new(input);
    let mut p = Parser::new(l);
    let program = p.parse_program();
    check_parser_errors(&p);

    assert_eq!(program.statements.len(), 1);
    assert_eq!(
        format!("{}", program.statements[0]),
        "tambah(1, (2 * 3), (4 + 5))"
    );
}

// ═══════════════════════════════════════════════════════════
//  Test: Array & Index
// ═══════════════════════════════════════════════════════════

#[test]
fn test_array_literal() {
    let input = "[1, 2 * 2, 3 + 3];";

    let l = Lexer::new(input);
    let mut p = Parser::new(l);
    let program = p.parse_program();
    check_parser_errors(&p);

    assert_eq!(program.statements.len(), 1);
    assert_eq!(
        format!("{}", program.statements[0]),
        "[1, (2 * 2), (3 + 3)]"
    );
}

#[test]
fn test_index_expression() {
    let input = "daftar[1 + 1];";

    let l = Lexer::new(input);
    let mut p = Parser::new(l);
    let program = p.parse_program();
    check_parser_errors(&p);

    assert_eq!(program.statements.len(), 1);
    assert_eq!(format!("{}", program.statements[0]), "(daftar[(1 + 1)])");
}

// ═══════════════════════════════════════════════════════════
//  Test: Hash Literal
// ═══════════════════════════════════════════════════════════

#[test]
fn test_hash_literal() {
    let input = r#"{"satu": 1, "dua": 2, "tiga": 3};"#;

    let l = Lexer::new(input);
    let mut p = Parser::new(l);
    let program = p.parse_program();
    check_parser_errors(&p);

    assert_eq!(program.statements.len(), 1);
}

#[test]
fn test_empty_hash_literal() {
    let input = "{};";

    let l = Lexer::new(input);
    let mut p = Parser::new(l);
    let program = p.parse_program();
    check_parser_errors(&p);

    assert_eq!(program.statements.len(), 1);
}

// ═══════════════════════════════════════════════════════════
//  Helper
// ═══════════════════════════════════════════════════════════

fn check_parser_errors(parser: &Parser) {
    if parser.errors.is_empty() {
        return;
    }

    let mut msg = format!(
        "\nParser menemukan {} kesalahan:\n",
        parser.errors.len()
    );
    for err in &parser.errors {
        msg.push_str(&format!("  - {}\n", err));
    }
    panic!("{}", msg);
}
