/// Modul Object untuk bahasa Taji.
///
/// Mendefinisikan semua tipe data yang bisa "hidup" di dalam
/// memori saat program Taji sedang dieksekusi oleh Evaluator.

use crate::ast::{BlockStatement, Identifier};
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

// ═══════════════════════════════════════════════════════════
//  Object — Representasi nilai di memori
// ═══════════════════════════════════════════════════════════

/// Semua tipe objek yang dikenali oleh runtime Taji.
#[derive(Debug, Clone)]
pub enum Object {
    /// Angka bulat: `42`, `-7`, `0`
    Integer(i64),

    /// Angka desimal: `3.14` (reserved)
    Float(f64),

    /// Nilai boolean: `benar` / `salah`
    Boolean(bool),

    /// Teks (string): `"halo dunia"`
    Str(String),

    /// Nilai "kosong" / tidak ada
    Null,

    /// Pembungkus nilai kembalian dari fungsi.
    ReturnValue(Box<Object>),

    /// Objek kesalahan runtime.
    Error(String),

    /// Fungsi yang didefinisikan pengguna.
    Function(FunctionObject),

    /// Fungsi bawaan (built-in) sistem.
    Builtin(BuiltinFunction),

    /// Daftar (array): `[1, 2, 3]`
    Array(Vec<Object>),

    /// Kamus (hash map): `{"kunci": "nilai"}`
    Hash(HashMap<HashKey, Object>),
}

impl Object {
    /// Mengembalikan nama tipe objek dalam Bahasa Indonesia.
    pub fn type_name(&self) -> &str {
        match self {
            Object::Integer(_) => "BILANGAN",
            Object::Float(_) => "DESIMAL",
            Object::Boolean(_) => "BOOLEAN",
            Object::Str(_) => "TEKS",
            Object::Null => "KOSONG",
            Object::ReturnValue(_) => "NILAI_KEMBALI",
            Object::Error(_) => "KESALAHAN",
            Object::Function(_) => "FUNGSI",
            Object::Builtin(_) => "FUNGSI_BAWAAN",
            Object::Array(_) => "DAFTAR",
            Object::Hash(_) => "KAMUS",
        }
    }

    /// Memeriksa apakah objek ini adalah error.
    pub fn is_error(&self) -> bool {
        matches!(self, Object::Error(_))
    }

    /// Mengonversi objek menjadi HashKey jika memungkinkan.
    pub fn to_hash_key(&self) -> Option<HashKey> {
        match self {
            Object::Integer(val) => Some(HashKey::Integer(*val)),
            Object::Boolean(val) => Some(HashKey::Boolean(*val)),
            Object::Str(val) => Some(HashKey::Str(val.clone())),
            _ => None,
        }
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Object::Integer(val) => write!(f, "{}", val),
            Object::Float(val) => write!(f, "{}", val),
            Object::Boolean(val) => {
                write!(f, "{}", if *val { "benar" } else { "salah" })
            }
            Object::Str(val) => write!(f, "{}", val),
            Object::Null => write!(f, "kosong"),
            Object::ReturnValue(val) => write!(f, "{}", val),
            Object::Error(msg) => write!(f, "KESALAHAN: {}", msg),
            Object::Function(func) => {
                let params: Vec<String> =
                    func.parameters.iter().map(|p| p.value.clone()).collect();
                write!(f, "fungsi({}) {{ ... }}", params.join(", "))
            }
            Object::Builtin(_) => write!(f, "fungsi bawaan"),
            Object::Array(elements) => {
                let elems: Vec<String> = elements.iter().map(|e| e.to_string()).collect();
                write!(f, "[{}]", elems.join(", "))
            }
            Object::Hash(pairs) => {
                let ps: Vec<String> = pairs
                    .iter()
                    .map(|(k, v)| format!("{}: {}", k, v))
                    .collect();
                write!(f, "{{{}}}", ps.join(", "))
            }
        }
    }
}

// ═══════════════════════════════════════════════════════════
//  Function Object
// ═══════════════════════════════════════════════════════════

/// Objek fungsi yang didefinisikan pengguna.
#[derive(Debug, Clone)]
pub struct FunctionObject {
    pub parameters: Vec<Identifier>,
    pub body: BlockStatement,
    pub env: Environment,
}

// ═══════════════════════════════════════════════════════════
//  Builtin Function
// ═══════════════════════════════════════════════════════════

/// Tipe untuk fungsi bawaan sistem.
pub type BuiltinFunction = fn(Vec<Object>) -> Object;

// ═══════════════════════════════════════════════════════════
//  Hash Key — Kunci untuk Kamus
// ═══════════════════════════════════════════════════════════

/// Kunci yang bisa digunakan dalam Kamus (HashMap).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HashKey {
    Integer(i64),
    Boolean(bool),
    Str(String),
}

impl fmt::Display for HashKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HashKey::Integer(val) => write!(f, "{}", val),
            HashKey::Boolean(val) => {
                write!(f, "{}", if *val { "benar" } else { "salah" })
            }
            HashKey::Str(val) => write!(f, "\"{}\"", val),
        }
    }
}

// ═══════════════════════════════════════════════════════════
//  Environment — Lingkup Variabel (Shared Reference)
// ═══════════════════════════════════════════════════════════

/// Lingkup (scope) penyimpanan variabel.
///
/// Menggunakan `Rc<RefCell<...>>` agar fungsi yang menangkap
/// environment (closure) bisa berbagi referensi dengan lingkup
/// tempat mereka didefinisikan. Ini memungkinkan **rekursi**:
/// saat fungsi disimpan ke variabel, closure-nya otomatis bisa
/// melihat variabel tersebut karena berbagi memori yang sama.
#[derive(Debug, Clone)]
pub struct Environment {
    store: Rc<RefCell<HashMap<String, Object>>>,
    outer: Option<Box<Environment>>,
}

impl Environment {
    /// Membuat lingkup baru (global / top-level).
    pub fn new() -> Self {
        Environment {
            store: Rc::new(RefCell::new(HashMap::new())),
            outer: None,
        }
    }

    /// Membuat lingkup baru yang "mewarisi" lingkup luar.
    /// Digunakan saat memasuki blok fungsi.
    pub fn new_enclosed(outer: Environment) -> Self {
        Environment {
            store: Rc::new(RefCell::new(HashMap::new())),
            outer: Some(Box::new(outer)),
        }
    }

    /// Mengambil nilai variabel dari lingkup.
    /// Jika tidak ditemukan, cari di lingkup induk.
    pub fn get(&self, name: &str) -> Option<Object> {
        match self.store.borrow().get(name) {
            Some(obj) => Some(obj.clone()),
            None => match &self.outer {
                Some(outer) => outer.get(name),
                None => None,
            },
        }
    }

    /// Menyimpan nilai variabel ke lingkup saat ini.
    pub fn set(&mut self, name: String, val: Object) -> Object {
        self.store.borrow_mut().insert(name, val.clone());
        val
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}
