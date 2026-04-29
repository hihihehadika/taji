//! Definisi semua Kode Operasi (`OpCode`) yang dikenali Taji VM.
//!
//! Setiap varian diberi nilai diskriminan `u8` eksplisit untuk menjamin
//! stabilitas representasi biner. Penambahan opcode baru HARUS ditempatkan
//! sebelum baris `// -- SENTINEL --` dan TIDAK boleh mengubah nilai opcode
//! yang sudah ada (akan merusak bytecode yang sudah terkompilasi).
//!
//! Format instruksi dalam stream bytecode:
//!   [OpCode: u8] [operand_0: u8] [operand_1: u8] ...
//!
//! Jumlah operand per opcode didefinisikan di dalam `DEFINISI_OPCODE`.

/// Satu kode operasi Taji VM, direpresentasikan sebagai `u8`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum OpCode {
    // ------------------------------------------------------------------ //
    // KELOMPOK 1: Konstanta & Literal
    // ------------------------------------------------------------------ //
    /// Muat konstanta dari pool konstanta ke puncak stack.
    /// Operand: [index_hi: u8, index_lo: u8] (indeks 16-bit big-endian ke pool konstanta).
    /// Meski opcode adalah u8, indeks pool bisa mencapai 65535 konstanta.
    OpTulisPuncak = 0x00,

    /// Dorong nilai `benar` (true) ke puncak stack.
    OpBenar = 0x01,

    /// Dorong nilai `salah` (false) ke puncak stack.
    OpSalah = 0x02,

    /// Dorong nilai `nihil` (null) ke puncak stack.
    OpNihil = 0x03,

    // ------------------------------------------------------------------ //
    // KELOMPOK 2: Aritmatika Biner
    // ------------------------------------------------------------------ //
    /// Pop dua nilai dari stack, tambahkan, push hasilnya.
    OpTambah = 0x10,

    /// Pop dua nilai dari stack, kurangkan (kiri - kanan), push hasilnya.
    OpKurang = 0x11,

    /// Pop dua nilai dari stack, kalikan, push hasilnya.
    OpKali = 0x12,

    /// Pop dua nilai dari stack, bagi (kiri / kanan), push hasilnya.
    OpBagi = 0x13,

    /// Pop dua nilai dari stack, modulo (kiri % kanan), push hasilnya.
    OpSisa = 0x14,

    // ------------------------------------------------------------------ //
    // KELOMPOK 3: Perbandingan & Logika
    // ------------------------------------------------------------------ //
    /// Pop dua nilai, push `benar` jika sama persis.
    OpSamaDengan = 0x20,

    /// Pop dua nilai, push `benar` jika tidak sama.
    OpTidakSama = 0x21,

    /// Pop dua nilai, push `benar` jika kiri > kanan.
    OpLebihDari = 0x22,

    /// Pop dua nilai, push `benar` jika kiri < kanan.
    OpKurangDari = 0x23,

    /// Pop satu nilai, negasikan boolean-nya, push hasilnya.
    OpTidak = 0x24,

    /// Pop satu nilai numerik, negasikan tandanya (-x), push hasilnya.
    OpNegasi = 0x25,

    // ------------------------------------------------------------------ //
    // KELOMPOK 4: Lompatan (Control Flow)
    // ------------------------------------------------------------------ //
    /// Lompat tanpa syarat ke offset bytecode.
    /// Operand: [offset_hi: u8, offset_lo: u8] (target absolut 16-bit).
    OpLompat = 0x30,

    /// Lompat jika puncak stack adalah `salah` atau `nihil`.
    /// Operand: [offset_hi: u8, offset_lo: u8].
    OpLompatJikaTidak = 0x31,

    /// Lompat jika puncak stack adalah `benar` (untuk ekspresi `atau`).
    /// Operand: [offset_hi: u8, offset_lo: u8].
    OpLompatJikaBenar = 0x32,

    // ------------------------------------------------------------------ //
    // KELOMPOK 5: Variabel & Lingkup
    // ------------------------------------------------------------------ //
    /// Tetapkan nilai di puncak stack ke variabel global.
    /// Operand: [index_hi: u8, index_lo: u8] (indeks ke tabel simbol global).
    OpTetapkanGlobal = 0x40,

    /// Ambil variabel global, push ke stack.
    /// Operand: [index_hi: u8, index_lo: u8].
    OpAmbilGlobal = 0x41,

    /// Tetapkan nilai di puncak stack ke variabel lokal (frame saat ini).
    /// Operand: [slot: u8] (offset relatif ke base pointer frame).
    OpTetapkanLokal = 0x42,

    /// Ambil variabel lokal dari frame saat ini, push ke stack.
    /// Operand: [slot: u8].
    OpAmbilLokal = 0x43,

    // ------------------------------------------------------------------ //
    // KELOMPOK 6: Struktur Data
    // ------------------------------------------------------------------ //
    /// Pop N elemen dari stack (dari bawah ke atas), bangun Array, push hasilnya.
    /// Operand: [panjang_hi: u8, panjang_lo: u8].
    OpBangunArray = 0x50,

    /// Pop N*2 elemen (kunci, nilai) dari stack, bangun Kamus, push hasilnya.
    /// Operand: [panjang_hi: u8, panjang_lo: u8] (jumlah pasangan kv).
    OpBangunKamus = 0x51,

    /// Pop indeks lalu koleksi dari stack, push elemen pada indeks tersebut.
    OpAmbilIndeks = 0x52,

    /// Pop nilai, indeks, lalu koleksi. Tetapkan elemen pada indeks.
    OpTetapkanIndeks = 0x53,

    // ------------------------------------------------------------------ //
    // KELOMPOK 7: Fungsi & Pemanggilan
    // ------------------------------------------------------------------ //
    /// Panggil fungsi di puncak stack.
    /// Operand: [jumlah_argumen: u8].
    OpPanggil = 0x60,

    /// Kembalikan nilai dari fungsi saat ini ke pemanggil.
    OpKembalikan = 0x61,

    /// Ambil upvalue (variabel dari lingkup luar) dan push ke stack.
    /// Operand: [index: u8].
    OpAmbilUpvalue = 0x62,

    /// Tutup upvalue yang masih hidup di stack sebelum lingkup ditutup.
    OpTutupUpvalue = 0x63,

    /// Bangun obyek Closure dari fungsi mentah di stack.
    /// Operand: [index_hi: u8, index_lo: u8] (indeks konstanta fungsi)
    /// Diikuti oleh deskriptor upvalue.
    OpClosure = 0x64,

    /// Tetapkan nilai puncak stack ke upvalue.
    /// Operand: [index: u8].
    OpTetapkanUpvalue = 0x65,

    // ------------------------------------------------------------------ //
    // KELOMPOK 8: Manajemen Stack & Misc
    // ------------------------------------------------------------------ //
    /// Buang (pop dan abaikan) nilai di puncak stack.
    OpBuang = 0x70,

    /// Cetak nilai di puncak stack ke stdout (instruksi builtin sementara).
    OpCetak = 0x71,

    // ------------------------------------------------------------------ //
    // KELOMPOK 9: Penanganan Galat (Fase 8)
    // ------------------------------------------------------------------ //
    /// Lemparkan nilai galat dari puncak stack.
    ///
    /// Stack sebelum : [..., pesan_error: Str | nilai_apapun]
    /// Stack sesudah : (kosong — stack di-unwind oleh VM ke handler terdekat)
    ///
    /// VM akan mencari `KonteksCoba` terdekat di `stack_coba`. Jika ada,
    /// IP di-set ke `offset_handler`, stack di-truncate ke `base_stack`,
    /// dan pesan error di-push sebagai variabel catch. Jika tidak ada,
    /// VM berhenti dengan `GalatVM::GalatDilempar`.
    OpLemparkan = 0x80,

    /// Mulai blok `coba { ... }` — daftarkan exception handler.
    ///
    /// Operand: [offset_handler_hi: u8, offset_handler_lo: u8]
    ///
    /// Stack sebelum : [...]
    /// Stack sesudah : [...] (tidak berubah)
    ///
    /// VM mendorong satu `KonteksCoba` ke `stack_coba` yang menyimpan
    /// offset bytecode handler `tangkap` dan tinggi stack saat ini.
    /// Setelah blok `coba` selesai tanpa error, `OpAkhiriCoba` membuang
    /// konteks tersebut dan melompati blok `tangkap`.
    OpCoba = 0x81,

    /// Akhiri blok `coba` yang sukses — pop handler & lompat ke setelah `tangkap`.
    ///
    /// Operand: [offset_akhir_hi: u8, offset_akhir_lo: u8]
    ///
    /// Stack sebelum : [...]
    /// Stack sesudah : [...] (tidak berubah)
    ///
    /// Membuang `KonteksCoba` terdalam dari `stack_coba`, lalu
    /// melompat ke offset setelah seluruh blok coba/tangkap.
    OpAkhiriCoba = 0x82,
    /// Muat dan eksekusi modul eksternal, merger globals modul ke globals
    /// VM pemanggil, kembalikan kamus ekspor ke puncak stack.
    ///
    /// Instruksi ini menggantikan pemanggilan builtin `masukkan()` agar
    /// fungsi-fungsi yang diekspor dari modul dapat memanggil satu sama
    /// lain menggunakan globals VM pemanggil yang sudah di-merger.
    ///
    /// Stack sebelum : [..., jalur: Str]
    /// Stack sesudah : [..., kamus_ekspor: Kamus]
    OpMasukkan = 0x83,
    // -- SENTINEL -- Jangan menempatkan opcode di bawah baris ini. -- //
}

impl TryFrom<u8> for OpCode {
    type Error = u8;

    /// Konversi dari byte mentah ke `OpCode`. Mengembalikan `Err(byte)`
    /// jika nilai tidak dikenali sebagai opcode yang valid.
    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        match byte {
            0x00 => Ok(Self::OpTulisPuncak),
            0x01 => Ok(Self::OpBenar),
            0x02 => Ok(Self::OpSalah),
            0x03 => Ok(Self::OpNihil),
            0x10 => Ok(Self::OpTambah),
            0x11 => Ok(Self::OpKurang),
            0x12 => Ok(Self::OpKali),
            0x13 => Ok(Self::OpBagi),
            0x14 => Ok(Self::OpSisa),
            0x20 => Ok(Self::OpSamaDengan),
            0x21 => Ok(Self::OpTidakSama),
            0x22 => Ok(Self::OpLebihDari),
            0x23 => Ok(Self::OpKurangDari),
            0x24 => Ok(Self::OpTidak),
            0x25 => Ok(Self::OpNegasi),
            0x30 => Ok(Self::OpLompat),
            0x31 => Ok(Self::OpLompatJikaTidak),
            0x32 => Ok(Self::OpLompatJikaBenar),
            0x40 => Ok(Self::OpTetapkanGlobal),
            0x41 => Ok(Self::OpAmbilGlobal),
            0x42 => Ok(Self::OpTetapkanLokal),
            0x43 => Ok(Self::OpAmbilLokal),
            0x50 => Ok(Self::OpBangunArray),
            0x51 => Ok(Self::OpBangunKamus),
            0x52 => Ok(Self::OpAmbilIndeks),
            0x53 => Ok(Self::OpTetapkanIndeks),
            0x60 => Ok(Self::OpPanggil),
            0x61 => Ok(Self::OpKembalikan),
            0x62 => Ok(Self::OpAmbilUpvalue),
            0x63 => Ok(Self::OpTutupUpvalue),
            0x64 => Ok(Self::OpClosure),
            0x65 => Ok(Self::OpTetapkanUpvalue),
            0x70 => Ok(Self::OpBuang),
            0x71 => Ok(Self::OpCetak),
            0x80 => Ok(Self::OpLemparkan),
            0x81 => Ok(Self::OpCoba),
            0x82 => Ok(Self::OpAkhiriCoba),
            0x83 => Ok(Self::OpMasukkan),
            b => Err(b),
        }
    }
}

/// Metadata statis satu definisi opcode: nama dan jumlah byte operand-nya.
pub struct DefinisiOpCode {
    pub nama: &'static str,
    pub lebar_operand: &'static [u8],
}

/// Tabel definisi untuk setiap opcode. Digunakan oleh disassembler dan debugger.
/// `lebar_operand` adalah slice yang menyatakan lebar (dalam byte) tiap operand.
/// Contoh: `&[2]` berarti satu operand 2-byte; `&[1, 1]` berarti dua operand 1-byte.
pub const DEFINISI_OPCODE: &[(OpCode, DefinisiOpCode)] = &[
    (
        OpCode::OpTulisPuncak,
        DefinisiOpCode {
            nama: "OpTulisPuncak",
            lebar_operand: &[2],
        },
    ),
    (
        OpCode::OpBenar,
        DefinisiOpCode {
            nama: "OpBenar",
            lebar_operand: &[],
        },
    ),
    (
        OpCode::OpSalah,
        DefinisiOpCode {
            nama: "OpSalah",
            lebar_operand: &[],
        },
    ),
    (
        OpCode::OpNihil,
        DefinisiOpCode {
            nama: "OpNihil",
            lebar_operand: &[],
        },
    ),
    (
        OpCode::OpTambah,
        DefinisiOpCode {
            nama: "OpTambah",
            lebar_operand: &[],
        },
    ),
    (
        OpCode::OpKurang,
        DefinisiOpCode {
            nama: "OpKurang",
            lebar_operand: &[],
        },
    ),
    (
        OpCode::OpKali,
        DefinisiOpCode {
            nama: "OpKali",
            lebar_operand: &[],
        },
    ),
    (
        OpCode::OpBagi,
        DefinisiOpCode {
            nama: "OpBagi",
            lebar_operand: &[],
        },
    ),
    (
        OpCode::OpSisa,
        DefinisiOpCode {
            nama: "OpSisa",
            lebar_operand: &[],
        },
    ),
    (
        OpCode::OpSamaDengan,
        DefinisiOpCode {
            nama: "OpSamaDengan",
            lebar_operand: &[],
        },
    ),
    (
        OpCode::OpTidakSama,
        DefinisiOpCode {
            nama: "OpTidakSama",
            lebar_operand: &[],
        },
    ),
    (
        OpCode::OpLebihDari,
        DefinisiOpCode {
            nama: "OpLebihDari",
            lebar_operand: &[],
        },
    ),
    (
        OpCode::OpKurangDari,
        DefinisiOpCode {
            nama: "OpKurangDari",
            lebar_operand: &[],
        },
    ),
    (
        OpCode::OpTidak,
        DefinisiOpCode {
            nama: "OpTidak",
            lebar_operand: &[],
        },
    ),
    (
        OpCode::OpNegasi,
        DefinisiOpCode {
            nama: "OpNegasi",
            lebar_operand: &[],
        },
    ),
    (
        OpCode::OpLompat,
        DefinisiOpCode {
            nama: "OpLompat",
            lebar_operand: &[2],
        },
    ),
    (
        OpCode::OpLompatJikaTidak,
        DefinisiOpCode {
            nama: "OpLompatJikaTidak",
            lebar_operand: &[2],
        },
    ),
    (
        OpCode::OpLompatJikaBenar,
        DefinisiOpCode {
            nama: "OpLompatJikaBenar",
            lebar_operand: &[2],
        },
    ),
    (
        OpCode::OpTetapkanGlobal,
        DefinisiOpCode {
            nama: "OpTetapkanGlobal",
            lebar_operand: &[2],
        },
    ),
    (
        OpCode::OpAmbilGlobal,
        DefinisiOpCode {
            nama: "OpAmbilGlobal",
            lebar_operand: &[2],
        },
    ),
    (
        OpCode::OpTetapkanLokal,
        DefinisiOpCode {
            nama: "OpTetapkanLokal",
            lebar_operand: &[1],
        },
    ),
    (
        OpCode::OpAmbilLokal,
        DefinisiOpCode {
            nama: "OpAmbilLokal",
            lebar_operand: &[1],
        },
    ),
    (
        OpCode::OpBangunArray,
        DefinisiOpCode {
            nama: "OpBangunArray",
            lebar_operand: &[2],
        },
    ),
    (
        OpCode::OpBangunKamus,
        DefinisiOpCode {
            nama: "OpBangunKamus",
            lebar_operand: &[2],
        },
    ),
    (
        OpCode::OpAmbilIndeks,
        DefinisiOpCode {
            nama: "OpAmbilIndeks",
            lebar_operand: &[],
        },
    ),
    (
        OpCode::OpTetapkanIndeks,
        DefinisiOpCode {
            nama: "OpTetapkanIndeks",
            lebar_operand: &[],
        },
    ),
    (
        OpCode::OpPanggil,
        DefinisiOpCode {
            nama: "OpPanggil",
            lebar_operand: &[1],
        },
    ),
    (
        OpCode::OpKembalikan,
        DefinisiOpCode {
            nama: "OpKembalikan",
            lebar_operand: &[],
        },
    ),
    (
        OpCode::OpAmbilUpvalue,
        DefinisiOpCode {
            nama: "OpAmbilUpvalue",
            lebar_operand: &[1],
        },
    ),
    (
        OpCode::OpTutupUpvalue,
        DefinisiOpCode {
            nama: "OpTutupUpvalue",
            lebar_operand: &[],
        },
    ),
    (
        OpCode::OpClosure,
        DefinisiOpCode {
            nama: "OpClosure",
            lebar_operand: &[2],
        },
    ),
    (
        OpCode::OpTetapkanUpvalue,
        DefinisiOpCode {
            nama: "OpTetapkanUpvalue",
            lebar_operand: &[1],
        },
    ),
    (
        OpCode::OpBuang,
        DefinisiOpCode {
            nama: "OpBuang",
            lebar_operand: &[],
        },
    ),
    (
        OpCode::OpCetak,
        DefinisiOpCode {
            nama: "OpCetak",
            lebar_operand: &[],
        },
    ),
    (
        OpCode::OpLemparkan,
        DefinisiOpCode {
            nama: "OpLemparkan",
            lebar_operand: &[],
        },
    ),
    (
        OpCode::OpCoba,
        DefinisiOpCode {
            nama: "OpCoba",
            lebar_operand: &[2],
        },
    ),
    (
        OpCode::OpAkhiriCoba,
        DefinisiOpCode {
            nama: "OpAkhiriCoba",
            lebar_operand: &[2],
        },
    ),
    (
        OpCode::OpMasukkan,
        DefinisiOpCode {
            nama: "OpMasukkan",
            lebar_operand: &[],
        },
    ),
];
