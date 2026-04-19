//! Tipe galat kompilasi Taji.

use std::fmt;

/// Semua kemungkinan kegagalan yang bisa terjadi saat kompilasi AST ke bytecode.
#[derive(Debug, Clone, PartialEq)]
pub enum GalatKompilasi {
    /// Operator yang ditemukan tidak dikenal oleh kompilator.
    OperatorTidakDikenal(String),

    /// Invariant internal kompilator dilanggar (bug programmer, bukan user).
    InvarianDilanggar(String),

    /// Referensi ke variabel yang belum dideklarasikan dengan `misalkan`.
    SimbolTidakTerdefinisi(String),

    /// Ukuran program (bytecode/konstanta/variabel) melampaui batas 16-bit (65535).
    /// Tanpa guard ini, konversi `as u16` akan mem-truncate nilai secara diam-diam
    /// dan menyebabkan VM melompat ke alamat yang salah.
    BatasanTerlampaui(String),

    /// `berhenti` atau `lanjut` ditemukan di dalam blok `jika`/`lainnya` yang
    /// tidak berada di dalam loop. Ini akan mengakibatkan stack leak karena
    /// nilai yang sudah dipush tidak sempat di-pop sebelum lompatan paksa terjadi.
    BerhentiBukanDiLoop(String),

    /// File modul yang di-`masukkan` tidak ditemukan di sistem berkas.
    ModulTidakDitemukan(String),

    /// Terdeteksi siklus impor (A masukkan B, B masukkan A).
    /// Mencegah infinite recursion di kompilator.
    ModulSirkular(String),
}

impl fmt::Display for GalatKompilasi {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GalatKompilasi::OperatorTidakDikenal(op) => {
                write!(f, "GalatKompilasi: operator tidak dikenal '{}'", op)
            }
            GalatKompilasi::InvarianDilanggar(msg) => {
                write!(f, "GalatKompilasi: invarian dilanggar - {}", msg)
            }
            GalatKompilasi::SimbolTidakTerdefinisi(nama) => {
                write!(f, "GalatKompilasi: simbol '{}' belum dideklarasikan", nama)
            }
            GalatKompilasi::BatasanTerlampaui(msg) => {
                write!(f, "GalatKompilasi: batasan 16-bit terlampaui - {}", msg)
            }
            GalatKompilasi::BerhentiBukanDiLoop(msg) => {
                write!(f, "GalatKompilasi: {}", msg)
            }
            GalatKompilasi::ModulTidakDitemukan(path) => {
                write!(f, "GalatKompilasi: modul '{}' tidak ditemukan", path)
            }
            GalatKompilasi::ModulSirkular(path) => {
                write!(
                    f,
                    "GalatKompilasi: siklus impor terdeteksi untuk modul '{}'",
                    path
                )
            }
        }
    }
}

impl std::error::Error for GalatKompilasi {}
