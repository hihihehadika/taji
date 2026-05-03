//! Modul REPL (Read-Eval-Print Loop) untuk bahasa Taji.
//!
//! ## Strategi Eksekusi (100% Bytecode VM)
//!
//! Pipeline tunggal: Lexer -> Parser -> Kompilator -> VM.
//! Tidak ada fallback ke evaluator tree-walking.
//!
//! State VM yang persisten lintas baris:
//! - `tabel_simbol_vm`: simbol variabel yang sudah dideklarasikan di TVM.
//! - `globals_vm`: nilai variabel global yang tersimpan di slot numerik.

use crate::compiler::Kompilator;
use crate::lexer::Lexer;
use crate::object::Object;
use crate::parser::Parser;
use crate::vm::VM;
use std::io;

const PROMPT: &str = "taji >> ";

const BANNER: &str = r#"
  ======================================================
        TAJI - Bahasa Pemrograman Indonesia
        Versi 1.1.1 [TVM-murni]
        Ketik 'keluar' untuk berhenti.
  ======================================================
"#;

pub fn start<R, W>(mut input: R, mut output: W)
where
    R: io::BufRead,
    W: io::Write,
{
    let _ = writeln!(output, "{}", BANNER);

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
                let _ = writeln!(output, "    -> {}", err);
            }
            continue;
        }

        // ---- Jalur VM (absolut, tanpa fallback) ----
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
                                let _ = writeln!(output, "  -> {}", nilai);
                            }
                        }
                        // Simpan globals dan konstanta yang diperbarui
                        konstanta_vm = vm.ambil_konstanta();
                        globals_vm = vm.ambil_globals();
                    }
                    Err(e) => {
                        use crate::vm::galat::GalatVM;
                        match &e {
                            GalatVM::DenganBaris(info) => {
                                let msg = crate::keluaran::format_galat_dengan_cuplikan(
                                    "KESALAHAN RUNTIME",
                                    &info.sumber.to_string(),
                                    "<repl>",
                                    trimmed,
                                    info.baris,
                                    info.kolom,
                                    info.panjang,
                                    None,
                                    info.jejak.clone(),
                                );
                                let _ = write!(output, "{}", msg);
                            }
                            _ => {
                                let _ = writeln!(output, "  KESALAHAN VM:\n  = {}", e);
                            }
                        }
                        // Rollback tabel simbol dan konstanta agar tidak korup
                        tabel_simbol_vm = crate::bawaan::bikin_tabel_awal();
                        globals_vm = crate::bawaan::bikin_globals_awal();
                        konstanta_vm = Vec::new();
                    }
                }
            }

            Err(e) => {
                use crate::compiler::galat::GalatKompilasi;
                match &e {
                    GalatKompilasi::SimbolTidakTerdefinisi(nama, baris, kolom, saran) => {
                        let msg = crate::keluaran::format_galat_dengan_cuplikan(
                            "KESALAHAN KOMPILASI",
                            &format!("simbol '{}' belum dideklarasikan", nama),
                            "<repl>",
                            trimmed,
                            *baris,
                            *kolom,
                            nama.len(),
                            saran.clone(),
                            vec![],
                        );
                        let _ = write!(output, "{}", msg);
                    }
                    _ => {
                        let _ = writeln!(output, "  KESALAHAN KOMPILASI:\n  = {}", e);
                    }
                }
            }
        }
    }
}
