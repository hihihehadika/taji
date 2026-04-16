//! Modul Objek untuk bahasa Taji.

use crate::ast::{BlokPernyataan, Pengenal};
use crate::code::Bytecode;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

// ═══════════════════════════════════════════════════════════
//  Objek
// ═══════════════════════════════════════════════════════════

#[derive(Debug, Clone)]
pub enum Object {
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Str(String),
    Null,
    ReturnValue(Box<Object>),
    Error(String),
    Fungsi(ObjekFungsi),
    FungsiVM(ObjekFungsiTerkompilasi),
    Closure(ObjekClosure),
    Upvalue(Rc<RefCell<ObjekUpvalue>>),
    Bawaan(FungsiBawaan),
    Array(Rc<RefCell<Vec<Object>>>),
    Hash(Rc<RefCell<HashMap<KunciKamus, Object>>>),
    Break,
    Continue,
}

impl Object {
    pub fn type_name(&self) -> &str {
        match self {
            Object::Integer(_) => "BILANGAN",
            Object::Float(_) => "DESIMAL",
            Object::Boolean(_) => "BOOLEAN",
            Object::Str(_) => "TEKS",
            Object::Null => "KOSONG",
            Object::ReturnValue(_) => "NILAI_KEMBALI",
            Object::Error(_) => "KESALAHAN",
            Object::Fungsi(_) | Object::Closure(_) | Object::FungsiVM(_) => "FUNGSI",
            Object::Upvalue(_) => "UPVALUE",
            Object::Bawaan(_) => "FUNGSI_BAWAAN",
            Object::Array(_) => "DAFTAR",
            Object::Hash(_) => "KAMUS",
            Object::Break => "BERHENTI",
            Object::Continue => "LANJUT",
        }
    }

    pub fn is_error(&self) -> bool {
        matches!(self, Object::Error(_))
    }

    pub fn to_hash_key(&self) -> Option<KunciKamus> {
        match self {
            Object::Integer(val) => Some(KunciKamus::Integer(*val)),
            Object::Boolean(val) => Some(KunciKamus::Boolean(*val)),
            Object::Str(val) => Some(KunciKamus::Str(val.clone())),
            _ => None,
        }
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Object::Integer(val) => write!(f, "{}", val),
            Object::Float(val) => {
                if val.fract() == 0.0 {
                    write!(f, "{:.1}", val)
                } else {
                    write!(f, "{}", val)
                }
            }
            Object::Boolean(val) => write!(f, "{}", if *val { "benar" } else { "salah" }),
            Object::Str(val) => write!(f, "{}", val),
            Object::Null => write!(f, "kosong"),
            Object::ReturnValue(val) => write!(f, "{}", val),
            Object::Error(msg) => write!(f, "KESALAHAN: {}", msg),
            Object::Fungsi(_) | Object::FungsiVM(_) | Object::Closure(_) => write!(f, "<fungsi>"),
            Object::Upvalue(_) => write!(f, "<upvalue>"),
            Object::Bawaan(_) => write!(f, "fungsi bawaan"),
            Object::Array(elements) => {
                let elems: Vec<String> = elements.borrow().iter().map(|e| e.to_string()).collect();
                write!(f, "[{}]", elems.join(", "))
            }
            Object::Hash(pairs) => {
                let ps: Vec<String> = pairs
                    .borrow()
                    .iter()
                    .map(|(k, v)| format!("{}: {}", k, v))
                    .collect();
                write!(f, "{{{}}}", ps.join(", "))
            }
            Object::Break => write!(f, "berhenti"),
            Object::Continue => write!(f, "lanjut"),
        }
    }
}

// ═══════════════════════════════════════════════════════════
//  Closures & Upvalues
// ═══════════════════════════════════════════════════════════

#[derive(Debug, Clone)]
pub struct ObjekClosure {
    pub fungsi: ObjekFungsiTerkompilasi,
    pub upvalues: Vec<Rc<RefCell<ObjekUpvalue>>>,
}

#[derive(Debug, Clone)]
pub enum ObjekUpvalue {
    /// Masih di stack: menyimpan indeks ke stack VM.
    Open(usize),
    /// Sudah di heap: menyimpan nilai obyek secara mandiri.
    Closed(Object),
}

#[derive(Debug, Clone)]
pub struct ObjekFungsiTerkompilasi {
    pub instruksi: Bytecode,
    pub jumlah_parameter: usize,
    pub jumlah_lokal: usize,
    pub nama: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ObjekFungsi {
    pub parameters: Vec<Pengenal>,
    pub body: BlokPernyataan,
    pub env: Lingkungan,
}

pub type FungsiBawaan = fn(Vec<Object>) -> Object;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum KunciKamus {
    Integer(i64),
    Boolean(bool),
    Str(String),
}

impl fmt::Display for KunciKamus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KunciKamus::Integer(val) => write!(f, "{}", val),
            KunciKamus::Boolean(val) => write!(f, "{}", if *val { "benar" } else { "salah" }),
            KunciKamus::Str(val) => write!(f, "\"{}\"", val),
        }
    }
}

// ═══════════════════════════════════════════════════════════
//  Lingkungan (Scope Chain - Bridge untuk Evaluator Lama)
// ═══════════════════════════════════════════════════════════

#[derive(Debug, Clone)]
pub struct Lingkungan {
    store: Rc<RefCell<HashMap<String, Object>>>,
    outer: Option<Box<Lingkungan>>,
}

impl Lingkungan {
    pub fn new() -> Self {
        Lingkungan {
            store: Rc::new(RefCell::new(HashMap::new())),
            outer: None,
        }
    }

    pub fn new_enclosed(outer: Lingkungan) -> Self {
        Lingkungan {
            store: Rc::new(RefCell::new(HashMap::new())),
            outer: Some(Box::new(outer)),
        }
    }

    pub fn get(&self, name: &str) -> Option<Object> {
        match self.store.borrow().get(name) {
            Some(obj) => Some(obj.clone()),
            None => match &self.outer {
                Some(outer) => outer.get(name),
                None => None,
            },
        }
    }

    pub fn set(&mut self, name: String, val: Object) -> Object {
        self.store.borrow_mut().insert(name, val.clone());
        val
    }

    pub fn update(&mut self, name: &str, val: Object) -> Option<Object> {
        if self.store.borrow().contains_key(name) {
            self.store
                .borrow_mut()
                .insert(name.to_string(), val.clone());
            return Some(val);
        }
        if let Some(outer) = &mut self.outer {
            return outer.update(name, val);
        }
        None
    }

    pub fn get_all_local(&self) -> HashMap<KunciKamus, Object> {
        let mut result = HashMap::new();
        for (key, val) in self.store.borrow().iter() {
            result.insert(KunciKamus::Str(key.clone()), val.clone());
        }
        result
    }
}

impl Default for Lingkungan {
    fn default() -> Self {
        Self::new()
    }
}
