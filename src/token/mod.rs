/// Tipe-tipe token yang dikenali oleh bahasa Taji.
///
/// Setiap karakter atau kelompok karakter dalam kode sumber
/// akan dipetakan ke salah satu varian enum ini oleh Lexer.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum TokenType {
    Illegal,
    Eof,

    // ── Identifiers + Literals ──────────────────────────
    Ident,    // nama variabel, nama fungsi, dsb.
    Int,      // angka bulat: 0, 42, 1000
    Float,    // angka desimal: 3.14 (reserved, belum aktif di lexer)
    Str,      // teks literal: "halo dunia"

    // ── Operators ───────────────────────────────────────
    Assign,   // =
    Plus,     // +
    Minus,    // -
    Bang,     // !
    Asterisk, // *
    Slash,    // /
    Modulo,   // %

    Lt,       // <
    Gt,       // >
    LtEq,    // <=
    GtEq,    // >=
    Eq,       // ==
    NotEq,    // !=

    // ── Delimiters ──────────────────────────────────────
    Comma,     // ,
    Semicolon, // ;
    Colon,     // :

    Lparen,   // (
    Rparen,   // )
    Lbrace,   // {
    Rbrace,   // }
    Lbracket, // [
    Rbracket, // ]

    // ── Keywords (Bahasa Indonesia Baku) ────────────────
    Fungsi,     // function
    Misalkan,   // let
    Benar,      // true
    Salah,      // false
    Jika,       // if
    Lainnya,    // else
    Kembalikan, // return
    Selama,     // while
    Dan,        // && (logical and)
    Atau,       // || (logical or)
    Bukan,      // not (logical negation alias)
}

/// Representasi sebuah token hasil analisis leksikal.
///
/// Menyimpan tipe token (`type_`) dan teks asli dari kode
/// sumber (`literal`) agar bisa digunakan oleh Parser.
#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub type_: TokenType,
    pub literal: String,
}

impl Token {
    /// Membuat instance `Token` baru.
    pub fn new(type_: TokenType, literal: String) -> Self {
        Token { type_, literal }
    }

    /// Memeriksa apakah sebuah identifier merupakan kata kunci
    /// bahasa Indonesia baku milik Taji, atau sekadar nama variabel biasa.
    pub fn lookup_ident(ident: &str) -> TokenType {
        match ident {
            "fungsi"     => TokenType::Fungsi,
            "misalkan"   => TokenType::Misalkan,
            "benar"      => TokenType::Benar,
            "salah"      => TokenType::Salah,
            "jika"       => TokenType::Jika,
            "lainnya"    => TokenType::Lainnya,
            "kembalikan" => TokenType::Kembalikan,
            "selama"     => TokenType::Selama,
            "dan"        => TokenType::Dan,
            "atau"       => TokenType::Atau,
            "bukan"      => TokenType::Bukan,
            _            => TokenType::Ident,
        }
    }
}
