//! Modul `bawaan`: Registri Fungsi Bawaan Taji.

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[cfg(not(target_arch = "wasm32"))]
use std::io::{self, Write};

#[cfg(not(target_arch = "wasm32"))]
use std::time::{SystemTime, UNIX_EPOCH};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
use js_sys;

use crate::object::{KunciKamus, Object};

pub fn cari_bawaan(nama: &str) -> Option<Object> {
    match nama {
        "cetak" => Some(Object::Bawaan(builtin_cetak)),
        "panjang" => Some(Object::Bawaan(builtin_panjang)),
        "tipe" => Some(Object::Bawaan(builtin_tipe)),
        "dorong" => Some(Object::Bawaan(builtin_dorong)),
        "pertama" => Some(Object::Bawaan(builtin_pertama)),
        "terakhir" => Some(Object::Bawaan(builtin_terakhir)),
        "sisa" => Some(Object::Bawaan(builtin_sisa)),
        "tanya" => Some(Object::Bawaan(builtin_tanya)),
        "waktu" => Some(Object::Bawaan(builtin_waktu)),
        "teks" => Some(Object::Bawaan(builtin_teks)),
        "angka" => Some(Object::Bawaan(builtin_angka)),
        "pisah" => Some(Object::Bawaan(builtin_pisah)),
        "gabung" => Some(Object::Bawaan(builtin_gabung)),
        "baca_berkas" => Some(Object::Bawaan(builtin_baca_berkas)),
        "tulis_berkas" => Some(Object::Bawaan(builtin_tulis_berkas)),
        "format" => Some(Object::Bawaan(builtin_format)),
        "dari_json" => Some(Object::Bawaan(builtin_dari_json)),
        "ke_json" => Some(Object::Bawaan(builtin_ke_json)),
        "potong" => Some(Object::Bawaan(builtin_potong)),
        "ganti" => Some(Object::Bawaan(builtin_ganti)),
        "huruf_besar" => Some(Object::Bawaan(builtin_huruf_besar)),
        "huruf_kecil" => Some(Object::Bawaan(builtin_huruf_kecil)),
        "berisi" => Some(Object::Bawaan(builtin_berisi)),
        "jeda" => Some(Object::Bawaan(builtin_jeda)),
        "acak" => Some(Object::Bawaan(builtin_acak)),
        "masukkan" => Some(Object::Bawaan(builtin_masukkan)),
        "ambil_web" => Some(Object::Bawaan(builtin_ambil_web)),
        _ => None,
    }
}

pub const NAMA_BAWAAN: &[&str] = &[
    "cetak",
    "panjang",
    "tipe",
    "dorong",
    "pertama",
    "terakhir",
    "sisa",
    "tanya",
    "waktu",
    "teks",
    "angka",
    "pisah",
    "gabung",
    "baca_berkas",
    "tulis_berkas",
    "format",
    "dari_json",
    "ke_json",
    "potong",
    "ganti",
    "huruf_besar",
    "huruf_kecil",
    "berisi",
    "jeda",
    "acak",
    "masukkan",
    "ambil_web",
];

pub fn bikin_tabel_awal() -> crate::compiler::tabel_simbol::TabelSimbol {
    let mut tabel = crate::compiler::tabel_simbol::TabelSimbol::new();
    for nama in NAMA_BAWAAN {
        let _ = tabel.definisikan(nama);
    }
    tabel
}

pub fn bikin_globals_awal() -> Vec<Object> {
    NAMA_BAWAAN
        .iter()
        .map(|n| cari_bawaan(n).unwrap())
        .collect()
}

fn builtin_cetak(args: Vec<Object>) -> Object {
    for arg in &args {
        crate::keluaran::cetak_keluar(&format!("{}", arg));
    }
    Object::Null
}

fn builtin_panjang(args: Vec<Object>) -> Object {
    if args.len() != 1 {
        return Object::Error("panjang: butuh 1 argumen".to_string());
    }
    match &args[0] {
        Object::Str(s) => Object::Integer(s.chars().count() as i64),
        Object::Array(a) => Object::Integer(a.borrow().len() as i64),
        Object::Hash(h) => Object::Integer(h.borrow().len() as i64),
        _ => Object::Error("panjang: tipe tidak valid".to_string()),
    }
}

fn builtin_tipe(args: Vec<Object>) -> Object {
    if args.is_empty() {
        return Object::Null;
    }
    Object::Str(args[0].type_name().to_string())
}

fn builtin_dorong(args: Vec<Object>) -> Object {
    if args.len() != 2 {
        return Object::Error("dorong: butuh 2 argumen".to_string());
    }
    match &args[0] {
        Object::Array(arr) => {
            arr.borrow_mut().push(args[1].clone());
            args[0].clone()
        }
        _ => Object::Error("dorong: hanya untuk DAFTAR".to_string()),
    }
}

fn builtin_pertama(args: Vec<Object>) -> Object {
    if args.len() != 1 {
        return Object::Error("pertama: butuh 1 argumen".to_string());
    }
    match &args[0] {
        Object::Array(arr) => {
            let a = arr.borrow();
            if a.is_empty() {
                Object::Null
            } else {
                a[0].clone()
            }
        }
        _ => Object::Error("pertama: hanya untuk DAFTAR".to_string()),
    }
}

fn builtin_terakhir(args: Vec<Object>) -> Object {
    if args.len() != 1 {
        return Object::Error("terakhir: butuh 1 argumen".to_string());
    }
    match &args[0] {
        Object::Array(arr) => {
            let a = arr.borrow();
            if a.is_empty() {
                Object::Null
            } else {
                a[a.len() - 1].clone()
            }
        }
        _ => Object::Error("terakhir: hanya untuk DAFTAR".to_string()),
    }
}

fn builtin_sisa(args: Vec<Object>) -> Object {
    if args.len() != 1 {
        return Object::Error("sisa: butuh 1 argumen".to_string());
    }
    match &args[0] {
        Object::Array(arr) => {
            let a = arr.borrow();
            if a.is_empty() {
                Object::Array(Rc::new(RefCell::new(vec![])))
            } else {
                Object::Array(Rc::new(RefCell::new(a[1..].to_vec())))
            }
        }
        _ => Object::Error("sisa: hanya untuk DAFTAR".to_string()),
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn builtin_tanya(args: Vec<Object>) -> Object {
    if !args.is_empty() {
        print!("{}", args[0]);
        io::stdout().flush().ok();
    }
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => Object::Str(input.trim().to_string()),
        Err(e) => Object::Error(e.to_string()),
    }
}

// Implementasi tanya() untuk peramban menggunakan antrian masukan.
// Alih-alih memunculkan window.prompt() yang membekukan UI dan terasa
// amatir, fungsi ini membaca input dari buffer antrian yang telah diisi
// oleh frontend sebelum eksekusi dimulai. Prompt dan jawaban dicetak
// ke panel output agar alur interaksi terlihat natural layaknya terminal.
#[cfg(target_arch = "wasm32")]
fn builtin_tanya(args: Vec<Object>) -> Object {
    let msg = if !args.is_empty() {
        args[0].to_string()
    } else {
        "".to_string()
    };

    // Cetak prompt ke panel output (seperti terminal menampilkan pertanyaan)
    if !msg.is_empty() {
        crate::keluaran::cetak_keluar(&format!("{} ", msg));
    }

    // Ambil jawaban dari antrian masukan
    let jawaban = crate::masukan::ambil_masukan().unwrap_or_default();

    // Cetak jawaban ke panel output agar terlihat seolah diketik pengguna
    crate::keluaran::cetak_keluar(&jawaban);

    Object::Str(jawaban)
}

#[cfg(not(target_arch = "wasm32"))]
fn builtin_waktu(_: Vec<Object>) -> Object {
    Object::Integer(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as i64,
    )
}

#[cfg(target_arch = "wasm32")]
fn builtin_waktu(_: Vec<Object>) -> Object {
    Object::Integer(js_sys::Date::now() as i64)
}

fn builtin_teks(args: Vec<Object>) -> Object {
    if args.is_empty() {
        return Object::Str("".to_string());
    }
    Object::Str(args[0].to_string())
}

fn builtin_angka(args: Vec<Object>) -> Object {
    if args.is_empty() {
        return Object::Integer(0);
    }
    match &args[0] {
        Object::Str(s) => {
            if let Ok(i) = s.parse::<i64>() {
                Object::Integer(i)
            } else if let Ok(f) = s.parse::<f64>() {
                Object::Float(f)
            } else {
                Object::Error("bukan angka".to_string())
            }
        }
        Object::Integer(_) | Object::Float(_) => args[0].clone(),
        _ => Object::Error("tidak bisa dikonversi ke angka".to_string()),
    }
}

fn builtin_pisah(args: Vec<Object>) -> Object {
    if args.len() != 2 {
        return Object::Error("jumlah argumen untuk 'pisah' salah".to_string());
    }
    if args[0].type_name() != "TEKS" {
        return Object::Error("argumen pertama untuk 'pisah' harus TEKS".to_string());
    }
    let (s, p) = (args[0].to_string(), args[1].to_string());
    let res: Vec<Object> = s
        .split(&p)
        .map(|part| Object::Str(part.to_string()))
        .collect();
    Object::Array(Rc::new(RefCell::new(res)))
}

fn builtin_gabung(args: Vec<Object>) -> Object {
    if args.len() != 2 {
        return Object::Error("jumlah argumen untuk 'gabung' salah".to_string());
    }
    match &args[0] {
        Object::Array(arr) => {
            let p = args[1].to_string();
            let res: Vec<String> = arr.borrow().iter().map(|o| o.to_string()).collect();
            Object::Str(res.join(&p))
        }
        _ => Object::Error("argumen pertama untuk 'gabung' harus DAFTAR".to_string()),
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn builtin_baca_berkas(args: Vec<Object>) -> Object {
    if args.is_empty() {
        return Object::Error("gagal membaca berkas: butuh jalur".to_string());
    }
    match std::fs::read_to_string(args[0].to_string()) {
        Ok(c) => Object::Str(c),
        Err(_) => Object::Error("gagal membaca berkas".to_string()),
    }
}

// Implementasi VFS (Virtual File System) berbasis localStorage untuk peramban
#[cfg(target_arch = "wasm32")]
fn builtin_baca_berkas(args: Vec<Object>) -> Object {
    if args.is_empty() {
        return Object::Error("baca_berkas: butuh jalur berkas".to_string());
    }
    let jalur = args[0].to_string().replace('"', "\\\"").replace('`', "\\`");
    let skrip = format!(
        "(function() {{ \
            try {{ \
                var d = localStorage.getItem('taji_vfs:{jalur}'); \
                return d !== null ? d : '__TAJI_VFS_TIDAK_ADA__'; \
            }} catch(e) {{ \
                return '__TAJI_VFS_GALAT__:' + e.message; \
            }} \
        }})()"
    );
    match js_sys::eval(&skrip) {
        Ok(val) => match val.as_string() {
            Some(s) if s == "__TAJI_VFS_TIDAK_ADA__" => {
                Object::Error(format!("baca_berkas: '{}' tidak ditemukan di VFS", args[0]))
            }
            Some(s) if s.starts_with("__TAJI_VFS_GALAT__:") => {
                Object::Error(format!("baca_berkas gagal: {}", &s[19..]))
            }
            Some(s) => Object::Str(s),
            None => Object::Error("baca_berkas: gagal membaca VFS".to_string()),
        },
        Err(_) => Object::Error("baca_berkas: error mengakses localStorage".to_string()),
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn builtin_tulis_berkas(args: Vec<Object>) -> Object {
    if args.len() < 2 {
        return Object::Error("gagal menulis berkas: butuh jalur dan isi".to_string());
    }
    match std::fs::write(args[0].to_string(), args[1].to_string()) {
        Ok(_) => Object::Str("berkas berhasil menulis".to_string()),
        Err(e) => Object::Error(format!("gagal menulis berkas: {}", e)),
    }
}

// Menulis ke VFS (localStorage) di peramban
#[cfg(target_arch = "wasm32")]
fn builtin_tulis_berkas(args: Vec<Object>) -> Object {
    if args.len() < 2 {
        return Object::Error("tulis_berkas: butuh jalur dan isi".to_string());
    }
    let jalur = args[0].to_string().replace('"', "\\\"").replace('`', "\\`");
    // Enkode isi sebagai JSON string untuk menghindari masalah escaping
    let isi_json = match serde_json::to_string(&args[1].to_string()) {
        Ok(j) => j,
        Err(e) => return Object::Error(format!("tulis_berkas: gagal enkode isi: {}", e)),
    };
    let skrip = format!(
        "(function() {{ \
            try {{ \
                localStorage.setItem('taji_vfs:{jalur}', JSON.parse({isi_json})); \
                return '__TAJI_VFS_OK__'; \
            }} catch(e) {{ \
                return '__TAJI_VFS_GALAT__:' + e.message; \
            }} \
        }})()"
    );
    match js_sys::eval(&skrip) {
        Ok(val) => match val.as_string() {
            Some(s) if s == "__TAJI_VFS_OK__" => {
                Object::Str("berkas berhasil ditulis ke VFS peramban".to_string())
            }
            Some(s) if s.starts_with("__TAJI_VFS_GALAT__:") => {
                Object::Error(format!("tulis_berkas gagal: {}", &s[19..]))
            }
            _ => Object::Str("berkas berhasil ditulis ke VFS peramban".to_string()),
        },
        Err(_) => Object::Error("tulis_berkas: error mengakses localStorage".to_string()),
    }
}

fn builtin_format(args: Vec<Object>) -> Object {
    if args.is_empty() {
        return Object::Str("".to_string());
    }
    let mut res = args[0].to_string();
    for arg in args.into_iter().skip(1) {
        res = res.replacen("{}", &arg.to_string(), 1);
    }
    if res.contains("{}") {
        return Object::Error("format: argumen kurang".to_string());
    }
    Object::Str(res)
}

fn builtin_dari_json(args: Vec<Object>) -> Object {
    if args.is_empty() {
        return Object::Null;
    }
    parse_json_ke_object(&args[0].to_string())
}

fn parse_json_ke_object(s: &str) -> Object {
    let s = s.trim();
    if s == "null" {
        return Object::Null;
    }
    if s == "true" {
        return Object::Boolean(true);
    }
    if s == "false" {
        return Object::Boolean(false);
    }
    if let Ok(n) = s.parse::<i64>() {
        return Object::Integer(n);
    }
    if let Ok(f) = s.parse::<f64>() {
        return Object::Float(f);
    }
    if s.starts_with('"') && s.ends_with('"') {
        return Object::Str(s[1..s.len() - 1].to_string());
    }
    if s.starts_with('[') && s.ends_with(']') {
        let inner = s[1..s.len() - 1].trim();
        if inner.is_empty() {
            return Object::Array(Rc::new(RefCell::new(vec![])));
        }
        let elements: Vec<Object> = split_json_toplevel(inner, ',')
            .iter()
            .map(|p| parse_json_ke_object(p))
            .collect();
        return Object::Array(Rc::new(RefCell::new(elements)));
    }
    if s.starts_with('{') && s.ends_with('}') {
        let inner = s[1..s.len() - 1].trim();
        if inner.is_empty() {
            return Object::Hash(Rc::new(RefCell::new(HashMap::new())));
        }
        let mut map = HashMap::new();
        for pair in split_json_toplevel(inner, ',') {
            let parts: Vec<&str> = pair.splitn(2, ':').collect();
            if parts.len() == 2 {
                let k = parts[0].trim().trim_matches('"').to_string();
                let v = parse_json_ke_object(parts[1]);
                map.insert(KunciKamus::Str(k), v);
            }
        }
        return Object::Hash(Rc::new(RefCell::new(map)));
    }
    Object::Error("format JSON tidak valid".to_string())
}

fn split_json_toplevel(s: &str, sep: char) -> Vec<String> {
    let (mut res, mut curr, mut depth, mut in_str) = (Vec::new(), String::new(), 0, false);
    for c in s.chars() {
        if c == '"' {
            in_str = !in_str;
        }
        if !in_str {
            match c {
                '[' | '{' => depth += 1,
                ']' | '}' => depth -= 1,
                _ => {}
            }
        }
        if c == sep && depth == 0 && !in_str {
            res.push(curr.clone());
            curr.clear();
        } else {
            curr.push(c);
        }
    }
    res.push(curr);
    res.into_iter()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

fn builtin_ke_json(args: Vec<Object>) -> Object {
    if args.is_empty() {
        return Object::Str("null".to_string());
    }

    let pretty = args
        .get(1)
        .is_some_and(|o| matches!(o, Object::Boolean(true)));

    let res = if pretty {
        format!("{}\n", object_ke_json_pretty(&args[0], 0))
    } else {
        object_ke_json(&args[0])
    };
    Object::Str(res)
}

fn object_ke_json(obj: &Object) -> String {
    match obj {
        Object::Null => "null".to_string(),
        Object::Boolean(b) => b.to_string(),
        Object::Integer(n) => n.to_string(),
        Object::Float(f) => f.to_string(),
        Object::Str(s) => format!("\"{}\"", s.replace("\"", "\\\"")),
        Object::Array(arr) => {
            let res: Vec<String> = arr.borrow().iter().map(object_ke_json).collect();
            format!("[{}]", res.join(","))
        }
        Object::Hash(h) => {
            let mut res: Vec<String> = h
                .borrow()
                .iter()
                .map(|(k, v)| {
                    let key_str = match k {
                        crate::object::KunciKamus::Str(s) => s.clone(),
                        crate::object::KunciKamus::Integer(i) => i.to_string(),
                        crate::object::KunciKamus::Boolean(b) => b.to_string(),
                    };
                    format!("\"{}\":{}", key_str, object_ke_json(v))
                })
                .collect();
            res.sort(); // Deterministic for tests
            format!("{{{}}}", res.join(","))
        }
        _ => "null".to_string(),
    }
}

fn object_ke_json_pretty(obj: &Object, level: usize) -> String {
    let indent = "  ".repeat(level);
    let next_indent = "  ".repeat(level + 1);

    match obj {
        Object::Null => "null".to_string(),
        Object::Boolean(b) => b.to_string(),
        Object::Integer(n) => n.to_string(),
        Object::Float(f) => f.to_string(),
        Object::Str(s) => format!("\"{}\"", s.replace("\"", "\\\"")),
        Object::Array(arr) => {
            let a = arr.borrow();
            if a.is_empty() {
                return "[]".to_string();
            }
            let res: Vec<String> = a
                .iter()
                .map(|e| format!("{}{}", next_indent, object_ke_json_pretty(e, level + 1)))
                .collect();
            format!("[\n{}\n{}]", res.join(",\n"), indent)
        }
        Object::Hash(h) => {
            let map = h.borrow();
            if map.is_empty() {
                return "{}".to_string();
            }
            let mut res: Vec<String> = map
                .iter()
                .map(|(k, v)| {
                    let key_str = match k {
                        crate::object::KunciKamus::Str(s) => s.clone(),
                        crate::object::KunciKamus::Integer(i) => i.to_string(),
                        crate::object::KunciKamus::Boolean(b) => b.to_string(),
                    };
                    format!(
                        "{}\"{}\": {}",
                        next_indent,
                        key_str,
                        object_ke_json_pretty(v, level + 1)
                    )
                })
                .collect();
            res.sort();
            format!("{{\n{}\n{}}}", res.join(",\n"), indent)
        }
        _ => "null".to_string(),
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn builtin_ambil_web(args: Vec<Object>) -> Object {
    if args.len() != 1 {
        return Object::Error("jumlah argumen salah".to_string());
    }
    let url = args[0].to_string();
    let agent = ureq::Agent::new_with_defaults();

    match agent.get(&url).call() {
        Ok(mut response) => {
            let mut s = String::new();
            use std::io::Read;
            // Kita ambil pembaca bodi dan baca ke string
            match response.body_mut().as_reader().read_to_string(&mut s) {
                Ok(_) => Object::Str(s),
                Err(e) => Object::Error(format!("gagal mengambil URL: {}", e)),
            }
        }
        Err(e) => Object::Error(format!("gagal mengambil URL: {}", e)),
    }
}

// Implementasi ambil_web via XMLHttpRequest sinkron untuk peramban.
// Catatan arsitektur: XHR sinkron memiliki keterbatasan pada peramban modern,
// terutama terkait kebijakan CORS (Cross-Origin Resource Sharing). Pesan galat
// dirancang agar informatif dan memandu pengguna memahami batasan ini.
#[cfg(target_arch = "wasm32")]
fn builtin_ambil_web(args: Vec<Object>) -> Object {
    if args.is_empty() {
        return Object::Error("ambil_web: butuh minimal 1 argumen (URL)".to_string());
    }
    let url_mentah = args[0].to_string();
    // Enkode URL sebagai JSON string untuk menghindari injeksi
    let url_json = match serde_json::to_string(&url_mentah) {
        Ok(j) => j,
        Err(_) => return Object::Error("ambil_web: URL tidak valid".to_string()),
    };
    let skrip = format!(
        "(function() {{ \
            try {{ \
                var xhr = new XMLHttpRequest(); \
                xhr.open('GET', {url_json}, false); \
                xhr.send(null); \
                if (xhr.status >= 200 && xhr.status < 300) {{ \
                    return xhr.responseText; \
                }} else if (xhr.status === 0) {{ \
                    return '__TAJI_HTTP_CORS__'; \
                }} else {{ \
                    return '__TAJI_HTTP_GALAT__:HTTP ' + xhr.status + ' ' + xhr.statusText; \
                }} \
            }} catch(e) {{ \
                if (e.name === 'NetworkError' || e.message.indexOf('CORS') !== -1 \
                    || e.message.indexOf('cross-origin') !== -1 \
                    || e.message.indexOf('network') !== -1) {{ \
                    return '__TAJI_HTTP_CORS__'; \
                }} \
                return '__TAJI_HTTP_GALAT__:' + e.message; \
            }} \
        }})()"
    );
    match js_sys::eval(&skrip) {
        Ok(val) => match val.as_string() {
            Some(s) if s == "__TAJI_HTTP_CORS__" => Object::Error(format!(
                "ambil_web gagal: Permintaan ke '{}' diblokir oleh kebijakan CORS peramban. \
                     Server tujuan tidak mengizinkan akses dari domain ini.",
                url_mentah
            )),
            Some(s) if s.starts_with("__TAJI_HTTP_GALAT__:") => {
                Object::Error(format!("ambil_web gagal: {}", &s[20..]))
            }
            Some(s) => Object::Str(s),
            None => Object::Error("ambil_web: respons kosong atau tidak valid".to_string()),
        },
        Err(_) => Object::Error("ambil_web: gagal menjalankan permintaan HTTP".to_string()),
    }
}

fn builtin_potong(args: Vec<Object>) -> Object {
    if args[0].type_name() != "TEKS" {
        return Object::Error("potong: argumen pertama harus TEKS".to_string());
    }
    let s = args[0].to_string();
    let a = match &args[1] {
        Object::Integer(i) => *i as usize,
        _ => return Object::Error("potong: indeks harus BILANGAN".to_string()),
    };
    let k = match &args[2] {
        Object::Integer(i) => *i as usize,
        _ => return Object::Error("potong: indeks harus BILANGAN".to_string()),
    };

    let ch: Vec<char> = s.chars().collect();
    if a > ch.len() || k > ch.len() || a > k {
        return Object::Error("potong: indeks di luar batas".to_string());
    }
    Object::Str(ch[a..k].iter().collect())
}

fn builtin_ganti(args: Vec<Object>) -> Object {
    if args.len() < 3 {
        return Object::Error("ganti: butuh 3 argumen".to_string());
    }
    Object::Str(
        args[0]
            .to_string()
            .replace(&args[1].to_string(), &args[2].to_string()),
    )
}

fn builtin_huruf_besar(args: Vec<Object>) -> Object {
    if args.is_empty() || args[0].type_name() != "TEKS" {
        return Object::Error("argumen untuk 'huruf_besar' harus TEKS".to_string());
    }
    Object::Str(args[0].to_string().to_uppercase())
}

fn builtin_huruf_kecil(args: Vec<Object>) -> Object {
    if args.is_empty() || args[0].type_name() != "TEKS" {
        return Object::Error("argumen untuk 'huruf_kecil' harus TEKS".to_string());
    }
    Object::Str(args[0].to_string().to_lowercase())
}

fn builtin_berisi(args: Vec<Object>) -> Object {
    if args.len() < 2 {
        return Object::Boolean(false);
    }
    match &args[0] {
        Object::Str(s) => Object::Boolean(s.contains(&args[1].to_string())),
        Object::Array(arr) => {
            let target = &args[1];
            Object::Boolean(arr.borrow().iter().any(|e| e == target))
        }
        _ => Object::Boolean(false),
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn builtin_jeda(args: Vec<Object>) -> Object {
    if let Some(Object::Integer(ms)) = args.first() {
        if *ms < 0 {
            return Object::Error("jeda: durasi tidak boleh negatif".to_string());
        }
        std::thread::sleep(std::time::Duration::from_millis(*ms as u64));
    }
    Object::Null
}

// Implementasi jeda untuk peramban menggunakan teknik busy-wait.
// Karena WASM berjalan di utas tunggal (single-threaded) dan tidak mendukung
// std::thread::sleep, kita menahan eksekusi VM secara paksa dengan
// membandingkan waktu saat ini terhadap waktu target menggunakan js_sys.
#[cfg(target_arch = "wasm32")]
fn builtin_jeda(args: Vec<Object>) -> Object {
    if let Some(Object::Integer(_ms)) = args.first() {
        // Di WebAssembly (Main Thread), kita abaikan jeda agar tidak membekukan UI
        // dengan busy-wait. Loop tak terbatas yang mengandalkan jeda akan tertangkap
        // oleh batas_instruksi VM.
    }
    Object::Null
}

#[cfg(not(target_arch = "wasm32"))]
fn builtin_acak(args: Vec<Object>) -> Object {
    use std::time::SystemTime;
    let seed = SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64;
    let mut r = seed;
    r ^= r << 13;
    r ^= r >> 7;
    r ^= r << 17; // Xorshift

    if args.len() == 2 {
        if let (Object::Integer(min), Object::Integer(max)) = (&args[0], &args[1]) {
            if min == max {
                return Object::Integer(*min);
            }
            if max < min {
                return Object::Error("rentang acak tidak valid".to_string());
            }
            return Object::Integer(min + (r % (*max - *min) as u64) as i64);
        }
    }
    Object::Integer((r >> 1) as i64)
}

#[cfg(target_arch = "wasm32")]
fn builtin_acak(args: Vec<Object>) -> Object {
    let r = js_sys::Math::random();
    if args.len() == 2 {
        if let (Object::Integer(min), Object::Integer(max)) = (&args[0], &args[1]) {
            if min == max {
                return Object::Integer(*min);
            }
            if max < min {
                return Object::Error("rentang acak tidak valid".to_string());
            }
            return Object::Integer(min + (r * (*max - *min) as f64) as i64);
        }
    }
    Object::Integer((r * i64::MAX as f64) as i64)
}

#[cfg(not(target_arch = "wasm32"))]
fn builtin_masukkan(args: Vec<Object>) -> Object {
    if args.len() != 1 {
        return Object::Error(
            "masukkan: butuh 1 argumen (nama_modul atau jalur_berkas)".to_string(),
        );
    }
    let masukan = match &args[0] {
        Object::Str(s) => s.clone(),
        _ => return Object::Error("masukkan: argumen harus TEKS".to_string()),
    };

    // Resolusi jalur berkas menggunakan sistem prioritas:
    //
    // 1. Jika argumen diakhiri '.tj' atau mengandung pemisah jalur, perlakukan
    //    sebagai jalur eksplisit. Coba apa adanya dulu, lalu coba tambah '.tj'
    //    jika belum ada ekstensi dan berkas tidak ditemukan.
    //
    // 2. Jika argumen adalah nama modul pendek (tanpa pemisah, tanpa ekstensi):
    //    a. Cari di direktori lokal saat ini: `./nama.tj`
    //    b. Cari di folder pustaka TPM:       `./taji_modul/nama.tj`
    //
    // 3. Jika semua gagal, lempar galat dengan panduan yang informatif.

    let ada_pemisah = masukan.contains('/') || masukan.contains('\\');
    let ada_ekstensi = masukan.ends_with(".tj");

    let jalur_final = if ada_ekstensi || ada_pemisah {
        // Jalur eksplisit — coba apa adanya terlebih dahulu
        if std::path::Path::new(&masukan).exists() {
            Some(masukan.clone())
        } else if !ada_ekstensi {
            // Berkas tidak ditemukan dan belum ada ekstensi — coba tambah .tj
            let dengan_ekstensi = format!("{}.tj", masukan);
            if std::path::Path::new(&dengan_ekstensi).exists() {
                Some(dengan_ekstensi)
            } else {
                None
            }
        } else {
            None
        }
    } else {
        // Nama modul pendek — resolusi otomatis dua lokasi
        let nama_berkas = format!("{}.tj", masukan);

        // Prioritas 1: Direktori lokal
        let jalur_lokal = format!("./{}", nama_berkas);
        if std::path::Path::new(&jalur_lokal).exists() {
            Some(jalur_lokal)
        } else {
            // Prioritas 2: Folder pustaka TPM (taji_modul/)
            let jalur_tpm = format!("./{}/{}", crate::tpm::DIREKTORI_MODUL, nama_berkas);
            if std::path::Path::new(&jalur_tpm).exists() {
                Some(jalur_tpm)
            } else {
                None
            }
        }
    };

    let jalur = match jalur_final {
        Some(j) => j,
        None => {
            // Bangun pesan galat yang informatif dan memandu pengguna
            let nama_berkas = if masukan.ends_with(".tj") {
                masukan.clone()
            } else {
                format!("{}.tj", masukan)
            };
            return Object::Error(format!(
                "masukkan: modul '{}' tidak ditemukan.\n\
                 Dicari di:\n\
                 - ./{}\n\
                 - ./{}/{}\n\
                 Gunakan 'taji pasang <URL>' untuk memasang modul dari internet.",
                masukan,
                nama_berkas,
                crate::tpm::DIREKTORI_MODUL,
                nama_berkas
            ));
        }
    };

    let isi = match std::fs::read_to_string(&jalur) {
        Ok(c) => c,
        Err(e) => {
            return Object::Error(format!("masukkan: gagal membaca berkas '{}': {}", jalur, e))
        }
    };

    let lexer = crate::lexer::Lexer::new(&isi);
    let mut parser = crate::parser::Parser::new(lexer);
    let program = parser.parse_program();

    if !parser.errors.is_empty() {
        return Object::Error(format!(
            "masukkan: galat sintaks di '{}':\n{}",
            jalur,
            parser.errors.join("\n")
        ));
    }

    // Kompilasi AST ke bytecode
    let mut kompilator = crate::compiler::Kompilator::new_dengan_state(
        crate::bawaan::bikin_tabel_awal(),
        Vec::new(),
    );
    let hasil = match kompilator.kompilasi(&program) {
        Ok(h) => h,
        Err(e) => return Object::Error(format!("masukkan: galat kompilasi di '{}': {}", jalur, e)),
    };

    // Eksekusi menggunakan VM
    let mut vm = crate::vm::VM::new_dengan_globals(hasil, crate::bawaan::bikin_globals_awal());
    if let Err(e) = vm.jalankan() {
        return Object::Error(format!("masukkan: galat eksekusi di '{}': {}", jalur, e));
    }

    // Ekspor seluruh variabel global dari modul sebagai Kamus (Hash)
    let mut ekspor = std::collections::HashMap::new();
    let tabel = kompilator.tabel_simbol.ambil_store();
    let globals = vm.ambil_globals();

    for (nama, simbol) in tabel.iter() {
        if simbol.lingkup == crate::compiler::LingkupSimbol::Global && simbol.indeks < globals.len()
        {
            let nilai = &globals[simbol.indeks];
            ekspor.insert(crate::object::KunciKamus::Str(nama.clone()), nilai.clone());
        }
    }

    Object::Hash(Rc::new(RefCell::new(ekspor)))
}

// ============================================================
// Registri Modul Bawaan (Tertanam di WASM Binary)
// ============================================================
// Modul ini di-embed saat kompilasi sehingga tersedia langsung
// di peramban tanpa perlu akses sistem berkas.
#[cfg(target_arch = "wasm32")]
mod registri_modul {
    const MATEMATIKA: &str = include_str!("../contoh/modul/matematika.tj");
    const TEKS: &str = include_str!("../contoh/modul/teks.tj");
    const KOLEKSI: &str = include_str!("../contoh/modul/koleksi.tj");
    const VALIDASI: &str = include_str!("../contoh/modul/validasi.tj");

    /// Cari sumber modul berdasarkan nama atau jalur.
    /// Mendukung: "matematika", "modul/matematika.tj", "contoh/modul/matematika", dsb.
    pub fn cari(nama: &str) -> Option<&'static str> {
        // Ambil bagian akhir jalur lalu hilangkan ekstensi .tj
        let nama_bersih = nama
            .rsplit(['/', '\\'])
            .next()
            .unwrap_or(nama)
            .trim_end_matches(".tj");

        match nama_bersih {
            "matematika" => Some(MATEMATIKA),
            "teks" => Some(TEKS),
            "koleksi" => Some(KOLEKSI),
            "validasi" => Some(VALIDASI),
            _ => None,
        }
    }

    pub fn daftar_nama() -> &'static str {
        "matematika, teks, koleksi, validasi"
    }
}

// Kompilasi dan jalankan sumber modul, kembalikan ekspor sebagai Kamus.
// Digunakan oleh implementasi masukkan untuk target peramban (wasm32).
#[cfg(target_arch = "wasm32")]
fn kompilasi_dan_jalankan_modul(isi: &str, label: &str) -> Object {
    use std::collections::HashMap;

    let lexer = crate::lexer::Lexer::new(isi);
    let mut parser = crate::parser::Parser::new(lexer);
    let program = parser.parse_program();

    if !parser.errors.is_empty() {
        return Object::Error(format!(
            "masukkan: galat sintaks di '{}':\n{}",
            label,
            parser.errors.join("\n")
        ));
    }

    let mut kompilator = crate::compiler::Kompilator::new_dengan_state(
        crate::bawaan::bikin_tabel_awal(),
        Vec::new(),
    );
    let hasil = match kompilator.kompilasi(&program) {
        Ok(h) => h,
        Err(e) => return Object::Error(format!("masukkan: galat kompilasi di '{}': {}", label, e)),
    };

    let mut vm = crate::vm::VM::new_dengan_globals(hasil, crate::bawaan::bikin_globals_awal());
    if let Err(e) = vm.jalankan() {
        return Object::Error(format!("masukkan: galat eksekusi di '{}': {}", label, e));
    }

    let mut ekspor = HashMap::new();
    let tabel = kompilator.tabel_simbol.ambil_store();
    let globals = vm.ambil_globals();

    for (nama, simbol) in tabel.iter() {
        if simbol.lingkup == crate::compiler::LingkupSimbol::Global && simbol.indeks < globals.len()
        {
            ekspor.insert(
                crate::object::KunciKamus::Str(nama.clone()),
                globals[simbol.indeks].clone(),
            );
        }
    }

    Object::Hash(Rc::new(RefCell::new(ekspor)))
}

/// Implementasi masukkan() untuk peramban (WASM):
/// 1. Cek registri modul bawaan (embedded di binary)
/// 2. Fallback ke localStorage VFS
/// 3. Error informatif jika tidak ditemukan
#[cfg(target_arch = "wasm32")]
fn builtin_masukkan(args: Vec<Object>) -> Object {
    if args.len() != 1 {
        return Object::Error(
            "masukkan: butuh 1 argumen (nama_modul atau jalur_berkas)".to_string(),
        );
    }
    let masukan = match &args[0] {
        Object::Str(s) => s.clone(),
        _ => return Object::Error("masukkan: argumen harus TEKS".to_string()),
    };

    // --- Langkah 1: Cek registri modul bawaan ---
    if let Some(sumber) = registri_modul::cari(&masukan) {
        return kompilasi_dan_jalankan_modul(sumber, &masukan);
    }

    // --- Langkah 2: Coba localStorage VFS ---
    let nama_berkas = if masukan.ends_with(".tj") {
        masukan.clone()
    } else {
        format!("{}.tj", masukan)
    };

    let kunci_dicoba = [
        format!("taji_vfs:{}", masukan),
        format!("taji_vfs:{}", nama_berkas),
        format!("taji_vfs:./{}", nama_berkas),
    ];

    for kunci in &kunci_dicoba {
        let kunci_aman = kunci.replace('\'', "\\'");
        let skrip = format!(
            "(function(){{ var d=localStorage.getItem('{kunci_aman}'); \
             return d!==null?d:'__TAJI_TIDAK_ADA__'; }})()"
        );
        if let Ok(val) = js_sys::eval(&skrip) {
            if let Some(s) = val.as_string() {
                if s != "__TAJI_TIDAK_ADA__" {
                    return kompilasi_dan_jalankan_modul(&s, &masukan);
                }
            }
        }
    }

    // --- Langkah 3: Error informatif ---
    Object::Error(format!(
        "masukkan: modul '{}' tidak ditemukan.\n\
         Modul bawaan: {}\n\
         Untuk modul kustom, muat ke VFS dulu: tulis_berkas(\"nama.tj\", <isi_kode>)",
        masukan,
        registri_modul::daftar_nama()
    ))
}
