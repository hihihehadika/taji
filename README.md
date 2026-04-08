# Taji (v0.4.0)

<p align="center">
  <img src="https://img.shields.io/badge/bahasa-Rust-orange?style=for-the-badge&logo=rust&logoColor=white" alt="Rust">
  <img src="https://img.shields.io/badge/versi-0.4.0-blue?style=for-the-badge" alt="Version">
  <img src="https://img.shields.io/badge/lisensi-MIT-green?style=for-the-badge" alt="License">
  <img src="https://img.shields.io/badge/tests-107%20passed-brightgreen?style=for-the-badge&logo=checkmarx&logoColor=white" alt="Tests">
  <img src="https://img.shields.io/badge/dependensi-Minimal-purple?style=for-the-badge" alt="Minimal Dependencies">
  <img src="https://img.shields.io/badge/sintaks-Bahasa%20Indonesia-red?style=for-the-badge" alt="Bahasa Indonesia">
</p>

**Bahasa pemrograman dengan sintaks Bahasa Indonesia Baku, dibangun menggunakan Rust.**

Taji adalah interpreter bahasa pemrograman yang dirancang untuk menjadi ringkas dan fungsional. Proyek ini dibangun dari nol tanpa menggunakan _parser generator_, namun dilengkapi dengan berbagai fitur esensial seperti penanganan galat, sistem modul, dan manipulasi JSON terbuka.

---

## Fitur Utama v0.4.0

| Fitur | Spesifikasi Khusus |
| --- | --- |
| **Sintaks Bahasa Indonesia** | `misalkan`, `fungsi`, `jika`, `lainnya`, `selama`, `untuk`, `kembalikan`, `berhenti`, `lanjut`, `coba`, `tangkap`, `lemparkan`. |
| **Pemrograman Fungsional** | Dilengkapi fitur Array manipulation ala JS: `petakan (map)`, `saring (filter)`, `dorong (push)`. |
| **Sistem Tipe Dinamis** | Angka Bulat, Desimal (*float*), Teks, Boolean, Daftar (*array*), dan Kamus (*hash map* / *dictionary*). |
| **Ekosistem JSON Bawaan** | Ubah Kamus jadi JSON dan sebaliknya (`ke_json` & `dari_json`) secara natif untuk kebutuhan *Web-Service* / API. |
| **Penanganan Galat Modern** | Punya sistem `coba`, `tangkap(err)`, dan pemanggilan `lemparkan format("galat: {}", x);`. |
| **Pemformatan Teks Cerdas** | Rangkai *string* dengan interpolasi aman menggunakan `format("hallo {}", nama)`. |
| **Sistem Modul (*Import*)** | Bongkar-pasang komponen file kode dengan instan: `masukkan("pustaka.tj")`. |

---

## Instalasi dan Kompilasi

### Prasyarat
Kamu harus menginstal [Rust & Cargo](https://rustup.rs/) (Minimal versi 1.70.0).

### Build Cepat
```bash
git clone https://github.com/hihihehadika/taji.git
cd taji
cargo build --release
```
Program `taji` yang sudah di-kompilasi (*binary-executable*) akan diletakkan di dalam `target/release/taji`.

---

## Cara Menjalankan Taji

### 1. REPL (Terminal Interaktif Langsung)
Cocok untuk sekadar mencoba logika dan sintaks di memori.
```bash
cargo run
```
```text
  ==========================================
        TAJI - Bahasa Pemrograman Indonesia         
        Ketik 'keluar' untuk berhenti.                
  ==========================================

taji >> misalkan data = [1, 2, 3];
  → [1, 2, 3]
taji >> petakan(data, (x) => x * x)
  → [1, 4, 9]
```

### 2. Mengeksekusi Berkas (.tj)
Jalankan file script `.tj` melalui *command line*:
```bash
cargo run -- contoh/demo_lengkap.tj
```

---

## Referensi Sintaks Dasar

### Deklarasi, Loop, & IF Biasa
```taji
misalkan stok = 100;
untuk (misalkan i = 1; i <= 5; i += 1) {
    jika (i == 3) { lanjut; }; // Skip putaran 3
    stok -= i;
};
cetak(format("Sisa stok hari ini: {}", stok));
```

### Penanganan Galat (Error Handling)
Taji mendukung blok `coba` / `tangkap` serta kata kunci `lemparkan`:
```taji
misalkan transfer = fungsi(saldo, tarik) {
    jika (tarik > saldo) {
        lemparkan format("Uang tak cukup! Saldo hanya {}, diminta {}", saldo, tarik);
    };
    kembalikan saldo - tarik;
};

coba {
    transfer(100, 500); // Memicu galat
} tangkap(err) {
    cetak("[GALAT SISTEM] " + err);
};
```

### Pemrograman Fungsional & Panah Singkat
```taji
misalkan angka = [10, 15, 20, 25];
// Saring yang memenuhi kondisi (=> true)
misalkan genap = saring(angka, (x) => x % 2 == 0);
cetak(genap); // [10, 20]
```

### Manipulasi JSON
```taji
// Konversi langsung String format JSON ke Objek/Array
misalkan teks_json = "{\"status\": \"OKE\", \"nilai\": [1,2,3]}";
misalkan respon = dari_json(teks_json);

cetak(respon["status"]); // Mencetak: OKE
```

### Sistem Pustaka (Impor Berkas Lain)
**pustaka.tj**
```taji
misalkan PI = 3.14159;
```
**utama.tj**
```taji
misalkan matematika = masukkan("pustaka.tj");
cetak("Nilai Lingkaran: " + matematika.PI);
```

---

## Daftar Pustaka Fungsi Bawaan

| Kategori | Nama Fungsi & Contoh | Tipe Kembalian |
| --- | --- | --- |
| **I / O** | `cetak(nilai)`, `tanya("nama: ")` | `-`, `Teks` |
| **Konversi**| `teks(5) -> "5"`, `angka("3.1) -> 3.1` | `Teks`, `Desimal` |
| **Format** | `format("halo {}", nama)` | `Teks` |
| **Array** | `panjang(A)`, `pertama(A)`, `terakhir(A)`, `dorong(A, 5)` | `Campuran` |
| **String** | `pisah("A B", " ")`, `gabung(A, "-")` | `Array`, `Teks` |
| **Berkas** | `baca_berkas(X)`, `tulis_berkas(X, Y)` | `Teks`, `-` |
| **Logika** | `petakan(val, func)`, `saring(val, func)` | `Array` |
| **JSON** | `dari_json(str)`, `ke_json(obj)` | `Objek`, `Teks` |

---

## Struktur Mesin Evaluasi
Arsitektur interpreter proyek ini didesain secara modular menjadi 3 tahapan inti:
1. `Membaca` ➔ **Lexer** mengubah Teks kode mentah menjadi Token (Tanda Baca).
2. `Memahamkan` ➔ **Parser** (mekanisme Pratt Parser) merangkai token jadi Pohon Abstrak (*AST / Abstract Syntax Tree*).
3. `Mengeksekusi` ➔ **Evaluator** berjalan di atas AST untuk mendikte aksi ke sistem operasi.

---

## Riwayat Rilis Khusus
- **v0.4.0 (7 April 2026)**: Integrasi tipe data JSON eksternal, mekanisme galat `coba / tangkap / lemparkan`, dan fungsional `petakan`, `saring`.
- **v0.3.0 (5 April 2026)**: Penanganan galat awal (primitif), fungsi anonim (*arrow function* `=>`), dan sistem komputasi I/O berkas.
- **v0.2.0 (4 April 2026)**: Penambahan kendali alur (`untuk`, `selama`) dan sistem muat modul antar-berkas (`masukkan`).
- **v0.1.0 (4 April 2026)**: Inisialisasi proyek dan kerangka dasar (*Lexer*, *Pratt Parser*, *Evaluator*) serta REPL interaktif di terminal.

---

## Pengujian Unit (Unit Testing)
Proyek ini dilengkapi dengan pengujian unit otomatis untuk memastikan stabilitas setiap komponen.  
Terdapat lebih dari **107 pengujian (`cargo test`)** yang selalu memvalidasi fungsi mesin pada setiap perubahan fitur.
```bash
cargo test
```

<br>
<p align="center">
  Didesain dan dikembangkan agar relevan dan aplikatif.<br>
  <b>Oleh Dika.</b>
</p>

