# 🗡️ Taji

<p align="center">
  <img src="https://img.shields.io/badge/bahasa-Rust-orange?style=for-the-badge&logo=rust&logoColor=white" alt="Rust">
  <img src="https://img.shields.io/badge/versi-0.1.0-blue?style=for-the-badge" alt="Version">
  <img src="https://img.shields.io/badge/lisensi-MIT-green?style=for-the-badge" alt="License">
  <img src="https://img.shields.io/badge/tests-43%2B%20passed-brightgreen?style=for-the-badge&logo=checkmarx&logoColor=white" alt="Tests">
  <img src="https://img.shields.io/badge/dependensi-0-purple?style=for-the-badge" alt="Zero Dependencies">
  <img src="https://img.shields.io/badge/sintaks-Bahasa%20Indonesia-red?style=for-the-badge" alt="Bahasa Indonesia">
</p>

**Bahasa pemrograman modern dengan sintaks Bahasa Indonesia Baku, dibangun sepenuhnya menggunakan Rust.**

Taji adalah interpreter bahasa pemrograman yang dirancang untuk menjadi ringkas namun *powerful*—seperti namanya. Proyek ini dibangun dari nol tanpa menggunakan generator parser atau library eksternal.

---

## ✨ Fitur

| Fitur | Deskripsi |
| --- | --- |
| 🇮🇩 **Sintaks Bahasa Indonesia** | Kata kunci: `misalkan`, `fungsi`, `jika`, `lainnya`, `selama`, `kembalikan` |
| 🔢 **Tipe Data Lengkap** | Bilangan bulat, teks (*string*), boolean (`benar`/`salah`), daftar (*array*), kamus (*hash map*) |
| ⚡ **Fungsi & Closure** | Fungsi sebagai *first-class citizen*, mendukung closure dan rekursi |
| 🔁 **Kontrol Alur** | Percabangan `jika`/`lainnya`, perulangan `selama` |
| 🧮 **Operator Lengkap** | Aritmatika (`+`, `-`, `*`, `/`, `%`), perbandingan (`==`, `!=`, `<`, `>`, `<=`, `>=`), logika (`dan`, `atau`, `bukan`) |
| 📦 **Fungsi Bawaan** | `cetak()`, `panjang()`, `tipe()`, `dorong()`, `pertama()`, `terakhir()`, `sisa()` |
| 💬 **Komentar** | Komentar satu baris dengan `//` |
| 🛡️ **Error Handling** | Pesan kesalahan dalam Bahasa Indonesia yang jelas dan informatif |
| 🖥️ **REPL Interaktif** | Terminal interaktif untuk menguji kode secara langsung |
| 📄 **Eksekusi File** | Jalankan file script `.tj` dari command line |

---

## 🚀 Instalasi

### Prasyarat
- [Rust & Cargo](https://rustup.rs/) (versi 1.70.0 atau lebih baru)

### Build dari Sumber
```bash
git clone https://github.com/user/taji.git
cd taji
cargo build --release
```

Binary akan tersedia di `target/release/taji`.

---

## 📖 Cara Penggunaan

### Mode REPL (Interaktif)
```bash
cargo run
```
```
  ╔══════════════════════════════════════════════════╗
  ║     🗡️  TAJI — Bahasa Pemrograman Indonesia      ║
  ║     Versi 0.1.0                                   ║
  ║     Ketik 'keluar' untuk berhenti.                ║
  ╚══════════════════════════════════════════════════╝

taji >> misalkan x = 5 + 3 * 2;
  → 11
taji >> cetak(x);
11
taji >> keluar
  Sampai jumpa! 👋
```

### Mode File
```bash
cargo run -- contoh/halo.tj
```

---

## 📝 Referensi Bahasa

### Variabel
```taji
misalkan nama = "Taji";
misalkan umur = 1;
misalkan aktif = benar;
```

### Operasi Matematika
```taji
misalkan hasil = (10 + 5) * 2 - 3;   // 27
misalkan sisa = 17 % 5;               // 2
```

### Kondisional
```taji
misalkan nilai = 85;

jika (nilai >= 90) {
    cetak("Sangat Baik");
} lainnya {
    jika (nilai >= 80) {
        cetak("Baik");
    } lainnya {
        cetak("Cukup");
    };
};
```

### Perulangan
```taji
misalkan i = 0;
selama (i < 5) {
    cetak(i);
    misalkan i = i + 1;
};
```

### Fungsi & Rekursi
```taji
// Definisi fungsi
misalkan faktorial = fungsi(n) {
    jika (n <= 1) {
        kembalikan 1;
    };
    kembalikan n * faktorial(n - 1);
};

cetak(faktorial(10));   // 3628800

// Closure
misalkan pembuat_pengali = fungsi(faktor) {
    fungsi(angka) { angka * faktor; };
};

misalkan kali_dua = pembuat_pengali(2);
cetak(kali_dua(5));     // 10
```

### Daftar (Array)
```taji
misalkan buah = ["Apel", "Mangga", "Jeruk"];

cetak(buah[0]);            // Apel
cetak(panjang(buah));      // 3
cetak(pertama(buah));      // Apel
cetak(terakhir(buah));     // Jeruk

misalkan baru = dorong(buah, "Durian");
cetak(panjang(baru));      // 4
```

### Kamus (Hash Map)
```taji
misalkan profil = {
    "nama": "Dika",
    "umur": 20,
    "bahasa": "Taji"
};

cetak(profil["nama"]);     // Dika
cetak(profil["umur"]);     // 20
```

### Operator Logika
```taji
misalkan a = benar;
misalkan b = salah;

cetak(a dan b);     // salah
cetak(a atau b);    // benar
cetak(bukan a);     // salah
```

---

## 📦 Fungsi Bawaan

| Fungsi | Deskripsi | Contoh |
| --- | --- | --- |
| `cetak(nilai)` | Mencetak nilai ke layar | `cetak("Halo!")` |
| `panjang(obj)` | Panjang teks atau daftar | `panjang("Taji")` → `4` |
| `tipe(obj)` | Nama tipe data objek | `tipe(42)` → `"BILANGAN"` |
| `dorong(arr, val)` | Tambah elemen ke akhir daftar | `dorong([1,2], 3)` → `[1,2,3]` |
| `pertama(arr)` | Elemen pertama daftar | `pertama([1,2,3])` → `1` |
| `terakhir(arr)` | Elemen terakhir daftar | `terakhir([1,2,3])` → `3` |
| `sisa(arr)` | Semua elemen kecuali pertama | `sisa([1,2,3])` → `[2,3]` |

---

## 🏗️ Arsitektur

Taji dibangun dengan arsitektur interpreter klasik menggunakan **Pratt Parser**:

```
Kode Sumber (.tj)
       │
       ▼
   ┌────────┐
   │ Lexer  │  Memecah teks menjadi token
   └────┬───┘
       │
       ▼
   ┌────────┐
   │ Parser │  Menyusun token menjadi AST (Pratt Parser)
   └────┬───┘
       │
       ▼
   ┌───────────┐
   │ Evaluator │  Mengeksekusi AST dan menghasilkan output
   └───────────┘
```

### Struktur Kode
```
taji/
├── Cargo.toml
├── README.md
├── contoh/                  # Contoh script .tj
│   ├── halo.tj
│   └── algoritma.tj
├── src/
│   ├── main.rs              # Entry point (REPL + file runner)
│   ├── lib.rs               # Root modul library
│   ├── token/mod.rs         # Definisi kosakata bahasa
│   ├── lexer/mod.rs         # Tokenizer (pemindai leksikal)
│   ├── ast/mod.rs           # Abstract Syntax Tree
│   ├── parser/mod.rs        # Pratt Parser
│   ├── evaluator/mod.rs     # Mesin eksekusi
│   ├── object/mod.rs        # Sistem tipe data & environment
│   └── repl/mod.rs          # REPL interaktif
└── tests/                   # Integration tests
    ├── lexer_tests.rs
    ├── parser_tests.rs
    └── evaluator_tests.rs
```

---

## 🧪 Menjalankan Test

```bash
cargo test
```

Test suite mencakup **43+ test cases** yang menguji:
- Tokenisasi (lexer)
- Parsing ekspresi & prioritas operator
- Evaluasi aritmatika, boolean, string
- Fungsi, closure, dan rekursi
- Array, hash map, dan indeks
- Error handling

---

## 🛠️ Dibangun Dengan

- **[Rust](https://www.rust-lang.org/)** — Bahasa pemrograman sistem yang aman dan cepat
- **Tanpa dependensi eksternal** — Seluruh lexer, parser, dan evaluator ditulis dari nol

---

## 📄 Lisensi

Proyek ini dilisensikan di bawah [MIT License](LICENSE).

---

<p align="center">
  Dibuat oleh Dika.
</p>
