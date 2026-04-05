# Taji

<p align="center">
  <img src="https://img.shields.io/badge/bahasa-Rust-orange?style=for-the-badge&logo=rust&logoColor=white" alt="Rust">
  <img src="https://img.shields.io/badge/versi-0.3.0-blue?style=for-the-badge" alt="Version">
  <img src="https://img.shields.io/badge/lisensi-MIT-green?style=for-the-badge" alt="License">
  <img src="https://img.shields.io/badge/tests-92%2B%20passed-brightgreen?style=for-the-badge&logo=checkmarx&logoColor=white" alt="Tests">
  <img src="https://img.shields.io/badge/dependensi-0-purple?style=for-the-badge" alt="Zero Dependencies">
  <img src="https://img.shields.io/badge/sintaks-Bahasa%20Indonesia-red?style=for-the-badge" alt="Bahasa Indonesia">
</p>

**Bahasa pemrograman modern dengan sintaks Bahasa Indonesia Baku, dibangun sepenuhnya menggunakan Rust.**

Taji adalah interpreter bahasa pemrograman yang dirancang untuk menjadi ringkas namun *powerful*—seperti namanya. Proyek ini dibangun dari nol tanpa menggunakan generator parser atau library eksternal.

---

## Fitur

| Fitur | Deskripsi |
| --- | --- |
| **Sintaks Bahasa Indonesia** | Kata kunci: `misalkan`, `fungsi`, `jika`, `lainnya`, `selama`, `untuk`, `kembalikan`, `berhenti`, `lanjut`, `coba`, `tangkap` |
| **Tipe Data Lengkap** | Bilangan bulat, desimal (*float*), teks (*string*), boolean (`benar`/`salah`), daftar (*array*), kamus (*hash map*) |
| **Fungsi & Closure** | Fungsi tradisional dan Arrow Function (`=>`), mendukung closure dan rekursi |
| **Kontrol Alur** | Percabangan `jika`/`lainnya`, perulangan `selama` dan `untuk` |
| **Operator Lengkap** | Aritmatika (`+`, `-`, `*`, `/`, `%`), perbandingan (`==`, `!=`, `<`, `>`, `<=`, `>=`), logika (`dan`, `atau`, `bukan`), assignment (`=`, `+=`, `-=`, `*=`, `/=`) |
| **Fungsi Bawaan** | `cetak()`, `tanya()`, `waktu()`, `teks()`, `angka()`, `panjang()`, `tipe()`, `dorong()`, `pertama()`, `terakhir()`, `sisa()`, `pisah()`, `gabung()`, `baca_berkas()`, `tulis_berkas()` |
| **Sistem Modul** | Import langsung file lain menggunakan `masukkan("file.tj")` |
| **Komentar** | Komentar satu baris dengan `//` |
| **Error Handling** | Penanganan galat menggunakan blok `coba / tangkap` serta pesan galat berbahasa Indonesia. |
| **REPL Interaktif** | Terminal interaktif untuk menguji kode secara langsung |
| **Eksekusi File** | Jalankan file script `.tj` dari command line |

---

## Instalasi

### Prasyarat
- [Rust & Cargo](https://rustup.rs/) (versi 1.70.0 atau lebih baru)

### Build dari Sumber
```bash
git clone https://github.com/hihihehadika/taji.git
cd taji
cargo build --release
```

Binary akan tersedia di `target/release/taji`.

---

## Cara Penggunaan

### Mode REPL (Interaktif)
```bash
cargo run
```
```
  ======================================================
        TAJI - Bahasa Pemrograman Indonesia         
        Versi 0.3.0                                   
        Ketik 'keluar' untuk berhenti.                
  ======================================================

taji >> misalkan x = 5 + 3 * 2;
  → 11
taji >> cetak(x);
11
taji >> keluar
  Sampai jumpa!
```

### Mode File
```bash
cargo run -- contoh/halo.tj
```

---

## Referensi Bahasa

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
// Menggunakan 'selama' (while loop)
misalkan i = 0;
selama (i < 5) {
    cetak(i);
    i += 1;
};

// Menggunakan 'untuk' (for loop gaya C)
misalkan total = 0;
untuk (misalkan j = 1; j <= 5; j += 1) {
    jika (j == 3) { lanjut; }; // Lewati 3
    total += j;
};
```

### Fungsi & Arrow Function
```taji
// Definisi fungsi klasik
misalkan faktorial = fungsi(n) {
    jika (n <= 1) {
        kembalikan 1;
    };
    kembalikan n * faktorial(n - 1);
};

// Arrow Function
misalkan kuadrat = (x) => x * x;
misalkan tambah = (a, b) => a + b;

cetak(kuadrat(5));   // 25
```

### Penanganan Galat (Error Handling)
```taji
misalkan hasil = coba {
    10 / 0;
} tangkap (err) {
    cetak("Terjadi galat: " + err);
    0;
};
```

### Daftar (Array)
```taji
misalkan buah = ["Apel", "Mangga", "Jeruk"];

cetak(buah[0]);            // Apel
cetak(panjang(buah));      // 3

// Manipulasi teks & daftar
misalkan huruf = pisah("a,b,c", ",");
cetak(gabung(huruf, "-")); // "a-b-c"
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

### Sistem Modul (Import)

Kamu bisa memanggil file `.tj` lain dan menggunakan fungsi serta variabelnya dari objek Kamus.

**matematika.tj**
```taji
misalkan PI = 3.14;
misalkan tambah = (a, b) => a + b;
```

**utama.tj**
```taji
misalkan mate = masukkan("matematika.tj");
cetak(mate.PI);
cetak(mate.tambah(10, 5));
```

---

## Fungsi Bawaan

| Fungsi | Deskripsi | Contoh |
| --- | --- | --- |
| `cetak(nilai)` | Mencetak nilai ke layar | `cetak("Halo!")` |
| `tanya(prompt)` | Membaca input dari terminal | `tanya("Siapa namamu? ")` |
| `waktu()` | Waktu sistem (ms) saat ini | `waktu()` → `1712242191` |
| `teks(obj)` | Konversi ke Teks | `teks(42)` → `"42"` |
| `angka(teks)` | Konversi ke Angka | `angka("3.14")` → `3.14` |
| `panjang(obj)` | Panjang teks atau daftar | `panjang("Taji")` → `4` |
| `tipe(obj)` | Nama tipe data objek | `tipe(42)` → `"BILANGAN"` |
| `dorong(arr, val)` | Tambah elemen ke akhir daftar | `dorong([1,2], 3)` → `[1,2,3]` |
| `pertama(arr)` | Elemen pertama daftar | `pertama([1,2,3])` → `1` |
| `terakhir(arr)` | Elemen terakhir daftar | `terakhir([1,2,3])` → `3` |
| `sisa(arr)` | Semua elemen kecuali pertama | `sisa([1,2,3])` → `[2,3]` |
| `pisah(txt, sep)` | Memecah teks ke daftar | `pisah("a,b", ",")` → `["a", "b"]` |
| `gabung(arr, sep)` | Menggabungkan daftar ke teks | `gabung([1,2], "-")` → `"1-2"` |
| `baca_berkas(path)`| Membaca isi file teks | `baca_berkas("data.txt")` |
| `tulis_berkas(path, txt)`| Menulis teks ke file | `tulis_berkas("data.txt", "oke")` |

---

## Arsitektur

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

---

## Menjalankan Test

```bash
cargo test
```

Test suite mencakup **92+ test cases** yang menguji:
- Tokenisasi (lexer)
- Parsing ekspresi & prioritas operator
- Evaluasi aritmatika (Int & Float), boolean, string
- Loop kontrol (`untuk`, `selama`, `berhenti`, `lanjut`)
- Fungsi, arrow function, closure, dan rekursi
- Penanganan galat (`coba` / `tangkap`)
- Array, hash map, dot access, dan indeks
- Sistem import (`masukkan`)
- Fungsi I/O bawaan

---

## Dibangun Dengan

- **[Rust](https://www.rust-lang.org/)** — Bahasa pemrograman sistem yang aman dan cepat
- **Tanpa dependensi eksternal** — Seluruh lexer, parser, dan evaluator ditulis dari nol

---

## Lisensi

Proyek ini dilisensikan di bawah [MIT License](LICENSE).

---

<p align="center">
  Dibuat oleh Dika.
</p>
