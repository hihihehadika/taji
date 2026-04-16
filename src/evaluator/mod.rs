//! Modul Evaluator untuk bahasa Taji.
//! (Sistem Legacy - Tetap dipertahankan untuk kompatibilitas)

use crate::ast::*;
use crate::object::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

const TRUE: Object = Object::Boolean(true);
const FALSE: Object = Object::Boolean(false);
const NULL: Object = Object::Null;

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
        Statement::Lemparkan(lempar) => {
            let val = eval_expression(&lempar.value, env);
            if val.is_error() {
                return val;
            }
            Object::Error(val.to_string())
        }
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

fn eval_expression(expr: &Expression, env: &mut Lingkungan) -> Object {
    match expr {
        Expression::IntegerLiteral(val) => Object::Integer(*val),
        Expression::FloatLiteral(val) => Object::Float(*val),
        Expression::BooleanLiteral(val) => {
            if *val {
                TRUE
            } else {
                FALSE
            }
        }
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
        Expression::FungsiLiteral(fungsi) => Object::Fungsi(ObjekFungsi {
            parameters: fungsi.parameters.clone(),
            body: fungsi.body.clone(),
            env: env.clone(),
        }),
        Expression::Panggilan(pk) => {
            // Cek fungsi khusus: petakan, saring
            if let Expression::Pengenal(p) = &*pk.function {
                if p.value == "petakan" {
                    return eval_petakan(&pk.arguments, env);
                }
                if p.value == "saring" {
                    return eval_saring(&pk.arguments, env);
                }
            }

            let func = eval_expression(&pk.function, env);
            if func.is_error() {
                return func;
            }
            let args = eval_expressions(&pk.arguments, env);
            if args.len() == 1 && args[0].is_error() {
                return args[0].clone();
            }
            apply_function(func, args)
        }
        Expression::ArrayLiteral(elements) => {
            let elems = eval_expressions(elements, env);
            if elems.len() == 1 && elems[0].is_error() {
                return elems[0].clone();
            }
            Object::Array(Rc::new(RefCell::new(elems)))
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
        Expression::Penugasan(ps) => eval_penugasan_expression(ps, env),
        Expression::Titik(titik) => {
            let left = eval_expression(&titik.left, env);
            if left.is_error() {
                return left;
            }
            eval_indeks_expression(left, Object::Str(titik.key.clone()))
        }
        Expression::Null => NULL,
        Expression::Untuk(expr) => eval_untuk_expression(expr, env),
        Expression::Masukkan(expr) => eval_masukkan_expression(expr, env),
        Expression::Coba(expr) => eval_coba_expression(expr, env),
        Expression::FungsiPanah(expr) => Object::Fungsi(ObjekFungsi {
            parameters: expr.parameters.clone(),
            body: expr.body.clone(),
            env: env.clone(),
        }),
    }
}

fn eval_expressions(exprs: &[Expression], env: &mut Lingkungan) -> Vec<Object> {
    exprs.iter().map(|e| eval_expression(e, env)).collect()
}

fn eval_pengenal(ident: &Pengenal, env: &Lingkungan) -> Object {
    if let Some(val) = env.get(&ident.value) {
        return val;
    }
    if let Some(builtin) = crate::bawaan::cari_bawaan(&ident.value) {
        return builtin;
    }
    Object::Error(format!("pengenal tidak dikenal: '{}'", ident.value))
}

fn eval_awalan_expression(operator: &str, right: Object) -> Object {
    match operator {
        "!" | "bukan" => match right {
            Object::Boolean(true) => FALSE,
            Object::Null => TRUE,
            Object::Boolean(false) => TRUE,
            _ => FALSE,
        },
        "-" => match right {
            Object::Integer(v) => Object::Integer(-v),
            Object::Float(v) => Object::Float(-v),
            _ => Object::Error(format!("operator tidak dikenal: -{}", right.type_name())),
        },
        _ => Object::Error(format!("operator awalan tidak dikenal: {}", operator)),
    }
}

fn eval_sisipan_expression(operator: &str, left: Object, right: Object) -> Object {
    match (&left, &right) {
        (Object::Integer(l), Object::Integer(r)) => match operator {
            "+" => Object::Integer(l + r),
            "-" => Object::Integer(l - r),
            "*" => Object::Integer(l * r),
            "/" => {
                if *r == 0 {
                    Object::Error("pembagian dengan nol tidak diizinkan".to_string())
                } else {
                    Object::Integer(l / r)
                }
            }
            "%" => {
                if *r == 0 {
                    Object::Error("pembagian dengan nol tidak diizinkan".to_string())
                } else {
                    Object::Integer(l % r)
                }
            }
            "==" => Object::Boolean(l == r),
            "!=" => Object::Boolean(l != r),
            ">" => Object::Boolean(l > r),
            "<" => Object::Boolean(l < r),
            ">=" => Object::Boolean(l >= r),
            "<=" => Object::Boolean(l <= r),
            _ => Object::Error(format!(
                "operator tidak dikenal: {} {} {}",
                left.type_name(),
                operator,
                right.type_name()
            )),
        },
        (Object::Float(l), Object::Float(r)) => match operator {
            "+" => Object::Float(l + r),
            "-" => Object::Float(l - r),
            "*" => Object::Float(l * r),
            "/" => Object::Float(l / r),
            "==" => Object::Boolean((l - r).abs() < 1e-9),
            "!=" => Object::Boolean((l - r).abs() > 1e-9),
            ">" => Object::Boolean(l > r),
            "<" => Object::Boolean(l < r),
            ">=" => Object::Boolean(l >= r),
            "<=" => Object::Boolean(l <= r),
            _ => Object::Error(format!("operator float tidak dikenal: {}", operator)),
        },
        (Object::Integer(l), Object::Float(r)) => {
            eval_sisipan_expression(operator, Object::Float(*l as f64), Object::Float(*r))
        }
        (Object::Float(l), Object::Integer(r)) => {
            eval_sisipan_expression(operator, Object::Float(*l), Object::Float(*r as f64))
        }
        (Object::Str(l), r) if operator == "+" => Object::Str(format!("{}{}", l, r)),
        (l, Object::Str(r)) if operator == "+" => Object::Str(format!("{}{}", l, r)),
        (Object::Str(l), Object::Str(r)) if operator == "+" => Object::Str(format!("{}{}", l, r)),
        (Object::Boolean(l), Object::Boolean(r)) => match operator {
            "==" => Object::Boolean(l == r),
            "!=" => Object::Boolean(l != r),
            "dan" => Object::Boolean(*l && *r),
            "atau" => Object::Boolean(*l || *r),
            _ => Object::Error(format!(
                "operator tidak dikenal: {} {} {}",
                left.type_name(),
                operator,
                right.type_name()
            )),
        },
        _ => Object::Error(format!(
            "tipe tidak cocok: {} {} {}",
            left.type_name(),
            operator,
            right.type_name()
        )),
    }
}

fn eval_penugasan_expression(ps: &PenugasanExpression, env: &mut Lingkungan) -> Object {
    let mut val = eval_expression(&ps.value, env);
    if val.is_error() {
        return val;
    }

    if ps.operator != "=" {
        let left_val = match &*ps.left {
            Expression::Pengenal(id) => env.get(&id.value).unwrap_or(Object::Null),
            Expression::Indeks(idx_expr) => {
                let kol = eval_expression(&idx_expr.left, env);
                let idx = eval_expression(&idx_expr.index, env);
                eval_indeks_expression(kol, idx)
            }
            _ => Object::Error("sisi kiri penugasan tidak valid".to_string()),
        };

        if left_val.is_error() {
            return left_val;
        }

        // Extract the math operator from +=, -=, etc.
        let op = &ps.operator[..ps.operator.len() - 1];
        val = eval_sisipan_expression(op, left_val, val);
        if val.is_error() {
            return val;
        }
    }

    match &*ps.left {
        Expression::Pengenal(id) => {
            env.update(&id.value, val.clone())
                .unwrap_or_else(|| env.set(id.value.clone(), val.clone()));
            val
        }
        Expression::Indeks(idx_expr) => {
            let kol = eval_expression(&idx_expr.left, env);
            let idx = eval_expression(&idx_expr.index, env);
            match (kol, idx) {
                (Object::Array(arr), Object::Integer(i)) => {
                    let mut a = arr.borrow_mut();
                    if i >= 0 && i < a.len() as i64 {
                        a[i as usize] = val.clone();
                        val
                    } else {
                        Object::Error("indeks di luar batas".to_string())
                    }
                }
                (Object::Hash(h), k_obj) => {
                    if let Some(k) = k_obj.to_hash_key() {
                        h.borrow_mut().insert(k, val.clone());
                        val
                    } else {
                        Object::Error("kunci kamus tidak valid".to_string())
                    }
                }
                _ => Object::Error("penugasan indeks gagal".to_string()),
            }
        }
        _ => Object::Error("sisi kiri penugasan tidak valid".to_string()),
    }
}

fn eval_jika_expression(jika: &JikaExpression, env: &mut Lingkungan) -> Object {
    let cond = eval_expression(&jika.condition, env);
    if is_truthy(&cond) {
        eval_blok_pernyataan(&jika.consequence, env)
    } else if let Some(alt) = &jika.alternative {
        eval_blok_pernyataan(alt, env)
    } else {
        NULL
    }
}

fn eval_selama_expression(selama: &SelamaExpression, env: &mut Lingkungan) -> Object {
    let mut res = NULL;
    while is_truthy(&eval_expression(&selama.condition, env)) {
        res = eval_blok_pernyataan(&selama.body, env);
        match res {
            Object::Break => break,
            Object::Continue => continue,
            Object::ReturnValue(_) | Object::Error(_) => return res,
            _ => {}
        }
    }
    if matches!(res, Object::Break | Object::Continue) {
        NULL
    } else {
        res
    }
}

fn is_truthy(obj: &Object) -> bool {
    match obj {
        Object::Null => false,
        Object::Boolean(b) => *b,
        Object::Integer(0) => false,
        _ => true,
    }
}

fn apply_function(func: Object, args: Vec<Object>) -> Object {
    match func {
        Object::Fungsi(f) => {
            let mut e = Lingkungan::new_enclosed(f.env.clone());
            for (p, a) in f.parameters.iter().zip(args.into_iter()) {
                e.set(p.value.clone(), a);
            }
            let res = eval_blok_pernyataan(&f.body, &mut e);
            match res {
                Object::ReturnValue(v) => *v,
                _ => res,
            }
        }
        Object::Bawaan(b) => b(args),
        _ => Object::Error("bukan fungsi".to_string()),
    }
}

fn eval_indeks_expression(left: Object, index: Object) -> Object {
    match (left, index) {
        (Object::Array(arr), Object::Integer(i)) => {
            let a = arr.borrow();
            if i >= 0 && i < a.len() as i64 {
                a[i as usize].clone()
            } else {
                NULL
            }
        }
        (Object::Hash(h), k_obj) => {
            if let Some(k) = k_obj.to_hash_key() {
                h.borrow().get(&k).cloned().unwrap_or(NULL)
            } else {
                NULL
            }
        }
        _ => Object::Error("indeks tidak didukung".to_string()),
    }
}

fn eval_hash_literal(pairs: &[(Expression, Expression)], env: &mut Lingkungan) -> Object {
    let mut map = HashMap::new();
    for (ke, ve) in pairs {
        let k = eval_expression(ke, env);
        let v = eval_expression(ve, env);
        if let Some(kh) = k.to_hash_key() {
            map.insert(kh, v);
        }
    }
    Object::Hash(Rc::new(RefCell::new(map)))
}

fn eval_masukkan_expression(expr: &MasukkanExpression, env: &mut Lingkungan) -> Object {
    let path_obj = eval_expression(&expr.path, env);
    let path = match path_obj {
        Object::Str(s) => s,
        _ => return Object::Error("jalur masukkan harus TEKS".to_string()),
    };

    let content = match std::fs::read_to_string(&path) {
        Ok(c) => c,
        Err(e) => return Object::Error(format!("gagal membaca berkas '{}': {}", path, e)),
    };

    let lexer = crate::lexer::Lexer::new(&content);
    let mut parser = crate::parser::Parser::new(lexer);
    let program = parser.parse_program();

    if !parser.errors.is_empty() {
        return Object::Error(format!("galat sintaks di modul '{}'", path));
    }

    let mut nested_env = Lingkungan::new();
    // Berikan akses bawaan ke modul yang diimpor
    for nama in crate::bawaan::NAMA_BAWAAN {
        if let Some(b) = crate::bawaan::cari_bawaan(nama) {
            nested_env.set(nama.to_string(), b);
        }
    }

    eval(&program, &mut nested_env);
    Object::Hash(Rc::new(RefCell::new(nested_env.get_all_local())))
}

fn eval_coba_expression(expr: &CobaExpression, env: &mut Lingkungan) -> Object {
    let res = eval_blok_pernyataan(&expr.body, env);
    if let Object::Error(msg) = res {
        let mut catch_env = Lingkungan::new_enclosed(env.clone());
        catch_env.set(expr.error_ident.value.clone(), Object::Str(msg));
        return eval_blok_pernyataan(&expr.handler, &mut catch_env);
    }
    res
}
fn eval_untuk_expression(expr: &UntukExpression, env: &mut Lingkungan) -> Object {
    // Kita buat scope baru agar variabel inisialisasi terlokalisasi
    let mut loop_env = Lingkungan::new_enclosed(env.clone());

    // 1. Inisialisasi
    eval_statement(&expr.init, &mut loop_env);

    let mut res = NULL;

    // 2. Loop
    while is_truthy(&eval_expression(&expr.condition, &mut loop_env)) {
        // 3. Badan
        res = eval_blok_pernyataan(&expr.body, &mut loop_env);

        match res {
            Object::Break => break,
            Object::Continue => {} // Lanjut ke update
            Object::ReturnValue(_) | Object::Error(_) => return res,
            _ => {}
        }

        // 4. Update
        eval_statement(&expr.update, &mut loop_env);
    }

    if matches!(res, Object::Break | Object::Continue) {
        NULL
    } else {
        res
    }
}
fn eval_petakan(args_expr: &[Expression], env: &mut Lingkungan) -> Object {
    if args_expr.len() != 2 {
        return Object::Error("petakan: butuh 2 argumen (daftar, fungsi)".to_string());
    }
    let list_obj = eval_expression(&args_expr[0], env);
    let func_obj = eval_expression(&args_expr[1], env);

    match (list_obj, func_obj) {
        (Object::Array(arr), func) => {
            let mut result = vec![];
            for item in arr.borrow().iter() {
                let res = apply_function(func.clone(), vec![item.clone()]);
                if res.is_error() {
                    return res;
                }
                result.push(res);
            }
            Object::Array(Rc::new(RefCell::new(result)))
        }
        (obj, _) if obj.type_name() != "DAFTAR" => {
            Object::Error("petakan: argumen pertama harus DAFTAR".to_string())
        }
        _ => Object::Error("petakan: gagal".to_string()),
    }
}

fn eval_saring(args_expr: &[Expression], env: &mut Lingkungan) -> Object {
    if args_expr.len() != 2 {
        return Object::Error("saring: butuh 2 argumen (daftar, fungsi)".to_string());
    }
    let list_obj = eval_expression(&args_expr[0], env);
    let func_obj = eval_expression(&args_expr[1], env);

    match (list_obj, func_obj) {
        (Object::Array(arr), func) => {
            let mut result = vec![];
            for item in arr.borrow().iter() {
                let res = apply_function(func.clone(), vec![item.clone()]);
                if res.is_error() {
                    return res;
                }
                if is_truthy(&res) {
                    result.push(item.clone());
                }
            }
            Object::Array(Rc::new(RefCell::new(result)))
        }
        (obj, _) if obj.type_name() != "DAFTAR" => {
            Object::Error("saring: argumen pertama harus DAFTAR".to_string())
        }
        _ => Object::Error("saring: gagal".to_string()),
    }
}
