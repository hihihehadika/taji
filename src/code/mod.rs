//! Modul `code`: Definisi bahasa mesin internal Taji VM (TVM).
//!
//! Modul ini mendefinisikan seluruh kode operasi (`OpCode`) yang dimengerti
//! oleh Taji Virtual Machine, beserta tipe `Bytecode` sebagai representasi
//! aliran instruksi dalam memori.

pub mod definisi;
pub mod encoder;

/// Aliran bytecode mentah. Satu unit kompilasi (fungsi, blok, atau program
/// utama) direpresentasikan sebagai `Vec<u8>`.
pub type Bytecode = Vec<u8>;
