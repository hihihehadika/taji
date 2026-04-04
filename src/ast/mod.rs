/// Modul AST (Abstract Syntax Tree) untuk bahasa Taji.

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
//  Statements
// ═══════════════════════════════════════════════════════════

#[derive(Debug, Clone)]
pub enum Statement {
    Misalkan(MisalkanStatement),
    Kembalikan(KembalikanStatement),
    Ekspresi(EkspresiStatement),
    Berhenti,
    Lanjut,
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Statement::Misalkan(s) => write!(f, "{}", s),
            Statement::Kembalikan(s) => write!(f, "{}", s),
            Statement::Ekspresi(s) => write!(f, "{}", s),
            Statement::Berhenti => write!(f, "berhenti;"),
            Statement::Lanjut => write!(f, "lanjut;"),
        }
    }
}

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

// ═══════════════════════════════════════════════════════════
//  Expressions
// ═══════════════════════════════════════════════════════════

#[derive(Debug, Clone)]
pub enum Expression {
    Identifier(Identifier),
    IntegerLiteral(i64),
    FloatLiteral(f64),
    StringLiteral(String),
    BooleanLiteral(bool),
    Prefix(PrefixExpression),
    Infix(InfixExpression),
    Jika(JikaExpression),
    Selama(SelamaExpression),
    Untuk(UntukExpression),
    FungsiLiteral(FungsiLiteral),
    Panggilan(PanggilanExpression),
    ArrayLiteral(Vec<Expression>),
    IndexExpression(IndexExpression),
    HashLiteral(Vec<(Expression, Expression)>),
    /// Assignment: `x = 5`, `x += 3`
    Assign(AssignExpression),
    /// Akses properti: `obj.kunci`
    DotExpression(DotExpression),
    /// Import modul: `masukkan("file.tj")`
    Masukkan(MasukkanExpression),
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Identifier(id) => write!(f, "{}", id),
            Expression::IntegerLiteral(val) => write!(f, "{}", val),
            Expression::FloatLiteral(val) => write!(f, "{}", val),
            Expression::StringLiteral(val) => write!(f, "\"{}\"", val),
            Expression::BooleanLiteral(val) => {
                write!(f, "{}", if *val { "benar" } else { "salah" })
            }
            Expression::Prefix(expr) => write!(f, "{}", expr),
            Expression::Infix(expr) => write!(f, "{}", expr),
            Expression::Jika(expr) => write!(f, "{}", expr),
            Expression::Selama(expr) => write!(f, "{}", expr),
            Expression::Untuk(expr) => write!(f, "{}", expr),
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
            Expression::Assign(expr) => write!(f, "{}", expr),
            Expression::DotExpression(expr) => write!(f, "{}", expr),
            Expression::Masukkan(expr) => write!(f, "{}", expr),
        }
    }
}

// ═══════════════════════════════════════════════════════════
//  Sub-structures
// ═══════════════════════════════════════════════════════════

#[derive(Debug, Clone)]
pub struct Identifier {
    pub value: String,
}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

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

/// C-style for loop: `untuk (<init>; <condition>; <update>) { <body> }`
#[derive(Debug, Clone)]
pub struct UntukExpression {
    pub init: Box<Statement>,
    pub condition: Box<Expression>,
    pub update: Box<Statement>,
    pub body: BlockStatement,
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

/// Assignment expression: `x = 5` or `x += 3`
#[derive(Debug, Clone)]
pub struct AssignExpression {
    pub name: Identifier,
    pub operator: String, // "=", "+=", "-=", "*=", "/="
    pub value: Box<Expression>,
}

impl fmt::Display for AssignExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.name, self.operator, self.value)
    }
}

/// Dot access: `obj.kunci`
#[derive(Debug, Clone)]
pub struct DotExpression {
    pub left: Box<Expression>,
    pub key: String,
}

impl fmt::Display for DotExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}", self.left, self.key)
    }
}

/// Import: `masukkan("file.tj")`
#[derive(Debug, Clone)]
pub struct MasukkanExpression {
    pub path: Box<Expression>,
}

impl fmt::Display for MasukkanExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "masukkan({})", self.path)
    }
}
