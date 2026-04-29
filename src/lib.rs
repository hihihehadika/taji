//! Pustaka inti bahasa pemrograman Taji.
//!
//! Mengekspos modul token, lexer, parser, object, repl,
//! infrastruktur bytecode VM (code, compiler, vm), dan
//! Taji Package Manager (tpm) untuk digunakan oleh binary
//! utama maupun pengujian.

pub mod ast;
pub mod bawaan;
pub mod code;
pub mod compiler;
pub mod keluaran;
pub mod lexer;
pub mod object;
pub mod parser;
pub mod repl;
pub mod token;
pub mod tpm;
pub mod vm;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

/// Titik masuk utama untuk eksekusi kode Taji dari lingkungan WebAssembly.
///
/// Fungsi ini menerima kode sumber Taji sebagai string, menjalankan
/// seluruh pipeline (Lexer -> Parser -> Kompilator -> VM), dan
/// mengembalikan output sebagai string tunggal.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn jalankan_taji(kode_sumber: &str) -> String {
    // Aktifkan buffer penangkapan output
    keluaran::aktifkan_buffer();

    // Tahap 1: Analisis Leksikal
    let lexer = lexer::Lexer::new(kode_sumber);

    // Tahap 2: Analisis Sintaksis
    let mut parser = parser::Parser::new(lexer);
    let program = parser.parse_program();

    if !parser.errors.is_empty() {
        let galat = parser
            .errors
            .iter()
            .map(|e| format!("GALAT SINTAKS: {}", e))
            .collect::<Vec<_>>()
            .join("\n");
        keluaran::ambil_dan_bersihkan_buffer();
        return galat;
    }

    // Tahap 3: Kompilasi ke Bytecode
    let mut kompilator = compiler::Kompilator::new_dengan_state(
        bawaan::bikin_tabel_awal(),
        Vec::new(),
    );

    let hasil = match kompilator.kompilasi(&program) {
        Ok(h) => h,
        Err(e) => {
            keluaran::ambil_dan_bersihkan_buffer();
            return format!("GALAT KOMPILASI: {}", e);
        }
    };

    // Tahap 4: Eksekusi di Mesin Virtual
    let mut mesin = vm::VM::new_dengan_globals(hasil, bawaan::bikin_globals_awal());

    // Batasi instruksi untuk mencegah perulangan tak terbatas di browser
    mesin.batas_instruksi = Some(10_000_000);

    if let Err(e) = mesin.jalankan() {
        let buffer = keluaran::ambil_dan_bersihkan_buffer();
        let mut output = buffer.join("\n");
        if !output.is_empty() {
            output.push('\n');
        }
        output.push_str(&format!("GALAT VM: {}", e));
        return output;
    }

    // Kumpulkan semua output yang tertangkap
    let buffer = keluaran::ambil_dan_bersihkan_buffer();
    buffer.join("\n")
}

/// Versi non-WASM dari jalankan_taji untuk pengujian native.
/// Menggunakan mekanisme buffer yang sama sehingga perilakunya identik.
#[cfg(not(target_arch = "wasm32"))]
pub fn jalankan_taji(kode_sumber: &str) -> String {
    keluaran::aktifkan_buffer();

    let lexer = lexer::Lexer::new(kode_sumber);
    let mut parser = parser::Parser::new(lexer);
    let program = parser.parse_program();

    if !parser.errors.is_empty() {
        let galat = parser
            .errors
            .iter()
            .map(|e| format!("GALAT SINTAKS: {}", e))
            .collect::<Vec<_>>()
            .join("\n");
        keluaran::ambil_dan_bersihkan_buffer();
        return galat;
    }

    let mut kompilator = compiler::Kompilator::new_dengan_state(
        bawaan::bikin_tabel_awal(),
        Vec::new(),
    );

    let hasil = match kompilator.kompilasi(&program) {
        Ok(h) => h,
        Err(e) => {
            keluaran::ambil_dan_bersihkan_buffer();
            return format!("GALAT KOMPILASI: {}", e);
        }
    };

    let mut mesin = vm::VM::new_dengan_globals(hasil, bawaan::bikin_globals_awal());

    if let Err(e) = mesin.jalankan() {
        let buffer = keluaran::ambil_dan_bersihkan_buffer();
        let mut output = buffer.join("\n");
        if !output.is_empty() {
            output.push('\n');
        }
        output.push_str(&format!("GALAT VM: {}", e));
        return output;
    }

    let buffer = keluaran::ambil_dan_bersihkan_buffer();
    buffer.join("\n")
}
