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
}
