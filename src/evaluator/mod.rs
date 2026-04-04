/// Modul Evaluator untuk bahasa Taji.
///
/// Evaluator adalah "otak" yang berjalan menelusuri pohon AST
/// dan benar-benar mengeksekusi setiap instruksi: menghitung
/// matematika, mengevaluasi kondisi, menjalankan fungsi, dsb.
///
/// Setiap node AST dievaluasi menjadi sebuah `Object` yang
/// disimpan di dalam `Environment` (lingkup variabel).

use crate::ast::*;
use crate::object::*;
use std::collections::HashMap;

// ═══════════════════════════════════════════════════════════
//  Konstanta Boolean & Null (optimisasi memori)
// ═══════════════════════════════════════════════════════════

const TRUE: Object = Object::Boolean(true);
const FALSE: Object = Object::Boolean(false);
const NULL: Object = Object::Null;

// ═══════════════════════════════════════════════════════════
//  Entry point — Evaluasi Program
// ═══════════════════════════════════════════════════════════

/// Mengevaluasi seluruh program (urutan pernyataan).
pub fn eval(program: &Program, env: &mut Environment) -> Object {
    let mut result = NULL;

    for stmt in &program.statements {
        result = eval_statement(stmt, env);

        // Jika menemukan `kembalikan` atau error, hentikan segera
        match &result {
            Object::ReturnValue(val) => return *val.clone(),
            Object::Error(_) => return result,
            _ => {}
        }
    }

    result
}

// ═══════════════════════════════════════════════════════════
//  Statement evaluation
// ═══════════════════════════════════════════════════════════

/// Mengevaluasi satu pernyataan.
fn eval_statement(stmt: &Statement, env: &mut Environment) -> Object {
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
    }
}

/// Mengevaluasi blok pernyataan (isi fungsi, if, while).
fn eval_block_statement(block: &BlockStatement, env: &mut Environment) -> Object {
    let mut result = NULL;

    for stmt in &block.statements {
        result = eval_statement(stmt, env);

        // Hentikan jika menemukan return atau error
        match &result {
            Object::ReturnValue(_) | Object::Error(_) => return result,
            _ => {}
        }
    }

    result
}

// ═══════════════════════════════════════════════════════════
//  Expression evaluation
// ═══════════════════════════════════════════════════════════

/// Mengevaluasi sebuah ekspresi menjadi Object.
fn eval_expression(expr: &Expression, env: &mut Environment) -> Object {
    match expr {
        Expression::IntegerLiteral(val) => Object::Integer(*val),
        Expression::BooleanLiteral(val) => native_bool_to_object(*val),
        Expression::StringLiteral(val) => Object::Str(val.clone()),

        Expression::Identifier(ident) => eval_identifier(ident, env),

        Expression::Prefix(prefix) => {
            let right = eval_expression(&prefix.right, env);
            if right.is_error() {
                return right;
            }
            eval_prefix_expression(&prefix.operator, right)
        }

        Expression::Infix(infix) => {
            let left = eval_expression(&infix.left, env);
            if left.is_error() {
                return left;
            }
            let right = eval_expression(&infix.right, env);
            if right.is_error() {
                return right;
            }
            eval_infix_expression(&infix.operator, left, right)
        }

        Expression::Jika(jika) => eval_jika_expression(jika, env),
        Expression::Selama(selama) => eval_selama_expression(selama, env),

        Expression::FungsiLiteral(fungsi) => Object::Function(FunctionObject {
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

        Expression::IndexExpression(idx) => {
            let left = eval_expression(&idx.left, env);
            if left.is_error() {
                return left;
            }
            let index = eval_expression(&idx.index, env);
            if index.is_error() {
                return index;
            }
            eval_index_expression(left, index)
        }

        Expression::HashLiteral(pairs) => eval_hash_literal(pairs, env),
    }
}

/// Mengevaluasi daftar ekspresi (argumen fungsi, elemen array).
fn eval_expressions(exprs: &[Expression], env: &mut Environment) -> Vec<Object> {
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
//  Identifier resolution
// ═══════════════════════════════════════════════════════════

/// Mencari nilai identifier di environment atau built-in functions.
fn eval_identifier(ident: &Identifier, env: &Environment) -> Object {
    // Cari di environment (variabel user)
    if let Some(val) = env.get(&ident.value) {
        return val;
    }

    // Cari di fungsi bawaan (built-in)
    if let Some(builtin) = get_builtin(&ident.value) {
        return builtin;
    }

    Object::Error(format!(
        "pengenal tidak dikenal: '{}'",
        ident.value
    ))
}

// ═══════════════════════════════════════════════════════════
//  Prefix expression evaluation
// ═══════════════════════════════════════════════════════════

/// Mengevaluasi ekspresi prefix: `!x`, `-x`, `bukan x`.
fn eval_prefix_expression(operator: &str, right: Object) -> Object {
    match operator {
        "!" | "bukan" => eval_bang_operator(right),
        "-" => eval_minus_prefix_operator(right),
        _ => Object::Error(format!(
            "operator tidak dikenal: {}{}",
            operator,
            right.type_name()
        )),
    }
}

/// Evaluasi operator `!` / `bukan` (negasi logika).
fn eval_bang_operator(right: Object) -> Object {
    match right {
        Object::Boolean(true) => FALSE,
        Object::Boolean(false) => TRUE,
        Object::Null => TRUE,
        _ => FALSE,
    }
}

/// Evaluasi operator `-` prefix (negasi angka).
fn eval_minus_prefix_operator(right: Object) -> Object {
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
//  Infix expression evaluation
// ═══════════════════════════════════════════════════════════

/// Mengevaluasi ekspresi infix: `a + b`, `x == y`, dsb.
fn eval_infix_expression(operator: &str, left: Object, right: Object) -> Object {
    match (&left, &right) {
        (Object::Integer(l), Object::Integer(r)) => {
            eval_integer_infix(operator, *l, *r)
        }
        (Object::Str(l), Object::Str(r)) => {
            eval_string_infix(operator, l, r)
        }
        (Object::Boolean(l), Object::Boolean(r)) => {
            eval_boolean_infix(operator, *l, *r)
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

/// Evaluasi operasi bilangan bulat.
fn eval_integer_infix(operator: &str, left: i64, right: i64) -> Object {
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

/// Evaluasi operasi string (concatenation & comparison).
fn eval_string_infix(operator: &str, left: &str, right: &str) -> Object {
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

/// Evaluasi operasi boolean.
fn eval_boolean_infix(operator: &str, left: bool, right: bool) -> Object {
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
//  Conditional & Loop evaluation
// ═══════════════════════════════════════════════════════════

/// Mengevaluasi `jika (kondisi) { ... } lainnya { ... }`
fn eval_jika_expression(jika: &JikaExpression, env: &mut Environment) -> Object {
    let condition = eval_expression(&jika.condition, env);
    if condition.is_error() {
        return condition;
    }

    if is_truthy(&condition) {
        eval_block_statement(&jika.consequence, env)
    } else if let Some(alt) = &jika.alternative {
        eval_block_statement(alt, env)
    } else {
        NULL
    }
}

/// Mengevaluasi `selama (kondisi) { ... }`
fn eval_selama_expression(selama: &SelamaExpression, env: &mut Environment) -> Object {
    let mut result = NULL;

    loop {
        let condition = eval_expression(&selama.condition, env);
        if condition.is_error() {
            return condition;
        }

        if !is_truthy(&condition) {
            break;
        }

        result = eval_block_statement(&selama.body, env);

        match &result {
            Object::ReturnValue(_) | Object::Error(_) => return result,
            _ => {}
        }
    }

    result
}

/// Memeriksa apakah sebuah objek dianggap "benar" (truthy).
fn is_truthy(obj: &Object) -> bool {
    match obj {
        Object::Null => false,
        Object::Boolean(val) => *val,
        Object::Integer(0) => false,
        Object::Str(s) if s.is_empty() => false,
        _ => true,
    }
}

// ═══════════════════════════════════════════════════════════
//  Function application
// ═══════════════════════════════════════════════════════════

/// Memanggil fungsi (user-defined atau built-in) dengan argumen.
fn apply_function(func: Object, args: Vec<Object>) -> Object {
    match func {
        Object::Function(f) => {
            // Validasi jumlah argumen
            if args.len() != f.parameters.len() {
                return Object::Error(format!(
                    "jumlah argumen salah: diharapkan {}, diterima {}",
                    f.parameters.len(),
                    args.len()
                ));
            }

            // Buat lingkup baru (enclosed) untuk fungsi
            let mut enclosed_env = Environment::new_enclosed(f.env.clone());

            // Ikat argumen ke parameter
            for (param, arg) in f.parameters.iter().zip(args.into_iter()) {
                enclosed_env.set(param.value.clone(), arg);
            }

            // Evaluasi body fungsi
            let result = eval_block_statement(&f.body, &mut enclosed_env);

            // "Buka" ReturnValue agar tidak bocor ke luar fungsi
            match result {
                Object::ReturnValue(val) => *val,
                other => other,
            }
        }
        Object::Builtin(builtin_fn) => builtin_fn(args),
        _ => Object::Error(format!(
            "'{}' bukan sebuah fungsi",
            func.type_name()
        )),
    }
}

// ═══════════════════════════════════════════════════════════
//  Index & Hash evaluation
// ═══════════════════════════════════════════════════════════

/// Mengevaluasi akses indeks: `daftar[0]`, `kamus["kunci"]`
fn eval_index_expression(left: Object, index: Object) -> Object {
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

/// Mengevaluasi hash literal: `{"kunci": nilai, ...}`
fn eval_hash_literal(pairs: &[(Expression, Expression)], env: &mut Environment) -> Object {
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
//  Built-in Functions (Fungsi Bawaan)
// ═══════════════════════════════════════════════════════════

/// Mengembalikan fungsi bawaan berdasarkan nama.
fn get_builtin(name: &str) -> Option<Object> {
    match name {
        "cetak" => Some(Object::Builtin(builtin_cetak)),
        "panjang" => Some(Object::Builtin(builtin_panjang)),
        "tipe" => Some(Object::Builtin(builtin_tipe)),
        "dorong" => Some(Object::Builtin(builtin_dorong)),
        "pertama" => Some(Object::Builtin(builtin_pertama)),
        "terakhir" => Some(Object::Builtin(builtin_terakhir)),
        "sisa" => Some(Object::Builtin(builtin_sisa)),
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

/// `panjang(objek)` — Mengembalikan panjang teks atau daftar.
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
            "argumen untuk 'panjang' tidak didukung: {}",
            args[0].type_name()
        )),
    }
}

/// `tipe(objek)` — Mengembalikan nama tipe sebagai teks.
fn builtin_tipe(args: Vec<Object>) -> Object {
    if args.len() != 1 {
        return Object::Error(format!(
            "jumlah argumen salah untuk 'tipe': diharapkan 1, diterima {}",
            args.len()
        ));
    }

    Object::Str(args[0].type_name().to_string())
}

/// `dorong(daftar, nilai)` — Menambahkan elemen ke akhir daftar (immutable).
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
            "argumen pertama untuk 'dorong' harus DAFTAR, diterima {}",
            args[0].type_name()
        )),
    }
}

/// `pertama(daftar)` — Mengembalikan elemen pertama.
fn builtin_pertama(args: Vec<Object>) -> Object {
    if args.len() != 1 {
        return Object::Error(format!(
            "jumlah argumen salah untuk 'pertama': diharapkan 1, diterima {}",
            args.len()
        ));
    }

    match &args[0] {
        Object::Array(arr) => {
            if arr.is_empty() {
                NULL
            } else {
                arr[0].clone()
            }
        }
        _ => Object::Error(format!(
            "argumen untuk 'pertama' harus DAFTAR, diterima {}",
            args[0].type_name()
        )),
    }
}

/// `terakhir(daftar)` — Mengembalikan elemen terakhir.
fn builtin_terakhir(args: Vec<Object>) -> Object {
    if args.len() != 1 {
        return Object::Error(format!(
            "jumlah argumen salah untuk 'terakhir': diharapkan 1, diterima {}",
            args.len()
        ));
    }

    match &args[0] {
        Object::Array(arr) => {
            if arr.is_empty() {
                NULL
            } else {
                arr[arr.len() - 1].clone()
            }
        }
        _ => Object::Error(format!(
            "argumen untuk 'terakhir' harus DAFTAR, diterima {}",
            args[0].type_name()
        )),
    }
}

/// `sisa(daftar)` — Mengembalikan semua elemen kecuali pertama.
fn builtin_sisa(args: Vec<Object>) -> Object {
    if args.len() != 1 {
        return Object::Error(format!(
            "jumlah argumen salah untuk 'sisa': diharapkan 1, diterima {}",
            args.len()
        ));
    }

    match &args[0] {
        Object::Array(arr) => {
            if arr.is_empty() {
                NULL
            } else {
                Object::Array(arr[1..].to_vec())
            }
        }
        _ => Object::Error(format!(
            "argumen untuk 'sisa' harus DAFTAR, diterima {}",
            args[0].type_name()
        )),
    }
}

// ═══════════════════════════════════════════════════════════
//  Helper
// ═══════════════════════════════════════════════════════════

/// Mengonversi boolean Rust menjadi Object Boolean.
fn native_bool_to_object(val: bool) -> Object {
    if val { TRUE } else { FALSE }
}
