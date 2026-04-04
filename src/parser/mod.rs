/// Modul Parser untuk bahasa Taji.
///
/// Menggunakan algoritma **Pratt Parser** (Top-Down Operator Precedence).

use crate::ast::*;
use crate::lexer::Lexer;
use crate::token::{Token, TokenType};

// ═══════════════════════════════════════════════════════════
//  Precedence
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
    Dot,         // obj.key
}

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

    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
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
        let msg = format!(
            "Kesalahan: tidak ada aturan parsing prefix untuk {:?}",
            t
        );
        self.errors.push(msg);
    }

    // ═══════════════════════════════════════════════════════
    //  Entry point
    // ═══════════════════════════════════════════════════════

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

    fn parse_statement(&mut self) -> Option<Statement> {
        match self.cur_token.type_ {
            TokenType::Misalkan => self.parse_misalkan_statement(),
            TokenType::Kembalikan => self.parse_kembalikan_statement(),
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

    fn parse_expression(&mut self, precedence: Precedence) -> Option<Expression> {
        let mut left = self.parse_prefix()?;

        while !self.peek_token_is(&TokenType::Semicolon)
            && precedence < self.peek_precedence()
        {
            if !self.has_infix_rule(&self.peek_token.type_.clone()) {
                return Some(left);
            }

            self.next_token();
            left = self.parse_infix(left)?;
        }

        Some(left)
    }

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

    // ── Prefix parsing ─────────────────────────────────

    fn parse_prefix(&mut self) -> Option<Expression> {
        match self.cur_token.type_ {
            TokenType::Ident => Some(self.parse_identifier()),
            TokenType::Int => self.parse_integer_literal(),
            TokenType::Float => self.parse_float_literal(),
            TokenType::Str => Some(self.parse_string_literal()),
            TokenType::Benar | TokenType::Salah => Some(self.parse_boolean_literal()),
            TokenType::Bang | TokenType::Minus | TokenType::Bukan => {
                self.parse_prefix_expression()
            }
            TokenType::Lparen => self.parse_grouped_expression(),
            TokenType::Jika => self.parse_jika_expression(),
            TokenType::Selama => self.parse_selama_expression(),
            TokenType::Untuk => self.parse_untuk_expression(),
            TokenType::Fungsi => self.parse_fungsi_literal(),
            TokenType::Lbracket => self.parse_array_literal(),
            TokenType::Lbrace => self.parse_hash_literal(),
            TokenType::Masukkan => self.parse_masukkan_expression(),
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

        Some(Expression::Prefix(PrefixExpression {
            operator,
            right: Box::new(right),
        }))
    }

    fn parse_grouped_expression(&mut self) -> Option<Expression> {
        self.next_token();
        let expr = self.parse_expression(Precedence::Lowest)?;

        if !self.expect_peek(&TokenType::Rparen) {
            return None;
        }

        Some(expr)
    }

    // ── Infix parsing ──────────────────────────────────

    fn parse_infix(&mut self, left: Expression) -> Option<Expression> {
        match self.cur_token.type_ {
            TokenType::Lparen => self.parse_panggilan_expression(left),
            TokenType::Lbracket => self.parse_index_expression(left),
            TokenType::Dot => self.parse_dot_expression(left),
            // Assignment operators: = += -= *= /=
            TokenType::Assign
            | TokenType::PlusEq
            | TokenType::MinusEq
            | TokenType::MulEq
            | TokenType::DivEq => self.parse_assign_expression(left),
            _ => self.parse_infix_expression(left),
        }
    }

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

    /// Parse assignment: `x = 5`, `x += 3`, `x -= 1`
    fn parse_assign_expression(&mut self, left: Expression) -> Option<Expression> {
        let operator = self.cur_token.literal.clone();

        // Left side must be an identifier
        let name = match left {
            Expression::Identifier(ident) => ident,
            _ => {
                self.errors.push(
                    "Kesalahan: sisi kiri assignment harus berupa nama variabel".to_string(),
                );
                return None;
            }
        };

        self.next_token();
        let value = self.parse_expression(Precedence::Lowest)?;

        Some(Expression::Assign(AssignExpression {
            name,
            operator,
            value: Box::new(value),
        }))
    }

    /// Parse dot access: `obj.kunci`
    fn parse_dot_expression(&mut self, left: Expression) -> Option<Expression> {
        if !self.expect_peek(&TokenType::Ident) {
            return None;
        }

        let key = self.cur_token.literal.clone();

        Some(Expression::DotExpression(DotExpression {
            left: Box::new(left),
            key,
        }))
    }

    // ── Complex expression parsing ─────────────────────

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

    /// Parse C-style for loop:
    /// `untuk (<init>; <condition>; <update>) { <body> }`
    fn parse_untuk_expression(&mut self) -> Option<Expression> {
        if !self.expect_peek(&TokenType::Lparen) {
            return None;
        }

        // Parse init statement
        self.next_token();
        let init = self.parse_statement()?;

        // After init, we should be at semicolon or just past it
        // The parse_statement already consumes the semicolon if present
        // Now we parse the condition
        self.next_token();
        let condition = self.parse_expression(Precedence::Lowest)?;

        // Expect semicolon after condition
        if !self.expect_peek(&TokenType::Semicolon) {
            return None;
        }

        // Parse update statement
        self.next_token();
        let update = self.parse_statement()?;

        // After update, we need Rparen
        // If the statement already consumed something, check for rparen
        if !self.expect_peek(&TokenType::Rparen) {
            // Maybe update didn't have semicolon and we're at rparen already
            if !self.cur_token_is(&TokenType::Rparen) {
                return None;
            }
        }

        if !self.expect_peek(&TokenType::Lbrace) {
            return None;
        }

        let body = self.parse_block_statement();

        Some(Expression::Untuk(UntukExpression {
            init: Box::new(init),
            condition: Box::new(condition),
            update: Box::new(update),
            body,
        }))
    }

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
            self.next_token();
            self.next_token();

            identifiers.push(Identifier {
                value: self.cur_token.literal.clone(),
            });
        }

        if !self.expect_peek(&TokenType::Rparen) {
            return None;
        }

        Some(identifiers)
    }

    fn parse_panggilan_expression(&mut self, function: Expression) -> Option<Expression> {
        let arguments = self.parse_expression_list(&TokenType::Rparen)?;

        Some(Expression::Panggilan(PanggilanExpression {
            function: Box::new(function),
            arguments,
        }))
    }

    fn parse_array_literal(&mut self) -> Option<Expression> {
        let elements = self.parse_expression_list(&TokenType::Rbracket)?;
        Some(Expression::ArrayLiteral(elements))
    }

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

    /// Parse `masukkan("path")` or `masukkan(expr)`
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

    // ── Helper ────────────────────────────────────────

    fn parse_expression_list(&mut self, end: &TokenType) -> Option<Vec<Expression>> {
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
