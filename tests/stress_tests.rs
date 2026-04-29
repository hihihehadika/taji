use taji_lib::compiler::Kompilator;
use taji_lib::lexer::Lexer;
use taji_lib::parser::Parser;
use taji_lib::vm::VM;

#[test]
fn test_stress_deeply_nested_expressions() {
    // Generate a very deep expression: (((...(1 + 1)...)))
    let depth = 500;
    let mut input = String::new();
    for _ in 0..depth {
        input.push('(');
    }
    input.push_str("1 + 1");
    for _ in 0..depth {
        input.push(')');
    }
    input.push(';');

    let lexer = Lexer::new(&input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    // Parser must handle this without stack overflow
    if !parser.errors.is_empty() {
        panic!("Parser errors on deep nesting: {:?}", parser.errors);
    }

    let tabel = taji_lib::bawaan::bikin_tabel_awal();
    let hasil = Kompilator::new_dengan_state(tabel, Vec::new())
        .kompilasi(&program)
        .expect("Kompilasi gagal");

    let mut vm = VM::new_dengan_globals(hasil, taji_lib::bawaan::bikin_globals_awal());
    vm.jalankan().expect("VM gagal");
}

#[test]
fn test_stress_massive_array_allocation() {
    // Build an array with 10,000 elements and sum them.
    // This tests heap allocation and cleanup.
    let input = "
        misalkan n = 10000;
        misalkan arr = [];
        misalkan i = 0;
        selama (i < n) {
            arr = arr + [i];
            i = i + 1;
        };
        panjang(arr);
    ";

    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    let hasil = Kompilator::new_dengan_state(taji_lib::bawaan::bikin_tabel_awal(), Vec::new())
        .kompilasi(&program)
        .expect("Kompilasi gagal");

    let mut vm = VM::new_dengan_globals(hasil, taji_lib::bawaan::bikin_globals_awal());
    vm.jalankan().expect("VM gagal");

    // Nilai terakhir di stack (panjang array)
    // Note: taji::vm::VM::terakhir_dibuang is private, but we can check if it finishes.
}

#[test]
fn test_stress_closure_leak_check() {
    // Note: Fase 7 added closures (OpAmbilUpvalue etc were mentioned as part of Fase 8+).
    // Let's test a simple high-frequency function call.
    let input = "
        misalkan buat_tambah = fungsi(x) {
            kembalikan fungsi(y) { x + y; };
        };
        misalkan i = 0;
        selama (i < 1000) {
            misalkan penambah = buat_tambah(i);
            penambah(5);
            i = i + 1;
        };
    ";

    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    let hasil = Kompilator::new_dengan_state(taji_lib::bawaan::bikin_tabel_awal(), Vec::new())
        .kompilasi(&program)
        .expect("Kompilasi gagal");

    let mut vm = VM::new_dengan_globals(hasil, taji_lib::bawaan::bikin_globals_awal());
    vm.jalankan().expect("VM gagal");
}
