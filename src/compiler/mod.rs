//! Kompilator Taji (TVM Compiler).
//! Mengubah AST menjadi Bytecode TVM.

use crate::ast::*;
use crate::code::definisi::OpCode;
use crate::code::encoder::{encode, tulis_operand_u16};
use crate::compiler::galat::GalatKompilasi;
pub use crate::compiler::tabel_simbol::{LingkupSimbol, TabelSimbol};
use crate::object::{Object, ObjekFungsiTerkompilasi};

pub mod galat;
pub mod tabel_simbol;

pub struct Kompilator {
    pub instruksi: Vec<u8>,
    pub konstanta: Vec<Object>,
    pub tabel_simbol: TabelSimbol,
    /// Pemetaan offset instruksi -> (baris, kolom, panjang) kode sumber.
    pub tabel_baris: Vec<(usize, usize, usize)>,
    /// Posisi (baris, kolom, panjang) saat ini yang sedang dikompilasi.
    posisi_sekarang: (usize, usize, usize),
    /// Stack patch offset untuk `berhenti` di setiap level loop
    loop_berhenti_patches: Vec<Vec<usize>>,
    /// Stack target offset untuk `lanjut` di setiap level loop
    loop_lanjut_target: Vec<usize>,
}

pub struct HasilKompilasi {
    pub instruksi: Vec<u8>,
    pub konstanta: Vec<Object>,
    pub tabel_simbol: TabelSimbol,
    /// Pemetaan offset instruksi -> (baris, kolom, panjang) kode sumber untuk pelaporan galat VM.
    pub tabel_baris: Vec<(usize, usize, usize)>,
}

impl HasilKompilasi {
    pub fn main_fungsi(&self) -> ObjekFungsiTerkompilasi {
        ObjekFungsiTerkompilasi {
            instruksi: self.instruksi.clone(),
            jumlah_parameter: 0,
            jumlah_lokal: self.tabel_simbol.jumlah_definisi,
            nama: Some("utama".to_string()),
            tabel_baris: self.tabel_baris.clone(),
            pool_konstanta_lokal: None,
        }
    }
}

impl Default for Kompilator {
    fn default() -> Self {
        Self::new()
    }
}

impl Kompilator {
    pub fn new() -> Self {
        Kompilator {
            instruksi: Vec::new(),
            konstanta: Vec::new(),
            tabel_simbol: TabelSimbol::new(),
            tabel_baris: Vec::new(),
            posisi_sekarang: (1, 1, 1),
            loop_berhenti_patches: Vec::new(),
            loop_lanjut_target: Vec::new(),
        }
    }

    pub fn new_dengan_state(ts: TabelSimbol, k: Vec<Object>) -> Self {
        Kompilator {
            instruksi: Vec::new(),
            konstanta: k,
            tabel_simbol: ts,
            tabel_baris: Vec::new(),
            posisi_sekarang: (1, 1, 1),
            loop_berhenti_patches: Vec::new(),
            loop_lanjut_target: Vec::new(),
        }
    }

    /// Memperbarui posisi (baris, kolom, panjang) yang sedang dikompilasi.
    pub fn tetapkan_posisi(&mut self, baris: usize, kolom: usize, panjang: usize) {
        self.posisi_sekarang = (baris, kolom, panjang);
    }

    /// Alias untuk kompatibilitas; hanya memperbarui baris.
    pub fn tetapkan_baris(&mut self, baris: usize) {
        self.posisi_sekarang = (baris, self.posisi_sekarang.1, self.posisi_sekarang.2);
    }

    pub fn kompilasi(&mut self, program: &Program) -> Result<HasilKompilasi, GalatKompilasi> {
        for stmt in &program.statements {
            self.kompilasi_pernyataan(stmt)?;
        }
        Ok(HasilKompilasi {
            instruksi: self.instruksi.clone(),
            konstanta: self.konstanta.clone(),
            tabel_simbol: self.tabel_simbol.clone(),
            tabel_baris: self.tabel_baris.clone(),
        })
    }

    // ================================================================== //
    // KOMPILASI PERNYATAAN (STATEMENT)
    // ================================================================== //

    fn kompilasi_pernyataan(&mut self, stmt: &Statement) -> Result<(), GalatKompilasi> {
        match stmt {
            Statement::Ekspresi(expr_stmt) => {
                self.kompilasi_ekspresi(&expr_stmt.expression)?;
                self.emit(OpCode::OpBuang, &[])?;
                Ok(())
            }
            Statement::Misalkan(mis) => {
                let simbol = self.tabel_simbol.definisikan(&mis.name.value)?;
                self.kompilasi_ekspresi(&mis.value)?;
                if simbol.lingkup == LingkupSimbol::Global {
                    self.emit(OpCode::OpTetapkanGlobal, &[simbol.indeks])?;
                } else {
                    self.emit(OpCode::OpTetapkanLokal, &[simbol.indeks])?;
                }
                // Karena OpTetapkan hanya peek (tidak pop), kita harus buang nilainya dari stack
                // agar eksekusi statement ini tidak membocorkan memori.
                self.emit(OpCode::OpBuang, &[])?;
                Ok(())
            }
            Statement::Kembalikan(ret) => {
                self.kompilasi_ekspresi(&ret.return_value)?;
                self.emit(OpCode::OpKembalikan, &[])?;
                Ok(())
            }
            Statement::Lemparkan(lem) => {
                self.kompilasi_ekspresi(&lem.value)?;
                self.emit(OpCode::OpLemparkan, &[])?;
                Ok(())
            }
            Statement::Berhenti => {
                if self.loop_berhenti_patches.is_empty() {
                    return Err(GalatKompilasi::BerhentiBukanDiLoop(
                        "'berhenti' di luar loop".to_string(),
                    ));
                }
                let pos = self.emit_lompat_placeholder(OpCode::OpLompat)?;
                self.loop_berhenti_patches.last_mut().unwrap().push(pos);
                Ok(())
            }
            Statement::Lanjut => {
                if self.loop_lanjut_target.is_empty() {
                    return Err(GalatKompilasi::BerhentiBukanDiLoop(
                        "'lanjut' di luar loop".to_string(),
                    ));
                }
                let target = *self.loop_lanjut_target.last().unwrap();
                self.emit(OpCode::OpLompat, &[target])?;
                Ok(())
            }
        }
    }

    // ================================================================== //
    // KOMPILASI EKSPRESI (EXPRESSION)
    // ================================================================== //

    fn kompilasi_ekspresi(&mut self, expr: &Expression) -> Result<(), GalatKompilasi> {
        match expr {
            Expression::IntegerLiteral(val) => {
                let idx = self.tambah_konstanta(Object::Integer(*val))?;
                self.emit(OpCode::OpTulisPuncak, &[idx])?;
                Ok(())
            }
            Expression::FloatLiteral(val) => {
                let idx = self.tambah_konstanta(Object::Float(*val))?;
                self.emit(OpCode::OpTulisPuncak, &[idx])?;
                Ok(())
            }
            Expression::BooleanLiteral(b) => {
                self.emit(if *b { OpCode::OpBenar } else { OpCode::OpSalah }, &[])?;
                Ok(())
            }
            Expression::StringLiteral(s) => {
                let idx = self.tambah_konstanta(Object::Str(s.clone()))?;
                self.emit(OpCode::OpTulisPuncak, &[idx])?;
                Ok(())
            }
            Expression::Null => {
                self.emit(OpCode::OpNihil, &[])?;
                Ok(())
            }

            // ── Pengenal ──
            Expression::Pengenal(id) => {
                // Tetapkan posisi sumber dari node Pengenal sebelum emit
                self.tetapkan_posisi(id.posisi.baris, id.posisi.kolom, id.posisi.panjang);
                let sim = self.tabel_simbol.selesaikan(&id.value).ok_or_else(|| {
                    let saran = self.tabel_simbol.cari_saran(&id.value);
                    GalatKompilasi::SimbolTidakTerdefinisi(
                        id.value.clone(),
                        id.posisi.baris,
                        id.posisi.kolom,
                        saran,
                    )
                })?;
                self.emit_ambil_simbol(&sim)?;
                Ok(())
            }

            // ── Penugasan ──
            Expression::Penugasan(ps) => self.kompilasi_penugasan(ps),

            // ── Awalan (Prefix): -x, !x ──
            Expression::Awalan(aw) => {
                self.kompilasi_ekspresi(&aw.right)?;
                match aw.operator.as_str() {
                    "!" | "bukan" => {
                        self.emit(OpCode::OpTidak, &[])?;
                    }
                    "-" => {
                        self.emit(OpCode::OpNegasi, &[])?;
                    }
                    _ => return Err(GalatKompilasi::OperatorTidakDikenal(aw.operator.clone())),
                }
                Ok(())
            }

            // ── Sisipan (Infix): a + b, a && b, dll ──
            Expression::Sisipan(sis) => self.kompilasi_sisipan(sis),

            // ── Jika (If/Else) ──
            Expression::Jika(jika) => self.kompilasi_jika(jika),

            // ── Selama (While) ──
            Expression::Selama(sel) => self.kompilasi_selama(sel),

            // ── Untuk (For) ──
            Expression::Untuk(utk) => self.kompilasi_untuk(utk),

            // ── Fungsi Literal ──
            Expression::FungsiLiteral(fl) => self.kompilasi_fungsi(&fl.parameters, &fl.body, None),

            // ── Fungsi Panah (Arrow Function) ──
            Expression::FungsiPanah(fp) => self.kompilasi_fungsi(&fp.parameters, &fp.body, None),

            // ── Panggilan Fungsi ──
            Expression::Panggilan(pg) => {
                self.kompilasi_ekspresi(&pg.function)?;
                for arg in &pg.arguments {
                    self.kompilasi_ekspresi(arg)?;
                }
                self.emit(OpCode::OpPanggil, &[pg.arguments.len()])?;
                Ok(())
            }

            // ── Array Literal ──
            Expression::ArrayLiteral(el) => {
                for e in el {
                    self.kompilasi_ekspresi(e)?;
                }
                self.emit(OpCode::OpBangunArray, &[el.len()])?;
                Ok(())
            }

            // ── Hash Literal ──
            Expression::HashLiteral(pairs) => {
                for (k, v) in pairs {
                    self.kompilasi_ekspresi(k)?;
                    self.kompilasi_ekspresi(v)?;
                }
                self.emit(OpCode::OpBangunKamus, &[pairs.len()])?;
                Ok(())
            }

            // ── Indeks ──
            Expression::Indeks(idx) => {
                self.kompilasi_ekspresi(&idx.left)?;
                self.kompilasi_ekspresi(&idx.index)?;
                self.emit(OpCode::OpAmbilIndeks, &[])?;
                Ok(())
            }

            // ── Titik (Dot Access): obj.kunci → obj["kunci"] ──
            Expression::Titik(titik) => {
                self.kompilasi_ekspresi(&titik.left)?;
                let idx = self.tambah_konstanta(Object::Str(titik.key.clone()))?;
                self.emit(OpCode::OpTulisPuncak, &[idx])?;
                self.emit(OpCode::OpAmbilIndeks, &[])?;
                Ok(())
            }

            // ── Coba / Tangkap ──
            Expression::Coba(coba) => self.kompilasi_coba(coba),

            // ── Masukkan (Import) — dikompilasi sebagai instruksi OpMasukkan ──
            // Tidak lagi memanggil builtin masukkan() karena OpMasukkan
            // memiliki akses langsung ke globals VM pemanggil, sehingga
            // fungsi-fungsi modul dapat saling memanggil satu sama lain
            // melalui globals VM pemanggil yang sudah di-merger.
            Expression::Masukkan(msk) => {
                self.kompilasi_ekspresi(&msk.path)?;
                self.emit(OpCode::OpMasukkan, &[])?;
                Ok(())
            }
        }
    }

    // ================================================================== //
    // KOMPILASI EKSPRESI — SUB-HANDLER
    // ================================================================== //

    fn kompilasi_penugasan(&mut self, ps: &PenugasanExpression) -> Result<(), GalatKompilasi> {
        let is_compound = ps.operator != "=";

        match &*ps.left {
            Expression::Pengenal(id) => {
                let sim = self.tabel_simbol.selesaikan(&id.value).ok_or_else(|| {
                    let saran = self.tabel_simbol.cari_saran(&id.value);
                    GalatKompilasi::SimbolTidakTerdefinisi(
                        id.value.clone(),
                        id.posisi.baris,
                        id.posisi.kolom,
                        saran,
                    )
                })?;

                if is_compound {
                    self.emit_ambil_simbol(&sim)?;
                }

                self.kompilasi_ekspresi(&ps.value)?;

                if is_compound {
                    match ps.operator.as_str() {
                        "+=" => self.emit(OpCode::OpTambah, &[])?,
                        "-=" => self.emit(OpCode::OpKurang, &[])?,
                        "*=" => self.emit(OpCode::OpKali, &[])?,
                        "/=" => self.emit(OpCode::OpBagi, &[])?,
                        _ => return Err(GalatKompilasi::OperatorTidakDikenal(ps.operator.clone())),
                    };
                }

                self.emit_tetapkan_simbol(&sim)?;
            }
            Expression::Indeks(idx_expr) => {
                self.kompilasi_ekspresi(&idx_expr.left)?;
                self.kompilasi_ekspresi(&idx_expr.index)?;

                if is_compound {
                    self.kompilasi_ekspresi(&idx_expr.left)?;
                    self.kompilasi_ekspresi(&idx_expr.index)?;
                    self.emit(OpCode::OpAmbilIndeks, &[])?;

                    self.kompilasi_ekspresi(&ps.value)?;
                    match ps.operator.as_str() {
                        "+=" => self.emit(OpCode::OpTambah, &[])?,
                        "-=" => self.emit(OpCode::OpKurang, &[])?,
                        "*=" => self.emit(OpCode::OpKali, &[])?,
                        "/=" => self.emit(OpCode::OpBagi, &[])?,
                        _ => return Err(GalatKompilasi::OperatorTidakDikenal(ps.operator.clone())),
                    };
                } else {
                    self.kompilasi_ekspresi(&ps.value)?;
                }

                self.emit(OpCode::OpTetapkanIndeks, &[])?;
            }
            Expression::Titik(titik) => {
                self.kompilasi_ekspresi(&titik.left)?;
                let idx = self.tambah_konstanta(Object::Str(titik.key.clone()))?;
                self.emit(OpCode::OpTulisPuncak, &[idx])?;

                if is_compound {
                    self.kompilasi_ekspresi(&titik.left)?;
                    self.emit(OpCode::OpTulisPuncak, &[idx])?;
                    self.emit(OpCode::OpAmbilIndeks, &[])?;

                    self.kompilasi_ekspresi(&ps.value)?;
                    match ps.operator.as_str() {
                        "+=" => self.emit(OpCode::OpTambah, &[])?,
                        "-=" => self.emit(OpCode::OpKurang, &[])?,
                        "*=" => self.emit(OpCode::OpKali, &[])?,
                        "/=" => self.emit(OpCode::OpBagi, &[])?,
                        _ => return Err(GalatKompilasi::OperatorTidakDikenal(ps.operator.clone())),
                    };
                } else {
                    self.kompilasi_ekspresi(&ps.value)?;
                }

                self.emit(OpCode::OpTetapkanIndeks, &[])?;
            }
            _ => {
                return Err(GalatKompilasi::OperatorTidakDikenal(
                    "kiri penugasan tidak valid".to_string(),
                ))
            }
        }
        Ok(())
    }

    fn kompilasi_sisipan(&mut self, sis: &SisipanExpression) -> Result<(), GalatKompilasi> {
        // Short-circuit: && / dan  dan  || / atau
        match sis.operator.as_str() {
            "&&" | "dan" => {
                self.kompilasi_ekspresi(&sis.left)?;
                let pos = self.emit_lompat_placeholder(OpCode::OpLompatJikaTidak)?;
                self.kompilasi_ekspresi(&sis.right)?;
                let akhir = self.emit_lompat_placeholder(OpCode::OpLompat)?;
                self.patch_lompat(pos, self.instruksi.len())?;
                self.emit(OpCode::OpSalah, &[])?;
                self.patch_lompat(akhir, self.instruksi.len())?;
                return Ok(());
            }
            "||" | "atau" => {
                self.kompilasi_ekspresi(&sis.left)?;
                let pos = self.emit_lompat_placeholder(OpCode::OpLompatJikaBenar)?;
                self.emit(OpCode::OpBuang, &[])?;
                self.kompilasi_ekspresi(&sis.right)?;
                self.patch_lompat(pos, self.instruksi.len())?;
                return Ok(());
            }
            _ => {}
        }

        self.kompilasi_ekspresi(&sis.left)?;
        self.kompilasi_ekspresi(&sis.right)?;
        match sis.operator.as_str() {
            "+" => {
                self.emit(OpCode::OpTambah, &[])?;
            }
            "-" => {
                self.emit(OpCode::OpKurang, &[])?;
            }
            "*" => {
                self.emit(OpCode::OpKali, &[])?;
            }
            "/" => {
                self.emit(OpCode::OpBagi, &[])?;
            }
            "%" => {
                self.emit(OpCode::OpSisa, &[])?;
            }
            "==" => {
                self.emit(OpCode::OpSamaDengan, &[])?;
            }
            "!=" => {
                self.emit(OpCode::OpTidakSama, &[])?;
            }
            ">" => {
                self.emit(OpCode::OpLebihDari, &[])?;
            }
            "<" => {
                self.emit(OpCode::OpKurangDari, &[])?;
            }
            ">=" => {
                // a >= b  ≡  !(a < b)
                self.emit(OpCode::OpKurangDari, &[])?;
                self.emit(OpCode::OpTidak, &[])?;
            }
            "<=" => {
                // a <= b  ≡  !(a > b)
                self.emit(OpCode::OpLebihDari, &[])?;
                self.emit(OpCode::OpTidak, &[])?;
            }
            _ => return Err(GalatKompilasi::OperatorTidakDikenal(sis.operator.clone())),
        }
        Ok(())
    }

    fn kompilasi_jika(&mut self, jika: &JikaExpression) -> Result<(), GalatKompilasi> {
        self.kompilasi_ekspresi(&jika.condition)?;
        let pos_jika_tidak = self.emit_lompat_placeholder(OpCode::OpLompatJikaTidak)?;

        // Blok konsekuensi
        self.kompilasi_blok(&jika.consequence)?;

        if let Some(alt) = &jika.alternative {
            let pos_lompat_akhir = self.emit_lompat_placeholder(OpCode::OpLompat)?;
            self.patch_lompat(pos_jika_tidak, self.instruksi.len())?;
            self.kompilasi_blok(alt)?;
            self.patch_lompat(pos_lompat_akhir, self.instruksi.len())?;
        } else {
            self.patch_lompat(pos_jika_tidak, self.instruksi.len())?;
            self.emit(OpCode::OpNihil, &[])?;
        }
        Ok(())
    }

    fn kompilasi_selama(&mut self, sel: &SelamaExpression) -> Result<(), GalatKompilasi> {
        let awal_loop = self.instruksi.len();

        // Masuk konteks loop
        self.loop_berhenti_patches.push(Vec::new());
        self.loop_lanjut_target.push(awal_loop);

        self.kompilasi_ekspresi(&sel.condition)?;
        let pos_keluar = self.emit_lompat_placeholder(OpCode::OpLompatJikaTidak)?;

        self.kompilasi_blok_tanpa_nilai(&sel.body)?;
        self.emit(OpCode::OpLompat, &[awal_loop])?;

        self.patch_lompat(pos_keluar, self.instruksi.len())?;

        // Patch semua `berhenti`
        let patches = self.loop_berhenti_patches.pop().unwrap();
        for p in patches {
            self.patch_lompat(p, self.instruksi.len())?;
        }
        self.loop_lanjut_target.pop();

        self.emit(OpCode::OpNihil, &[])?;
        Ok(())
    }

    fn kompilasi_untuk(&mut self, utk: &UntukExpression) -> Result<(), GalatKompilasi> {
        // Inisialisasi
        self.kompilasi_pernyataan(&utk.init)?;

        // Lompat ke kondisi pada iterasi pertama (melewati blok update)
        let lompat_pertama = self.emit_lompat_placeholder(OpCode::OpLompat)?;

        // Label Update: (target dari `lanjut` dan lompatan setelah body selesai)
        let offset_update = self.instruksi.len();
        self.loop_berhenti_patches.push(Vec::new());
        self.loop_lanjut_target.push(offset_update);

        self.kompilasi_pernyataan(&utk.update)?;

        // Label Kondisi:
        let offset_kondisi = self.instruksi.len();
        self.patch_lompat(lompat_pertama, offset_kondisi)?;

        self.kompilasi_ekspresi(&utk.condition)?;
        let pos_keluar = self.emit_lompat_placeholder(OpCode::OpLompatJikaTidak)?;

        self.kompilasi_blok_tanpa_nilai(&utk.body)?;

        // Lompat kembali ke update setelah body
        self.emit(OpCode::OpLompat, &[offset_update])?;

        self.patch_lompat(pos_keluar, self.instruksi.len())?;

        let patches = self.loop_berhenti_patches.pop().unwrap();
        for p in patches {
            self.patch_lompat(p, self.instruksi.len())?;
        }
        self.loop_lanjut_target.pop();

        self.emit(OpCode::OpNihil, &[])?;
        Ok(())
    }

    fn kompilasi_coba(&mut self, coba: &CobaExpression) -> Result<(), GalatKompilasi> {
        // OpCoba [offset_handler]
        let pos_coba = self.emit_lompat_placeholder(OpCode::OpCoba)?;

        // Blok coba
        self.kompilasi_blok(&coba.body)?;

        // OpAkhiriCoba [offset_akhir] — lompat melewati blok tangkap
        let pos_akhiri = self.emit_lompat_placeholder(OpCode::OpAkhiriCoba)?;

        // Patch OpCoba → mulai blok tangkap
        self.patch_lompat(pos_coba, self.instruksi.len())?;

        // Variabel error di tangkap: simpan sebagai variabel lokal
        let sim_err = self.tabel_simbol.definisikan(&coba.error_ident.value)?;
        if sim_err.lingkup == LingkupSimbol::Global {
            self.emit(OpCode::OpTetapkanGlobal, &[sim_err.indeks])?;
        } else {
            self.emit(OpCode::OpTetapkanLokal, &[sim_err.indeks])?;
        }

        // Blok tangkap
        self.kompilasi_blok(&coba.handler)?;

        // Patch OpAkhiriCoba → akhir seluruh blok
        self.patch_lompat(pos_akhiri, self.instruksi.len())?;

        Ok(())
    }

    fn kompilasi_fungsi(
        &mut self,
        parameters: &[Pengenal],
        body: &BlokPernyataan,
        nama: Option<String>,
    ) -> Result<(), GalatKompilasi> {
        let mut k_anak = Kompilator::new_dengan_state(
            TabelSimbol::new_terlampir(self.tabel_simbol.clone()),
            self.konstanta.clone(),
        );
        k_anak.posisi_sekarang = self.posisi_sekarang;
        for param in parameters {
            k_anak.tabel_simbol.definisikan(&param.value)?;
        }

        k_anak.kompilasi_blok(body)?;
        k_anak.emit(OpCode::OpKembalikan, &[])?;

        let ups = k_anak.tabel_simbol.upvalues.clone();
        let f_obj = ObjekFungsiTerkompilasi {
            instruksi: k_anak.instruksi,
            jumlah_parameter: parameters.len(),
            jumlah_lokal: k_anak.tabel_simbol.jumlah_definisi,
            nama,
            tabel_baris: k_anak.tabel_baris,
            pool_konstanta_lokal: None,
        };
        self.konstanta = k_anak.konstanta;
        let f_idx = self.tambah_konstanta(Object::FungsiVM(f_obj))?;
        self.emit(OpCode::OpClosure, &[f_idx])?;
        self.emit_raw_byte(ups.len() as u8);
        for uv in ups {
            self.emit_raw_byte(if uv.adalah_lokal { 1 } else { 0 });
            self.emit_raw_byte(uv.indeks as u8);
        }
        Ok(())
    }

    // ================================================================== //
    // KOMPILASI BLOK
    // ================================================================== //

    /// Kompilasi blok dan hasilkan nilai terakhir di stack.
    fn kompilasi_blok(&mut self, blok: &BlokPernyataan) -> Result<(), GalatKompilasi> {
        if blok.statements.is_empty() {
            self.emit(OpCode::OpNihil, &[])?;
            return Ok(());
        }
        let terakhir = blok.statements.len() - 1;
        for (i, stmt) in blok.statements.iter().enumerate() {
            if i == terakhir {
                // Pernyataan terakhir: jika Ekspresi, JANGAN buang nilainya
                if let Statement::Ekspresi(expr_stmt) = stmt {
                    self.kompilasi_ekspresi(&expr_stmt.expression)?;
                } else {
                    self.kompilasi_pernyataan(stmt)?;
                    self.emit(OpCode::OpNihil, &[])?;
                }
            } else {
                self.kompilasi_pernyataan(stmt)?;
            }
        }
        Ok(())
    }

    /// Kompilasi blok tanpa menyisakan nilai di stack (untuk loop body).
    fn kompilasi_blok_tanpa_nilai(&mut self, blok: &BlokPernyataan) -> Result<(), GalatKompilasi> {
        for stmt in &blok.statements {
            self.kompilasi_pernyataan(stmt)?;
        }
        Ok(())
    }

    // ================================================================== //
    // HELPER: EMIT & BACKPATCH
    // ================================================================== //

    fn emit(&mut self, op: OpCode, operands: &[usize]) -> Result<usize, GalatKompilasi> {
        let pos = self.instruksi.len();
        encode(&mut self.instruksi, op, operands);
        // Rekam pemetaan offset -> (baris, kolom) untuk setiap byte instruksi yang ditulis
        let jumlah_baru = self.instruksi.len() - pos;
        for _ in 0..jumlah_baru {
            self.tabel_baris.push(self.posisi_sekarang);
        }
        Ok(pos)
    }

    fn emit_raw_byte(&mut self, b: u8) {
        self.instruksi.push(b);
        self.tabel_baris.push(self.posisi_sekarang);
    }

    /// Emit instruksi lompat dengan placeholder operand 0x0000.
    /// Mengembalikan offset byte PERTAMA operand (untuk backpatch).
    fn emit_lompat_placeholder(&mut self, op: OpCode) -> Result<usize, GalatKompilasi> {
        self.emit(op, &[0x0000])?;
        // Operand u16 ada di offset: instruksi.len() - 2
        Ok(self.instruksi.len() - 2)
    }

    /// Patch instruksi lompat di `offset_operand` dengan `target`.
    fn patch_lompat(&mut self, offset_operand: usize, target: usize) -> Result<(), GalatKompilasi> {
        if target > 0xFFFF {
            return Err(GalatKompilasi::BatasanTerlampaui(
                "target lompatan melebihi 65535".to_string(),
            ));
        }
        tulis_operand_u16(&mut self.instruksi, offset_operand, target as u16);
        Ok(())
    }

    fn tambah_konstanta(&mut self, obj: Object) -> Result<usize, GalatKompilasi> {
        self.konstanta.push(obj);
        Ok(self.konstanta.len() - 1)
    }

    /// Helper: emit instruksi ambil berdasarkan lingkup simbol.
    fn emit_ambil_simbol(
        &mut self,
        sim: &tabel_simbol::SimbolDefinisi,
    ) -> Result<(), GalatKompilasi> {
        match sim.lingkup {
            LingkupSimbol::Global => {
                self.emit(OpCode::OpAmbilGlobal, &[sim.indeks])?;
            }
            LingkupSimbol::Lokal => {
                self.emit(OpCode::OpAmbilLokal, &[sim.indeks])?;
            }
            LingkupSimbol::Upvalue => {
                self.emit(OpCode::OpAmbilUpvalue, &[sim.indeks])?;
            }
        }
        Ok(())
    }

    /// Helper: emit instruksi tetapkan berdasarkan lingkup simbol.
    fn emit_tetapkan_simbol(
        &mut self,
        sim: &tabel_simbol::SimbolDefinisi,
    ) -> Result<(), GalatKompilasi> {
        match sim.lingkup {
            LingkupSimbol::Global => {
                self.emit(OpCode::OpTetapkanGlobal, &[sim.indeks])?;
            }
            LingkupSimbol::Lokal => {
                self.emit(OpCode::OpTetapkanLokal, &[sim.indeks])?;
            }
            LingkupSimbol::Upvalue => {
                self.emit(OpCode::OpTetapkanUpvalue, &[sim.indeks])?;
            }
        }
        Ok(())
    }
}
