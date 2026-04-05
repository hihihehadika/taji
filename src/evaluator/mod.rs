/// Modul Evaluator untuk bahasa Taji.
///
/// Evaluator adalah "otak" yang menelusuri AST dan mengeksekusi
/// setiap instruksi. Mendukung: aritmatika (bilangan bulat & desimal),
/// kondisi, fungsi, perulangan, berhenti/lanjut, penugasan,
/// dan impor modul.

use crate::ast::*;
use crate::lexer::Lexer;
use crate::object::*;
use crate::parser::Parser;
use std::collections::HashMap;
use std::io::{self, Write};
use std::time::{SystemTime, UNIX_EPOCH};

// ═══════════════════════════════════════════════════════════
//  Konstanta
// ═══════════════════════════════════════════════════════════

const TRUE: Object = Object::Boolean(true);
const FALSE: Object = Object::Boolean(false);
const NULL: Object = Object::Null;

// ═══════════════════════════════════════════════════════════
//  Titik masuk
// ═══════════════════════════════════════════════════════════

pub fn eval(program: &Program, env: &mut Lingkungan) -> Object {
    let mut result = NULL;

    for stmt in &program.statements {
        result = eval_statement(stmt, env);

        match &result {
            Object::ReturnValue(val) => return *val.clone(),
            Object::Error(_) => return result,
            _ => {}
        }
    }

    result
}

// ═══════════════════════════════════════════════════════════
//  Evaluasi pernyataan
// ═══════════════════════════════════════════════════════════

fn eval_statement(stmt: &Statement, env: &mut Lingkungan) -> Object {
    match stmt {
        Statement::Ekspresi(expr_stmt) => eval_expression(&expr_stmt.expression, env),
        Statement::Misalkan(misalkan) => {
            let val = eval_expression(&misalkan.value, env);
            if val.is_error() {
                return val;
            }
            env.set(misalkan.name.value.clone(), val)
        }
        Statement::Kembalikan(kembali) => {
            let val = eval_expression(&kembali.return_value, env);
            if val.is_error() {
                return val;
            }
            Object::ReturnValue(Box::new(val))
        }
        Statement::Berhenti => Object::Break,
        Statement::Lanjut => Object::Continue,
    }
}

fn eval_blok_pernyataan(block: &BlokPernyataan, env: &mut Lingkungan) -> Object {
    let mut result = NULL;

    for stmt in &block.statements {
        result = eval_statement(stmt, env);

        match &result {
            Object::ReturnValue(_) | Object::Error(_) | Object::Break | Object::Continue => {
                return result
            }
            _ => {}
        }
    }

    result
}

// ═══════════════════════════════════════════════════════════
//  Evaluasi ekspresi
// ═══════════════════════════════════════════════════════════

fn eval_expression(expr: &Expression, env: &mut Lingkungan) -> Object {
    match expr {
        Expression::IntegerLiteral(val) => Object::Integer(*val),
        Expression::FloatLiteral(val) => Object::Float(*val),
        Expression::BooleanLiteral(val) => native_bool_to_object(*val),
        Expression::StringLiteral(val) => Object::Str(val.clone()),

        Expression::Pengenal(ident) => eval_pengenal(ident, env),

        Expression::Awalan(awalan) => {
            let right = eval_expression(&awalan.right, env);
            if right.is_error() {
                return right;
            }
            eval_awalan_expression(&awalan.operator, right)
        }

        Expression::Sisipan(sisipan) => {
            let left = eval_expression(&sisipan.left, env);
            if left.is_error() {
                return left;
            }
            let right = eval_expression(&sisipan.right, env);
            if right.is_error() {
                return right;
            }
            eval_sisipan_expression(&sisipan.operator, left, right)
        }

        Expression::Jika(jika) => eval_jika_expression(jika, env),
        Expression::Selama(selama) => eval_selama_expression(selama, env),
        Expression::Untuk(untuk) => eval_untuk_expression(untuk, env),

        Expression::FungsiLiteral(fungsi) => Object::Fungsi(ObjekFungsi {
            parameters: fungsi.parameters.clone(),
            body: fungsi.body.clone(),
            env: env.clone(),
        }),

        Expression::Panggilan(panggilan) => {
            let function = eval_expression(&panggilan.function, env);
            if function.is_error() {
                return function;
            }
            let args = eval_expressions(&panggilan.arguments, env);
            if args.len() == 1 && args[0].is_error() {
                return args[0].clone();
            }
            apply_function(function, args)
        }

        Expression::ArrayLiteral(elements) => {
            let elems = eval_expressions(elements, env);
            if elems.len() == 1 && elems[0].is_error() {
                return elems[0].clone();
            }
            Object::Array(elems)
        }

        Expression::Indeks(idx) => {
            let left = eval_expression(&idx.left, env);
            if left.is_error() {
                return left;
            }
            let index = eval_expression(&idx.index, env);
            if index.is_error() {
                return index;
            }
            eval_indeks_expression(left, index)
        }

        Expression::HashLiteral(pairs) => eval_hash_literal(pairs, env),

        Expression::Penugasan(penugasan) => eval_penugasan_expression(penugasan, env),

        Expression::Titik(titik) => eval_titik_expression(titik, env),

        Expression::Masukkan(masukkan) => eval_masukkan_expression(masukkan, env),

        // Fungsi panah menghasilkan ObjekFungsi yang sama persis dengan FungsiLiteral.
        // Perbedaannya hanya di sintaks parsing, bukan di evaluasi.
        Expression::FungsiPanah(panah) => Object::Fungsi(ObjekFungsi {
            parameters: panah.parameters.clone(),
            body: panah.body.clone(),
            env: env.clone(),
        }),

        Expression::Coba(coba) => eval_coba_expression(coba, env),
    }
}

fn eval_expressions(exprs: &[Expression], env: &mut Lingkungan) -> Vec<Object> {
    let mut result = vec![];

    for expr in exprs {
        let evaluated = eval_expression(expr, env);
        if evaluated.is_error() {
            return vec![evaluated];
        }
        result.push(evaluated);
    }

    result
}

// ═══════════════════════════════════════════════════════════
//  Resolusi pengenal
// ═══════════════════════════════════════════════════════════

fn eval_pengenal(ident: &Pengenal, env: &Lingkungan) -> Object {
    if let Some(val) = env.get(&ident.value) {
        return val;
    }

    if let Some(builtin) = get_builtin(&ident.value) {
        return builtin;
    }

    Object::Error(format!(
        "pengenal tidak dikenal: '{}'",
        ident.value
    ))
}

// ═══════════════════════════════════════════════════════════
//  Evaluasi ekspresi awalan (prefix)
// ═══════════════════════════════════════════════════════════

fn eval_awalan_expression(operator: &str, right: Object) -> Object {
    match operator {
        "!" | "bukan" => eval_bang_operator(right),
        "-" => eval_minus_awalan_operator(right),
        _ => Object::Error(format!(
            "operator tidak dikenal: {}{}",
            operator,
            right.type_name()
        )),
    }
}

fn eval_bang_operator(right: Object) -> Object {
    match right {
        Object::Boolean(true) => FALSE,
        Object::Boolean(false) => TRUE,
        Object::Null => TRUE,
        _ => FALSE,
    }
}

fn eval_minus_awalan_operator(right: Object) -> Object {
    match right {
        Object::Integer(val) => Object::Integer(-val),
        Object::Float(val) => Object::Float(-val),
        _ => Object::Error(format!(
            "operator tidak dikenal: -{}",
            right.type_name()
        )),
    }
}

// ═══════════════════════════════════════════════════════════
//  Evaluasi ekspresi sisipan (infix)
// ═══════════════════════════════════════════════════════════

fn eval_sisipan_expression(operator: &str, left: Object, right: Object) -> Object {
    match (&left, &right) {
        (Object::Integer(l), Object::Integer(r)) => {
            eval_integer_sisipan(operator, *l, *r)
        }
        (Object::Float(l), Object::Float(r)) => {
            eval_float_sisipan(operator, *l, *r)
        }
        // Campuran: Bilangan + Desimal → Desimal
        (Object::Integer(l), Object::Float(r)) => {
            eval_float_sisipan(operator, *l as f64, *r)
        }
        (Object::Float(l), Object::Integer(r)) => {
            eval_float_sisipan(operator, *l, *r as f64)
        }
        (Object::Str(l), Object::Str(r)) => {
            eval_string_sisipan(operator, l, r)
        }
        // Teks + non-teks → konversi otomatis ke teks
        (Object::Str(l), _) => {
            if operator == "+" {
                Object::Str(format!("{}{}", l, right))
            } else {
                Object::Error(format!(
                    "operator tidak dikenal: TEKS {} {}",
                    operator,
                    right.type_name()
                ))
            }
        }
        (_, Object::Str(r)) => {
            if operator == "+" {
                Object::Str(format!("{}{}", left, r))
            } else {
                Object::Error(format!(
                    "operator tidak dikenal: {} {} TEKS",
                    left.type_name(),
                    operator
                ))
            }
        }
        (Object::Boolean(l), Object::Boolean(r)) => {
            eval_boolean_sisipan(operator, *l, *r)
        }
        _ => {
            if left.type_name() != right.type_name() {
                Object::Error(format!(
                    "tipe tidak cocok: {} {} {}",
                    left.type_name(),
                    operator,
                    right.type_name()
                ))
            } else {
                Object::Error(format!(
                    "operator tidak dikenal: {} {} {}",
                    left.type_name(),
                    operator,
                    right.type_name()
                ))
            }
        }
    }
}

fn eval_integer_sisipan(operator: &str, left: i64, right: i64) -> Object {
    match operator {
        "+" => Object::Integer(left + right),
        "-" => Object::Integer(left - right),
        "*" => Object::Integer(left * right),
        "/" => {
            if right == 0 {
                Object::Error("pembagian dengan nol tidak diizinkan".to_string())
            } else {
                Object::Integer(left / right)
            }
        }
        "%" => {
            if right == 0 {
                Object::Error("modulo dengan nol tidak diizinkan".to_string())
            } else {
                Object::Integer(left % right)
            }
        }
        "<" => native_bool_to_object(left < right),
        ">" => native_bool_to_object(left > right),
        "<=" => native_bool_to_object(left <= right),
        ">=" => native_bool_to_object(left >= right),
        "==" => native_bool_to_object(left == right),
        "!=" => native_bool_to_object(left != right),
        _ => Object::Error(format!(
            "operator tidak dikenal: BILANGAN {} BILANGAN",
            operator
        )),
    }
}

fn eval_float_sisipan(operator: &str, left: f64, right: f64) -> Object {
    match operator {
        "+" => Object::Float(left + right),
        "-" => Object::Float(left - right),
        "*" => Object::Float(left * right),
        "/" => {
            if right == 0.0 {
                Object::Error("pembagian dengan nol tidak diizinkan".to_string())
            } else {
                Object::Float(left / right)
            }
        }
        "%" => {
            if right == 0.0 {
                Object::Error("modulo dengan nol tidak diizinkan".to_string())
            } else {
                Object::Float(left % right)
            }
        }
        "<" => native_bool_to_object(left < right),
        ">" => native_bool_to_object(left > right),
        "<=" => native_bool_to_object(left <= right),
        ">=" => native_bool_to_object(left >= right),
        "==" => native_bool_to_object(left == right),
        "!=" => native_bool_to_object(left != right),
        _ => Object::Error(format!(
            "operator tidak dikenal: DESIMAL {} DESIMAL",
            operator
        )),
    }
}

fn eval_string_sisipan(operator: &str, left: &str, right: &str) -> Object {
    match operator {
        "+" => Object::Str(format!("{}{}", left, right)),
        "==" => native_bool_to_object(left == right),
        "!=" => native_bool_to_object(left != right),
        _ => Object::Error(format!(
            "operator tidak dikenal: TEKS {} TEKS",
            operator
        )),
    }
}

fn eval_boolean_sisipan(operator: &str, left: bool, right: bool) -> Object {
    match operator {
        "==" => native_bool_to_object(left == right),
        "!=" => native_bool_to_object(left != right),
        "dan" => native_bool_to_object(left && right),
        "atau" => native_bool_to_object(left || right),
        _ => Object::Error(format!(
            "operator tidak dikenal: BOOLEAN {} BOOLEAN",
            operator
        )),
    }
}

// ═══════════════════════════════════════════════════════════
//  Evaluasi penugasan (assignment)
// ═══════════════════════════════════════════════════════════

fn eval_penugasan_expression(penugasan: &PenugasanExpression, env: &mut Lingkungan) -> Object {
    let new_val = eval_expression(&penugasan.value, env);
    if new_val.is_error() {
        return new_val;
    }

    let final_val = if penugasan.operator == "=" {
        new_val
    } else {
        // Untuk +=, -=, *=, /= kita perlu nilai lama
        let old_val = match env.get(&penugasan.name.value) {
            Some(val) => val,
            None => {
                return Object::Error(format!(
                    "pengenal tidak dikenal: '{}'",
                    penugasan.name.value
                ));
            }
        };

        let op = match penugasan.operator.as_str() {
            "+=" => "+",
            "-=" => "-",
            "*=" => "*",
            "/=" => "/",
            _ => {
                return Object::Error(format!(
                    "operator penugasan tidak dikenal: '{}'",
                    penugasan.operator
                ));
            }
        };

        eval_sisipan_expression(op, old_val, new_val)
    };

    if final_val.is_error() {
        return final_val;
    }

    // Coba perbarui di scope chain, kalau gagal simpan di scope saat ini
    match env.update(&penugasan.name.value, final_val.clone()) {
        Some(val) => val,
        None => env.set(penugasan.name.value.clone(), final_val),
    }
}

// ═══════════════════════════════════════════════════════════
//  Evaluasi ekspresi titik (dot access)
// ═══════════════════════════════════════════════════════════

fn eval_titik_expression(titik: &TitikExpression, env: &mut Lingkungan) -> Object {
    let left = eval_expression(&titik.left, env);
    if left.is_error() {
        return left;
    }

    match left {
        Object::Hash(ref pairs) => {
            let key = KunciKamus::Str(titik.key.clone());
            match pairs.get(&key) {
                Some(val) => val.clone(),
                None => NULL,
            }
        }
        _ => Object::Error(format!(
            "operator '.' tidak didukung untuk: {}",
            left.type_name()
        )),
    }
}

// ═══════════════════════════════════════════════════════════
//  Evaluasi kondisional & perulangan
// ═══════════════════════════════════════════════════════════

fn eval_jika_expression(jika: &JikaExpression, env: &mut Lingkungan) -> Object {
    let condition = eval_expression(&jika.condition, env);
    if condition.is_error() {
        return condition;
    }

    if is_truthy(&condition) {
        eval_blok_pernyataan(&jika.consequence, env)
    } else if let Some(alt) = &jika.alternative {
        eval_blok_pernyataan(alt, env)
    } else {
        NULL
    }
}

fn eval_selama_expression(selama: &SelamaExpression, env: &mut Lingkungan) -> Object {
    let mut result = NULL;

    loop {
        let condition = eval_expression(&selama.condition, env);
        if condition.is_error() {
            return condition;
        }

        if !is_truthy(&condition) {
            break;
        }

        result = eval_blok_pernyataan(&selama.body, env);

        match &result {
            Object::ReturnValue(_) | Object::Error(_) => return result,
            Object::Break => break,
            Object::Continue => continue,
            _ => {}
        }
    }

    // Jangan kembalikan sinyal Break/Continue ke luar loop
    if matches!(result, Object::Break | Object::Continue) {
        NULL
    } else {
        result
    }
}

/// Evaluasi perulangan untuk (gaya C):
/// `untuk (init; kondisi; pembaruan) { badan }`
fn eval_untuk_expression(untuk: &UntukExpression, env: &mut Lingkungan) -> Object {
    // Jalankan inisialisasi
    let init_result = eval_statement(&untuk.init, env);
    if init_result.is_error() {
        return init_result;
    }

    let mut result = NULL;

    loop {
        // Cek kondisi
        let condition = eval_expression(&untuk.condition, env);
        if condition.is_error() {
            return condition;
        }

        if !is_truthy(&condition) {
            break;
        }

        // Jalankan badan loop
        result = eval_blok_pernyataan(&untuk.body, env);

        match &result {
            Object::ReturnValue(_) | Object::Error(_) => return result,
            Object::Break => break,
            Object::Continue => {
                // Lanjut: langsung ke pembaruan
            }
            _ => {}
        }

        // Jalankan pembaruan
        let update_result = eval_statement(&untuk.update, env);
        if update_result.is_error() {
            return update_result;
        }
    }

    if matches!(result, Object::Break | Object::Continue) {
        NULL
    } else {
        result
    }
}

/// Evaluasi `coba { ... } tangkap (err) { ... }`
///
/// Menjalankan blok `coba`. Jika menghasilkan Object::Error,
/// tangkap pesan error-nya, simpan di variabel `tangkap`,
/// lalu jalankan blok penanganan.
fn eval_coba_expression(coba: &CobaExpression, env: &mut Lingkungan) -> Object {
    let result = eval_blok_pernyataan(&coba.body, env);

    match result {
        Object::Error(msg) => {
            // Buat lingkungan baru untuk blok tangkap
            let mut handler_env = Lingkungan::new_enclosed(env.clone());
            handler_env.set(coba.error_ident.value.clone(), Object::Str(msg));

            let handler_result = eval_blok_pernyataan(&coba.handler, &mut handler_env);

            // Propagasikan ReturnValue jika ada
            match handler_result {
                Object::ReturnValue(_) => handler_result,
                _ => handler_result,
            }
        }
        // Tidak ada error → kembalikan hasil biasa
        _ => result,
    }
}

/// Menentukan apakah sebuah objek bernilai "benar" (truthy).
///
/// Aturan konsisten:
/// - `kosong` → salah
/// - `salah` → salah    
/// - `0` (bilangan bulat) → salah
/// - `0.0` (desimal) → salah
/// - `""` (teks kosong) → salah
/// - Selain itu → benar
fn is_truthy(obj: &Object) -> bool {
    match obj {
        Object::Null => false,
        Object::Boolean(val) => *val,
        Object::Integer(0) => false,
        Object::Float(val) if *val == 0.0 => false,
        Object::Str(s) if s.is_empty() => false,
        _ => true,
    }
}

// ═══════════════════════════════════════════════════════════
//  Aplikasi fungsi
// ═══════════════════════════════════════════════════════════

fn apply_function(func: Object, args: Vec<Object>) -> Object {
    match func {
        Object::Fungsi(f) => {
            if args.len() != f.parameters.len() {
                return Object::Error(format!(
                    "jumlah argumen salah: diharapkan {}, diterima {}",
                    f.parameters.len(),
                    args.len()
                ));
            }

            let mut enclosed_env = Lingkungan::new_enclosed(f.env.clone());

            for (param, arg) in f.parameters.iter().zip(args.into_iter()) {
                enclosed_env.set(param.value.clone(), arg);
            }

            let result = eval_blok_pernyataan(&f.body, &mut enclosed_env);

            match result {
                Object::ReturnValue(val) => *val,
                other => other,
            }
        }
        Object::Bawaan(builtin_fn) => builtin_fn(args),
        _ => Object::Error(format!(
            "'{}' bukan sebuah fungsi",
            func.type_name()
        )),
    }
}

// ═══════════════════════════════════════════════════════════
//  Evaluasi indeks & kamus
// ═══════════════════════════════════════════════════════════

fn eval_indeks_expression(left: Object, index: Object) -> Object {
    match (&left, &index) {
        (Object::Array(elements), Object::Integer(idx)) => {
            let max = elements.len() as i64;
            if *idx < 0 || *idx >= max {
                NULL
            } else {
                elements[*idx as usize].clone()
            }
        }
        (Object::Hash(pairs), _) => {
            match index.to_hash_key() {
                Some(key) => match pairs.get(&key) {
                    Some(val) => val.clone(),
                    None => NULL,
                },
                None => Object::Error(format!(
                    "tipe {} tidak bisa digunakan sebagai kunci kamus",
                    index.type_name()
                )),
            }
        }
        _ => Object::Error(format!(
            "operator indeks tidak didukung untuk: {}",
            left.type_name()
        )),
    }
}

fn eval_hash_literal(pairs: &[(Expression, Expression)], env: &mut Lingkungan) -> Object {
    let mut hash = HashMap::new();

    for (key_expr, val_expr) in pairs {
        let key = eval_expression(key_expr, env);
        if key.is_error() {
            return key;
        }

        let hash_key = match key.to_hash_key() {
            Some(k) => k,
            None => {
                return Object::Error(format!(
                    "tipe {} tidak bisa digunakan sebagai kunci kamus",
                    key.type_name()
                ));
            }
        };

        let val = eval_expression(val_expr, env);
        if val.is_error() {
            return val;
        }

        hash.insert(hash_key, val);
    }

    Object::Hash(hash)
}

// ═══════════════════════════════════════════════════════════
//  Evaluasi masukkan (impor modul)
// ═══════════════════════════════════════════════════════════

fn eval_masukkan_expression(masukkan: &MasukkanExpression, env: &mut Lingkungan) -> Object {
    let path_obj = eval_expression(&masukkan.path, env);
    if path_obj.is_error() {
        return path_obj;
    }

    let path = match &path_obj {
        Object::Str(s) => s.clone(),
        _ => {
            return Object::Error(format!(
                "argumen untuk 'masukkan' harus TEKS, diterima {}",
                path_obj.type_name()
            ));
        }
    };

    // Baca file
    let content = match std::fs::read_to_string(&path) {
        Ok(c) => c,
        Err(e) => {
            return Object::Error(format!(
                "gagal membaca file '{}': {}",
                path, e
            ));
        }
    };

    // Parsing file
    let lexer = Lexer::new(&content);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    if !parser.errors.is_empty() {
        return Object::Error(format!(
            "kesalahan parsing di file '{}': {}",
            path,
            parser.errors.join(", ")
        ));
    }

    // Evaluasi di lingkungan tersendiri
    let mut module_env = Lingkungan::new();
    let result = eval(&program, &mut module_env);
    if result.is_error() {
        return result;
    }

    // Kembalikan semua variabel lokal sebagai Kamus
    Object::Hash(module_env.get_all_local())
}

// ═══════════════════════════════════════════════════════════
//  Fungsi Bawaan (Built-in Functions)
// ═══════════════════════════════════════════════════════════

fn get_builtin(name: &str) -> Option<Object> {
    match name {
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
        _ => None,
    }
}

/// `cetak(nilai)` — Mencetak nilai ke layar.
fn builtin_cetak(args: Vec<Object>) -> Object {
    for arg in &args {
        println!("{}", arg);
    }
    NULL
}

/// `panjang(objek)` — Panjang teks atau daftar.
fn builtin_panjang(args: Vec<Object>) -> Object {
    if args.len() != 1 {
        return Object::Error(format!(
            "jumlah argumen salah untuk 'panjang': diharapkan 1, diterima {}",
            args.len()
        ));
    }

    match &args[0] {
        Object::Str(s) => Object::Integer(s.len() as i64),
        Object::Array(arr) => Object::Integer(arr.len() as i64),
        _ => Object::Error(format!(
            "argumen untuk 'panjang' harus TEKS atau DAFTAR, diterima {}",
            args[0].type_name()
        )),
    }
}

/// `tipe(objek)` — Nama tipe sebagai teks.
fn builtin_tipe(args: Vec<Object>) -> Object {
    if args.len() != 1 {
        return Object::Error(format!(
            "jumlah argumen salah untuk 'tipe': diharapkan 1, diterima {}",
            args.len()
        ));
    }
    Object::Str(args[0].type_name().to_string())
}

/// `dorong(daftar, nilai)` — Tambah elemen ke akhir daftar.
fn builtin_dorong(args: Vec<Object>) -> Object {
    if args.len() != 2 {
        return Object::Error(format!(
            "jumlah argumen salah untuk 'dorong': diharapkan 2, diterima {}",
            args.len()
        ));
    }

    match &args[0] {
        Object::Array(arr) => {
            let mut new_arr = arr.clone();
            new_arr.push(args[1].clone());
            Object::Array(new_arr)
        }
        _ => Object::Error(format!(
            "argumen untuk 'dorong' harus DAFTAR, diterima {}",
            args[0].type_name()
        )),
    }
}

/// `pertama(daftar)` — Elemen pertama.
fn builtin_pertama(args: Vec<Object>) -> Object {
    if args.len() != 1 {
        return Object::Error(format!(
            "jumlah argumen salah untuk 'pertama': diharapkan 1, diterima {}",
            args.len()
        ));
    }

    match &args[0] {
        Object::Array(arr) => {
            if arr.is_empty() { NULL } else { arr[0].clone() }
        }
        _ => Object::Error(format!(
            "argumen untuk 'pertama' harus DAFTAR, diterima {}",
            args[0].type_name()
        )),
    }
}

/// `terakhir(daftar)` — Elemen terakhir.
fn builtin_terakhir(args: Vec<Object>) -> Object {
    if args.len() != 1 {
        return Object::Error(format!(
            "jumlah argumen salah untuk 'terakhir': diharapkan 1, diterima {}",
            args.len()
        ));
    }

    match &args[0] {
        Object::Array(arr) => {
            if arr.is_empty() { NULL } else { arr[arr.len() - 1].clone() }
        }
        _ => Object::Error(format!(
            "argumen untuk 'terakhir' harus DAFTAR, diterima {}",
            args[0].type_name()
        )),
    }
}

/// `sisa(daftar)` — Semua elemen kecuali pertama.
fn builtin_sisa(args: Vec<Object>) -> Object {
    if args.len() != 1 {
        return Object::Error(format!(
            "jumlah argumen salah untuk 'sisa': diharapkan 1, diterima {}",
            args.len()
        ));
    }

    match &args[0] {
        Object::Array(arr) => {
            if arr.is_empty() { NULL } else { Object::Array(arr[1..].to_vec()) }
        }
        _ => Object::Error(format!(
            "argumen untuk 'sisa' harus DAFTAR, diterima {}",
            args[0].type_name()
        )),
    }
}

/// `tanya(prompt)` — Baca input dari terminal.
fn builtin_tanya(args: Vec<Object>) -> Object {
    if args.len() > 1 {
        return Object::Error(format!(
            "jumlah argumen salah untuk 'tanya': diharapkan 0 atau 1, diterima {}",
            args.len()
        ));
    }

    // Tampilkan prompt jika ada
    if !args.is_empty() {
        print!("{}", args[0]);
        io::stdout().flush().unwrap_or(());
    }

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => Object::Str(input.trim_end_matches('\n').trim_end_matches('\r').to_string()),
        Err(e) => Object::Error(format!("gagal membaca input: {}", e)),
    }
}

/// `waktu()` — Timestamp saat ini (dalam milidetik).
fn builtin_waktu(args: Vec<Object>) -> Object {
    if !args.is_empty() {
        return Object::Error(format!(
            "jumlah argumen salah untuk 'waktu': diharapkan 0, diterima {}",
            args.len()
        ));
    }

    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(d) => Object::Integer(d.as_millis() as i64),
        Err(_) => Object::Error("gagal mendapatkan waktu sistem".to_string()),
    }
}

/// `teks(objek)` — Konversi apapun ke teks.
fn builtin_teks(args: Vec<Object>) -> Object {
    if args.len() != 1 {
        return Object::Error(format!(
            "jumlah argumen salah untuk 'teks': diharapkan 1, diterima {}",
            args.len()
        ));
    }
    Object::Str(format!("{}", args[0]))
}

/// `angka(teks)` — Konversi teks ke bilangan.
fn builtin_angka(args: Vec<Object>) -> Object {
    if args.len() != 1 {
        return Object::Error(format!(
            "jumlah argumen salah untuk 'angka': diharapkan 1, diterima {}",
            args.len()
        ));
    }

    match &args[0] {
        Object::Str(s) => {
            // Coba parsing sebagai bilangan bulat dulu
            if let Ok(val) = s.parse::<i64>() {
                return Object::Integer(val);
            }
            // Lalu coba sebagai desimal
            if let Ok(val) = s.parse::<f64>() {
                return Object::Float(val);
            }
            Object::Error(format!(
                "argumen untuk 'angka' tidak bisa dikonversi: '{}'",
                s
            ))
        }
        Object::Integer(_) => args[0].clone(),
        Object::Float(_) => args[0].clone(),
        Object::Boolean(b) => Object::Integer(if *b { 1 } else { 0 }),
        _ => Object::Error(format!(
            "argumen untuk 'angka' harus TEKS, diterima {}",
            args[0].type_name()
        )),
    }
}

/// `pisah(teks, pemisah)` — Memecah teks menjadi daftar berdasarkan pemisah.
fn builtin_pisah(args: Vec<Object>) -> Object {
    if args.len() != 2 {
        return Object::Error(format!(
            "jumlah argumen salah untuk 'pisah': diharapkan 2, diterima {}",
            args.len()
        ));
    }

    let teks = match &args[0] {
        Object::Str(s) => s.clone(),
        _ => {
            return Object::Error(format!(
                "argumen pertama untuk 'pisah' harus TEKS, diterima {}",
                args[0].type_name()
            ));
        }
    };

    let pemisah = match &args[1] {
        Object::Str(s) => s.clone(),
        _ => {
            return Object::Error(format!(
                "argumen kedua untuk 'pisah' harus TEKS, diterima {}",
                args[1].type_name()
            ));
        }
    };

    let parts: Vec<Object> = teks
        .split(&pemisah)
        .map(|s| Object::Str(s.to_string()))
        .collect();

    Object::Array(parts)
}

/// `gabung(daftar, penyambung)` — Menggabungkan daftar menjadi satu teks.
fn builtin_gabung(args: Vec<Object>) -> Object {
    if args.len() != 2 {
        return Object::Error(format!(
            "jumlah argumen salah untuk 'gabung': diharapkan 2, diterima {}",
            args.len()
        ));
    }

    let daftar = match &args[0] {
        Object::Array(arr) => arr.clone(),
        _ => {
            return Object::Error(format!(
                "argumen pertama untuk 'gabung' harus DAFTAR, diterima {}",
                args[0].type_name()
            ));
        }
    };

    let penyambung = match &args[1] {
        Object::Str(s) => s.clone(),
        _ => {
            return Object::Error(format!(
                "argumen kedua untuk 'gabung' harus TEKS, diterima {}",
                args[1].type_name()
            ));
        }
    };

    let parts: Vec<String> = daftar.iter().map(|o| format!("{}", o)).collect();
    Object::Str(parts.join(&penyambung))
}

/// `baca_berkas(path)` — Membaca isi file dan mengembalikan sebagai teks.
fn builtin_baca_berkas(args: Vec<Object>) -> Object {
    if args.len() != 1 {
        return Object::Error(format!(
            "jumlah argumen salah untuk 'baca_berkas': diharapkan 1, diterima {}",
            args.len()
        ));
    }

    let path = match &args[0] {
        Object::Str(s) => s.clone(),
        _ => {
            return Object::Error(format!(
                "argumen untuk 'baca_berkas' harus TEKS, diterima {}",
                args[0].type_name()
            ));
        }
    };

    match std::fs::read_to_string(&path) {
        Ok(content) => Object::Str(content),
        Err(e) => Object::Error(format!("gagal membaca berkas '{}': {}", path, e)),
    }
}

/// `tulis_berkas(path, isi)` — Menulis teks ke dalam file.
fn builtin_tulis_berkas(args: Vec<Object>) -> Object {
    if args.len() != 2 {
        return Object::Error(format!(
            "jumlah argumen salah untuk 'tulis_berkas': diharapkan 2, diterima {}",
            args.len()
        ));
    }

    let path = match &args[0] {
        Object::Str(s) => s.clone(),
        _ => {
            return Object::Error(format!(
                "argumen pertama untuk 'tulis_berkas' harus TEKS, diterima {}",
                args[0].type_name()
            ));
        }
    };

    let isi = match &args[1] {
        Object::Str(s) => s.clone(),
        _ => {
            return Object::Error(format!(
                "argumen kedua untuk 'tulis_berkas' harus TEKS, diterima {}",
                args[1].type_name()
            ));
        }
    };

    match std::fs::write(&path, &isi) {
        Ok(_) => Object::Str(format!("berhasil menulis ke '{}'", path)),
        Err(e) => Object::Error(format!("gagal menulis berkas '{}': {}", path, e)),
    }
}

// ═══════════════════════════════════════════════════════════
//  Fungsi pembantu
// ═══════════════════════════════════════════════════════════

fn native_bool_to_object(val: bool) -> Object {
    if val { TRUE } else { FALSE }
}
