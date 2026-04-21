/// Entry point untuk bahasa pemrograman Taji (.tj).
///
/// Mendukung tiga mode operasi:
/// - **Mode Interaktif (REPL):** Jalankan `taji` tanpa argumen.
/// - **Mode Berkas:** Jalankan `taji script.tj` untuk mengeksekusi berkas.
/// - **Mode TPM:** Jalankan `taji pasang <URL>` untuk memasang modul eksternal.
use std::env;
use std::fs;
use std::io;
use std::process;

use taji::compiler::Kompilator;
use taji::lexer::Lexer;
use taji::parser::Parser;
use taji::repl;
use taji::tpm;
use taji::vm::VM;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        // Tanpa argumen → masuk mode REPL interaktif
        1 => {
            repl::start(io::stdin().lock(), io::stdout().lock());
        }

        // Satu argumen → jalankan berkas .tj
        2 => {
            let argumen = &args[1];
            // Periksa jika pengguna hanya mengetik 'taji pasang' tanpa URL
            if argumen == "pasang" {
                eprintln!("tpm: URL modul tidak disertakan.");
                eprintln!();
                tpm::tampilkan_bantuan_tpm();
                process::exit(1);
            }
            run_file(argumen);
        }

        // Dua argumen → bisa jadi perintah TPM atau salah penggunaan
        3 => {
            let perintah = &args[1];
            let nilai = &args[2];

            if perintah == "pasang" {
                // Mode TPM: unduh dan pasang modul dari URL
                match tpm::pasang_modul(nilai) {
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!("KESALAHAN TPM: {}", e);
                        process::exit(1);
                    }
                }
            } else {
                eprintln!("Perintah tidak dikenal: '{} {}'", perintah, nilai);
                eprintln!();
                tampilkan_bantuan();
                process::exit(1);
            }
        }

        // Argumen tidak valid
        _ => {
            tampilkan_bantuan();
            process::exit(1);
        }
    }
}

/// Menampilkan panduan penggunaan ke terminal.
fn tampilkan_bantuan() {
    eprintln!("Penggunaan:");
    eprintln!("  taji                     Masuk mode interaktif (REPL)");
    eprintln!("  taji <berkas.tj>          Jalankan berkas skrip Taji");
    eprintln!("  taji pasang <URL>         Unduh dan pasang modul eksternal");
}

/// Membaca dan mengeksekusi berkas kode Taji (.tj).
fn run_file(filename: &str) {
    // Baca isi file
    let isi = match fs::read_to_string(filename) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Gagal membaca berkas '{}': {}", filename, e);
            process::exit(1);
        }
    };

    // Lexing
    let lexer = Lexer::new(&isi);

    // Penguraian (Parsing)
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    if !parser.errors.is_empty() {
        eprintln!("Ditemukan {} kesalahan penguraian:", parser.errors.len());
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
