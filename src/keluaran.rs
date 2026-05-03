//! Modul `keluaran`: Virtualisasi Standar Output.
//!
//! Menyediakan mekanisme penangkapan output (I/O Hijacking) yang
//! memungkinkan fungsi `cetak()` dan opcode `OpCetak` mengarahkan
//! teks ke buffer internal alih-alih stdout. Ini krusial untuk
//! kompilasi WebAssembly di mana stdout tidak tersedia.

use std::cell::RefCell;

thread_local! {
    /// Buffer keluaran thread-local. Ketika diisi `Some`, semua output
    /// ditangkap ke dalam vektor. Ketika `None`, output dicetak langsung
    /// ke stdout seperti biasa.
    static BUFFER_KELUARAN: RefCell<Option<Vec<String>>> = const { RefCell::new(None) };
}

/// Mengarahkan teks ke buffer jika aktif, atau ke stdout jika tidak.
/// Fungsi ini menggantikan semua pemanggilan `println!` untuk output
/// yang dihasilkan oleh kode Taji.
pub fn cetak_keluar(teks: &str) {
    BUFFER_KELUARAN.with(|buf| {
        let mut b = buf.borrow_mut();
        if let Some(ref mut vec) = *b {
            vec.push(teks.to_string());
        } else {
            println!("{}", teks);
        }
    });
}

/// Mengaktifkan mode penangkapan output. Semua pemanggilan `cetak_keluar`
/// setelah ini akan menyimpan teks ke buffer internal.
pub fn aktifkan_buffer() {
    BUFFER_KELUARAN.with(|buf| {
        *buf.borrow_mut() = Some(Vec::new());
    });
}

/// Mengambil seluruh isi buffer dan menonaktifkan mode penangkapan.
/// Mengembalikan semua baris output yang tertangkap sejak `aktifkan_buffer`.
pub fn ambil_dan_bersihkan_buffer() -> Vec<String> {
    BUFFER_KELUARAN.with(|buf| buf.borrow_mut().take().unwrap_or_default())
}

/// Menghasilkan string pesan galat dengan cuplikan kode sumber dan penunjuk kolom (^ marker).
#[allow(clippy::too_many_arguments)]
pub fn format_galat_dengan_cuplikan(
    jenis: &str,
    pesan: &str,
    filename: &str,
    isi: &str,
    baris: usize,
    kolom: usize,
    panjang_sorot: usize,
    saran: Option<String>,
    jejak: Vec<String>,
) -> String {
    use crate::warna::{warnai, BIRU, CYAN, HIJAU, KUNING, MERAH, PUTIH};
    let mut hasil = String::new();
    hasil.push_str(&format!("{}:\n", warnai(jenis, MERAH)));
    if baris > 0 {
        let lokasi = warnai(&format!("{}:{}:{}", filename, baris, kolom), CYAN);
        hasil.push_str(&format!("  --> {}\n", lokasi));
        hasil.push_str(&format!("   {}\n", warnai("|", BIRU)));
        if let Some(baris_kode) = isi.lines().nth(baris.saturating_sub(1)) {
            let nomor = format!("{:>2}", baris);
            hasil.push_str(&format!(
                " {} {} {}\n",
                warnai(&nomor, CYAN),
                warnai("|", BIRU),
                baris_kode
            ));
            let spasi_kolom = " ".repeat(kolom.saturating_sub(1));
            let sorot = warnai(&"^".repeat(panjang_sorot.max(1)), KUNING);
            hasil.push_str(&format!(
                "    {} {}{}\n",
                warnai("|", BIRU),
                spasi_kolom,
                sorot
            ));
        }
        hasil.push_str(&format!("   {}\n", warnai("|", BIRU)));
    }
    hasil.push_str(&format!("   = {}", pesan));
    if let Some(s) = saran {
        hasil.push_str(&format!(
            "\n   = {} Apakah maksudmu '{}'?",
            warnai("BANTUAN:", HIJAU),
            s
        ));
    }

    if !jejak.is_empty() {
        hasil.push_str(&format!(
            "\n\n{}",
            warnai("Jejak Pemanggilan (paling baru terakhir):", PUTIH)
        ));
        for item in jejak {
            hasil.push_str(&format!("\n  - {}", item));
        }
    }
    hasil
}
