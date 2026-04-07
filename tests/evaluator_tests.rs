//! Pengujian unit untuk Evaluator bahasa Taji.
use taji::evaluator;
use taji::lexer::Lexer;
use taji::object::{Lingkungan, Object};
use taji::parser::Parser;

// ── Fungsi pembantu ──────────────────────────────────

fn test_eval(input: &str) -> Object {
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    if !parser.errors.is_empty() {
        panic!("Parser errors for input '{}':\n{}", input, parser.errors.join("\n"));
    }

    let mut env = Lingkungan::new();
    evaluator::eval(&program, &mut env)
}

fn test_integer_object(obj: &Object, expected: i64) {
    match obj {
        Object::Integer(val) => assert_eq!(*val, expected, "expected {}, got {}", expected, val),
        _ => panic!("expected Integer({}), got {:?}", expected, obj),
    }
}

fn test_float_object(obj: &Object, expected: f64) {
    match obj {
        Object::Float(val) => {
            assert!(
                (*val - expected).abs() < 0.0001,
                "expected ~{}, got {}",
                expected,
                val
            );
        }
        _ => panic!("expected Float({}), got {:?}", expected, obj),
    }
}

fn test_boolean_object(obj: &Object, expected: bool) {
    match obj {
        Object::Boolean(val) => assert_eq!(*val, expected),
        _ => panic!("expected Boolean({}), got {:?}", expected, obj),
    }
}

// ═══════════════════════════════════════════════════════════
//  Pengujian dasar (v0.1.0)
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
        test_integer_object(&result, expected);
    }
}

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
        ("benar dan benar", true),
        ("benar dan salah", false),
        ("benar atau salah", true),
        ("salah atau salah", false),
    ];

    for (input, expected) in tests {
        let result = test_eval(input);
        test_boolean_object(&result, expected);
    }
}

#[test]
fn test_bang_operator() {
    let tests = vec![
        ("!benar", false),
        ("!salah", true),
        ("!5", false),
        ("!!benar", true),
        ("!!salah", false),
        ("!!5", true),
        ("bukan benar", false),
        ("bukan salah", true),
    ];

    for (input, expected) in tests {
        let result = test_eval(input);
        test_boolean_object(&result, expected);
    }
}

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
            Some(val) => test_integer_object(&result, val),
            None => assert!(matches!(result, Object::Null)),
        }
    }
}

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
        test_integer_object(&result, expected);
    }
}

#[test]
fn test_error_handling() {
    let tests = vec![
        ("5 + benar;", "tipe tidak cocok: BILANGAN + BOOLEAN"),
        ("5 + benar; 5;", "tipe tidak cocok: BILANGAN + BOOLEAN"),
        ("-benar", "operator tidak dikenal: -BOOLEAN"),
        ("benar + salah;", "operator tidak dikenal: BOOLEAN + BOOLEAN"),
        ("5; benar + salah; 5", "operator tidak dikenal: BOOLEAN + BOOLEAN"),
        ("foobar", "pengenal tidak dikenal: 'foobar'"),
    ];

    for (input, expected_msg) in tests {
        let result = test_eval(input);
        match result {
            Object::Error(msg) => assert_eq!(msg, expected_msg, "input: {}", input),
            _ => panic!("expected error for '{}', got {:?}", input, result),
        }
    }
}

#[test]
fn test_misalkan_statements() {
    let tests = vec![
        ("misalkan a = 5; a;", 5),
        ("misalkan a = 5 * 5; a;", 25),
        ("misalkan a = 5; misalkan b = a; b;", 5),
        ("misalkan a = 5; misalkan b = a; misalkan c = a + b + 5; c;", 15),
    ];

    for (input, expected) in tests {
        let result = test_eval(input);
        test_integer_object(&result, expected);
    }
}

#[test]
fn test_function_object() {
    let input = "fungsi(x) { x + 2; };";
    let result = test_eval(input);
    match result {
        Object::Fungsi(f) => {
            assert_eq!(f.parameters.len(), 1);
            assert_eq!(f.parameters[0].value, "x");
        }
        _ => panic!("expected Fungsi, got {:?}", result),
    }
}

#[test]
fn test_function_application() {
    let tests = vec![
        ("misalkan id = fungsi(x) { x; }; id(5);", 5),
        ("misalkan id = fungsi(x) { kembalikan x; }; id(5);", 5),
        ("misalkan ganda = fungsi(x) { x * 2; }; ganda(5);", 10),
        ("misalkan tambah = fungsi(x, y) { x + y; }; tambah(5, 5);", 10),
        (
            "misalkan tambah = fungsi(x, y) { x + y; }; tambah(5 + 5, tambah(5, 5));",
            20,
        ),
        ("fungsi(x) { x; }(5)", 5),
    ];

    for (input, expected) in tests {
        let result = test_eval(input);
        test_integer_object(&result, expected);
    }
}

#[test]
fn test_closures() {
    let input = "
        misalkan pembuat_penambah = fungsi(x) {
            fungsi(y) { x + y; };
        };
        misalkan tambah_dua = pembuat_penambah(2);
        tambah_dua(3);
    ";
    let result = test_eval(input);
    test_integer_object(&result, 5);
}

#[test]
fn test_recursion() {
    let input = "
        misalkan fibonacci = fungsi(x) {
            jika (x < 2) {
                kembalikan x;
            };
            kembalikan fibonacci(x - 1) + fibonacci(x - 2);
        };
        fibonacci(10);
    ";
    let result = test_eval(input);
    test_integer_object(&result, 55);
}

#[test]
fn test_string_literal() {
    let result = test_eval("\"halo dunia\"");
    match result {
        Object::Str(s) => assert_eq!(s, "halo dunia"),
        _ => panic!("expected String, got {:?}", result),
    }
}

#[test]
fn test_string_concatenation() {
    let result = test_eval("\"Halo\" + \" \" + \"Dunia!\"");
    match result {
        Object::Str(s) => assert_eq!(s, "Halo Dunia!"),
        _ => panic!("expected String, got {:?}", result),
    }
}

#[test]
fn test_builtin_panjang() {
    let tests: Vec<(&str, i64)> = vec![
        ("panjang(\"\")", 0),
        ("panjang(\"empat\")", 5),
        ("panjang(\"halo dunia\")", 10),
    ];

    for (input, expected) in tests {
        let result = test_eval(input);
        test_integer_object(&result, expected);
    }
}

#[test]
fn test_builtin_panjang_array() {
    let result = test_eval("panjang([1, 2, 3])");
    test_integer_object(&result, 3);
}

#[test]
fn test_array_literal() {
    let result = test_eval("[1, 2 * 2, 3 + 3]");
    match result {
        Object::Array(elements) => {
            assert_eq!(elements.len(), 3);
            test_integer_object(&elements[0], 1);
            test_integer_object(&elements[1], 4);
            test_integer_object(&elements[2], 6);
        }
        _ => panic!("expected Array, got {:?}", result),
    }
}

#[test]
fn test_array_index_expressions() {
    let tests = vec![
        ("[1, 2, 3][0]", 1),
        ("[1, 2, 3][1]", 2),
        ("[1, 2, 3][2]", 3),
        ("misalkan i = 0; [1][i];", 1),
        ("[1, 2, 3][1 + 1];", 3),
        ("misalkan arr = [1, 2, 3]; arr[2];", 3),
    ];

    for (input, expected) in tests {
        let result = test_eval(input);
        test_integer_object(&result, expected);
    }
}

#[test]
fn test_hash_literal() {
    let input = r#"
        misalkan dua = "dua";
        {
            "satu": 10 - 9,
            dua: 1 + 1,
            "ti" + "ga": 6 / 2,
            4: 4,
            benar: 5,
            salah: 6
        }
    "#;
    let result = test_eval(input);
    match result {
        Object::Hash(pairs) => {
            assert_eq!(pairs.len(), 6);
        }
        _ => panic!("expected Hash, got {:?}", result),
    }
}

#[test]
fn test_selama_loop() {
    let input = "
        misalkan x = 0;
        misalkan i = 0;
        selama (i < 5) {
            x += 1;
            i += 1;
        };
        x;
    ";
    let result = test_eval(input);
    test_integer_object(&result, 5);
}

#[test]
fn test_division_by_zero() {
    let result = test_eval("10 / 0");
    match result {
        Object::Error(msg) => {
            assert_eq!(msg, "pembagian dengan nol tidak diizinkan");
        }
        _ => panic!("expected error, got {:?}", result),
    }
}

// ═══════════════════════════════════════════════════════════
//  Pengujian fitur baru (v0.2.0)
// ═══════════════════════════════════════════════════════════

#[test]
fn test_float_literals() {
    let tests: Vec<(&str, f64)> = vec![
        ("3.14", 3.14),
        ("0.5", 0.5),
        ("-2.5", -2.5),
    ];

    for (input, expected) in tests {
        let result = test_eval(input);
        test_float_object(&result, expected);
    }
}

#[test]
fn test_float_arithmetic() {
    let tests: Vec<(&str, f64)> = vec![
        ("1.5 + 2.5", 4.0),
        ("10.0 - 3.5", 6.5),
        ("2.0 * 3.0", 6.0),
        ("10.0 / 4.0", 2.5),
    ];

    for (input, expected) in tests {
        let result = test_eval(input);
        test_float_object(&result, expected);
    }
}

#[test]
fn test_mixed_int_float_arithmetic() {
    let tests: Vec<(&str, f64)> = vec![
        ("5 + 2.5", 7.5),
        ("2.5 + 5", 7.5),
        ("10 * 1.5", 15.0),
        ("7.0 / 2", 3.5),
    ];

    for (input, expected) in tests {
        let result = test_eval(input);
        test_float_object(&result, expected);
    }
}

#[test]
fn test_float_comparison() {
    let tests = vec![
        ("1.5 < 2.5", true),
        ("2.5 > 1.5", true),
        ("1.5 == 1.5", true),
        ("1.5 != 2.5", true),
        ("3.0 <= 3.0", true),
        ("3.0 >= 4.0", false),
    ];

    for (input, expected) in tests {
        let result = test_eval(input);
        test_boolean_object(&result, expected);
    }
}

#[test]
fn test_compound_assignment() {
    let tests = vec![
        ("misalkan x = 10; x += 5; x;", 15),
        ("misalkan x = 10; x -= 3; x;", 7),
        ("misalkan x = 10; x *= 2; x;", 20),
        ("misalkan x = 10; x /= 2; x;", 5),
    ];

    for (input, expected) in tests {
        let result = test_eval(input);
        test_integer_object(&result, expected);
    }
}

#[test]
fn test_simple_assignment() {
    let input = "misalkan x = 5; x = 10; x;";
    let result = test_eval(input);
    test_integer_object(&result, 10);
}

#[test]
fn test_untuk_loop() {
    let input = "
        misalkan total = 0;
        untuk (misalkan i = 1; i <= 5; i += 1) {
            total += i;
        };
        total;
    ";
    let result = test_eval(input);
    test_integer_object(&result, 15);  // 1+2+3+4+5
}

#[test]
fn test_untuk_loop_nested() {
    let input = "
        misalkan total = 0;
        untuk (misalkan i = 0; i < 3; i += 1) {
            untuk (misalkan j = 0; j < 3; j += 1) {
                total += 1;
            };
        };
        total;
    ";
    let result = test_eval(input);
    test_integer_object(&result, 9);  // 3 * 3
}

#[test]
fn test_berhenti_selama() {
    let input = "
        misalkan x = 0;
        selama (benar) {
            x += 1;
            jika (x == 5) {
                berhenti;
            };
        };
        x;
    ";
    let result = test_eval(input);
    test_integer_object(&result, 5);
}

#[test]
fn test_berhenti_untuk() {
    let input = "
        misalkan total = 0;
        untuk (misalkan i = 0; i < 100; i += 1) {
            jika (i == 5) {
                berhenti;
            };
            total += 1;
        };
        total;
    ";
    let result = test_eval(input);
    test_integer_object(&result, 5);  // 0,1,2,3,4 → 5 iterasi
}

#[test]
fn test_lanjut_untuk() {
    let input = "
        misalkan total = 0;
        untuk (misalkan i = 0; i < 10; i += 1) {
            jika (i % 2 == 0) {
                lanjut;
            };
            total += 1;
        };
        total;
    ";
    let result = test_eval(input);
    test_integer_object(&result, 5);  // ganjil: 1,3,5,7,9
}

#[test]
fn test_dot_expression() {
    let input = r#"
        misalkan profil = {
            "nama": "Dika",
            "umur": 20
        };
        profil.nama;
    "#;
    let result = test_eval(input);
    match result {
        Object::Str(s) => assert_eq!(s, "Dika"),
        _ => panic!("expected String 'Dika', got {:?}", result),
    }
}

#[test]
fn test_dot_expression_integer() {
    let input = r#"
        misalkan obj = { "x": 42 };
        obj.x;
    "#;
    let result = test_eval(input);
    test_integer_object(&result, 42);
}

#[test]
fn test_builtin_teks() {
    let tests: Vec<(&str, &str)> = vec![
        ("teks(42)", "42"),
        ("teks(benar)", "benar"),
        ("teks(3.14)", "3.14"),
    ];

    for (input, expected) in tests {
        let result = test_eval(input);
        match result {
            Object::Str(s) => assert_eq!(s, expected, "input: {}", input),
            _ => panic!("expected String '{}', got {:?}", expected, result),
        }
    }
}

#[test]
fn test_builtin_angka() {
    let tests = vec![
        ("angka(\"42\")", 42),
        ("angka(\"100\")", 100),
    ];

    for (input, expected) in tests {
        let result = test_eval(input);
        test_integer_object(&result, expected);
    }
}

#[test]
fn test_builtin_angka_float() {
    let input = "angka(\"3.14\")";
    let result = test_eval(input);
    test_float_object(&result, 3.14);
}

#[test]
fn test_builtin_waktu() {
    let result = test_eval("waktu()");
    match result {
        Object::Integer(val) => assert!(val > 0, "timestamp harus positif"),
        _ => panic!("expected Integer timestamp, got {:?}", result),
    }
}

#[test]
fn test_string_auto_concat() {
    // Teks + Bilangan → konversi otomatis ke teks
    let input = "\"Umur: \" + 20";
    let result = test_eval(input);
    match result {
        Object::Str(s) => assert_eq!(s, "Umur: 20"),
        _ => panic!("expected String, got {:?}", result),
    }
}

#[test]
fn test_masukkan_import() {
    // Buat file sementara untuk test import
    let test_dir = std::env::current_dir().unwrap();
    let test_file = test_dir.join("_test_modul.tj");
    std::fs::write(&test_file, "misalkan x = 42; misalkan y = 100;").unwrap();

    let input = format!(
        "misalkan m = masukkan(\"{}\"); m.x + m.y;",
        test_file.to_str().unwrap().replace('\\', "\\\\")
    );

    let result = test_eval(&input);
    test_integer_object(&result, 142);

    // Cleanup
    std::fs::remove_file(test_file).unwrap();
}

// ═══════════════════════════════════════════════════════════
//  Pengujian fitur baru (v0.3.0)
// ═══════════════════════════════════════════════════════════

#[test]
fn test_builtin_pisah() {
    let input = r#"pisah("a,b,c", ",")"#;
    let result = test_eval(input);
    match result {
        Object::Array(arr) => {
            assert_eq!(arr.len(), 3);
            assert_eq!(format!("{}", arr[0]), "a");
            assert_eq!(format!("{}", arr[1]), "b");
            assert_eq!(format!("{}", arr[2]), "c");
        }
        _ => panic!("diharapkan DAFTAR, diterima {:?}", result),
    }
}

#[test]
fn test_builtin_pisah_spasi() {
    let input = r#"pisah("halo dunia taji", " ")"#;
    let result = test_eval(input);
    match result {
        Object::Array(arr) => {
            assert_eq!(arr.len(), 3);
            assert_eq!(format!("{}", arr[0]), "halo");
            assert_eq!(format!("{}", arr[1]), "dunia");
            assert_eq!(format!("{}", arr[2]), "taji");
        }
        _ => panic!("diharapkan DAFTAR, diterima {:?}", result),
    }
}

#[test]
fn test_builtin_pisah_error() {
    let result = test_eval(r#"pisah(123, ",")"#);
    match result {
        Object::Error(msg) => {
            assert!(
                msg.contains("argumen pertama untuk 'pisah' harus TEKS"),
                "pesan error tidak sesuai: {}", msg
            );
        }
        _ => panic!("diharapkan Error, diterima {:?}", result),
    }
}

#[test]
fn test_builtin_gabung() {
    let input = r#"gabung(["A", "B", "C"], "-")"#;
    let result = test_eval(input);
    match result {
        Object::Str(s) => assert_eq!(s, "A-B-C"),
        _ => panic!("diharapkan TEKS 'A-B-C', diterima {:?}", result),
    }
}

#[test]
fn test_builtin_gabung_kosong() {
    let input = r#"gabung([], ",")"#;
    let result = test_eval(input);
    match result {
        Object::Str(s) => assert_eq!(s, ""),
        _ => panic!("diharapkan TEKS kosong, diterima {:?}", result),
    }
}

#[test]
fn test_builtin_gabung_error() {
    let result = test_eval(r#"gabung("bukan daftar", ",")"#);
    match result {
        Object::Error(msg) => {
            assert!(
                msg.contains("argumen pertama untuk 'gabung' harus DAFTAR"),
                "pesan error tidak sesuai: {}", msg
            );
        }
        _ => panic!("diharapkan Error, diterima {:?}", result),
    }
}

#[test]
fn test_builtin_pisah_gabung_roundtrip() {
    // Pisah lalu gabung harus mengembalikan teks asli
    let input = r#"gabung(pisah("halo-dunia-taji", "-"), "-")"#;
    let result = test_eval(input);
    match result {
        Object::Str(s) => assert_eq!(s, "halo-dunia-taji"),
        _ => panic!("diharapkan TEKS asli, diterima {:?}", result),
    }
}

#[test]
fn test_builtin_tulis_dan_baca_berkas() {
    let test_dir = std::env::current_dir().unwrap();
    let test_file = test_dir.join("_test_berkas.txt");

    // Tulis ke file
    let input_tulis = format!(
        r#"tulis_berkas("{}", "Halo dari Taji!")"#,
        test_file.to_str().unwrap().replace('\\', "\\\\")
    );
    let result = test_eval(&input_tulis);
    match &result {
        Object::Str(s) => assert!(s.contains("berhasil menulis")),
        _ => panic!("diharapkan pesan berhasil menulis, diterima {:?}", result),
    }

    // Baca dari file
    let input_baca = format!(
        r#"baca_berkas("{}")"#,
        test_file.to_str().unwrap().replace('\\', "\\\\")
    );
    let result = test_eval(&input_baca);
    match result {
        Object::Str(s) => assert_eq!(s, "Halo dari Taji!"),
        _ => panic!("diharapkan TEKS 'Halo dari Taji!', diterima {:?}", result),
    }

    // Cleanup
    std::fs::remove_file(test_file).unwrap();
}

#[test]
fn test_builtin_baca_berkas_tidak_ada() {
    let result = test_eval(r#"baca_berkas("_file_yang_tidak_ada.txt")"#);
    match result {
        Object::Error(msg) => {
            assert!(
                msg.contains("gagal membaca berkas"),
                "pesan error tidak sesuai: {}", msg
            );
        }
        _ => panic!("diharapkan Error, diterima {:?}", result),
    }
}

// ── Arrow Functions ─────────────────────────────────

#[test]
fn test_arrow_function_ekspresi_tunggal() {
    // (x) => x * 2  → ekspresi tunggal (pengembalian implisit)
    let input = "misalkan kali_dua = (x) => x * 2; kali_dua(5);";
    let result = test_eval(input);
    test_integer_object(&result, 10);
}

#[test]
fn test_arrow_function_blok() {
    // (x) => { kembalikan x + 10; }
    let input = "misalkan tambah_sepuluh = (x) => { kembalikan x + 10; }; tambah_sepuluh(5);";
    let result = test_eval(input);
    test_integer_object(&result, 15);
}

#[test]
fn test_arrow_function_multi_param() {
    let input = "misalkan tambah = (a, b) => a + b; tambah(3, 7);";
    let result = test_eval(input);
    test_integer_object(&result, 10);
}

#[test]
fn test_arrow_function_tanpa_param() {
    let input = "misalkan salam = () => \"halo\"; salam();";
    let result = test_eval(input);
    match result {
        Object::Str(s) => assert_eq!(s, "halo"),
        _ => panic!("diharapkan TEKS 'halo', diterima {:?}", result),
    }
}

#[test]
fn test_arrow_function_closure() {
    // Arrow function juga harus mendukung closure sama seperti fungsi biasa
    let input = "
        misalkan pembuat = (faktor) => {
            kembalikan (x) => x * faktor;
        };
        misalkan kali_tiga = pembuat(3);
        kali_tiga(4);
    ";
    let result = test_eval(input);
    test_integer_object(&result, 12);
}

#[test]
fn test_arrow_function_sama_dengan_fungsi() {
    // Arrow function dan fungsi biasa harus menghasilkan hasil yang sama
    let input_fungsi = "misalkan f = fungsi(x) { x * x; }; f(6);";
    let input_panah = "misalkan f = (x) => x * x; f(6);";
    let result_fungsi = test_eval(input_fungsi);
    let result_panah = test_eval(input_panah);

    test_integer_object(&result_fungsi, 36);
    test_integer_object(&result_panah, 36);
}

// ── Coba / Tangkap ──────────────────────────────────

#[test]
fn test_coba_tangkap_menangkap_error() {
    let input = r#"
        misalkan hasil = coba {
            10 / 0;
        } tangkap (err) {
            "tertangkap: " + err;
        };
        hasil;
    "#;
    let result = test_eval(input);
    match result {
        Object::Str(s) => assert!(
            s.contains("tertangkap"),
            "pesan harus mengandung 'tertangkap', diterima: {}", s
        ),
        _ => panic!("diharapkan TEKS, diterima {:?}", result),
    }
}

#[test]
fn test_coba_tangkap_tanpa_error() {
    // Jika tidak ada error, blok tangkap tidak dieksekusi
    let input = "
        misalkan hasil = coba {
            42;
        } tangkap (err) {
            0;
        };
        hasil;
    ";
    let result = test_eval(input);
    test_integer_object(&result, 42);
}

#[test]
fn test_coba_tangkap_pengenal_tidak_dikenal() {
    let input = r#"
        misalkan hasil = coba {
            variabel_tidak_ada;
        } tangkap (err) {
            "aman: " + err;
        };
        hasil;
    "#;
    let result = test_eval(input);
    match result {
        Object::Str(s) => assert!(
            s.contains("aman"),
            "pesan harus mengandung 'aman', diterima: {}", s
        ),
        _ => panic!("diharapkan TEKS, diterima {:?}", result),
    }
}

#[test]
fn test_coba_tangkap_error_variable_scope() {
    // Variabel `err` hanya tersedia di dalam blok tangkap
    let input = r#"
        coba {
            10 / 0;
        } tangkap (galat) {
            cetak(galat);
        };
    "#;
    // Ini harus berjalan tanpa crash
    let result = test_eval(input);
    assert!(!result.is_error(), "tidak seharusnya error: {:?}", result);
}

// ── Lemparkan (Throw) ───────────────────────────────

#[test]
fn test_lemparkan_dasar() {
    let input = r#"lemparkan "ada masalah";"#;
    let result = test_eval(input);
    match result {
        Object::Error(msg) => assert_eq!(msg, "ada masalah"),
        _ => panic!("diharapkan Error, diterima {:?}", result),
    }
}

#[test]
fn test_lemparkan_ditangkap_coba() {
    let input = r#"
        misalkan hasil = coba {
            lemparkan "galat kustom";
        } tangkap (err) {
            "tertangkap: " + err;
        };
        hasil;
    "#;
    let result = test_eval(input);
    match result {
        Object::Str(s) => assert_eq!(s, "tertangkap: galat kustom"),
        _ => panic!("diharapkan TEKS, diterima {:?}", result),
    }
}

#[test]
fn test_lemparkan_dari_fungsi() {
    let input = r#"
        misalkan bagi = fungsi(a, b) {
            jika (b == 0) {
                lemparkan "tidak boleh bagi nol";
            };
            kembalikan a / b;
        };
        misalkan hasil = coba {
            bagi(10, 0);
        } tangkap (err) {
            err;
        };
        hasil;
    "#;
    let result = test_eval(input);
    match result {
        Object::Str(s) => assert_eq!(s, "tidak boleh bagi nol"),
        _ => panic!("diharapkan TEKS, diterima {:?}", result),
    }
}

// ── Petakan (Map) ───────────────────────────────────

#[test]
fn test_petakan_dasar() {
    let input = "misalkan arr = petakan([1, 2, 3], (x) => x * 2); arr;";
    let result = test_eval(input);
    match result {
        Object::Array(arr) => {
            assert_eq!(arr.len(), 3);
            test_integer_object(&arr[0], 2);
            test_integer_object(&arr[1], 4);
            test_integer_object(&arr[2], 6);
        }
        _ => panic!("diharapkan DAFTAR, diterima {:?}", result),
    }
}

#[test]
fn test_petakan_error_bukan_daftar() {
    let input = r#"petakan("bukan array", (x) => x);"#;
    let result = test_eval(input);
    assert!(result.is_error(), "seharusnya error: {:?}", result);
}

// ── Saring (Filter) ────────────────────────────────

#[test]
fn test_saring_dasar() {
    let input = "misalkan arr = saring([1, 2, 3, 4, 5, 6], (x) => x % 2 == 0); arr;";
    let result = test_eval(input);
    match result {
        Object::Array(arr) => {
            assert_eq!(arr.len(), 3);
            test_integer_object(&arr[0], 2);
            test_integer_object(&arr[1], 4);
            test_integer_object(&arr[2], 6);
        }
        _ => panic!("diharapkan DAFTAR, diterima {:?}", result),
    }
}

#[test]
fn test_saring_error_bukan_daftar() {
    let input = "saring(42, (x) => x);";
    let result = test_eval(input);
    assert!(result.is_error(), "seharusnya error: {:?}", result);
}

#[test]
fn test_petakan_saring_gabungan() {
    // Ambil angka genap, lalu kalikan 10
    let input = "
        misalkan data = [1, 2, 3, 4, 5, 6];
        misalkan genap = saring(data, (x) => x % 2 == 0);
        misalkan hasil = petakan(genap, (x) => x * 10);
        hasil;
    ";
    let result = test_eval(input);
    match result {
        Object::Array(arr) => {
            assert_eq!(arr.len(), 3);
            test_integer_object(&arr[0], 20);
            test_integer_object(&arr[1], 40);
            test_integer_object(&arr[2], 60);
        }
        _ => panic!("diharapkan DAFTAR, diterima {:?}", result),
    }
}

// ── Format ──────────────────────────────────────────

#[test]
fn test_format_dasar() {
    let input = r#"format("Halo, {}! Usia: {}", "Dika", 20);"#;
    let result = test_eval(input);
    match result {
        Object::Str(s) => assert_eq!(s, "Halo, Dika! Usia: 20"),
        _ => panic!("diharapkan TEKS, diterima {:?}", result),
    }
}

#[test]
fn test_format_tanpa_placeholder() {
    let input = r#"format("Tidak ada placeholder");"#;
    let result = test_eval(input);
    match result {
        Object::Str(s) => assert_eq!(s, "Tidak ada placeholder"),
        _ => panic!("diharapkan TEKS, diterima {:?}", result),
    }
}

#[test]
fn test_format_argumen_kurang() {
    let input = r#"format("A {} B {}", 1);"#;
    let result = test_eval(input);
    assert!(result.is_error(), "seharusnya error: {:?}", result);
}

// ── dari_json & ke_json ─────────────────────────────

#[test]
fn test_dari_json_objek() {
    let input = r#"
        misalkan data = dari_json("{\"nama\": \"Dika\", \"umur\": 20}");
        data["nama"];
    "#;
    let result = test_eval(input);
    match result {
        Object::Str(s) => assert_eq!(s, "Dika"),
        _ => panic!("diharapkan TEKS, diterima {:?}", result),
    }
}

#[test]
fn test_dari_json_array() {
    let input = r#"
        misalkan data = dari_json("[1, 2, 3]");
        panjang(data);
    "#;
    let result = test_eval(input);
    test_integer_object(&result, 3);
}

#[test]
fn test_ke_json_rapat() {
    let input = r#"
        misalkan data = {"nama": "Taji", "versi": 4};
        ke_json(data);
    "#;
    let result = test_eval(input);
    match result {
        Object::Str(s) => {
            assert!(s.contains("nama"));
            assert!(s.contains("Taji"));
            assert!(!s.contains('\n'), "mode rapat tidak boleh ada newline");
        }
        _ => panic!("diharapkan TEKS, diterima {:?}", result),
    }
}

#[test]
fn test_ke_json_rapi() {
    let input = r#"
        misalkan data = {"nama": "Taji"};
        ke_json(data, benar);
    "#;
    let result = test_eval(input);
    match result {
        Object::Str(s) => {
            assert!(s.contains('\n'), "mode rapi harus ada newline");
            assert!(s.contains("nama"));
        }
        _ => panic!("diharapkan TEKS, diterima {:?}", result),
    }
}
