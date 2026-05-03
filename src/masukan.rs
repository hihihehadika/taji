//! Modul `masukan`: Virtualisasi Standar Input.
//!
//! Menyediakan mekanisme antrian masukan (Input Queue) yang memungkinkan
//! fungsi `tanya()` membaca input dari buffer internal alih-alih
//! `window.prompt()` di peramban. Ini krusial untuk kompilasi WebAssembly
//! agar interaksi input-output terlihat natural di panel konsol web,
//! bukan melalui kotak dialog popup bawaan peramban.
//!
//! Arsitektur:
//! - Frontend TypeScript memecah teks area "Masukan Program" per baris
//!   dan mengirimkannya ke WASM melalui `atur_antrian()`.
//! - Saat `tanya()` dipanggil di dalam skrip Taji, VM mengambil (dequeue)
//!   baris pertama dari antrian ini.
//! - Jika antrian kosong, mengembalikan string kosong.

use std::cell::RefCell;

thread_local! {
    /// Buffer antrian masukan thread-local. Setiap elemen mewakili satu
    /// baris input yang akan dikonsumsi oleh pemanggilan `tanya()`.
    static ANTRIAN_MASUKAN: RefCell<Vec<String>> = const { RefCell::new(Vec::new()) };
}

/// Mengisi antrian masukan dengan daftar baris input.
/// Dipanggil oleh frontend sebelum eksekusi kode Taji dimulai.
/// Antrian sebelumnya akan ditimpa sepenuhnya.
pub fn atur_antrian(baris_input: Vec<String>) {
    ANTRIAN_MASUKAN.with(|antrian| {
        *antrian.borrow_mut() = baris_input;
    });
}

/// Mengambil (dequeue) satu baris dari depan antrian masukan.
/// Mengembalikan `Some(baris)` jika antrian masih berisi data,
/// atau `None` jika antrian sudah kosong.
pub fn ambil_masukan() -> Option<String> {
    ANTRIAN_MASUKAN.with(|antrian| {
        let mut a = antrian.borrow_mut();
        if a.is_empty() {
            None
        } else {
            Some(a.remove(0))
        }
    })
}

/// Mengosongkan seluruh antrian masukan.
/// Dipanggil setelah eksekusi selesai untuk membersihkan sisa antrian.
pub fn bersihkan_antrian() {
    ANTRIAN_MASUKAN.with(|antrian| {
        antrian.borrow_mut().clear();
    });
}
