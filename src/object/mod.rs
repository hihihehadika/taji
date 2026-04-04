/// Modul Object untuk bahasa Taji.

use crate::ast::{BlockStatement, Identifier};
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

// ═══════════════════════════════════════════════════════════
//  Object
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
    Function(FunctionObject),
    Builtin(BuiltinFunction),
    Array(Vec<Object>),
    Hash(HashMap<HashKey, Object>),
    /// Signal untuk `berhenti` (break) dalam loop.
    Break,
    /// Signal untuk `lanjut` (continue) dalam loop.
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
            Object::Function(_) => "FUNGSI",
            Object::Builtin(_) => "FUNGSI_BAWAAN",
            Object::Array(_) => "DAFTAR",
            Object::Hash(_) => "KAMUS",
            Object::Break => "BERHENTI",
            Object::Continue => "LANJUT",
        }
    }

    pub fn is_error(&self) -> bool {
        matches!(self, Object::Error(_))
    }

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
            Object::Float(val) => {
                // Tampilkan desimal dengan rapi (hilangkan trailing zeros)
                if val.fract() == 0.0 {
                    write!(f, "{:.1}", val)
                } else {
                    write!(f, "{}", val)
                }
            }
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
            Object::Break => write!(f, "berhenti"),
            Object::Continue => write!(f, "lanjut"),
        }
    }
}

// ═══════════════════════════════════════════════════════════
//  Function Object
// ═══════════════════════════════════════════════════════════

#[derive(Debug, Clone)]
pub struct FunctionObject {
    pub parameters: Vec<Identifier>,
    pub body: BlockStatement,
    pub env: Environment,
}

// ═══════════════════════════════════════════════════════════
//  Builtin Function
// ═══════════════════════════════════════════════════════════

pub type BuiltinFunction = fn(Vec<Object>) -> Object;

// ═══════════════════════════════════════════════════════════
//  Hash Key
// ═══════════════════════════════════════════════════════════

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
//  Environment
// ═══════════════════════════════════════════════════════════

#[derive(Debug, Clone)]
pub struct Environment {
    store: Rc<RefCell<HashMap<String, Object>>>,
    outer: Option<Box<Environment>>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            store: Rc::new(RefCell::new(HashMap::new())),
            outer: None,
        }
    }

    pub fn new_enclosed(outer: Environment) -> Self {
        Environment {
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

    /// Update variabel yang sudah ada (cari di scope chain).
    /// Digunakan untuk assignment (x = 5, x += 3).
    pub fn update(&mut self, name: &str, val: Object) -> Option<Object> {
        if self.store.borrow().contains_key(name) {
            self.store.borrow_mut().insert(name.to_string(), val.clone());
            return Some(val);
        }
        if let Some(outer) = &mut self.outer {
            return outer.update(name, val);
        }
        None
    }

    /// Mengambil semua variabel dari lingkup saat ini (tanpa outer).
    /// Digunakan untuk sistem `masukkan` (import).
    pub fn get_all_local(&self) -> HashMap<HashKey, Object> {
        let mut result = HashMap::new();
        for (key, val) in self.store.borrow().iter() {
            result.insert(HashKey::Str(key.clone()), val.clone());
        }
        result
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}
