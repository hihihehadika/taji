use taji::evaluator;
use taji::lexer::Lexer;
use taji::object::{Environment, Object};
use taji::parser::Parser;

/// Helper: parse & eval kode, kembalikan Object.
fn test_eval(input: &str) -> Object {
    let l = Lexer::new(input);
    let mut p = Parser::new(l);
    let program = p.parse_program();

    assert!(
        p.errors.is_empty(),
        "Parser errors: {:?}",
        p.errors
    );

    let mut env = Environment::new();
    evaluator::eval(&program, &mut env)
}

// ═══════════════════════════════════════════════════════════
//  Test: Evaluasi Angka Bulat
// ═══════════════════════════════════════════════════════════

#[test]
fn test_eval_integer_expression() {
    let tests = vec![
        ("5", 5),
        ("10", 10),
        ("-5", -5),
        ("-10", -10),
        ("5 + 5 + 5 + 5 - 10", 10),
        ("2 * 2 * 2 * 2 * 2", 32),
        ("-50 + 100 + -50", 0),
        ("5 * 2 + 10", 20),
        ("5 + 2 * 10", 25),
        ("20 + 2 * -10", 0),
        ("50 / 2 * 2 + 10", 60),
        ("2 * (5 + 10)", 30),
        ("3 * 3 * 3 + 10", 37),
        ("3 * (3 * 3) + 10", 37),
        ("(5 + 10 * 2 + 15 / 3) * 2 + -10", 50),
        ("10 % 3", 1),
        ("15 % 4", 3),
    ];

    for (input, expected) in tests {
        let result = test_eval(input);
        assert_integer_object(result, expected, input);
    }
}

// ═══════════════════════════════════════════════════════════
//  Test: Evaluasi Boolean
// ═══════════════════════════════════════════════════════════

#[test]
fn test_eval_boolean_expression() {
    let tests = vec![
        ("benar", true),
        ("salah", false),
        ("1 < 2", true),
        ("1 > 2", false),
        ("1 < 1", false),
        ("1 > 1", false),
        ("1 == 1", true),
        ("1 != 1", false),
        ("1 == 2", false),
        ("1 != 2", true),
        ("benar == benar", true),
        ("salah == salah", true),
        ("benar == salah", false),
        ("benar != salah", true),
        ("(1 < 2) == benar", true),
        ("(1 < 2) == salah", false),
        ("(1 > 2) == benar", false),
        ("(1 > 2) == salah", true),
        ("1 <= 2", true),
        ("2 <= 2", true),
        ("3 <= 2", false),
        ("2 >= 1", true),
        ("2 >= 2", true),
        ("2 >= 3", false),
    ];

    for (input, expected) in tests {
        let result = test_eval(input);
        assert_boolean_object(result, expected, input);
    }
}

// ═══════════════════════════════════════════════════════════
//  Test: Operator Bang (!)
// ═══════════════════════════════════════════════════════════

#[test]
fn test_bang_operator() {
    let tests = vec![
        ("!benar", false),
        ("!salah", true),
        ("!5", false),
        ("!!benar", true),
        ("!!salah", false),
        ("!!5", true),
    ];

    for (input, expected) in tests {
        let result = test_eval(input);
        assert_boolean_object(result, expected, input);
    }
}

// ═══════════════════════════════════════════════════════════
//  Test: Ekspresi Jika/Lainnya
// ═══════════════════════════════════════════════════════════

#[test]
fn test_jika_lainnya_expressions() {
    let tests: Vec<(&str, Option<i64>)> = vec![
        ("jika (benar) { 10 }", Some(10)),
        ("jika (salah) { 10 }", None),
        ("jika (1) { 10 }", Some(10)),
        ("jika (1 < 2) { 10 }", Some(10)),
        ("jika (1 > 2) { 10 }", None),
        ("jika (1 > 2) { 10 } lainnya { 20 }", Some(20)),
        ("jika (1 < 2) { 10 } lainnya { 20 }", Some(10)),
    ];

    for (input, expected) in tests {
        let result = test_eval(input);
        match expected {
            Some(val) => assert_integer_object(result, val, input),
            None => assert!(
                matches!(result, Object::Null),
                "input '{}': diharapkan Null, diterima {:?}",
                input,
                result
            ),
        }
    }
}

// ═══════════════════════════════════════════════════════════
//  Test: Pernyataan Kembalikan
// ═══════════════════════════════════════════════════════════

#[test]
fn test_kembalikan_statements() {
    let tests = vec![
        ("kembalikan 10;", 10),
        ("kembalikan 10; 9;", 10),
        ("kembalikan 2 * 5; 9;", 10),
        ("9; kembalikan 2 * 5; 9;", 10),
    ];

    for (input, expected) in tests {
        let result = test_eval(input);
        assert_integer_object(result, expected, input);
    }
}

// ═══════════════════════════════════════════════════════════
//  Test: Pernyataan Misalkan (Variabel)
// ═══════════════════════════════════════════════════════════

#[test]
fn test_misalkan_statements() {
    let tests = vec![
        ("misalkan a = 5; a;", 5),
        ("misalkan a = 5 * 5; a;", 25),
        ("misalkan a = 5; misalkan b = a; b;", 5),
        (
            "misalkan a = 5; misalkan b = a; misalkan c = a + b + 5; c;",
            15,
        ),
    ];

    for (input, expected) in tests {
        let result = test_eval(input);
        assert_integer_object(result, expected, input);
    }
}

// ═══════════════════════════════════════════════════════════
//  Test: Fungsi & Pemanggilan
// ═══════════════════════════════════════════════════════════

#[test]
fn test_function_object() {
    let input = "fungsi(x) { x + 2; };";
    let result = test_eval(input);

    match result {
        Object::Function(f) => {
            assert_eq!(f.parameters.len(), 1);
            assert_eq!(f.parameters[0].value, "x");
        }
        _ => panic!("diharapkan Function, diterima {:?}", result),
    }
}

#[test]
fn test_function_application() {
    let tests = vec![
        ("misalkan ident = fungsi(x) { x; }; ident(5);", 5),
        ("misalkan ident = fungsi(x) { kembalikan x; }; ident(5);", 5),
        ("misalkan double = fungsi(x) { x * 2; }; double(5);", 10),
        ("misalkan add = fungsi(x, y) { x + y; }; add(5, 5);", 10),
        (
            "misalkan add = fungsi(x, y) { x + y; }; add(5 + 5, add(5, 5));",
            20,
        ),
        ("fungsi(x) { x; }(5)", 5),
    ];

    for (input, expected) in tests {
        let result = test_eval(input);
        assert_integer_object(result, expected, input);
    }
}

#[test]
fn test_closures() {
    let input = r#"
        misalkan pembuat_penambah = fungsi(x) {
            fungsi(y) { x + y; };
        };
        misalkan tambah_dua = pembuat_penambah(2);
        tambah_dua(3);
    "#;

    let result = test_eval(input);
    assert_integer_object(result, 5, input);
}

#[test]
fn test_recursion() {
    let input = r#"
        misalkan fibonacci = fungsi(x) {
            jika (x < 2) {
                kembalikan x;
            };
            kembalikan fibonacci(x - 1) + fibonacci(x - 2);
        };
        fibonacci(10);
    "#;

    let result = test_eval(input);
    assert_integer_object(result, 55, input);
}

// ═══════════════════════════════════════════════════════════
//  Test: String
// ═══════════════════════════════════════════════════════════

#[test]
fn test_string_literal() {
    let input = r#""halo dunia""#;
    let result = test_eval(input);

    match result {
        Object::Str(s) => assert_eq!(s, "halo dunia"),
        _ => panic!("diharapkan Str, diterima {:?}", result),
    }
}

#[test]
fn test_string_concatenation() {
    let input = r#""Halo" + " " + "Dunia!""#;
    let result = test_eval(input);

    match result {
        Object::Str(s) => assert_eq!(s, "Halo Dunia!"),
        _ => panic!("diharapkan Str, diterima {:?}", result),
    }
}

// ═══════════════════════════════════════════════════════════
//  Test: Fungsi Bawaan (Built-in)
// ═══════════════════════════════════════════════════════════

#[test]
fn test_builtin_panjang() {
    let tests = vec![
        (r#"panjang("")"#, 0),
        (r#"panjang("empat")"#, 5),
        (r#"panjang("halo dunia")"#, 10),
    ];

    for (input, expected) in tests {
        let result = test_eval(input);
        assert_integer_object(result, expected, input);
    }
}

#[test]
fn test_builtin_panjang_array() {
    let tests = vec![
        ("panjang([1, 2, 3])", 3),
        ("panjang([])", 0),
    ];

    for (input, expected) in tests {
        let result = test_eval(input);
        assert_integer_object(result, expected, input);
    }
}

// ═══════════════════════════════════════════════════════════
//  Test: Array
// ═══════════════════════════════════════════════════════════

#[test]
fn test_array_literal() {
    let input = "[1, 2 * 2, 3 + 3]";
    let result = test_eval(input);

    match result {
        Object::Array(elements) => {
            assert_eq!(elements.len(), 3);
            assert_integer_object(elements[0].clone(), 1, "arr[0]");
            assert_integer_object(elements[1].clone(), 4, "arr[1]");
            assert_integer_object(elements[2].clone(), 6, "arr[2]");
        }
        _ => panic!("diharapkan Array, diterima {:?}", result),
    }
}

#[test]
fn test_array_index_expressions() {
    let tests: Vec<(&str, Option<i64>)> = vec![
        ("[1, 2, 3][0]", Some(1)),
        ("[1, 2, 3][1]", Some(2)),
        ("[1, 2, 3][2]", Some(3)),
        ("misalkan i = 0; [1][i];", Some(1)),
        ("[1, 2, 3][1 + 1];", Some(3)),
        ("misalkan arr = [1, 2, 3]; arr[2];", Some(3)),
        ("[1, 2, 3][3]", None),
        ("[1, 2, 3][-1]", None),
    ];

    for (input, expected) in tests {
        let result = test_eval(input);
        match expected {
            Some(val) => assert_integer_object(result, val, input),
            None => assert!(
                matches!(result, Object::Null),
                "input '{}': diharapkan Null, diterima {:?}",
                input,
                result
            ),
        }
    }
}

// ═══════════════════════════════════════════════════════════
//  Test: Hash (Kamus)
// ═══════════════════════════════════════════════════════════

#[test]
fn test_hash_literal() {
    let input = r#"
        misalkan data = {"nama": "Taji", "versi": 1};
        data["nama"];
    "#;

    let result = test_eval(input);
    match result {
        Object::Str(s) => assert_eq!(s, "Taji"),
        _ => panic!("diharapkan Str, diterima {:?}", result),
    }
}

// ═══════════════════════════════════════════════════════════
//  Test: Error Handling
// ═══════════════════════════════════════════════════════════

#[test]
fn test_error_handling() {
    let tests = vec![
        ("5 + benar;", "tipe tidak cocok: BILANGAN + BOOLEAN"),
        ("5 + benar; 5;", "tipe tidak cocok: BILANGAN + BOOLEAN"),
        ("-benar", "operator tidak dikenal: -BOOLEAN"),
        (
            "benar + salah;",
            "operator tidak dikenal: BOOLEAN + BOOLEAN",
        ),
        ("foobar", "pengenal tidak dikenal: 'foobar'"),
    ];

    for (input, expected_msg) in tests {
        let result = test_eval(input);
        match result {
            Object::Error(msg) => assert_eq!(
                msg, expected_msg,
                "pesan error salah untuk input: {}",
                input
            ),
            _ => panic!(
                "input '{}': diharapkan Error, diterima {:?}",
                input, result
            ),
        }
    }
}

#[test]
fn test_division_by_zero() {
    let result = test_eval("10 / 0");
    match result {
        Object::Error(msg) => {
            assert_eq!(msg, "pembagian dengan nol tidak diizinkan")
        }
        _ => panic!("diharapkan Error, diterima {:?}", result),
    }
}

// ═══════════════════════════════════════════════════════════
//  Test: Selama (While Loop)
// ═══════════════════════════════════════════════════════════

#[test]
fn test_selama_loop() {
    let input = r#"
        misalkan x = 0;
        misalkan hasil = 0;
        selama (x < 5) {
            misalkan hasil = hasil + x;
            misalkan x = x + 1;
        };
        hasil;
    "#;

    let result = test_eval(input);
    assert_integer_object(result, 10, input);
}

// ═══════════════════════════════════════════════════════════
//  Helpers
// ═══════════════════════════════════════════════════════════

fn assert_integer_object(obj: Object, expected: i64, context: &str) {
    match obj {
        Object::Integer(val) => assert_eq!(
            val, expected,
            "input '{}': diharapkan {}, diterima {}",
            context, expected, val
        ),
        _ => panic!(
            "input '{}': diharapkan Integer({}), diterima {:?}",
            context, expected, obj
        ),
    }
}

fn assert_boolean_object(obj: Object, expected: bool, context: &str) {
    match obj {
        Object::Boolean(val) => assert_eq!(
            val, expected,
            "input '{}': diharapkan {}, diterima {}",
            context, expected, val
        ),
        _ => panic!(
            "input '{}': diharapkan Boolean({}), diterima {:?}",
            context, expected, obj
        ),
    }
}
