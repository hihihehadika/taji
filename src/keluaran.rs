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
    BUFFER_KELUARAN.with(|buf| {
        buf.borrow_mut()
            .take()
            .unwrap_or_default()
    })
}
