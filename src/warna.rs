//! Modul Warna ANSI untuk penyorotan di terminal.
//!
//! Menyediakan konstanta dan fungsi untuk membungkus teks dengan warna
//! ANSI agar tampilan galat lebih profesional.

pub const MERAH: &str = "\x1b[31;1m";
pub const KUNING: &str = "\x1b[33;1m";
pub const BIRU: &str = "\x1b[34;1m";
pub const CYAN: &str = "\x1b[36;1m";
pub const HIJAU: &str = "\x1b[32;1m";
pub const PUTIH: &str = "\x1b[37;1m";
pub const RESET: &str = "\x1b[0m";

/// Membungkus teks dengan warna dan me-reset kembali setelahnya.
pub fn warnai(teks: &str, warna: &str) -> String {
    format!("{}{}{}", warna, teks, RESET)
}
