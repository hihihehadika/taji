//! Modul Galat untuk Taji VM (TVM).

use crate::object::Object;
use std::fmt;

#[derive(Debug, Clone)]
pub enum GalatVM {
    StackLuapan,
    StackKosong,
    StackFrameKosong,
    StackFramePenuh,
    PembagianDenganNol,
    OpCodeTidakDikenal(u8),
    OpCodeBelumDiimplementasikan(String),
    TipeOperanTidakValid(String),
    SimbolTidakTerdefinisi(String),
    JumlahArgumenSalah { diharapkan: usize, diterima: usize },
    AksesIndeksGagal(String),
    IndeksDiLuarBatas(usize),
    KunciKamusTidakDitemukan,
    GalatDilempar(Object),
}

impl fmt::Display for GalatVM {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GalatVM::StackLuapan => write!(f, "Stack VM meluap"),
            GalatVM::StackKosong => write!(f, "Stack VM kosong"),
            GalatVM::StackFrameKosong => write!(f, "Frame eksekusi kosong"),
            GalatVM::StackFramePenuh => {
                write!(f, "Terlalu banyak pemanggilan fungsi (frame penuh)")
            }
            GalatVM::PembagianDenganNol => write!(f, "Pembagian dengan nol"),
            GalatVM::OpCodeTidakDikenal(op) => write!(f, "OpCode tidak dikenal: 0x{:02X}", op),
            GalatVM::OpCodeBelumDiimplementasikan(s) => write!(f, "Instruksi belum siap: {}", s),
            GalatVM::TipeOperanTidakValid(s) => write!(f, "Tipe operan tidak valid: {}", s),
            GalatVM::SimbolTidakTerdefinisi(s) => write!(f, "Simbol tidak dikenal: {}", s),
            GalatVM::JumlahArgumenSalah {
                diharapkan,
                diterima,
            } => {
                write!(
                    f,
                    "Argumen salah: diharapkan {}, diterima {}",
                    diharapkan, diterima
                )
            }
            GalatVM::AksesIndeksGagal(s) => write!(f, "Gagal mengakses indeks: {}", s),
            GalatVM::IndeksDiLuarBatas(i) => write!(f, "Indeks di luar batas: {}", i),
            GalatVM::KunciKamusTidakDitemukan => write!(f, "Kunci tidak ditemukan di Kamus"),
            GalatVM::GalatDilempar(obj) => write!(f, "{}", obj),
        }
    }
}

impl std::error::Error for GalatVM {}
