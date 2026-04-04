/// Entry point untuk bahasa pemrograman Taji (.tj).
///
/// Mendukung dua mode operasi:
/// - **Mode Interaktif (REPL):** Jalankan `taji` tanpa argumen.
/// - **Mode File:** Jalankan `taji script.tj` untuk mengeksekusi file.

use std::env;
use std::fs;
use std::io;
use std::process;

use taji::evaluator;
use taji::lexer::Lexer;
use taji::object::Environment;
use taji::parser::Parser;
use taji::repl;

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
            eprintln!("❌ Gagal membaca file '{}': {}", filename, e);
            process::exit(1);
        }
    };

    // Lexing
    let lexer = Lexer::new(&content);

    // Parsing
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    if !parser.errors.is_empty() {
        eprintln!("⚠️  Ditemukan {} kesalahan parsing:", parser.errors.len());
        for err in &parser.errors {
            eprintln!("  → {}", err);
        }
        process::exit(1);
    }

    // Evaluasi
    let mut env = Environment::new();
    let result = evaluator::eval(&program, &mut env);

    // Tampilkan error jika ada
    if result.is_error() {
        eprintln!("❌ {}", result);
        process::exit(1);
    }
}
