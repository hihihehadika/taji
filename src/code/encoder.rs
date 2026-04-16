//! Encoder dan Disassembler untuk bytecode Taji VM.
//!
//! `encode` menghasilkan representasi biner dari satu instruksi.
//! `decode_satu` membaca satu instruksi dari offset tertentu dalam stream.
//! `disassemble` mencetak representasi teks dari seluruh stream bytecode
//! (dipakai untuk debugging dan verifikasi kompilasi).

use super::definisi::{OpCode, DEFINISI_OPCODE};
use super::Bytecode;

// ======================================================================== //
// ENCODING
// ======================================================================== //

/// Encode satu instruksi menjadi byte-sequence dan append ke `target`.
///
/// - `op`      : Opcode yang akan di-encode.
/// - `operand` : Slice nilai operand. Setiap elemen di-encode sesuai lebar
///   yang didefinisikan di `DEFINISI_OPCODE`. Operand 2-byte
///   di-encode sebagai big-endian u16.
///
/// Mengembalikan offset awal instruksi ini di dalam `target` (sebelum append).
///
/// # Panic
/// Panic jika jumlah elemen `operand` tidak sesuai dengan `lebar_operand`
/// yang terdefinisi untuk opcode tersebut. Ini adalah bug programmer, bukan
/// runtime error, sehingga panic adalah perilaku yang benar.
pub fn encode(target: &mut Bytecode, op: OpCode, operand: &[usize]) -> usize {
    let definisi = cari_definisi(op)
        .unwrap_or_else(|| panic!("BUG: opcode {:?} tidak terdaftar di DEFINISI_OPCODE", op));

    assert_eq!(
        operand.len(),
        definisi.lebar_operand.len(),
        "BUG: opcode {:?} membutuhkan {} operand, diberikan {}",
        op,
        definisi.lebar_operand.len(),
        operand.len()
    );

    let offset_awal = target.len();
    target.push(op as u8);

    for (nilai, &lebar) in operand.iter().zip(definisi.lebar_operand.iter()) {
        match lebar {
            1 => {
                target.push(*nilai as u8);
            }
            2 => {
                // Big-endian u16
                target.push((*nilai >> 8) as u8);
                target.push(*nilai as u8);
            }
            l => panic!("BUG: lebar operand {} tidak didukung encoder", l),
        }
    }

    offset_awal
}

/// Tulis ulang operand 2-byte di offset yang sudah ada dalam stream.
/// Dipakai untuk *backpatching* instruksi lompat.
///
/// # Panic
/// Panic jika `offset` berada di luar batas `target`.
pub fn tulis_operand_u16(target: &mut Bytecode, offset: usize, nilai: u16) {
    target[offset] = (nilai >> 8) as u8;
    target[offset + 1] = nilai as u8;
}

// ======================================================================== //
// DECODING
// ======================================================================== //

/// Satu instruksi yang sudah terdekode dari stream.
pub struct InstruksiTerdekode {
    pub op: OpCode,
    /// Nilai masing-masing operand (sudah dikonversi ke `usize`).
    pub operand: Vec<usize>,
    /// Total lebar instruksi dalam byte (1 opcode + semua operand).
    pub lebar: usize,
}

/// Baca satu instruksi dari `bytecode` mulai di `offset`.
/// Mengembalikan `None` jika offset berada di luar batas atau opcode tidak dikenal.
pub fn decode_satu(bytecode: &[u8], offset: usize) -> Option<InstruksiTerdekode> {
    let byte_op = *bytecode.get(offset)?;
    let op = OpCode::try_from(byte_op).ok()?;
    let definisi = cari_definisi(op)?;

    let mut operand = Vec::with_capacity(definisi.lebar_operand.len());
    let mut kursor = offset + 1;

    for &lebar in definisi.lebar_operand {
        let nilai = match lebar {
            1 => {
                let b = *bytecode.get(kursor)? as usize;
                kursor += 1;
                b
            }
            2 => {
                let hi = *bytecode.get(kursor)? as usize;
                let lo = *bytecode.get(kursor + 1)? as usize;
                kursor += 2;
                (hi << 8) | lo
            }
            _ => return None,
        };
        operand.push(nilai);
    }

    Some(InstruksiTerdekode {
        op,
        operand,
        lebar: kursor - offset,
    })
}

// ======================================================================== //
// DISASSEMBLER
// ======================================================================== //

/// Cetak representasi human-readable dari seluruh `bytecode` ke `String`.
/// Format: `OFFSET  OPCODE_NAME  [OPERAND...]`
///
/// Dipakai oleh test, debugger, dan mode `--dump-bytecode` di CLI.
pub fn disassemble(nama: &str, bytecode: &[u8]) -> String {
    let mut hasil = format!("== {} ==\n", nama);
    let mut offset = 0;

    while offset < bytecode.len() {
        match decode_satu(bytecode, offset) {
            Some(instruksi) => {
                let definisi = cari_definisi(instruksi.op)
                    .expect("decode_satu berhasil tapi definisi tidak ditemukan - ini bug");

                // Format: "0000  OpTambah"  atau "0000  OpTulisPuncak  0001"
                let operand_str: Vec<String> = instruksi
                    .operand
                    .iter()
                    .map(|v| format!("{:04}", v))
                    .collect();

                if operand_str.is_empty() {
                    hasil.push_str(&format!("{:04}  {}\n", offset, definisi.nama));
                } else {
                    hasil.push_str(&format!(
                        "{:04}  {}  {}\n",
                        offset,
                        definisi.nama,
                        operand_str.join("  ")
                    ));
                }

                offset += instruksi.lebar;
            }
            None => {
                // Byte tidak dikenal - cetak mentah lalu maju 1 byte
                hasil.push_str(&format!(
                    "{:04}  ERROR: byte tidak dikenal 0x{:02X}\n",
                    offset, bytecode[offset]
                ));
                offset += 1;
            }
        }
    }

    hasil
}

// ======================================================================== //
// INTERNAL HELPERS
// ======================================================================== //

fn cari_definisi(op: OpCode) -> Option<&'static super::definisi::DefinisiOpCode> {
    DEFINISI_OPCODE
        .iter()
        .find(|(kode, _)| *kode == op)
        .map(|(_, def)| def)
}

// ======================================================================== //
// TES UNIT
// ======================================================================== //

#[cfg(test)]
mod tes {
    use super::*;
    use crate::code::definisi::OpCode;

    #[test]
    fn tes_encode_optulis_puncak() {
        let mut bytecode = Bytecode::new();
        let offset = encode(&mut bytecode, OpCode::OpTulisPuncak, &[65534]);

        assert_eq!(offset, 0);
        // OpTulisPuncak = 0x00, 65534 big-endian = [0xFF, 0xFE]
        assert_eq!(bytecode, vec![0x00, 0xFF, 0xFE]);
    }

    #[test]
    fn tes_encode_optambah_tanpa_operand() {
        let mut bytecode = Bytecode::new();
        encode(&mut bytecode, OpCode::OpTambah, &[]);

        assert_eq!(bytecode, vec![0x10]);
    }

    #[test]
    fn tes_decode_satu_instruksi() {
        let mut bytecode = Bytecode::new();
        encode(&mut bytecode, OpCode::OpTulisPuncak, &[1]);
        encode(&mut bytecode, OpCode::OpTambah, &[]);

        let instruksi_0 = decode_satu(&bytecode, 0).expect("harus terdekode");
        assert_eq!(instruksi_0.op, OpCode::OpTulisPuncak);
        assert_eq!(instruksi_0.operand, vec![1]);
        assert_eq!(instruksi_0.lebar, 3); // 1 opcode + 2 byte operand

        let instruksi_1 = decode_satu(&bytecode, 3).expect("harus terdekode");
        assert_eq!(instruksi_1.op, OpCode::OpTambah);
        assert!(instruksi_1.operand.is_empty());
        assert_eq!(instruksi_1.lebar, 1);
    }

    #[test]
    fn tes_backpatch_operand_u16() {
        let mut bytecode = Bytecode::new();
        encode(&mut bytecode, OpCode::OpLompat, &[0x0000]); // placeholder dulu
                                                            // Simulasi backpatch: isi target lompat ke offset 42
        tulis_operand_u16(&mut bytecode, 1, 42);

        let instruksi = decode_satu(&bytecode, 0).unwrap();
        assert_eq!(instruksi.operand[0], 42);
    }

    #[test]
    fn tes_disassemble_output_format() {
        let mut bytecode = Bytecode::new();
        encode(&mut bytecode, OpCode::OpTulisPuncak, &[0]);
        encode(&mut bytecode, OpCode::OpTulisPuncak, &[1]);
        encode(&mut bytecode, OpCode::OpTambah, &[]);
        encode(&mut bytecode, OpCode::OpKembalikan, &[]);

        let output = disassemble("tes_sederhana", &bytecode);
        assert!(output.contains("OpTulisPuncak"));
        assert!(output.contains("OpTambah"));
        assert!(output.contains("OpKembalikan"));
    }
}
