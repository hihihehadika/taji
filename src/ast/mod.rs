/// Modul AST (Abstract Syntax Tree) untuk bahasa Taji.
///
/// AST adalah representasi terstruktur dari kode sumber setelah
/// dianalisis oleh Parser. Setiap node dalam pohon mewakili
/// sebuah konstruksi sintaksis (ekspresi, pernyataan, dsb.).

use std::fmt;

// ═══════════════════════════════════════════════════════════
//  Program — Akar dari seluruh pohon AST
// ═══════════════════════════════════════════════════════════

/// Representasi keseluruhan program Taji.
///
/// Sebuah program adalah urutan pernyataan (`Statement`) yang
/// dieksekusi secara berurutan dari atas ke bawah.
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
//  Statements — Pernyataan (tidak menghasilkan nilai)
// ═══════════════════════════════════════════════════════════

/// Semua jenis pernyataan yang didukung Taji.
#[derive(Debug, Clone)]
pub enum Statement {
    /// `misalkan x = <ekspresi>;`
    Misalkan(MisalkanStatement),

    /// `kembalikan <ekspresi>;`
    Kembalikan(KembalikanStatement),

    /// Pernyataan ekspresi biasa (misal: `tambah(5, 10);`)
    Ekspresi(EkspresiStatement),
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Statement::Misalkan(s) => write!(f, "{}", s),
            Statement::Kembalikan(s) => write!(f, "{}", s),
            Statement::Ekspresi(s) => write!(f, "{}", s),
        }
    }
}

/// Pernyataan penetapan variabel: `misalkan <nama> = <nilai>;`
#[derive(Debug, Clone)]
pub struct MisalkanStatement {
    pub name: Identifier,
    pub value: Expression,
}

impl fmt::Display for MisalkanStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "misalkan {} = {};", self.name, self.value)
    }
}

/// Pernyataan pengembalian nilai: `kembalikan <ekspresi>;`
#[derive(Debug, Clone)]
pub struct KembalikanStatement {
    pub return_value: Expression,
}

impl fmt::Display for KembalikanStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "kembalikan {};", self.return_value)
    }
}

/// Pernyataan yang berisi sebuah ekspresi.
#[derive(Debug, Clone)]
pub struct EkspresiStatement {
    pub expression: Expression,
}

impl fmt::Display for EkspresiStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.expression)
    }
}

// ═══════════════════════════════════════════════════════════
//  Expressions — Ekspresi (menghasilkan nilai)
// ═══════════════════════════════════════════════════════════

/// Semua jenis ekspresi yang didukung Taji.
///
/// Ekspresi adalah "sesuatu yang menghasilkan nilai",
/// misalnya `5 + 3`, `benar`, atau `fungsi(x) { x * 2 }`.
#[derive(Debug, Clone)]
pub enum Expression {
    /// Nama variabel atau fungsi: `x`, `tambah`, `harga`
    Identifier(Identifier),

    /// Angka bulat literal: `42`, `1000`
    IntegerLiteral(i64),

    /// Teks literal: `"halo dunia"`
    StringLiteral(String),

    /// Nilai boolean: `benar` atau `salah`
    BooleanLiteral(bool),

    /// Ekspresi awalan (prefix): `-5`, `!benar`, `bukan salah`
    Prefix(PrefixExpression),

    /// Ekspresi sisipan (infix): `5 + 3`, `x * y`, `a == b`
    Infix(InfixExpression),

    /// Blok kondisional: `jika (kondisi) { ... } lainnya { ... }`
    Jika(JikaExpression),

    /// Perulangan: `selama (kondisi) { ... }`
    Selama(SelamaExpression),

    /// Definisi fungsi: `fungsi(x, y) { kembalikan x + y; }`
    FungsiLiteral(FungsiLiteral),

    /// Pemanggilan fungsi: `tambah(5, 10)`
    Panggilan(PanggilanExpression),

    /// Daftar literal (array): `[1, 2, 3]`
    ArrayLiteral(Vec<Expression>),

    /// Akses indeks: `daftar[0]`
    IndexExpression(IndexExpression),

    /// Kamus literal (hash map): `{"kunci": "nilai"}`
    HashLiteral(Vec<(Expression, Expression)>),
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Identifier(id) => write!(f, "{}", id),
            Expression::IntegerLiteral(val) => write!(f, "{}", val),
            Expression::StringLiteral(val) => write!(f, "\"{}\"", val),
            Expression::BooleanLiteral(val) => {
                write!(f, "{}", if *val { "benar" } else { "salah" })
            }
            Expression::Prefix(expr) => write!(f, "{}", expr),
            Expression::Infix(expr) => write!(f, "{}", expr),
            Expression::Jika(expr) => write!(f, "{}", expr),
            Expression::Selama(expr) => write!(f, "{}", expr),
            Expression::FungsiLiteral(expr) => write!(f, "{}", expr),
            Expression::Panggilan(expr) => write!(f, "{}", expr),
            Expression::ArrayLiteral(elements) => {
                let elems: Vec<String> = elements.iter().map(|e| e.to_string()).collect();
                write!(f, "[{}]", elems.join(", "))
            }
            Expression::IndexExpression(expr) => write!(f, "{}", expr),
            Expression::HashLiteral(pairs) => {
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
//  Sub-structures — Struktur pendukung ekspresi
// ═══════════════════════════════════════════════════════════

/// Identifier (nama variabel/fungsi).
#[derive(Debug, Clone)]
pub struct Identifier {
    pub value: String,
}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

/// Ekspresi awalan: `<operator><operand>` (misal: `-5`, `!benar`)
#[derive(Debug, Clone)]
pub struct PrefixExpression {
    pub operator: String,
    pub right: Box<Expression>,
}

impl fmt::Display for PrefixExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}{})", self.operator, self.right)
    }
}

/// Ekspresi sisipan: `<kiri> <operator> <kanan>` (misal: `5 + 3`)
#[derive(Debug, Clone)]
pub struct InfixExpression {
    pub left: Box<Expression>,
    pub operator: String,
    pub right: Box<Expression>,
}

impl fmt::Display for InfixExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} {} {})", self.left, self.operator, self.right)
    }
}

/// Blok kode yang berisi beberapa pernyataan.
#[derive(Debug, Clone)]
pub struct BlockStatement {
    pub statements: Vec<Statement>,
}

impl fmt::Display for BlockStatement {
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
    pub consequence: BlockStatement,
    pub alternative: Option<BlockStatement>,
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

/// Perulangan: `selama (kondisi) { ... }`
#[derive(Debug, Clone)]
pub struct SelamaExpression {
    pub condition: Box<Expression>,
    pub body: BlockStatement,
}

impl fmt::Display for SelamaExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "selama ({}) {{ {} }}", self.condition, self.body)
    }
}

/// Definisi fungsi: `fungsi(param1, param2) { ... }`
#[derive(Debug, Clone)]
pub struct FungsiLiteral {
    pub parameters: Vec<Identifier>,
    pub body: BlockStatement,
}

impl fmt::Display for FungsiLiteral {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let params: Vec<String> = self.parameters.iter().map(|p| p.to_string()).collect();
        write!(f, "fungsi({}) {{ {} }}", params.join(", "), self.body)
    }
}

/// Pemanggilan fungsi: `<fungsi>(<argumen>)`
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

/// Akses indeks: `<objek>[<indeks>]`
#[derive(Debug, Clone)]
pub struct IndexExpression {
    pub left: Box<Expression>,
    pub index: Box<Expression>,
}

impl fmt::Display for IndexExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}[{}])", self.left, self.index)
    }
}
