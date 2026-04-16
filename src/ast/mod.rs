//! Modul AST (Abstract Syntax Tree) untuk bahasa Taji.
//!
//! Mendefinisikan semua node yang membentuk pohon sintaks
//! hasil parsing kode sumber Taji.

use std::fmt;

// ═══════════════════════════════════════════════════════════
//  Program
// ═══════════════════════════════════════════════════════════

#[derive(Debug, Clone)]
pub struct Program {
    pub statements: Vec<Statement>,
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for stmt in &self.statements {
            write!(f, "{}", stmt)?;
        }
        Ok(())
    }
}

// ═══════════════════════════════════════════════════════════
//  Pernyataan (Statements)
// ═══════════════════════════════════════════════════════════

#[derive(Debug, Clone)]
pub enum Statement {
    Misalkan(MisalkanStatement),
    Kembalikan(KembalikanStatement),
    Ekspresi(EkspresiStatement),
    Berhenti,
    Lanjut,
    /// Pelemparan galat: `lemparkan "pesan";`
    Lemparkan(LemparkanStatement),
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Statement::Misalkan(s) => write!(f, "{}", s),
            Statement::Kembalikan(s) => write!(f, "{}", s),
            Statement::Ekspresi(s) => write!(f, "{}", s),
            Statement::Berhenti => write!(f, "berhenti;"),
            Statement::Lanjut => write!(f, "lanjut;"),
            Statement::Lemparkan(s) => write!(f, "{}", s),
        }
    }
}

#[derive(Debug, Clone)]
pub struct MisalkanStatement {
    pub name: Pengenal,
    pub value: Expression,
}

impl fmt::Display for MisalkanStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "misalkan {} = {};", self.name, self.value)
    }
}

#[derive(Debug, Clone)]
pub struct KembalikanStatement {
    pub return_value: Expression,
}

impl fmt::Display for KembalikanStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "kembalikan {};", self.return_value)
    }
}

#[derive(Debug, Clone)]
pub struct EkspresiStatement {
    pub expression: Expression,
}

impl fmt::Display for EkspresiStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.expression)
    }
}

/// Pelemparan galat: `lemparkan "pesan galat";`
#[derive(Debug, Clone)]
pub struct LemparkanStatement {
    pub value: Expression,
}

impl fmt::Display for LemparkanStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "lemparkan {};", self.value)
    }
}

// ═══════════════════════════════════════════════════════════
//  Ekspresi (Expressions)
// ═══════════════════════════════════════════════════════════

#[derive(Debug, Clone)]
pub enum Expression {
    Pengenal(Pengenal),
    IntegerLiteral(i64),
    FloatLiteral(f64),
    StringLiteral(String),
    BooleanLiteral(bool),
    Awalan(AwalanExpression),
    Sisipan(SisipanExpression),
    Jika(JikaExpression),
    Selama(SelamaExpression),
    Untuk(UntukExpression),
    FungsiLiteral(FungsiLiteral),
    Panggilan(PanggilanExpression),
    ArrayLiteral(Vec<Expression>),
    Indeks(IndeksExpression),
    HashLiteral(Vec<(Expression, Expression)>),
    /// Penugasan: `x = 5`, `x += 3`
    Penugasan(PenugasanExpression),
    /// Akses properti: `obj.kunci`
    Titik(TitikExpression),
    /// Impor modul: `masukkan("file.tj")`
    Masukkan(MasukkanExpression),
    /// Fungsi panah: `(x) => x * 2` atau `(x) => { ... }`
    FungsiPanah(FungsiPanahLiteral),
    /// Penanganan galat: `coba { ... } tangkap (err) { ... }`
    Coba(CobaExpression),
    Null,
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Pengenal(id) => write!(f, "{}", id),
            Expression::IntegerLiteral(val) => write!(f, "{}", val),
            Expression::FloatLiteral(val) => write!(f, "{}", val),
            Expression::StringLiteral(val) => write!(f, "\"{}\"", val),
            Expression::BooleanLiteral(val) => {
                write!(f, "{}", if *val { "benar" } else { "salah" })
            }
            Expression::Awalan(expr) => write!(f, "{}", expr),
            Expression::Sisipan(expr) => write!(f, "{}", expr),
            Expression::Jika(expr) => write!(f, "{}", expr),
            Expression::Selama(expr) => write!(f, "{}", expr),
            Expression::Untuk(expr) => write!(f, "{}", expr),
            Expression::FungsiLiteral(expr) => write!(f, "{}", expr),
            Expression::Panggilan(expr) => write!(f, "{}", expr),
            Expression::ArrayLiteral(elements) => {
                let elems: Vec<String> = elements.iter().map(|e| e.to_string()).collect();
                write!(f, "[{}]", elems.join(", "))
            }
            Expression::Indeks(expr) => write!(f, "{}", expr),
            Expression::HashLiteral(pairs) => {
                let ps: Vec<String> = pairs.iter().map(|(k, v)| format!("{}: {}", k, v)).collect();
                write!(f, "{{{}}}", ps.join(", "))
            }
            Expression::Penugasan(expr) => write!(f, "{}", expr),
            Expression::Titik(expr) => write!(f, "{}", expr),
            Expression::Masukkan(expr) => write!(f, "{}", expr),
            Expression::FungsiPanah(expr) => write!(f, "{}", expr),
            Expression::Coba(expr) => write!(f, "{}", expr),
            Expression::Null => write!(f, "kosong"),
        }
    }
}

// ═══════════════════════════════════════════════════════════
//  Sub-struktur
// ═══════════════════════════════════════════════════════════

/// Pengenal (nama variabel, nama fungsi, dsb.)
#[derive(Debug, Clone)]
pub struct Pengenal {
    pub value: String,
}

impl fmt::Display for Pengenal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

/// Ekspresi awalan (prefix): `-x`, `!x`, `bukan x`
#[derive(Debug, Clone)]
pub struct AwalanExpression {
    pub operator: String,
    pub right: Box<Expression>,
}

impl fmt::Display for AwalanExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}{})", self.operator, self.right)
    }
}

/// Ekspresi sisipan (infix): `a + b`, `x == y`
#[derive(Debug, Clone)]
pub struct SisipanExpression {
    pub left: Box<Expression>,
    pub operator: String,
    pub right: Box<Expression>,
}

impl fmt::Display for SisipanExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} {} {})", self.left, self.operator, self.right)
    }
}

/// Blok pernyataan: `{ ... }`
#[derive(Debug, Clone)]
pub struct BlokPernyataan {
    pub statements: Vec<Statement>,
}

impl fmt::Display for BlokPernyataan {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for stmt in &self.statements {
            write!(f, "{}", stmt)?;
        }
        Ok(())
    }
}

/// Ekspresi kondisional: `jika (kondisi) { ... } lainnya { ... }`
#[derive(Debug, Clone)]
pub struct JikaExpression {
    pub condition: Box<Expression>,
    pub consequence: BlokPernyataan,
    pub alternative: Option<BlokPernyataan>,
}

impl fmt::Display for JikaExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "jika ({}) {{ {} }}", self.condition, self.consequence)?;
        if let Some(alt) = &self.alternative {
            write!(f, " lainnya {{ {} }}", alt)?;
        }
        Ok(())
    }
}

/// Perulangan selama: `selama (kondisi) { ... }`
#[derive(Debug, Clone)]
pub struct SelamaExpression {
    pub condition: Box<Expression>,
    pub body: BlokPernyataan,
}

impl fmt::Display for SelamaExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "selama ({}) {{ {} }}", self.condition, self.body)
    }
}

/// Perulangan untuk (gaya C): `untuk (<init>; <kondisi>; <pembaruan>) { ... }`
#[derive(Debug, Clone)]
pub struct UntukExpression {
    pub init: Box<Statement>,
    pub condition: Box<Expression>,
    pub update: Box<Statement>,
    pub body: BlokPernyataan,
}

impl fmt::Display for UntukExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "untuk ({}; {}; {}) {{ {} }}",
            self.init, self.condition, self.update, self.body
        )
    }
}

/// Literal fungsi: `fungsi(param) { ... }`
#[derive(Debug, Clone)]
pub struct FungsiLiteral {
    pub parameters: Vec<Pengenal>,
    pub body: BlokPernyataan,
}

impl fmt::Display for FungsiLiteral {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let params: Vec<String> = self.parameters.iter().map(|p| p.to_string()).collect();
        write!(f, "fungsi({}) {{ {} }}", params.join(", "), self.body)
    }
}

/// Pemanggilan fungsi: `fungsi(arg1, arg2)`
#[derive(Debug, Clone)]
pub struct PanggilanExpression {
    pub function: Box<Expression>,
    pub arguments: Vec<Expression>,
}

impl fmt::Display for PanggilanExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let args: Vec<String> = self.arguments.iter().map(|a| a.to_string()).collect();
        write!(f, "{}({})", self.function, args.join(", "))
    }
}

/// Akses indeks: `daftar[0]`, `kamus["kunci"]`
#[derive(Debug, Clone)]
pub struct IndeksExpression {
    pub left: Box<Expression>,
    pub index: Box<Expression>,
}

impl fmt::Display for IndeksExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}[{}])", self.left, self.index)
    }
}

/// Ekspresi penugasan: `x = 5`, `arr[0] = 5`, atau `x += 3`
#[derive(Debug, Clone)]
pub struct PenugasanExpression {
    pub left: Box<Expression>,
    pub operator: String, // "=", "+=", "-=", "*=", "/="
    pub value: Box<Expression>,
}

impl fmt::Display for PenugasanExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.left, self.operator, self.value)
    }
}

/// Akses properti titik: `obj.kunci`
#[derive(Debug, Clone)]
pub struct TitikExpression {
    pub left: Box<Expression>,
    pub key: String,
}

impl fmt::Display for TitikExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}", self.left, self.key)
    }
}

/// Impor modul: `masukkan("file.tj")`
#[derive(Debug, Clone)]
pub struct MasukkanExpression {
    pub path: Box<Expression>,
}

impl fmt::Display for MasukkanExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "masukkan({})", self.path)
    }
}

/// Fungsi panah: `(x, y) => x + y` atau `(x) => { ... }`
#[derive(Debug, Clone)]
pub struct FungsiPanahLiteral {
    pub parameters: Vec<Pengenal>,
    pub body: BlokPernyataan,
}

impl fmt::Display for FungsiPanahLiteral {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let params: Vec<String> = self.parameters.iter().map(|p| p.to_string()).collect();
        write!(f, "({}) => {{ {} }}", params.join(", "), self.body)
    }
}

/// Penanganan galat: `coba { ... } tangkap (err) { ... }`
#[derive(Debug, Clone)]
pub struct CobaExpression {
    pub body: BlokPernyataan,
    pub error_ident: Pengenal,
    pub handler: BlokPernyataan,
}

impl fmt::Display for CobaExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "coba {{ {} }} tangkap ({}) {{ {} }}",
            self.body, self.error_ident, self.handler
        )
    }
}
