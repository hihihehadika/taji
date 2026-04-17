# Taji (v1.0.0)

<p align="center">
  <img src="https://img.shields.io/badge/bahasa-Rust-orange?style=for-the-badge&logo=rust&logoColor=white" alt="Rust">
  <img src="https://img.shields.io/badge/versi-1.0.0-blue?style=for-the-badge" alt="Version">
  <img src="https://img.shields.io/badge/lisensi-MIT-green?style=for-the-badge" alt="License">
  <img src="https://img.shields.io/badge/tests-129%20passed-brightgreen?style=for-the-badge&logo=checkmarx&logoColor=white" alt="Tests">
  <img src="https://img.shields.io/badge/arsitektur-Bytecode%20VM-purple?style=for-the-badge" alt="Bytecode VM">
  <img src="https://img.shields.io/badge/sintaks-Bahasa%20Indonesia-red?style=for-the-badge" alt="Bahasa Indonesia">
</p>

**Bahasa pemrograman dengan sintaks Bahasa Indonesia Baku, didukung oleh arsitektur Bytecode Virtual Machine.**

Taji adalah proyek bahasa pemrograman yang dirancang untuk menjadi fungsional dan mudah dipahami. Pada versi 1.0.0, arsitektur Taji telah bermigrasi dari *Tree-Walking Evaluator* menuju **Bytecode Virtual Machine (VM)**. Pembaruan ini mengimplementasikan eksekusi berbasis tumpukan (*stack-based*) dan integrasi *Garbage Collector* dengan algoritma Mark-and-Sweep.

---

## Fitur Utama v1.0.0

| Fitur | Deskripsi |
| --- | --- |
| **Sintaks Bahasa Indonesia** | Kata kunci standar: `misalkan`, `fungsi`, `jika`, `lainnya`, `selama`, `untuk`, `kembalikan`, `berhenti`, `lanjut`, `coba`, `tangkap`, `lemparkan`. |
| **Arsitektur Bytecode VM** | Eksekusi instruksi berbasis tumpukan untuk meminimalisasi overhead rekursi pada AST. |
| **Manajemen Memori** | Menggunakan pendekatan hibrida (*Reference Counting* dan *Mark-and-Sweep GC*) untuk mendeteksi dan menyelesaikan siklus referensi (*circular references*). |
| **Pemrograman Fungsional** | Dukungan manipulasi *Array* (*higher-order functions*): `petakan`, `saring`, `dorong`. |
| **Pustaka Standar** | Menyediakan utilitas dasar seperti akses HTTP (`ambil_web`), manipulasi teks, utilitas waktu operasi sistem, dan pembangkit nilai acak. |
| **Sistem Tipe Dinamis** | Mendukung tipe data: Bilangan Bulat, Desimal (*float*), Teks, Boolean, Daftar (*array*), dan Kamus (*hash map*). |
| **Penanganan Format JSON** | Konversi struktur data internal ke format JSON dan sebaliknya (`ke_json` & `dari_json`) untuk keperluan interaksi data. |
| **Sistem Modul** | Mendukung impor kode dari berkas eksternal melalui fungsi `masukkan("berkas.tj")`. |

---

## Instalasi dan Kompilasi

### Prasyarat
Sistem harus memiliki [Rust & Cargo](https://rustup.rs/) (Minimal versi 1.75.0).

### Kompilasi dari Kode Sumber
```bash
git clone https://github.com/hihihehadika/taji.git
cd taji
cargo build --release
```
Berkas *binary executable* `taji` akan berada di direktori `target/release/taji`.

---

## Panduan Penggunaan

### 1. Mode Interaktif (REPL)
Jalankan tanpa argumen untuk masuk ke dalam sesi interaktif.
```bash
cargo run
```
```text
  ======================================================
        TAJI - Bahasa Pemrograman Indonesia
        Versi 1.0.0 [Bytecode VM]
        Ketik 'keluar' untuk berhenti.
  ======================================================

taji >> misalkan data = [1, 2, 3];
taji >> petakan(data, (x) => x * x)
  → [1, 4, 9]
```

### 2. Eksekusi Berkas (.tj)
Berikan rujukan ke berkas kode sumber Taji sebagai argumen eksekusi.
```bash
cargo run -- contoh/sintaks_dasar.tj
```

---

## Referensi Sintaks Dasar

### Deklarasi, Pengulangan, & Pengondisian
```taji
misalkan stok = 100;
untuk (misalkan i = 1; i <= 5; i += 1) {
    jika (i == 3) { lanjut; }; // Melewati iterasi ke-3
    stok -= i;
};
cetak(format("Sisa stok hari ini: {}", stok));
```

### Penanganan Kesalahan (Error Handling)
Taji mendukung struktur pengamanan `coba` / `tangkap` dan pelemparan kesalahan secara leksikal.
```taji
misalkan transfer = fungsi(saldo, tarik) {
    jika (tarik > saldo) {
        lemparkan format("Saldo tidak mencukupi. Saldo: {}, Penarikan: {}", saldo, tarik);
    };
    kembalikan saldo - tarik;
};

coba {
    transfer(100, 500); 
} tangkap(err) {
    cetak("Peringatan Sistem: " + err);
};
```

### Fungsi Kelas Pertama
Fungsi dapat disimpan dalam variabel atau dilewatkan sebagai argumen.
```taji
misalkan angka = [10, 15, 20, 25];
misalkan genap = saring(angka, (x) => x % 2 == 0);
cetak(genap); // [10, 20]
```

### Serialisasi JSON
```taji
misalkan teks_json = "{\"status\": \"Sukses\", \"kode\": 200}";
misalkan respon = dari_json(teks_json);
cetak(respon["status"]); // Sukses
```

---

## Struktur Arsitektur Taji (v1.0)
Arsitektur bahasa Taji terdiri dari empat tahap utama:
1. **Analisis Leksikal (Lexer):** Memindai teks kode sumber menjadi kumpulan token.
2. **Analisis Sintaksis (Parser):** Membangun *Abstract Syntax Tree (AST)* menggunakan metode Pratt Parser.
3. **Kompilasi (Kompilator):** Menerjemahkan representasi AST menjadi serangkaian *Bytecode* (Opcodes).
4. **Eksekusi (Virtual Machine):** Mengiterasi *Bytecode*, mengelola *Call Frames*, serta mengawasi manajemen memori melalui tumpukan (*stack*) dan *Garbage Collector*.

---

## Riwayat Pembaruan
- **v1.0.0 (17 April 2026)**: Migrasi arsitektur ke Bytecode VM. Implementasi Mark-and-Sweep GC dan mitigasi *infinite-loop sandbox* pada kompilator.
- **v0.5.0 (9 April 2026)**: Penambahan fungsionalitas HTTP bawaan (`ambil_web`) dan modul utilitas teks.
- **v0.4.0 (7 April 2026)**: Integrasi pustaka *parsing* JSON dan sistem eksepsi `coba / tangkap / lemparkan`.
- **v0.2.0 (4 April 2026)**: Dukungan alur perulangan (`untuk`, `selama`) dan resolusi berkas multi-modul (`masukkan`).
- **v0.1.0 (4 April 2026)**: Kerangka dasar (Lexer, Parser) dan *Tree-Walking Evaluator* eksperimental.

---

## Pengujian Unit (Unit Testing)
Proyek ini divalidasi oleh lebih dari **129 pengujian unit** otomatis (`cargo test`) untuk menjaga integritas pada komponen *Lexer*, *Parser*, *Compiler*, dan *VM*. Infrastruktur ini juga telah melalui pengujian dasar memori (Miri) dan batasan masukan (*Fuzzing*).

```bash
cargo test
```

<br>
<p align="center">
  Proyek pemrograman independen.<br>
  <b>Oleh Dika.</b>
</p>
