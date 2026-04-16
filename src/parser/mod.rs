/// Modul Parser untuk bahasa Taji.
///
/// Menggunakan algoritma **Pratt Parser** (Top-Down Operator Precedence)
/// untuk mengubah deretan token menjadi pohon sintaks (AST).
use crate::ast::*;
use crate::lexer::Lexer;
use crate::token::{Token, TokenType};

// ═══════════════════════════════════════════════════════════
//  Prioritas Operator (Precedence)
// ═══════════════════════════════════════════════════════════

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum Precedence {
    Lowest = 0,
    Assign,      // = += -= *= /=
    LogicalOr,   // atau
    LogicalAnd,  // dan
    Equals,      // == !=
    LessGreater, // < > <= >=
    Sum,         // + -
    Product,     // * / %
    Prefix,      // -x !x bukan x
    Call,        // fungsi(x)
    Index,       // daftar[0]
    Dot,         // obj.kunci
}

/// Menentukan prioritas operator berdasarkan tipe token.
fn token_precedence(token_type: &TokenType) -> Precedence {
    match token_type {
        TokenType::Assign
        | TokenType::PlusEq
        | TokenType::MinusEq
        | TokenType::MulEq
        | TokenType::DivEq => Precedence::Assign,
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
        TokenType::Dot => Precedence::Dot,
        _ => Precedence::Lowest,
    }
}

// ═══════════════════════════════════════════════════════════
//  Parser
// ═══════════════════════════════════════════════════════════

pub struct Parser {
    lexer: Lexer,
    cur_token: Token,
    peek_token: Token,
    pub errors: Vec<String>,
}

impl Parser {
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

    /// Memajukan ke token berikutnya.
    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    /// Menyimpan snapshot keadaan parser saat ini untuk backtracking.
    fn lexer_snapshot(&self) -> (Lexer, Token, Token, Vec<String>) {
        (
            self.lexer.clone(),
            self.cur_token.clone(),
            self.peek_token.clone(),
            self.errors.clone(),
        )
    }

    /// Mengembalikan parser ke keadaan snapshot sebelumnya.
    fn restore_snapshot(&mut self, snapshot: (Lexer, Token, Token, Vec<String>)) {
        self.lexer = snapshot.0;
        self.cur_token = snapshot.1;
        self.peek_token = snapshot.2;
        self.errors = snapshot.3;
    }

    fn cur_token_is(&self, t: &TokenType) -> bool {
        self.cur_token.type_ == *t
    }

    fn peek_token_is(&self, t: &TokenType) -> bool {
        self.peek_token.type_ == *t
    }

    fn peek_precedence(&self) -> Precedence {
        token_precedence(&self.peek_token.type_)
    }

    fn cur_precedence(&self) -> Precedence {
        token_precedence(&self.cur_token.type_)
    }

    /// Memeriksa apakah token berikutnya sesuai harapan, lalu maju.
    fn expect_peek(&mut self, t: &TokenType) -> bool {
        if self.peek_token_is(t) {
            self.next_token();
            true
        } else {
            self.peek_error(t);
            false
        }
    }

    fn peek_error(&mut self, expected: &TokenType) {
        let msg = format!(
            "Kesalahan: diharapkan token {:?}, tetapi ditemukan {:?} (\"{}\")",
            expected, self.peek_token.type_, self.peek_token.literal
        );
        self.errors.push(msg);
    }

    fn no_prefix_parse_error(&mut self, t: &TokenType) {
        let msg = format!("Kesalahan: tidak ada aturan parsing awalan untuk {:?}", t);
        self.errors.push(msg);
    }

    // ═══════════════════════════════════════════════════════
    //  Titik masuk
    // ═══════════════════════════════════════════════════════

    pub fn parse_program(&mut self) -> Program {
        let mut program = Program { statements: vec![] };

        while !self.cur_token_is(&TokenType::Eof) {
            if let Some(stmt) = self.parse_statement() {
                program.statements.push(stmt);
            }
            self.next_token();
        }

        program
    }

    // ═══════════════════════════════════════════════════════
    //  Parsing pernyataan
    // ═══════════════════════════════════════════════════════

    fn parse_statement(&mut self) -> Option<Statement> {
        match self.cur_token.type_ {
            TokenType::Misalkan => self.parse_misalkan_statement(),
            TokenType::Kembalikan => self.parse_kembalikan_statement(),
            TokenType::Lemparkan => self.parse_lemparkan_statement(),
            TokenType::Berhenti => {
                if self.peek_token_is(&TokenType::Semicolon) {
                    self.next_token();
                }
                Some(Statement::Berhenti)
            }
            TokenType::Lanjut => {
                if self.peek_token_is(&TokenType::Semicolon) {
                    self.next_token();
                }
                Some(Statement::Lanjut)
            }
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_misalkan_statement(&mut self) -> Option<Statement> {
        if !self.expect_peek(&TokenType::Ident) {
            return None;
        }

        let name = Pengenal {
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

    fn parse_kembalikan_statement(&mut self) -> Option<Statement> {
        self.next_token();

        let return_value = self.parse_expression(Precedence::Lowest)?;

        if self.peek_token_is(&TokenType::Semicolon) {
            self.next_token();
        }

        Some(Statement::Kembalikan(KembalikanStatement { return_value }))
    }

    /// Parsing `lemparkan <ekspresi>;`
    /// Mekanisme identis dengan `kembalikan` — melempar Object::Error.
    fn parse_lemparkan_statement(&mut self) -> Option<Statement> {
        self.next_token();

        let value = self.parse_expression(Precedence::Lowest)?;

        if self.peek_token_is(&TokenType::Semicolon) {
            self.next_token();
        }

        Some(Statement::Lemparkan(LemparkanStatement { value }))
    }

    fn parse_expression_statement(&mut self) -> Option<Statement> {
        let expression = self.parse_expression(Precedence::Lowest)?;

        if self.peek_token_is(&TokenType::Semicolon) {
            self.next_token();
        }

        Some(Statement::Ekspresi(EkspresiStatement { expression }))
    }

    // ═══════════════════════════════════════════════════════
    //  Parsing ekspresi (inti Pratt Parser)
    // ═══════════════════════════════════════════════════════

    fn parse_expression(&mut self, precedence: Precedence) -> Option<Expression> {
        let mut left = self.parse_prefix()?;

        while !self.peek_token_is(&TokenType::Semicolon) && precedence < self.peek_precedence() {
            if !self.has_infix_rule(&self.peek_token.type_.clone()) {
                return Some(left);
            }

            self.next_token();
            left = self.parse_infix(left)?;
        }

        Some(left)
    }

    /// Memeriksa apakah tipe token memiliki aturan parsing sisipan.
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
                | TokenType::Assign
                | TokenType::PlusEq
                | TokenType::MinusEq
                | TokenType::MulEq
                | TokenType::DivEq
                | TokenType::Dot
        )
    }

    // ── Parsing awalan (prefix) ─────────────────────────

    fn parse_prefix(&mut self) -> Option<Expression> {
        match self.cur_token.type_ {
            TokenType::Ident => Some(self.parse_identifier()),
            TokenType::Int => self.parse_integer_literal(),
            TokenType::Float => self.parse_float_literal(),
            TokenType::Str => Some(self.parse_string_literal()),
            TokenType::Benar | TokenType::Salah => Some(self.parse_boolean_literal()),
            TokenType::Bang | TokenType::Minus | TokenType::Bukan => self.parse_prefix_expression(),
            TokenType::Lparen => self.parse_grouped_expression(),
            TokenType::Jika => self.parse_jika_expression(),
            TokenType::Selama => self.parse_selama_expression(),
            TokenType::Untuk => self.parse_untuk_expression(),
            TokenType::Fungsi => self.parse_fungsi_literal(),
            TokenType::Lbracket => self.parse_array_literal(),
            TokenType::Lbrace => self.parse_hash_literal(),
            TokenType::Masukkan => self.parse_masukkan_expression(),
            TokenType::Coba => self.parse_coba_expression(),
            TokenType::Kosong => Some(Expression::Null),
            _ => {
                self.no_prefix_parse_error(&self.cur_token.type_.clone());
                None
            }
        }
    }

    fn parse_identifier(&self) -> Expression {
        Expression::Pengenal(Pengenal {
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

    fn parse_float_literal(&mut self) -> Option<Expression> {
        match self.cur_token.literal.parse::<f64>() {
            Ok(val) => Some(Expression::FloatLiteral(val)),
            Err(_) => {
                let msg = format!(
                    "Kesalahan: tidak bisa mengurai \"{}\" sebagai angka desimal",
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

    fn parse_prefix_expression(&mut self) -> Option<Expression> {
        let operator = self.cur_token.literal.clone();
        self.next_token();
        let right = self.parse_expression(Precedence::Prefix)?;

        Some(Expression::Awalan(AwalanExpression {
            operator,
            right: Box::new(right),
        }))
    }

    /// Parsing `(...)` — bisa jadi ekspresi grup ATAU fungsi panah.
    ///
    /// Strategi: setelah membuka `(`, jika isi nya berupa daftar
    /// pengenal (identifiers) dan diikuti `)` lalu `=>`, maka ini
    /// adalah fungsi panah. Jika tidak, ini ekspresi grup biasa.
    fn parse_grouped_expression(&mut self) -> Option<Expression> {
        // Kasus khusus: `() => ...` (tanpa parameter)
        if self.peek_token_is(&TokenType::Rparen) {
            self.next_token(); // konsumsi `)`
            if self.peek_token_is(&TokenType::Arrow) {
                self.next_token(); // konsumsi `=>`
                return self.parse_fungsi_panah_badan(vec![]);
            }
            // `()` tanpa `=>` → error, kurung kosong bukan ekspresi valid
            self.errors
                .push("Kesalahan: ekspresi kosong di dalam kurung".to_string());
            return None;
        }

        // Coba deteksi apakah ini daftar parameter untuk fungsi panah.
        // Jika token pertama setelah `(` adalah Ident, dan diikuti oleh
        // `,` atau `)`, ada kemungkinan ini fungsi panah.
        if self.peek_token_is(&TokenType::Ident) {
            // Simpan posisi untuk backtrack jika ternyata bukan arrow
            let saved_pos = self.lexer_snapshot();

            // Coba parse sebagai daftar parameter
            if let Some(params) = self.try_parse_arrow_params() {
                // Berhasil parse params, cek apakah diikuti `=>`
                if self.peek_token_is(&TokenType::Arrow) {
                    self.next_token(); // konsumsi `=>`
                    return self.parse_fungsi_panah_badan(params);
                }
            }

            // Bukan arrow function — restore posisi dan parse sebagai grouped expression
            self.restore_snapshot(saved_pos);
        }

        // Parse sebagai ekspresi grup biasa: `(ekspresi)`
        self.next_token();
        let expr = self.parse_expression(Precedence::Lowest)?;

        if !self.expect_peek(&TokenType::Rparen) {
            return None;
        }

        Some(expr)
    }

    /// Mencoba parsing daftar parameter arrow function: `(x)`, `(x, y, z)`
    /// Mengembalikan None jika format tidak cocok (bukan daftar identifier).
    fn try_parse_arrow_params(&mut self) -> Option<Vec<Pengenal>> {
        let mut params: Vec<Pengenal> = vec![];

        self.next_token(); // lewati `(`  → sekarang di Ident pertama
        if !self.cur_token_is(&TokenType::Ident) {
            return None;
        }

        params.push(Pengenal {
            value: self.cur_token.literal.clone(),
        });

        while self.peek_token_is(&TokenType::Comma) {
            self.next_token(); // konsumsi `,`
            self.next_token(); // maju ke Ident berikutnya
            if !self.cur_token_is(&TokenType::Ident) {
                return None;
            }
            params.push(Pengenal {
                value: self.cur_token.literal.clone(),
            });
        }

        if !self.peek_token_is(&TokenType::Rparen) {
            return None;
        }
        self.next_token(); // konsumsi `)`

        Some(params)
    }

    /// Parsing badan fungsi panah setelah `=>`.
    ///
    /// Mendukung dua bentuk:
    /// - `(x) => { ... }` → blok pernyataan eksplisit
    /// - `(x) => ekspresi` → pengembalian implisit (badan satu ekspresi)
    fn parse_fungsi_panah_badan(&mut self, params: Vec<Pengenal>) -> Option<Expression> {
        self.next_token(); // maju ke token setelah `=>`

        let body = if self.cur_token_is(&TokenType::Lbrace) {
            // Bentuk blok eksplisit: `(x) => { ... }`
            self.parse_blok_pernyataan()
        } else {
            // Bentuk ekspresi tunggal: `(x) => x * 2`
            // Bungkus sebagai satu pernyataan ekspresi di dalam blok
            let expr = self.parse_expression(Precedence::Lowest)?;
            BlokPernyataan {
                statements: vec![Statement::Ekspresi(EkspresiStatement { expression: expr })],
            }
        };

        Some(Expression::FungsiPanah(FungsiPanahLiteral {
            parameters: params,
            body,
        }))
    }

    /// Parsing `coba { ... } tangkap (err) { ... }`
    fn parse_coba_expression(&mut self) -> Option<Expression> {
        // Harapkan blok `{` setelah `coba`
        if !self.expect_peek(&TokenType::Lbrace) {
            return None;
        }

        let body = self.parse_blok_pernyataan();

        // Harapkan keyword `tangkap`
        if !self.expect_peek(&TokenType::Tangkap) {
            return None;
        }

        // Harapkan `(` setelah `tangkap`
        if !self.expect_peek(&TokenType::Lparen) {
            return None;
        }

        // Harapkan nama variabel error
        if !self.expect_peek(&TokenType::Ident) {
            return None;
        }

        let error_ident = Pengenal {
            value: self.cur_token.literal.clone(),
        };

        // Harapkan `)`
        if !self.expect_peek(&TokenType::Rparen) {
            return None;
        }

        // Harapkan blok handler `{`
        if !self.expect_peek(&TokenType::Lbrace) {
            return None;
        }

        let handler = self.parse_blok_pernyataan();

        Some(Expression::Coba(CobaExpression {
            body,
            error_ident,
            handler,
        }))
    }

    // ── Parsing sisipan (infix) ─────────────────────────

    fn parse_infix(&mut self, left: Expression) -> Option<Expression> {
        match self.cur_token.type_ {
            TokenType::Lparen => self.parse_panggilan_expression(left),
            TokenType::Lbracket => self.parse_indeks_expression(left),
            TokenType::Dot => self.parse_titik_expression(left),
            // Operator penugasan: = += -= *= /=
            TokenType::Assign
            | TokenType::PlusEq
            | TokenType::MinusEq
            | TokenType::MulEq
            | TokenType::DivEq => self.parse_penugasan_expression(left),
            _ => self.parse_infix_expression(left),
        }
    }

    fn parse_infix_expression(&mut self, left: Expression) -> Option<Expression> {
        let operator = self.cur_token.literal.clone();
        let precedence = self.cur_precedence();

        self.next_token();

        let right = self.parse_expression(precedence)?;

        Some(Expression::Sisipan(SisipanExpression {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }))
    }

    /// Parsing penugasan: `x = 5`, `arr[0] = 5`, `obj.kunci = 10`
    fn parse_penugasan_expression(&mut self, left: Expression) -> Option<Expression> {
        let operator = self.cur_token.literal.clone();

        self.next_token();
        let value = self.parse_expression(Precedence::Lowest)?;

        Some(Expression::Penugasan(PenugasanExpression {
            left: Box::new(left),
            operator,
            value: Box::new(value),
        }))
    }

    /// Parsing akses titik: `obj.kunci`
    fn parse_titik_expression(&mut self, left: Expression) -> Option<Expression> {
        if !self.expect_peek(&TokenType::Ident) {
            return None;
        }

        let key = self.cur_token.literal.clone();

        Some(Expression::Titik(TitikExpression {
            left: Box::new(left),
            key,
        }))
    }

    // ── Parsing ekspresi kompleks ───────────────────────

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

        let consequence = self.parse_blok_pernyataan();

        let alternative = if self.peek_token_is(&TokenType::Lainnya) {
            self.next_token();

            if !self.expect_peek(&TokenType::Lbrace) {
                return None;
            }

            Some(self.parse_blok_pernyataan())
        } else {
            None
        };

        Some(Expression::Jika(JikaExpression {
            condition: Box::new(condition),
            consequence,
            alternative,
        }))
    }

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

        let body = self.parse_blok_pernyataan();

        Some(Expression::Selama(SelamaExpression {
            condition: Box::new(condition),
            body,
        }))
    }

    /// Parsing perulangan untuk (gaya C):
    /// `untuk (<init>; <kondisi>; <pembaruan>) { <badan> }`
    fn parse_untuk_expression(&mut self) -> Option<Expression> {
        if !self.expect_peek(&TokenType::Lparen) {
            return None;
        }

        // Parsing pernyataan inisialisasi
        self.next_token();
        let init = self.parse_statement()?;

        // Setelah init, parse_statement sudah mengonsumsi titik koma
        // Sekarang parsing kondisi
        self.next_token();
        let condition = self.parse_expression(Precedence::Lowest)?;

        // Harapkan titik koma setelah kondisi
        if !self.expect_peek(&TokenType::Semicolon) {
            return None;
        }

        // Parsing pernyataan pembaruan
        self.next_token();
        let update = self.parse_statement()?;

        // Setelah pembaruan, butuh kurung tutup
        if !self.expect_peek(&TokenType::Rparen) && !self.cur_token_is(&TokenType::Rparen) {
            return None;
        }

        if !self.expect_peek(&TokenType::Lbrace) {
            return None;
        }

        let body = self.parse_blok_pernyataan();

        Some(Expression::Untuk(UntukExpression {
            init: Box::new(init),
            condition: Box::new(condition),
            update: Box::new(update),
            body,
        }))
    }

    /// Parsing blok pernyataan: `{ ... }`
    fn parse_blok_pernyataan(&mut self) -> BlokPernyataan {
        self.next_token();

        let mut statements = vec![];

        while !self.cur_token_is(&TokenType::Rbrace) && !self.cur_token_is(&TokenType::Eof) {
            if let Some(stmt) = self.parse_statement() {
                statements.push(stmt);
            }
            self.next_token();
        }

        BlokPernyataan { statements }
    }

    fn parse_fungsi_literal(&mut self) -> Option<Expression> {
        if !self.expect_peek(&TokenType::Lparen) {
            return None;
        }

        let parameters = self.parse_parameter_fungsi()?;

        if !self.expect_peek(&TokenType::Lbrace) {
            return None;
        }

        let body = self.parse_blok_pernyataan();

        Some(Expression::FungsiLiteral(FungsiLiteral {
            parameters,
            body,
        }))
    }

    /// Parsing daftar parameter fungsi: `(x, y, z)`
    fn parse_parameter_fungsi(&mut self) -> Option<Vec<Pengenal>> {
        let mut identifiers = vec![];

        if self.peek_token_is(&TokenType::Rparen) {
            self.next_token();
            return Some(identifiers);
        }

        self.next_token();

        identifiers.push(Pengenal {
            value: self.cur_token.literal.clone(),
        });

        while self.peek_token_is(&TokenType::Comma) {
            self.next_token();
            self.next_token();

            identifiers.push(Pengenal {
                value: self.cur_token.literal.clone(),
            });
        }

        if !self.expect_peek(&TokenType::Rparen) {
            return None;
        }

        Some(identifiers)
    }

    fn parse_panggilan_expression(&mut self, function: Expression) -> Option<Expression> {
        let arguments = self.parse_daftar_ekspresi(&TokenType::Rparen)?;

        Some(Expression::Panggilan(PanggilanExpression {
            function: Box::new(function),
            arguments,
        }))
    }

    fn parse_array_literal(&mut self) -> Option<Expression> {
        let elements = self.parse_daftar_ekspresi(&TokenType::Rbracket)?;
        Some(Expression::ArrayLiteral(elements))
    }

    fn parse_indeks_expression(&mut self, left: Expression) -> Option<Expression> {
        self.next_token();

        let index = self.parse_expression(Precedence::Lowest)?;

        if !self.expect_peek(&TokenType::Rbracket) {
            return None;
        }

        Some(Expression::Indeks(IndeksExpression {
            left: Box::new(left),
            index: Box::new(index),
        }))
    }

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

    /// Parsing `masukkan("path")` atau `masukkan(ekspresi)`
    fn parse_masukkan_expression(&mut self) -> Option<Expression> {
        if !self.expect_peek(&TokenType::Lparen) {
            return None;
        }

        self.next_token();
        let path = self.parse_expression(Precedence::Lowest)?;

        if !self.expect_peek(&TokenType::Rparen) {
            return None;
        }

        Some(Expression::Masukkan(MasukkanExpression {
            path: Box::new(path),
        }))
    }

    // ── Fungsi pembantu ─────────────────────────────────

    /// Parsing daftar ekspresi yang dipisahkan koma, diakhiri token tertentu.
    fn parse_daftar_ekspresi(&mut self, end: &TokenType) -> Option<Vec<Expression>> {
        let mut list = vec![];

        if self.peek_token_is(end) {
            self.next_token();
            return Some(list);
        }

        self.next_token();
        list.push(self.parse_expression(Precedence::Lowest)?);

        while self.peek_token_is(&TokenType::Comma) {
            self.next_token();
            self.next_token();
            list.push(self.parse_expression(Precedence::Lowest)?);
        }

        if !self.expect_peek(end) {
            return None;
        }

        Some(list)
    }
}
