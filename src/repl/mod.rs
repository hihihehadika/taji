//! Modul REPL (Read-Eval-Print Loop) untuk bahasa Taji.
//!
//! Menyediakan antarmuka interaktif di terminal di mana pengguna
//! bisa mengetik kode Taji dan langsung melihat hasil eksekusinya.

use crate::evaluator;
use crate::lexer::Lexer;
use crate::object::{Lingkungan, Object};
use crate::parser::Parser;
use std::io;

const PROMPT: &str = "taji >> ";

const BANNER: &str = r#"
  ======================================================
        TAJI - Bahasa Pemrograman Indonesia         
        Versi 0.4.0                                   
        Ketik 'keluar' untuk berhenti.                
  ======================================================
"#;

/// Memulai sesi REPL interaktif.
///
/// Membaca input baris demi baris, mem-parse dan mengevaluasi
/// kode Taji, lalu menampilkan hasil ke output.
pub fn start<R, W>(mut input: R, mut output: W)
where
    R: io::BufRead,
    W: io::Write,
{
    let _ = writeln!(output, "{}", BANNER);
    let mut env = Lingkungan::new();

    loop {
        let _ = write!(output, "{}", PROMPT);
        let _ = output.flush();

        let mut line = String::new();
        match input.read_line(&mut line) {
            Ok(0) => return,
            Err(_) => return,
            Ok(_) => {}
        }

        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        // Perintah khusus: keluar
        if trimmed == "keluar" {
            let _ = writeln!(output, "  Sampai jumpa!");
            return;
        }

        let l = Lexer::new(trimmed);
        let mut p = Parser::new(l);
        let program = p.parse_program();

        // Tampilkan error parsing jika ada
        if !p.errors.is_empty() {
            let _ = writeln!(output, "  Ditemukan kesalahan:");
            for err in &p.errors {
                let _ = writeln!(output, "    → {}", err);
            }
            continue;
        }

        // Evaluasi program
        let result = evaluator::eval(&program, &mut env);

        // Tampilkan hasil (tapi jangan tampilkan Null)
        match &result {
            Object::Null => {}
            Object::Error(msg) => {
                let _ = writeln!(output, "  KESALAHAN: {}", msg);
            }
            _ => {
                let _ = writeln!(output, "  → {}", result);
            }
        }
    }
}
