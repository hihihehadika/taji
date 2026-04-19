/// Tipe-tipe token yang dikenali oleh bahasa Taji.
///
/// Setiap karakter atau kelompok karakter dalam kode sumber
/// akan dipetakan ke salah satu varian enum ini oleh Lexer.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum TokenType {
    Illegal,
    Eof,

    // ── Identifiers + Literals ──────────────────────────
    Ident, // nama variabel, nama fungsi, dsb.
    Int,   // angka bulat: 0, 42, 1000
    Float, // angka desimal: 3.14
    Str,   // teks literal: "halo dunia"

    // ── Operators ───────────────────────────────────────
    Assign,   // =
    Plus,     // +
    Minus,    // -
    Bang,     // !
    Asterisk, // *
    Slash,    // /
    Modulo,   // %

    // Compound assignment operators
    PlusEq,  // +=
    MinusEq, // -=
    MulEq,   // *=
    DivEq,   // /=

    Lt,    // <
    Gt,    // >
    LtEq,  // <=
    GtEq,  // >=
    Eq,    // ==
    NotEq, // !=
    Arrow, // =>

    // ── Delimiters ──────────────────────────────────────
    Comma,     // ,
    Semicolon, // ;
    Colon,     // :
    Dot,       // .

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
    Untuk,      // for
    Berhenti,   // break
    Lanjut,     // continue
    Masukkan,   // import
    Dan,        // && (logical and)
    Atau,       // || (logical or)
    Bukan,      // not (logical negation alias)
    Coba,       // try
    Tangkap,    // catch
    Lemparkan,  // throw
    Kosong,     // null
}

/// Representasi sebuah token hasil analisis leksikal.
///
/// Setiap token menyimpan metadata posisi (baris dan kolom)
/// untuk mendukung pelaporan galat yang akurat.
#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub type_: TokenType,
    pub literal: String,
    /// Nomor baris (1-indexed) di mana token ini ditemukan.
    pub baris: usize,
    /// Nomor kolom (1-indexed) di mana token ini dimulai.
    pub kolom: usize,
}

impl Token {
    /// Membuat instance `Token` baru dengan metadata posisi.
    pub fn new(type_: TokenType, literal: String, baris: usize, kolom: usize) -> Self {
        Token {
            type_,
            literal,
            baris,
            kolom,
        }
    }

    /// Memeriksa apakah sebuah identifier merupakan kata kunci
    /// bahasa Indonesia baku milik Taji, atau sekadar nama variabel biasa.
    pub fn lookup_ident(ident: &str) -> TokenType {
        match ident {
            "fungsi" => TokenType::Fungsi,
            "misalkan" => TokenType::Misalkan,
            "benar" => TokenType::Benar,
            "salah" => TokenType::Salah,
            "jika" => TokenType::Jika,
            "lainnya" => TokenType::Lainnya,
            "kembalikan" => TokenType::Kembalikan,
            "selama" => TokenType::Selama,
            "untuk" => TokenType::Untuk,
            "berhenti" => TokenType::Berhenti,
            "lanjut" => TokenType::Lanjut,
            "masukkan" => TokenType::Masukkan,
            "dan" => TokenType::Dan,
            "atau" => TokenType::Atau,
            "bukan" => TokenType::Bukan,
            "coba" => TokenType::Coba,
            "tangkap" => TokenType::Tangkap,
            "lemparkan" => TokenType::Lemparkan,
            "kosong" => TokenType::Kosong,
            _ => TokenType::Ident,
        }
    }
}
