//! Modul REPL (Read-Eval-Print Loop) untuk bahasa Taji.
//!
//! ## Strategi Eksekusi (Hybrid Mode - Fase 3 & 4)
//!
//! Pipeline dua jalur:
//! 1. Kompilasi ke bytecode → jalankan di TVM (dengan state persisten).
//! 2. Jika sintaksis belum didukung VM → fallback ke evaluator tree-walking.
//!
//! State VM yang persisten lintas baris:
//! - `tabel_simbol_vm`: simbol variabel yang sudah dideklarasikan di TVM.
//! - `globals_vm`: nilai variabel global yang tersimpan di slot numerik.
//!
//! State evaluator yang persisten:
//! - `env`: lingkungan tree-walking lama (untuk fitur yang belum di-VM).

use crate::compiler::galat::GalatKompilasi;
use crate::compiler::Kompilator;
use crate::evaluator;
use crate::lexer::Lexer;
use crate::object::{Lingkungan, Object};
use crate::parser::Parser;
use crate::vm::VM;
use std::io;

const PROMPT: &str = "taji >> ";

const BANNER: &str = r#"
  ======================================================
        TAJI - Bahasa Pemrograman Indonesia
        Versi 0.5.0 [TVM-hybrid]
        Ketik 'keluar' untuk berhenti.
  ======================================================
"#;

pub fn start<R, W>(mut input: R, mut output: W)
where
    R: io::BufRead,
    W: io::Write,
{
    let _ = writeln!(output, "{}", BANNER);

    // ---- State Evaluator (fallback) ----
    let mut env = Lingkungan::new();

    // ---- State VM (persisten lintas baris) ----
    let mut tabel_simbol_vm = crate::bawaan::bikin_tabel_awal();
    let mut globals_vm: Vec<Object> = crate::bawaan::bikin_globals_awal();
    let mut konstanta_vm: Vec<Object> = Vec::new();

    loop {
        let _ = write!(output, "{}", PROMPT);
        let _ = output.flush();

        let mut line = String::new();
        match input.read_line(&mut line) {
            Ok(0) | Err(_) => return,
            Ok(_) => {}
        }

        let trimmed = line.trim();
        if trimmed == "keluar" || trimmed == "keluar()" {
            break;
        }

        if trimmed.is_empty() {
            continue;
        }

        let lexer = Lexer::new(trimmed);
        let mut p = Parser::new(lexer);
        let program = p.parse_program();

        if !p.errors.is_empty() {
            let _ = writeln!(output, "  Ditemukan kesalahan:");
            for err in &p.errors {
                let _ = writeln!(output, "    → {}", err);
            }
            continue;
        }

        // ---- Coba jalur VM ----
        let mut kompilator =
            Kompilator::new_dengan_state(tabel_simbol_vm.clone(), konstanta_vm.clone());
        match kompilator.kompilasi(&program) {
            Ok(hasil_kompilasi) => {
                // Perbarui tabel simbol dari hasil kompilasi
                tabel_simbol_vm = hasil_kompilasi.tabel_simbol.clone();

                let mut vm = VM::new_dengan_globals(hasil_kompilasi, globals_vm.clone());
                match vm.jalankan() {
                    Ok(nilai) => {
                        match nilai {
                            Object::Null => {}
                            _ => {
                                let _ = writeln!(output, "  → {}", nilai);
                            }
                        }
                        // Simpan globals dan konstanta yang diperbarui
                        konstanta_vm = vm.ambil_konstanta();
                        globals_vm = vm.ambil_globals();
                    }
                    Err(e) => {
                        let _ = writeln!(output, "  KESALAHAN VM: {}", e);
                        // Rollback tabel simbol dan konstanta agar tidak korup
                        tabel_simbol_vm = crate::bawaan::bikin_tabel_awal();
                        globals_vm = crate::bawaan::bikin_globals_awal();
                        konstanta_vm = Vec::new();
                    }
                }
            }

            // Sintaksis belum didukung VM → fallback ke evaluator
            Err(GalatKompilasi::SintaksisBelumdidukung(_)) => {
                let result = evaluator::eval(&program, &mut env);
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

            Err(e) => {
                let _ = writeln!(output, "  KESALAHAN KOMPILASI: {}", e);
            }
        }
    }
}
