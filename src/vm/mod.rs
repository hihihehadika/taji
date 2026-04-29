//! Taji VM (TVM) Implementation.

use crate::code::definisi::OpCode;
use crate::compiler::HasilKompilasi;
use crate::object::{Object, ObjekClosure, ObjekUpvalue};
use crate::vm::galat::GalatVM;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

pub mod galat;

const BATAS_STACK: usize = 2048;
const BATAS_FRAME: usize = 256;

struct CallFrame {
    closure: ObjekClosure,
    ip: usize,
    base: usize,
}

impl CallFrame {
    fn new(closure: ObjekClosure, base: usize) -> Self {
        CallFrame {
            closure,
            ip: 0,
            base,
        }
    }

    fn baca_u8(&mut self) -> u8 {
        let b = self.closure.fungsi.instruksi[self.ip];
        self.ip += 1;
        b
    }

    fn baca_u16(&mut self) -> usize {
        let hi = self.closure.fungsi.instruksi[self.ip] as usize;
        let lo = self.closure.fungsi.instruksi[self.ip + 1] as usize;
        self.ip += 2;
        (hi << 8) | lo
    }
}

struct KonteksCoba {
    offset_handler: usize,
    frame_indeks: usize,
    base_stack: usize,
}

pub struct VM {
    stack: Vec<Object>,
    frames: Vec<CallFrame>,
    globals: Vec<Object>,
    konstanta: Vec<Object>,
    stack_coba: Vec<KonteksCoba>,
    upvalues_terbuka: Vec<Rc<RefCell<ObjekUpvalue>>>,
    pub terakhir_dibuang: Option<Object>,
    /// Pemetaan offset bytecode -> nomor baris kode sumber.
    tabel_baris: Vec<usize>,

    // ── Garbage Collector (Mark and Sweep) Registry ──
    gc_arrays: Vec<Rc<RefCell<Vec<Object>>>>,
    gc_hashes: Vec<Rc<RefCell<HashMap<crate::object::KunciKamus, Object>>>>,
    gc_upvalues: Vec<Rc<RefCell<ObjekUpvalue>>>,
    alokasi_sejak_gc: usize,
    ambang_batas_gc: usize,

    // ── Keamanan Eksekusi (Untuk Fuzzing / Sandbox) ──
    pub batas_instruksi: Option<usize>,
}

impl VM {
    pub fn new(hasil: HasilKompilasi) -> Self {
        Self::new_dengan_globals(hasil, Vec::new())
    }

    pub fn new_dengan_globals(hasil: HasilKompilasi, globals: Vec<Object>) -> Self {
        let mut frames = Vec::with_capacity(BATAS_FRAME);
        let closure = ObjekClosure {
            fungsi: hasil.main_fungsi(),
            upvalues: Vec::new(),
        };
        frames.push(CallFrame::new(closure, 0));

        VM {
            stack: Vec::with_capacity(BATAS_STACK),
            frames,
            globals,
            konstanta: hasil.konstanta,
            stack_coba: Vec::new(),
            upvalues_terbuka: Vec::new(),
            terakhir_dibuang: None,
            tabel_baris: hasil.tabel_baris,
            gc_arrays: Vec::new(),
            gc_hashes: Vec::new(),
            gc_upvalues: Vec::new(),
            alokasi_sejak_gc: 0,
            ambang_batas_gc: 1000,
            batas_instruksi: None,
        }
    }

    /// Mengembalikan nomor baris kode sumber berdasarkan PC (instruction pointer) saat ini.
    fn baris_dari_ip(&self) -> usize {
        if let Some(frame) = self.frames.last() {
            let ip = frame.ip.saturating_sub(1);
            self.tabel_baris.get(ip).copied().unwrap_or(0)
        } else {
            0
        }
    }

    pub fn jalankan(&mut self) -> Result<Object, GalatVM> {
        let mut jumlah_dieksekusi = 0;
        while self.frame_aktif()?.ip < self.frame_aktif()?.closure.fungsi.instruksi.len() {
            if let Some(batas) = self.batas_instruksi {
                if jumlah_dieksekusi >= batas {
                    return Err(GalatVM::TipeOperanTidakValid(
                        "batas instruksi terlampaui".to_string(),
                    ));
                }
            }
            if self.eksekusi_instruksi().map_err(|e| {
                let baris = self.baris_dari_ip();
                GalatVM::DenganBaris {
                    baris,
                    sumber: Box::new(e),
                }
            })? {
                break;
            }
            jumlah_dieksekusi += 1;
            self.cek_gc();
        }
        Ok(self.terakhir_dibuang.clone().unwrap_or(Object::Null))
    }

    fn frame_aktif(&self) -> Result<&CallFrame, GalatVM> {
        self.frames.last().ok_or(GalatVM::StackFrameKosong)
    }

    fn frame_aktif_mut(&mut self) -> Result<&mut CallFrame, GalatVM> {
        self.frames.last_mut().ok_or(GalatVM::StackFrameKosong)
    }

    fn push(&mut self, obj: Object) -> Result<(), GalatVM> {
        if self.stack.len() >= BATAS_STACK {
            return Err(GalatVM::StackLuapan);
        }
        self.stack.push(obj);
        Ok(())
    }

    fn pop(&mut self) -> Result<Object, GalatVM> {
        self.stack.pop().ok_or(GalatVM::StackKosong)
    }

    fn peek(&self, distance: usize) -> &Object {
        &self.stack[self.stack.len() - 1 - distance]
    }

    fn eksekusi_instruksi(&mut self) -> Result<bool, GalatVM> {
        let op_byte = self.frame_aktif_mut()?.baca_u8();
        let op = OpCode::try_from(op_byte).map_err(GalatVM::OpCodeTidakDikenal)?;

        match op {
            OpCode::OpTulisPuncak => {
                let idx = self.frame_aktif_mut()?.baca_u16();
                // Jika frame aktif memiliki pool konstanta lokal (misal: fungsi dari modul
                // yang diekspor), gunakan pool tersebut. Jika tidak, gunakan pool VM global.
                let val = if let Some(pool) = &self.frame_aktif()?.closure.fungsi.pool_konstanta_lokal {
                    pool.get(idx).cloned().unwrap_or(Object::Null)
                } else {
                    self.konstanta.get(idx).cloned().unwrap_or(Object::Null)
                };
                self.push(val)?;
            }
            OpCode::OpBenar => self.push(Object::Boolean(true))?,
            OpCode::OpSalah => self.push(Object::Boolean(false))?,
            OpCode::OpNihil => self.push(Object::Null)?,

            OpCode::OpTambah
            | OpCode::OpKurang
            | OpCode::OpKali
            | OpCode::OpBagi
            | OpCode::OpSisa => self.eksekusi_aritmatika(op)?,
            OpCode::OpSamaDengan
            | OpCode::OpTidakSama
            | OpCode::OpLebihDari
            | OpCode::OpKurangDari => self.eksekusi_perbandingan(op)?,

            OpCode::OpTidak => {
                let val = self.pop()?;
                self.push(Object::Boolean(!kondisi_benar(&val)))?;
            }
            OpCode::OpNegasi => {
                let val = self.pop()?;
                match val {
                    Object::Integer(n) => self.push(Object::Integer(-n))?,
                    Object::Float(n) => self.push(Object::Float(-n))?,
                    _ => {
                        return Err(GalatVM::TipeOperanTidakValid(
                            "negasi hanya untuk angka".to_string(),
                        ))
                    }
                }
            }

            OpCode::OpLompat => {
                let off = self.frame_aktif_mut()?.baca_u16();
                self.frame_aktif_mut()?.ip = off;
            }
            OpCode::OpLompatJikaTidak => {
                let off = self.frame_aktif_mut()?.baca_u16();
                let kond = self.pop()?;
                if !kondisi_benar(&kond) {
                    self.frame_aktif_mut()?.ip = off;
                }
            }
            OpCode::OpLompatJikaBenar => {
                let off = self.frame_aktif_mut()?.baca_u16();
                let kond = self.peek(0);
                if kondisi_benar(kond) {
                    self.frame_aktif_mut()?.ip = off;
                }
            }

            OpCode::OpAmbilGlobal => {
                let idx = self.frame_aktif_mut()?.baca_u16();
                self.push(self.globals.get(idx).cloned().unwrap_or(Object::Null))?;
            }
            OpCode::OpTetapkanGlobal => {
                let idx = self.frame_aktif_mut()?.baca_u16();
                let val = self.peek(0).clone();
                if idx >= self.globals.len() {
                    self.globals.resize(idx + 1, Object::Null);
                }
                self.globals[idx] = val;
            }
            OpCode::OpAmbilLokal => {
                let slot = self.frame_aktif_mut()?.baca_u8() as usize;
                let base = self.frame_aktif()?.base;
                self.push(self.stack[base + slot].clone())?;
            }
            OpCode::OpTetapkanLokal => {
                let slot = self.frame_aktif_mut()?.baca_u8() as usize;
                let base = self.frame_aktif()?.base;
                let val = self.peek(0).clone();
                self.stack[base + slot] = val;
            }

            OpCode::OpBangunArray => {
                let len = self.frame_aktif_mut()?.baca_u16();
                let start = self.stack.len() - len;
                let elements = self.stack.split_off(start);
                let arr = self.alokasi_array(elements);
                self.push(Object::Array(arr))?;
            }
            OpCode::OpBangunKamus => {
                let len = self.frame_aktif_mut()?.baca_u16();
                let mut map = HashMap::with_capacity(len);
                for _ in 0..len {
                    let v = self.pop()?;
                    let k_obj = self.pop()?;
                    if let Some(k) = k_obj.to_hash_key() {
                        map.insert(k, v);
                    }
                }
                let hash = self.alokasi_hash(map);
                self.push(Object::Hash(hash))?;
            }
            OpCode::OpAmbilIndeks => {
                let i = self.pop()?;
                let k = self.pop()?;
                self.eksekusi_ambil_indeks(k, i)?;
            }
            OpCode::OpTetapkanIndeks => {
                let v = self.pop()?;
                let i = self.pop()?;
                let k = self.pop()?;
                self.eksekusi_tetapkan_indeks(k, i, v)?;
            }

            OpCode::OpClosure => {
                let idx = self.frame_aktif_mut()?.baca_u16();
                self.eksekusi_closure(idx)?;
            }
            OpCode::OpAmbilUpvalue => {
                let idx = self.frame_aktif_mut()?.baca_u8() as usize;
                let uv_rc = self.frame_aktif()?.closure.upvalues[idx].clone();
                let val = match &*uv_rc.borrow() {
                    ObjekUpvalue::Open(si) => self.stack[*si].clone(),
                    ObjekUpvalue::Closed(o) => o.clone(),
                };
                self.push(val)?;
            }
            OpCode::OpTetapkanUpvalue => {
                let idx = self.frame_aktif_mut()?.baca_u8() as usize;
                let val = self.peek(0).clone();
                let uv_rc = self.frame_aktif()?.closure.upvalues[idx].clone();
                let mut uv = uv_rc.borrow_mut();
                match &mut *uv {
                    ObjekUpvalue::Open(si) => {
                        self.stack[*si] = val;
                    }
                    ObjekUpvalue::Closed(o) => {
                        *o = val;
                    }
                }
            }
            OpCode::OpTutupUpvalue => {
                let si = self.stack.len() - 1;
                self.tutup_upvalues(si);
                self.pop()?;
            }

            OpCode::OpPanggil => {
                let n = self.frame_aktif_mut()?.baca_u8() as usize;
                self.eksekusi_panggil(n)?;
            }
            OpCode::OpKembalikan => {
                let v = self.pop()?;
                self.terapkan_return(v)?;
            }
            OpCode::OpBuang => {
                self.terakhir_dibuang = Some(self.pop()?);
            }
            OpCode::OpCetak => crate::keluaran::cetak_keluar(&format!("{}", self.pop()?)),
            OpCode::OpCoba => {
                let off = self.frame_aktif_mut()?.baca_u16();
                self.stack_coba.push(KonteksCoba {
                    offset_handler: off,
                    frame_indeks: self.frames.len() - 1,
                    base_stack: self.stack.len(),
                });
            }
            OpCode::OpAkhiriCoba => {
                self.stack_coba.pop();
                let t = self.frame_aktif_mut()?.baca_u16();
                self.frame_aktif_mut()?.ip = t;
            }
            OpCode::OpLemparkan => {
                let e = self.pop()?;
                self.tangani_lemparan(e)?;
            }
            OpCode::OpMasukkan => {
                let jalur_obj = self.pop()?;
                let hasil = self.eksekusi_masukkan(jalur_obj)?;
                self.push(hasil)?;
            }
        }
        Ok(false)
    }

    fn eksekusi_closure(&mut self, konst_idx: usize) -> Result<(), GalatVM> {
        let konst = self.konstanta[konst_idx].clone();
        let fungsi = match konst {
            Object::FungsiVM(f) => f,
            _ => return Err(GalatVM::TipeOperanTidakValid("bukan fungsi".to_string())),
        };

        let upvalue_count = self.frame_aktif_mut()?.baca_u8() as usize;
        let mut upvalues = Vec::with_capacity(upvalue_count);

        for _ in 0..upvalue_count {
            let is_local = self.frame_aktif_mut()?.baca_u8() == 1;
            let index = self.frame_aktif_mut()?.baca_u8() as usize;

            if is_local {
                let base = self.frame_aktif()?.base;
                upvalues.push(self.tangkap_upvalue(base + index));
            } else {
                upvalues.push(self.frame_aktif()?.closure.upvalues[index].clone());
            }
        }

        self.push(Object::Closure(ObjekClosure { fungsi, upvalues }))
    }

    fn tangkap_upvalue(&mut self, stack_idx: usize) -> Rc<RefCell<ObjekUpvalue>> {
        for uv_rc in &self.upvalues_terbuka {
            if let ObjekUpvalue::Open(idx) = *uv_rc.borrow() {
                if idx == stack_idx {
                    return uv_rc.clone();
                }
            }
        }
        let uv = self.alokasi_upvalue(ObjekUpvalue::Open(stack_idx));
        self.upvalues_terbuka.push(uv.clone());
        uv
    }

    fn tutup_upvalues(&mut self, ambang_bawah: usize) {
        let mut i = 0;
        while i < self.upvalues_terbuka.len() {
            let uv_rc = self.upvalues_terbuka[i].clone();
            let mut uv = uv_rc.borrow_mut();
            if let ObjekUpvalue::Open(idx) = *uv {
                if idx >= ambang_bawah {
                    *uv = ObjekUpvalue::Closed(self.stack[idx].clone());
                    self.upvalues_terbuka.remove(i);
                    continue;
                }
            }
            i += 1;
        }
    }

    fn terapkan_return(&mut self, nilai: Object) -> Result<(), GalatVM> {
        let frame = self.frames.pop().ok_or(GalatVM::StackFrameKosong)?;
        self.tutup_upvalues(frame.base);
        self.stack.truncate(frame.base - 1);
        self.push(nilai)
    }

    /// Muat modul dari berkas, kompilasi, jalankan dengan globals VM pemanggil
    /// sebagai basis, merger globals modul ke globals pemanggil, dan kembalikan
    /// Kamus ekspor. Dengan merger globals, fungsi-fungsi modul yang diekspor
    /// dapat saling memanggil satu sama lain via indeks global yang benar.
    fn eksekusi_masukkan(&mut self, jalur_obj: Object) -> Result<Object, GalatVM> {
        let masukan = match jalur_obj {
            Object::Str(s) => s,
            lain => {
                return Ok(Object::Error(format!(
                    "masukkan: argumen harus TEKS, bukan {}",
                    lain.type_name()
                )))
            }
        };

        // ── 1. Resolusi jalur berkas modul ──
        let jalur = {
            use std::path::Path;
            // Coba jalur langsung
            let p = Path::new(&masukan);
            if p.exists() {
                masukan.clone()
            } else {
                // Coba tambahkan ekstensi .tj
                let dengan_ekstensi = format!("{}.tj", masukan);
                if Path::new(&dengan_ekstensi).exists() {
                    dengan_ekstensi
                } else {
                    // Coba di folder taji_modul/
                    let nama_berkas = p
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or(&masukan);
                    let via_modul = format!("taji_modul/{}.tj", nama_berkas);
                    if Path::new(&via_modul).exists() {
                        via_modul
                    } else {
                        return Ok(Object::Error(format!(
                            "masukkan: modul '{}' tidak ditemukan.",
                            masukan
                        )));
                    }
                }
            }
        };

        // ── 2. Baca dan kompilasi isi modul ──
        let isi = match std::fs::read_to_string(&jalur) {
            Ok(c) => c,
            Err(e) => {
                return Ok(Object::Error(format!(
                    "masukkan: gagal membaca '{}': {}",
                    jalur, e
                )))
            }
        };

        let lexer = crate::lexer::Lexer::new(&isi);
        let mut parser = crate::parser::Parser::new(lexer);
        let program = parser.parse_program();
        if !parser.errors.is_empty() {
            return Ok(Object::Error(format!(
                "masukkan: galat sintaks di '{}':\n{}",
                jalur,
                parser.errors.join("\n")
            )));
        }

        let mut kompilator = crate::compiler::Kompilator::new_dengan_state(
            crate::bawaan::bikin_tabel_awal(),
            Vec::new(),
        );
        let hasil_kompilasi = match kompilator.kompilasi(&program) {
            Ok(h) => h,
            Err(e) => {
                return Ok(Object::Error(format!(
                    "masukkan: galat kompilasi di '{}': {}",
                    jalur, e
                )))
            }
        };

        // ── 3. Siapkan globals basis untuk VM modul ──
        // VM modul harus dimulai dengan globals builtin saja (bukan seluruh globals
        // pemanggil) agar indeks yang dikompilasi oleh kompilator modul sinkron.
        // Kompilator modul menggunakan bikin_tabel_awal() yang mendaftarkan N builtin
        // di indeks 0..N. Maka VM modul harus dimulai dengan persis N builtin yang sama.
        let globals_builtin = crate::bawaan::bikin_globals_awal();
        let jumlah_builtin = globals_builtin.len();

        // ── 4. Jalankan VM modul dengan globals builtin sebagai basis ──
        let mut vm_modul = VM::new_dengan_globals(hasil_kompilasi, globals_builtin);
        if let Err(e) = vm_modul.jalankan() {
            return Ok(Object::Error(format!(
                "masukkan: galat eksekusi di '{}': {}",
                jalur, e
            )));
        }

        // ── 5. Ambil globals dan konstanta hasil modul ──
        let globals_modul = vm_modul.ambil_globals();
        let konstanta_modul = vm_modul.ambil_konstanta();

        // ── 6. Merger globals modul ke globals VM pemanggil ──
        // Slot 0..jumlah_builtin adalah builtin — sudah ada di pemanggil, lewati.
        // Slot jumlah_builtin ke atas adalah variabel yang didefinisikan modul.
        // Kita tulis ke globals pemanggil pada indeks yang SAMA persis.
        // Jika globals pemanggil lebih pendek, resize dulu.
        if globals_modul.len() > self.globals.len() {
            self.globals.resize(globals_modul.len(), Object::Null);
        }
        for (i, obj_modul) in globals_modul.iter().enumerate().skip(jumlah_builtin) {
            // Suntikkan pool konstanta modul ke setiap Closure di globals modul
            // agar rekursi (OpAmbilGlobal lalu OpPanggil) tetap menggunakan pool yang benar.
            let nilai = match obj_modul.clone() {
                Object::Closure(mut c) => {
                    c.fungsi.pool_konstanta_lokal = Some(konstanta_modul.clone());
                    Object::Closure(c)
                }
                lain => lain,
            };
            self.globals[i] = nilai;
        }

        // ── 7. Bangun Kamus ekspor dari variabel global modul ──
        // Setiap Closure yang diekspor disuntikkan pool_konstanta_lokal
        // agar saat dipanggil di VM pemanggil, instruksi OpTulisPuncak
        // membaca dari pool konstanta modul — bukan pool pemanggil.
        let tabel = kompilator.tabel_simbol.ambil_store();
        let mut ekspor = HashMap::new();
        for (nama, simbol) in tabel.iter() {
            if simbol.lingkup == crate::compiler::LingkupSimbol::Global
                && simbol.indeks < globals_modul.len()
            {
                // Suntikkan pool konstanta modul ke setiap Closure yang diekspor
                let nilai = match globals_modul[simbol.indeks].clone() {
                    Object::Closure(mut c) => {
                        c.fungsi.pool_konstanta_lokal = Some(konstanta_modul.clone());
                        Object::Closure(c)
                    }
                    lain => lain,
                };
                ekspor.insert(
                    crate::object::KunciKamus::Str(nama.clone()),
                    nilai,
                );
            }
        }

        Ok(Object::Hash(self.alokasi_hash(ekspor)))
    }

    fn eksekusi_panggil(&mut self, jml_arg: usize) -> Result<(), GalatVM> {
        let fn_pos = self.stack.len() - jml_arg - 1;
        let fn_obj = self.stack[fn_pos].clone();
        match fn_obj {
            Object::Closure(c) => {
                if jml_arg != c.fungsi.jumlah_parameter {
                    return Err(GalatVM::JumlahArgumenSalah {
                        diharapkan: c.fungsi.jumlah_parameter,
                        diterima: jml_arg,
                    });
                }
                if self.frames.len() >= BATAS_FRAME {
                    return Err(GalatVM::StackFramePenuh);
                }
                let base = fn_pos + 1;
                for _ in 0..(c.fungsi.jumlah_lokal.saturating_sub(jml_arg)) {
                    self.push(Object::Null)?;
                }
                self.frames.push(CallFrame::new(c, base));
                Ok(())
            }
            Object::Bawaan(b) => {
                let args = self.stack.split_off(fn_pos + 1);
                self.stack.pop();
                self.push(b(args))
            }
            _ => Err(GalatVM::TipeOperanTidakValid("bukan fungsi".to_string())),
        }
    }

    fn eksekusi_tetapkan_indeks(
        &mut self,
        kol: Object,
        idx: Object,
        val: Object,
    ) -> Result<(), GalatVM> {
        match (kol, idx) {
            (Object::Array(arr), Object::Integer(i)) => {
                let mut a = arr.borrow_mut();
                let pos = if i < 0 { a.len() as i64 + i } else { i };
                if pos >= 0 && pos < a.len() as i64 {
                    a[pos as usize] = val.clone();
                    self.push(val)
                } else {
                    Err(GalatVM::IndeksDiLuarBatas(i.unsigned_abs() as usize))
                }
            }
            (Object::Hash(h), k_obj) => {
                if let Some(k) = k_obj.to_hash_key() {
                    h.borrow_mut().insert(k, val.clone());
                    self.push(val)
                } else {
                    Err(GalatVM::TipeOperanTidakValid(
                        "kunci tidak valid".to_string(),
                    ))
                }
            }
            _ => Err(GalatVM::TipeOperanTidakValid(
                "tipe bukan koleksi".to_string(),
            )),
        }
    }

    fn eksekusi_aritmatika(&mut self, op: OpCode) -> Result<(), GalatVM> {
        let (r, l) = (self.pop()?, self.pop()?);
        let res = match (l, r) {
            (Object::Integer(a), Object::Integer(b)) => match op {
                OpCode::OpTambah => Object::Integer(a.wrapping_add(b)),
                OpCode::OpKurang => Object::Integer(a.wrapping_sub(b)),
                OpCode::OpKali => Object::Integer(a.wrapping_mul(b)),
                OpCode::OpBagi => {
                    if b == 0 {
                        return Err(GalatVM::PembagianDenganNol);
                    }
                    Object::Integer(a / b)
                }
                OpCode::OpSisa => {
                    if b == 0 {
                        return Err(GalatVM::PembagianDenganNol);
                    }
                    Object::Integer(a % b)
                }
                _ => unreachable!(),
            },
            (Object::Float(a), Object::Float(b)) => match op {
                OpCode::OpTambah => Object::Float(a + b),
                OpCode::OpKurang => Object::Float(a - b),
                OpCode::OpKali => Object::Float(a * b),
                OpCode::OpBagi => Object::Float(a / b),
                OpCode::OpSisa => Object::Float(a % b),
                _ => unreachable!(),
            },
            // Promosi Int ↔ Float
            (Object::Integer(a), Object::Float(b)) => match op {
                OpCode::OpTambah => Object::Float(a as f64 + b),
                OpCode::OpKurang => Object::Float(a as f64 - b),
                OpCode::OpKali => Object::Float(a as f64 * b),
                OpCode::OpBagi => Object::Float(a as f64 / b),
                OpCode::OpSisa => Object::Float(a as f64 % b),
                _ => unreachable!(),
            },
            (Object::Float(a), Object::Integer(b)) => match op {
                OpCode::OpTambah => Object::Float(a + b as f64),
                OpCode::OpKurang => Object::Float(a - b as f64),
                OpCode::OpKali => Object::Float(a * b as f64),
                OpCode::OpBagi => Object::Float(a / b as f64),
                OpCode::OpSisa => Object::Float(a % b as f64),
                _ => unreachable!(),
            },
            // String + String
            (Object::Str(a), Object::Str(b)) if op == OpCode::OpTambah => {
                Object::Str(format!("{}{}", a, b))
            }
            // String + lainnya (auto-coercion)
            (Object::Str(a), b) if op == OpCode::OpTambah => Object::Str(format!("{}{}", a, b)),
            (a, Object::Str(b)) if op == OpCode::OpTambah => Object::Str(format!("{}{}", a, b)),
            // Array + Array
            (Object::Array(a), Object::Array(b)) if op == OpCode::OpTambah => {
                let mut gabung = a.borrow().clone();
                gabung.extend(b.borrow().iter().cloned());
                Object::Array(self.alokasi_array(gabung))
            }
            _ => {
                return Err(GalatVM::TipeOperanTidakValid(
                    "aritmatika gagal".to_string(),
                ))
            }
        };
        self.push(res)
    }

    fn eksekusi_perbandingan(&mut self, op: OpCode) -> Result<(), GalatVM> {
        let (r, l) = (self.pop()?, self.pop()?);
        let b = match op {
            OpCode::OpSamaDengan => l == r,
            OpCode::OpTidakSama => l != r,
            _ => match (l, r) {
                (Object::Integer(a), Object::Integer(b)) => match op {
                    OpCode::OpLebihDari => a > b,
                    OpCode::OpKurangDari => a < b,
                    _ => unreachable!(),
                },
                (Object::Float(a), Object::Float(b)) => match op {
                    OpCode::OpLebihDari => a > b,
                    OpCode::OpKurangDari => a < b,
                    _ => unreachable!(),
                },
                (Object::Integer(a), Object::Float(b)) => match op {
                    OpCode::OpLebihDari => (a as f64) > b,
                    OpCode::OpKurangDari => (a as f64) < b,
                    _ => unreachable!(),
                },
                (Object::Float(a), Object::Integer(b)) => match op {
                    OpCode::OpLebihDari => a > b as f64,
                    OpCode::OpKurangDari => a < (b as f64),
                    _ => unreachable!(),
                },
                _ => false,
            },
        };
        self.push(Object::Boolean(b))
    }

    fn eksekusi_ambil_indeks(&mut self, kol: Object, idx: Object) -> Result<(), GalatVM> {
        let res = match (kol, idx) {
            (Object::Array(arr), Object::Integer(i)) => {
                let a = arr.borrow();
                let pos = if i < 0 { a.len() as i64 + i } else { i };
                if pos >= 0 && pos < a.len() as i64 {
                    a[pos as usize].clone()
                } else {
                    Object::Null
                }
            }
            (Object::Hash(h), k_obj) => {
                if let Some(k) = k_obj.to_hash_key() {
                    h.borrow().get(&k).cloned().unwrap_or(Object::Null)
                } else {
                    Object::Null
                }
            }
            _ => Object::Null,
        };
        self.push(res)
    }

    /// Tangani lemparan galat dengan stack-unwinding ke handler `coba` terdekat.
    /// Jika tidak ada handler, kembalikan error ke pemanggil.
    fn tangani_lemparan(&mut self, nilai_galat: Object) -> Result<(), GalatVM> {
        if let Some(konteks) = self.stack_coba.pop() {
            // Unwind frame stack kembali ke frame saat coba didaftarkan
            while self.frames.len() > konteks.frame_indeks + 1 {
                if let Some(frame) = self.frames.pop() {
                    self.tutup_upvalues(frame.base);
                }
            }
            // Truncate stack ke tinggi saat coba didaftarkan
            self.stack.truncate(konteks.base_stack);
            // Push pesan galat sebagai string agar bisa diakses di blok tangkap
            let pesan = match &nilai_galat {
                Object::Str(s) => Object::Str(s.clone()),
                other => Object::Str(other.to_string()),
            };
            self.push(pesan)?;
            // Lompat ke handler tangkap
            self.frame_aktif_mut()?.ip = konteks.offset_handler;
            Ok(())
        } else {
            Err(GalatVM::GalatDilempar(nilai_galat))
        }
    }

    pub fn ambil_globals(&self) -> Vec<Object> {
        self.globals.clone()
    }
    pub fn ambil_konstanta(&self) -> Vec<Object> {
        self.konstanta.clone()
    }

    // ========================================================================= //
    // GARBAGE COLLECTOR (MARK AND SWEEP)
    // ========================================================================= //

    fn alokasi_array(&mut self, arr: Vec<Object>) -> Rc<RefCell<Vec<Object>>> {
        let rc = Rc::new(RefCell::new(arr));
        self.gc_arrays.push(rc.clone());
        rc
    }

    fn alokasi_hash(
        &mut self,
        hash: HashMap<crate::object::KunciKamus, Object>,
    ) -> Rc<RefCell<HashMap<crate::object::KunciKamus, Object>>> {
        let rc = Rc::new(RefCell::new(hash));
        self.gc_hashes.push(rc.clone());
        rc
    }

    fn alokasi_upvalue(&mut self, uv: ObjekUpvalue) -> Rc<RefCell<ObjekUpvalue>> {
        let rc = Rc::new(RefCell::new(uv));
        self.gc_upvalues.push(rc.clone());
        rc
    }

    fn cek_gc(&mut self) {
        self.alokasi_sejak_gc += 1;
        if self.alokasi_sejak_gc >= self.ambang_batas_gc {
            self.kumpulkan_sampah();
        }
    }

    pub fn kumpulkan_sampah(&mut self) {
        let mut marked_arrays = HashSet::new();
        let mut marked_hashes = HashSet::new();
        let mut marked_upvalues = HashSet::new();

        // 1. Kumpulkan semua objek "akar" (Roots)
        let mut worklist = Vec::new();
        worklist.extend(self.stack.iter().cloned());
        worklist.extend(self.globals.iter().cloned());
        worklist.extend(self.konstanta.iter().cloned());
        if let Some(terakhir) = &self.terakhir_dibuang {
            worklist.push(terakhir.clone());
        }
        for frame in &self.frames {
            worklist.push(Object::Closure(frame.closure.clone()));
        }
        for uv in &self.upvalues_terbuka {
            worklist.push(Object::Upvalue(uv.clone()));
        }

        // 2. Tandai (Mark) semua objek yang dapat dijangkau
        while let Some(obj) = worklist.pop() {
            match obj {
                Object::Array(arr) => {
                    let ptr = Rc::as_ptr(&arr) as usize;
                    if marked_arrays.insert(ptr) {
                        for item in arr.borrow().iter() {
                            worklist.push(item.clone());
                        }
                    }
                }
                Object::Hash(hash) => {
                    let ptr = Rc::as_ptr(&hash) as usize;
                    if marked_hashes.insert(ptr) {
                        for item in hash.borrow().values() {
                            worklist.push(item.clone());
                        }
                    }
                }
                Object::Closure(c) => {
                    for uv in &c.upvalues {
                        worklist.push(Object::Upvalue(uv.clone()));
                    }
                }
                Object::Upvalue(uv) => {
                    let ptr = Rc::as_ptr(&uv) as usize;
                    if marked_upvalues.insert(ptr) {
                        match &*uv.borrow() {
                            ObjekUpvalue::Closed(o) => worklist.push(o.clone()),
                            ObjekUpvalue::Open(_si) => {
                                // Jika terbuka, nilainya ada di stack yang sudah jadi akar.
                                // Tapi demi keamanan, kita tidak push ulang untuk mencegah loop tak terhingga.
                            }
                        }
                    }
                }
                Object::ReturnValue(rv) => {
                    worklist.push(*rv);
                }
                // Tipe dasar tidak berisi referensi ke objek koleksi, hentikan penelusuran
                _ => {}
            }
        }

        // 3. Sapu (Sweep) semua objek yang tidak ditandai
        // Ini akan memutus siklus (circular reference) sehingga Rc drop handler bisa bekerja penuh
        let _sebelum = self.gc_arrays.len() + self.gc_hashes.len() + self.gc_upvalues.len();

        self.gc_arrays.retain(|arr| {
            if marked_arrays.contains(&(Rc::as_ptr(arr) as usize)) {
                true
            } else {
                arr.borrow_mut().clear(); // Putus siklus
                false
            }
        });

        self.gc_hashes.retain(|hash| {
            if marked_hashes.contains(&(Rc::as_ptr(hash) as usize)) {
                true
            } else {
                hash.borrow_mut().clear(); // Putus siklus
                false
            }
        });

        self.gc_upvalues.retain(|uv| {
            if marked_upvalues.contains(&(Rc::as_ptr(uv) as usize)) {
                true
            } else {
                *uv.borrow_mut() = ObjekUpvalue::Closed(Object::Null); // Putus siklus
                false
            }
        });

        let sesudah = self.gc_arrays.len() + self.gc_hashes.len() + self.gc_upvalues.len();
        self.alokasi_sejak_gc = 0;

        // Gandakan ambang batas jika memori aktif tinggi untuk mencegah GC jalan terus menerus
        self.ambang_batas_gc = (sesudah * 2).max(1000);
    }
}

fn kondisi_benar(obj: &Object) -> bool {
    match obj {
        Object::Null => false,
        Object::Boolean(b) => *b,
        _ => true,
    }
}

impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Object::Integer(a), Object::Integer(b)) => a == b,
            (Object::Boolean(a), Object::Boolean(b)) => a == b,
            (Object::Str(a), Object::Str(b)) => a == b,
            (Object::Null, Object::Null) => true,
            _ => false,
        }
    }
}
