/// Modul Parser untuk bahasa Taji.
///
/// Menggunakan algoritma **Pratt Parser** (Top-Down Operator Precedence)
/// untuk mengubah urutan Token menjadi Abstract Syntax Tree (AST).
///
/// Pratt Parser bekerja dengan memberikan "kekuatan ikat" (precedence)
/// pada setiap operator, sehingga `2 + 3 * 4` otomatis diurai menjadi
/// `2 + (3 * 4)` tanpa memerlukan aturan tata bahasa yang eksplisit.

use crate::ast::*;
use crate::lexer::Lexer;
use crate::token::{Token, TokenType};

// ═══════════════════════════════════════════════════════════
//  Precedence — Tingkat Prioritas Operator
// ═══════════════════════════════════════════════════════════

/// Urutan prioritas operator dari terendah ke tertinggi.
///
/// Semakin tinggi nilainya, semakin "erat" operator itu mengikat
/// operand-nya. Contoh: `*` lebih erat dari `+`.
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum Precedence {
    Lowest = 0,
    LogicalOr,   // atau
    LogicalAnd,  // dan
    Equals,      // == !=
    LessGreater, // < > <= >=
    Sum,         // + -
    Product,     // * / %
    Prefix,      // -x !x bukan x
    Call,        // fungsi(x)
    Index,       // daftar[0]
}

/// Menentukan tingkat prioritas berdasarkan tipe token.
fn token_precedence(token_type: &TokenType) -> Precedence {
    match token_type {
        TokenType::Atau => Precedence::LogicalOr,
        TokenType::Dan => Precedence::LogicalAnd,
        TokenType::Eq | TokenType::NotEq => Precedence::Equals,
        TokenType::Lt | TokenType::Gt | TokenType::LtEq | TokenType::GtEq => {
            Precedence::LessGreater
        }
        TokenType::Plus | TokenType::Minus => Precedence::Sum,
        TokenType::Asterisk | TokenType::Slash | TokenType::Modulo => Precedence::Product,
        TokenType::Lparen => Precedence::Call,
        TokenType::Lbracket => Precedence::Index,
        _ => Precedence::Lowest,
    }
}

// ═══════════════════════════════════════════════════════════
//  Parser
// ═══════════════════════════════════════════════════════════

/// Parser utama untuk bahasa Taji.
///
/// Menyimpan state pemindaian token dan menghasilkan AST.
pub struct Parser {
    lexer: Lexer,
    cur_token: Token,
    peek_token: Token,
    pub errors: Vec<String>,
}

impl Parser {
    /// Membuat Parser baru dari Lexer.
    pub fn new(mut lexer: Lexer) -> Self {
        let cur_token = lexer.next_token();
        let peek_token = lexer.next_token();

        Parser {
            lexer,
            cur_token,
            peek_token,
            errors: vec![],
        }
    }

    // ── Token navigation ───────────────────────────────

    /// Memajukan parser ke token berikutnya.
    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    /// Memeriksa tipe token saat ini.
    fn cur_token_is(&self, t: &TokenType) -> bool {
        self.cur_token.type_ == *t
    }

    /// Memeriksa tipe token berikutnya (peek).
    fn peek_token_is(&self, t: &TokenType) -> bool {
        self.peek_token.type_ == *t
    }

    /// Mendapatkan prioritas token berikutnya.
    fn peek_precedence(&self) -> Precedence {
        token_precedence(&self.peek_token.type_)
    }

    /// Mendapatkan prioritas token saat ini.
    fn cur_precedence(&self) -> Precedence {
        token_precedence(&self.cur_token.type_)
    }

    /// Mengharapkan token berikutnya bertipe tertentu.
    /// Jika benar, parser maju. Jika salah, catat error.
    fn expect_peek(&mut self, t: &TokenType) -> bool {
        if self.peek_token_is(t) {
            self.next_token();
            true
        } else {
            self.peek_error(t);
            false
        }
    }

    /// Mencatat error ketika token berikutnya tidak sesuai harapan.
    fn peek_error(&mut self, expected: &TokenType) {
        let msg = format!(
            "Kesalahan: diharapkan token {:?}, tetapi ditemukan {:?} (\"{}\")",
            expected, self.peek_token.type_, self.peek_token.literal
        );
        self.errors.push(msg);
    }

    /// Mencatat error ketika tidak ada fungsi prefix untuk token ini.
    fn no_prefix_parse_error(&mut self, t: &TokenType) {
        let msg = format!(
            "Kesalahan: tidak ada aturan parsing prefix untuk {:?}",
            t
        );
        self.errors.push(msg);
    }

    // ═══════════════════════════════════════════════════════
    //  Entry point — Parsing program
    // ═══════════════════════════════════════════════════════

    /// Mem-parse seluruh kode sumber menjadi AST `Program`.
    pub fn parse_program(&mut self) -> Program {
        let mut program = Program {
            statements: vec![],
        };

        while !self.cur_token_is(&TokenType::Eof) {
            if let Some(stmt) = self.parse_statement() {
                program.statements.push(stmt);
            }
            self.next_token();
        }

        program
    }

    // ═══════════════════════════════════════════════════════
    //  Statement parsing
    // ═══════════════════════════════════════════════════════

    /// Mem-parse satu pernyataan berdasarkan token saat ini.
    fn parse_statement(&mut self) -> Option<Statement> {
        match self.cur_token.type_ {
            TokenType::Misalkan => self.parse_misalkan_statement(),
            TokenType::Kembalikan => self.parse_kembalikan_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    /// Mem-parse `misalkan <nama> = <ekspresi>;`
    fn parse_misalkan_statement(&mut self) -> Option<Statement> {
        if !self.expect_peek(&TokenType::Ident) {
            return None;
        }

        let name = Identifier {
            value: self.cur_token.literal.clone(),
        };

        if !self.expect_peek(&TokenType::Assign) {
            return None;
        }

        self.next_token();

        let value = self.parse_expression(Precedence::Lowest)?;

        if self.peek_token_is(&TokenType::Semicolon) {
            self.next_token();
        }

        Some(Statement::Misalkan(MisalkanStatement { name, value }))
    }

    /// Mem-parse `kembalikan <ekspresi>;`
    fn parse_kembalikan_statement(&mut self) -> Option<Statement> {
        self.next_token();

        let return_value = self.parse_expression(Precedence::Lowest)?;

        if self.peek_token_is(&TokenType::Semicolon) {
            self.next_token();
        }

        Some(Statement::Kembalikan(KembalikanStatement {
            return_value,
        }))
    }

    /// Mem-parse pernyataan ekspresi (ekspresi yang berdiri sendiri).
    fn parse_expression_statement(&mut self) -> Option<Statement> {
        let expression = self.parse_expression(Precedence::Lowest)?;

        if self.peek_token_is(&TokenType::Semicolon) {
            self.next_token();
        }

        Some(Statement::Ekspresi(EkspresiStatement { expression }))
    }

    // ═══════════════════════════════════════════════════════
    //  Expression parsing (Pratt Parser core)
    // ═══════════════════════════════════════════════════════

    /// Inti dari Pratt Parser.
    ///
    /// Mem-parse sebuah ekspresi dengan memperhatikan prioritas operator.
    /// Akan terus menggabungkan ekspresi selama operator berikutnya
    /// memiliki prioritas lebih tinggi dari `precedence`.
    fn parse_expression(&mut self, precedence: Precedence) -> Option<Expression> {
        // 1. Parse prefix (bagian kiri)
        let mut left = self.parse_prefix()?;

        // 2. Selama operator berikutnya lebih kuat, gabungkan ke kanan
        while !self.peek_token_is(&TokenType::Semicolon)
            && precedence < self.peek_precedence()
        {
            // Periksa apakah ada aturan infix untuk token berikutnya
            if !self.has_infix_rule(&self.peek_token.type_.clone()) {
                return Some(left);
            }

            self.next_token();
            left = self.parse_infix(left)?;
        }

        Some(left)
    }

    /// Memeriksa apakah tipe token memiliki aturan infix.
    fn has_infix_rule(&self, t: &TokenType) -> bool {
        matches!(
            t,
            TokenType::Plus
                | TokenType::Minus
                | TokenType::Asterisk
                | TokenType::Slash
                | TokenType::Modulo
                | TokenType::Eq
                | TokenType::NotEq
                | TokenType::Lt
                | TokenType::Gt
                | TokenType::LtEq
                | TokenType::GtEq
                | TokenType::Dan
                | TokenType::Atau
                | TokenType::Lparen
                | TokenType::Lbracket
        )
    }

    // ── Prefix parsing ─────────────────────────────────

    /// Mem-parse ekspresi prefix berdasarkan token saat ini.
    fn parse_prefix(&mut self) -> Option<Expression> {
        match self.cur_token.type_ {
            TokenType::Ident => Some(self.parse_identifier()),
            TokenType::Int => self.parse_integer_literal(),
            TokenType::Str => Some(self.parse_string_literal()),
            TokenType::Benar | TokenType::Salah => Some(self.parse_boolean_literal()),
            TokenType::Bang | TokenType::Minus | TokenType::Bukan => {
                self.parse_prefix_expression()
            }
            TokenType::Lparen => self.parse_grouped_expression(),
            TokenType::Jika => self.parse_jika_expression(),
            TokenType::Selama => self.parse_selama_expression(),
            TokenType::Fungsi => self.parse_fungsi_literal(),
            TokenType::Lbracket => self.parse_array_literal(),
            TokenType::Lbrace => self.parse_hash_literal(),
            _ => {
                self.no_prefix_parse_error(&self.cur_token.type_.clone());
                None
            }
        }
    }

    fn parse_identifier(&self) -> Expression {
        Expression::Identifier(Identifier {
            value: self.cur_token.literal.clone(),
        })
    }

    fn parse_integer_literal(&mut self) -> Option<Expression> {
        match self.cur_token.literal.parse::<i64>() {
            Ok(val) => Some(Expression::IntegerLiteral(val)),
            Err(_) => {
                let msg = format!(
                    "Kesalahan: tidak bisa mengurai \"{}\" sebagai angka bulat",
                    self.cur_token.literal
                );
                self.errors.push(msg);
                None
            }
        }
    }

    fn parse_string_literal(&self) -> Expression {
        Expression::StringLiteral(self.cur_token.literal.clone())
    }

    fn parse_boolean_literal(&self) -> Expression {
        Expression::BooleanLiteral(self.cur_token_is(&TokenType::Benar))
    }

    /// Mem-parse ekspresi prefix: `<op><ekspresi>` (misal: `-5`, `!benar`)
    fn parse_prefix_expression(&mut self) -> Option<Expression> {
        let operator = self.cur_token.literal.clone();

        self.next_token();

        let right = self.parse_expression(Precedence::Prefix)?;

        Some(Expression::Prefix(PrefixExpression {
            operator,
            right: Box::new(right),
        }))
    }

    /// Mem-parse ekspresi dalam tanda kurung: `( <ekspresi> )`
    fn parse_grouped_expression(&mut self) -> Option<Expression> {
        self.next_token();

        let expr = self.parse_expression(Precedence::Lowest)?;

        if !self.expect_peek(&TokenType::Rparen) {
            return None;
        }

        Some(expr)
    }

    // ── Infix parsing ──────────────────────────────────

    /// Mem-parse ekspresi infix berdasarkan token saat ini.
    fn parse_infix(&mut self, left: Expression) -> Option<Expression> {
        match self.cur_token.type_ {
            TokenType::Lparen => self.parse_panggilan_expression(left),
            TokenType::Lbracket => self.parse_index_expression(left),
            _ => self.parse_infix_expression(left),
        }
    }

    /// Mem-parse ekspresi infix: `<kiri> <op> <kanan>`
    fn parse_infix_expression(&mut self, left: Expression) -> Option<Expression> {
        let operator = self.cur_token.literal.clone();
        let precedence = self.cur_precedence();

        self.next_token();

        let right = self.parse_expression(precedence)?;

        Some(Expression::Infix(InfixExpression {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }))
    }

    // ── Complex expression parsing ─────────────────────

    /// Mem-parse `jika (kondisi) { ... } lainnya { ... }`
    fn parse_jika_expression(&mut self) -> Option<Expression> {
        if !self.expect_peek(&TokenType::Lparen) {
            return None;
        }

        self.next_token();
        let condition = self.parse_expression(Precedence::Lowest)?;

        if !self.expect_peek(&TokenType::Rparen) {
            return None;
        }

        if !self.expect_peek(&TokenType::Lbrace) {
            return None;
        }

        let consequence = self.parse_block_statement();

        let alternative = if self.peek_token_is(&TokenType::Lainnya) {
            self.next_token();

            if !self.expect_peek(&TokenType::Lbrace) {
                return None;
            }

            Some(self.parse_block_statement())
        } else {
            None
        };

        Some(Expression::Jika(JikaExpression {
            condition: Box::new(condition),
            consequence,
            alternative,
        }))
    }

    /// Mem-parse `selama (kondisi) { ... }`
    fn parse_selama_expression(&mut self) -> Option<Expression> {
        if !self.expect_peek(&TokenType::Lparen) {
            return None;
        }

        self.next_token();
        let condition = self.parse_expression(Precedence::Lowest)?;

        if !self.expect_peek(&TokenType::Rparen) {
            return None;
        }

        if !self.expect_peek(&TokenType::Lbrace) {
            return None;
        }

        let body = self.parse_block_statement();

        Some(Expression::Selama(SelamaExpression {
            condition: Box::new(condition),
            body,
        }))
    }

    /// Mem-parse blok kode: `{ <pernyataan>... }`
    fn parse_block_statement(&mut self) -> BlockStatement {
        self.next_token();

        let mut statements = vec![];

        while !self.cur_token_is(&TokenType::Rbrace) && !self.cur_token_is(&TokenType::Eof) {
            if let Some(stmt) = self.parse_statement() {
                statements.push(stmt);
            }
            self.next_token();
        }

        BlockStatement { statements }
    }

    /// Mem-parse `fungsi(param1, param2) { ... }`
    fn parse_fungsi_literal(&mut self) -> Option<Expression> {
        if !self.expect_peek(&TokenType::Lparen) {
            return None;
        }

        let parameters = self.parse_function_parameters()?;

        if !self.expect_peek(&TokenType::Lbrace) {
            return None;
        }

        let body = self.parse_block_statement();

        Some(Expression::FungsiLiteral(FungsiLiteral {
            parameters,
            body,
        }))
    }

    /// Mem-parse daftar parameter fungsi: `(a, b, c)`
    fn parse_function_parameters(&mut self) -> Option<Vec<Identifier>> {
        let mut identifiers = vec![];

        if self.peek_token_is(&TokenType::Rparen) {
            self.next_token();
            return Some(identifiers);
        }

        self.next_token();

        identifiers.push(Identifier {
            value: self.cur_token.literal.clone(),
        });

        while self.peek_token_is(&TokenType::Comma) {
            self.next_token(); // lewati koma
            self.next_token(); // pindah ke parameter berikutnya

            identifiers.push(Identifier {
                value: self.cur_token.literal.clone(),
            });
        }

        if !self.expect_peek(&TokenType::Rparen) {
            return None;
        }

        Some(identifiers)
    }

    /// Mem-parse pemanggilan fungsi: `<fungsi>(arg1, arg2)`
    fn parse_panggilan_expression(&mut self, function: Expression) -> Option<Expression> {
        let arguments = self.parse_expression_list(&TokenType::Rparen)?;

        Some(Expression::Panggilan(PanggilanExpression {
            function: Box::new(function),
            arguments,
        }))
    }

    /// Mem-parse daftar literal array: `[elem1, elem2, ...]`
    fn parse_array_literal(&mut self) -> Option<Expression> {
        let elements = self.parse_expression_list(&TokenType::Rbracket)?;
        Some(Expression::ArrayLiteral(elements))
    }

    /// Mem-parse akses indeks: `<objek>[<indeks>]`
    fn parse_index_expression(&mut self, left: Expression) -> Option<Expression> {
        self.next_token();

        let index = self.parse_expression(Precedence::Lowest)?;

        if !self.expect_peek(&TokenType::Rbracket) {
            return None;
        }

        Some(Expression::IndexExpression(IndexExpression {
            left: Box::new(left),
            index: Box::new(index),
        }))
    }

    /// Mem-parse kamus literal: `{ kunci: nilai, ... }`
    fn parse_hash_literal(&mut self) -> Option<Expression> {
        let mut pairs = vec![];

        while !self.peek_token_is(&TokenType::Rbrace) {
            self.next_token();
            let key = self.parse_expression(Precedence::Lowest)?;

            if !self.expect_peek(&TokenType::Colon) {
                return None;
            }

            self.next_token();
            let value = self.parse_expression(Precedence::Lowest)?;

            pairs.push((key, value));

            if !self.peek_token_is(&TokenType::Rbrace) && !self.expect_peek(&TokenType::Comma) {
                return None;
            }
        }

        if !self.expect_peek(&TokenType::Rbrace) {
            return None;
        }

        Some(Expression::HashLiteral(pairs))
    }

    // ── Helper: Parsing daftar ekspresi ────────────────

    /// Mem-parse daftar ekspresi yang dipisahkan koma
    /// hingga ditemukan token penutup (`end`).
    fn parse_expression_list(&mut self, end: &TokenType) -> Option<Vec<Expression>> {
        let mut list = vec![];

        if self.peek_token_is(end) {
            self.next_token();
            return Some(list);
        }

        self.next_token();
        list.push(self.parse_expression(Precedence::Lowest)?);

        while self.peek_token_is(&TokenType::Comma) {
            self.next_token(); // lewati koma
            self.next_token(); // pindah ke ekspresi berikutnya
            list.push(self.parse_expression(Precedence::Lowest)?);
        }

        if !self.expect_peek(end) {
            return None;
        }

        Some(list)
    }
}
