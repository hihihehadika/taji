/// Entry point untuk bahasa pemrograman Taji (.tj).
///
/// Mendukung dua mode operasi:
/// - **Mode Interaktif (REPL):** Jalankan `taji` tanpa argumen.
/// - **Mode File:** Jalankan `taji script.tj` untuk mengeksekusi file.
use std::env;
use std::fs;
use std::io;
use std::process;

use taji::compiler::Kompilator;
use taji::lexer::Lexer;
use taji::parser::Parser;
use taji::repl;
use taji::vm::VM;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        // Tanpa argumen → masuk mode REPL interaktif
        1 => {
            repl::start(io::stdin().lock(), io::stdout().lock());
        }

        // Satu argumen → jalankan file .tj
        2 => {
            let filename = &args[1];
            run_file(filename);
        }

        // Argumen tidak valid
        _ => {
            eprintln!("Penggunaan:");
            eprintln!("  taji           → Mode interaktif (REPL)");
            eprintln!("  taji <file.tj> → Jalankan file script");
            process::exit(1);
        }
    }
}

/// Membaca dan mengeksekusi file kode Taji (.tj).
fn run_file(filename: &str) {
    // Baca isi file
    let content = match fs::read_to_string(filename) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Gagal membaca file '{}': {}", filename, e);
            process::exit(1);
        }
    };

    // Lexing
    let lexer = Lexer::new(&content);

    // Parsing
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    if !parser.errors.is_empty() {
        eprintln!("Ditemukan {} kesalahan parsing:", parser.errors.len());
        for err in &parser.errors {
            eprintln!("  -> {}", err);
        }
        process::exit(1);
    }

    // Kompilasi ke bytecode
    let tabel = taji::bawaan::bikin_tabel_awal();
    let mut kompilator = Kompilator::new_dengan_state(tabel, Vec::new());
    let hasil = match kompilator.kompilasi(&program) {
        Ok(h) => h,
        Err(e) => {
            eprintln!("KESALAHAN KOMPILASI: {}", e);
            process::exit(1);
        }
    };

    // Eksekusi di VM
    let globals = taji::bawaan::bikin_globals_awal();
    let mut vm = VM::new_dengan_globals(hasil, globals);
    match vm.jalankan() {
        Ok(_) => {}
        Err(e) => {
            eprintln!("KESALAHAN VM: {}", e);
            process::exit(1);
        }
    }
}
