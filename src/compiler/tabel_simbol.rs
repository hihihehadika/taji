//! Tabel Simbol Kompilator Taji.

use crate::compiler::galat::GalatKompilasi;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum LingkupSimbol {
    Global,
    Lokal,
    Upvalue,
}

#[derive(Debug, Clone)]
pub struct SimbolDefinisi {
    pub nama: String,
    pub lingkup: LingkupSimbol,
    pub indeks: usize,
}

/// Representasi satu upvalue yang ditangkap oleh scope ini.
#[derive(Debug, Clone, PartialEq)]
pub struct SimbolUpvalue {
    /// Indeks di scope induk (bisa indeks lokal atau indeks upvalue induk).
    pub indeks: usize,
    /// Apakah upvalue ini berasal dari variabel lokal induk (true) atau upvalue induk (false).
    pub adalah_lokal: bool,
}

#[derive(Debug, Clone, Default)]
pub struct TabelSimbol {
    store: HashMap<String, SimbolDefinisi>,
    pub jumlah_definisi: usize,
    pub outer: Option<Box<TabelSimbol>>,
    /// Daftar upvalue yang ditangkap oleh scope ini.
    pub upvalues: Vec<SimbolUpvalue>,
}

impl TabelSimbol {
    pub fn new() -> Self {
        TabelSimbol::default()
    }

    pub fn new_terlampir(outer: TabelSimbol) -> Self {
        TabelSimbol {
            store: HashMap::new(),
            jumlah_definisi: 0,
            outer: Some(Box::new(outer)),
            upvalues: Vec::new(),
        }
    }

    pub fn adalah_global(&self) -> bool {
        self.outer.is_none()
    }

    pub fn definisikan(&mut self, nama: &str) -> Result<SimbolDefinisi, GalatKompilasi> {
        if let Some(ada) = self.store.get(nama) {
            return Ok(ada.clone());
        }
        let lingkup = if self.outer.is_none() {
            LingkupSimbol::Global
        } else {
            LingkupSimbol::Lokal
        };
        let simbol = SimbolDefinisi {
            nama: nama.to_string(),
            lingkup,
            indeks: self.jumlah_definisi,
        };
        self.store.insert(nama.to_string(), simbol.clone());
        self.jumlah_definisi += 1;
        Ok(simbol)
    }

    /// Selesaikan simbol dengan dukungan Upvalue.
    /// Memerlukan `&mut self` karena mungkin perlu mendaftarkan upvalue baru.
    pub fn selesaikan(&mut self, nama: &str) -> Option<SimbolDefinisi> {
        // 1. Cek di scope lokal saat ini.
        if let Some(sim) = self.store.get(nama) {
            return Some(sim.clone());
        }

        // 2. Jika tidak ada dan ada scope luar, cari di luar secara rekursif.
        if let Some(ref mut outer) = self.outer {
            let sim = outer.selesaikan(nama)?;

            // Jika hasil dari luar adalah Global, kembalikan apa adanya (global tidak perlu upvalue).
            if sim.lingkup == LingkupSimbol::Global {
                return Some(sim);
            }

            // Jika hasil dari luar adalah Lokal atau Upvalue, maka bagi scope INI itu adalah Upvalue.
            let adalah_lokal = sim.lingkup == LingkupSimbol::Lokal;
            let upvalue_idx = self.tambah_upvalue(sim.indeks, adalah_lokal);

            let simbol_upvalue = SimbolDefinisi {
                nama: nama.to_string(),
                lingkup: LingkupSimbol::Upvalue,
                indeks: upvalue_idx,
            };
            self.store.insert(nama.to_string(), simbol_upvalue.clone());
            return Some(simbol_upvalue);
        }

        None
    }

    fn tambah_upvalue(&mut self, indeks_sumber: usize, adalah_lokal: bool) -> usize {
        // Cek apakah upvalue ini sudah pernah didaftarkan.
        for (i, uv) in self.upvalues.iter().enumerate() {
            if uv.indeks == indeks_sumber && uv.adalah_lokal == adalah_lokal {
                return i;
            }
        }

        let i = self.upvalues.len();
        self.upvalues.push(SimbolUpvalue {
            indeks: indeks_sumber,
            adalah_lokal,
        });
        i
    }

    /// Mengambil seluruh referensi penyimpanan simbol
    pub fn ambil_store(&self) -> &HashMap<String, SimbolDefinisi> {
        &self.store
    }

    /// Mencari saran simbol terdekat berdasarkan Levenshtein Distance
    pub fn cari_saran(&self, nama_salah: &str) -> Option<String> {
        let mut kandidat_terbaik = None;
        let mut jarak_terkecil = usize::MAX;

        for kunci in self.store.keys() {
            let jarak = jarak_levenshtein(nama_salah, kunci);
            // Toleransi maksimal 3 karakter beda untuk typo
            if jarak <= 3 && jarak < jarak_terkecil {
                jarak_terkecil = jarak;
                kandidat_terbaik = Some(kunci.clone());
            }
        }

        // Cek di outer scope jika ada
        if let Some(ref outer) = self.outer {
            if let Some(saran_luar) = outer.cari_saran(nama_salah) {
                let jarak_luar = jarak_levenshtein(nama_salah, &saran_luar);
                if jarak_luar < jarak_terkecil {
                    return Some(saran_luar);
                }
            }
        }

        kandidat_terbaik
    }
}

/// Menghitung jarak Levenshtein antara dua string
fn jarak_levenshtein(a: &str, b: &str) -> usize {
    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();
    let mut matriks = vec![vec![0; b_chars.len() + 1]; a_chars.len() + 1];

    #[allow(clippy::needless_range_loop)]
    for i in 0..=a_chars.len() {
        matriks[i][0] = i;
    }
    #[allow(clippy::needless_range_loop)]
    for j in 0..=b_chars.len() {
        matriks[0][j] = j;
    }

    for i in 1..=a_chars.len() {
        for j in 1..=b_chars.len() {
            let cost = if a_chars[i - 1] == b_chars[j - 1] {
                0
            } else {
                1
            };
            matriks[i][j] = (matriks[i - 1][j] + 1)
                .min(matriks[i][j - 1] + 1)
                .min(matriks[i - 1][j - 1] + cost);
        }
    }
    matriks[a_chars.len()][b_chars.len()]
}
