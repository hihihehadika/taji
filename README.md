# Taji (v1.0.0)

<p align="center">
  <img src="https://img.shields.io/badge/bahasa-Rust-orange?style=for-the-badge&logo=rust&logoColor=white" alt="Rust">
  <img src="https://img.shields.io/badge/versi-1.0.0-blue?style=for-the-badge" alt="Version">
  <img src="https://img.shields.io/badge/lisensi-MIT-green?style=for-the-badge" alt="License">
  <img src="https://img.shields.io/badge/tests-129%20passed-brightgreen?style=for-the-badge&logo=checkmarx&logoColor=white" alt="Tests">
  <img src="https://img.shields.io/badge/arsitektur-Bytecode%20VM-purple?style=for-the-badge" alt="Bytecode VM">
  <img src="https://img.shields.io/badge/sintaks-Bahasa%20Indonesia-red?style=for-the-badge" alt="Bahasa Indonesia">
</p>

**Bahasa pemrograman modern dengan sintaks Bahasa Indonesia Baku, didukung oleh Bytecode Virtual Machine dan Garbage Collector (Mark-and-Sweep).**

Taji adalah bahasa pemrograman dinamis yang dirancang untuk menjadi ringkas, fungsional, dan berkinerja tinggi. Pada versi 1.0.0, Taji telah sepenuhnya menanggalkan *Tree-Walking Evaluator* yang lambat dan bermigrasi ke **Arsitektur Bytecode Virtual Machine (VM)** khusus, membuatnya sangat cepat dan aman dari kebocoran memori berkat *Garbage Collector* internal yang disesuaikan (*Custom Mark-and-Sweep GC*).

---

## 🔥 Fitur Utama v1.0.0 (The Bytecode Engine)

| Fitur | Spesifikasi Khusus |
| --- | --- |
| **Sintaks Bahasa Indonesia Baku** | `misalkan`, `fungsi`, `jika`, `lainnya`, `selama`, `untuk`, `kembalikan`, `berhenti`, `lanjut`, `coba`, `tangkap`, `lemparkan`. |
| **Bytecode Virtual Machine** | Eksekusi berbasis tumpukan (*stack-based*) satu dimensi tanpa rekursi pohon AST. Performa *looping* dan komputasi melesat tajam! |
| **Garbage Collector Cerdas** | Manajemen memori hibrida (Reference Counting + Mark-and-Sweep) yang mampu mendeteksi dan menghancurkan memori yatim (*Circular References*) secara otomatis! |
| **Pemrograman Fungsional** | Dilengkapi fitur Array manipulation ala JS: `petakan (map)`, `saring (filter)`, `dorong (push)`. |
| **Pustaka Standar Komprehensif** | Jaringan HTTP (`ambil_web`), Manipulasi Teks ekstensif (`potong`, `ganti`, `berisi`), utilitas Waktu native, dan Matematika acak bawaan. |
| **Sistem Tipe Dinamis** | Bilangan Bulat, Desimal (*float*), Teks, Boolean, Daftar (*array*), dan Kamus (*hash map*). |
| **Ekosistem JSON Bawaan** | Ubah Kamus jadi JSON dan sebaliknya (`ke_json` & `dari_json`) secara natif untuk kebutuhan API modern. |
| **Sistem Modul Otomatis** | Bongkar-pasang komponen file kode dengan `masukkan("pustaka.tj")` — kode yang diimpor akan langsung dieksekusi di *Call Frame* terisolasi VM. |

---

## 🚀 Instalasi dan Kompilasi

### Prasyarat
Anda harus menginstal [Rust & Cargo](https://rustup.rs/) (Minimal versi 1.75.0).

### Build Cepat
```bash
git clone https://github.com/hihihehadika/taji.git
cd taji
cargo build --release
```
Program `taji` yang sudah di-kompilasi (*binary-executable*) akan diletakkan di dalam `target/release/taji`.

---

## 💻 Cara Menjalankan Taji

### 1. REPL (Terminal Interaktif)
Cocok untuk sekadar mencoba logika dan sintaks langsung di dalam memori.
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

### 2. Mengeksekusi Berkas (.tj)
Jalankan file *script* `.tj` melalui antarmuka baris perintah (*Command Line*):
```bash
cargo run -- contoh/sintaks_dasar.tj
```

---

## 📖 Referensi Sintaks Dasar

### Deklarasi, Loop, & Pengondisian
```taji
misalkan stok = 100;
untuk (misalkan i = 1; i <= 5; i += 1) {
    jika (i == 3) { lanjut; }; // Lewati putaran ke-3
    stok -= i;
};
cetak(format("Sisa stok hari ini: {}", stok));
```

### Penanganan Galat Modern (VM Unwinding)
Taji mendukung blok `coba` / `tangkap` serta kata kunci `lemparkan`. VM Taji akan secara otomatis me- *restore* tumpukan (stack unwinding) ke kondisi semula jika terjadi kepanikan!
```taji
misalkan transfer = fungsi(saldo, tarik) {
    jika (tarik > saldo) {
        lemparkan format("Uang tak cukup! Saldo hanya {}, diminta {}", saldo, tarik);
    };
    kembalikan saldo - tarik;
};

coba {
    transfer(100, 500); // Memicu galat di tingkat Bytecode
} tangkap(err) {
    cetak("[GALAT SISTEM] " + err);
};
```

### Pemrograman Fungsional
Fungsi kelas utama (*First-Class Functions*) dan fungsi anonim (*Arrow Functions*):
```taji
misalkan angka = [10, 15, 20, 25];
// Saring elemen yang memenuhi kondisi genap
misalkan genap = saring(angka, (x) => x % 2 == 0);
cetak(genap); // [10, 20]
```

### Manipulasi API & JSON
```taji
// Konversi langsung String format JSON ke Objek Taji
misalkan teks_json = "{\"status\": \"OKE\", \"nilai\": [1,2,3]}";
misalkan respon = dari_json(teks_json);
cetak(respon["status"]); // Mencetak: OKE
```

---

## 🛠 Struktur Arsitektur Taji (v1.0)
Arsitektur Taji telah ditulis ulang (Refactor) secara radikal demi mengejar kecepatan komputasi C-Like:
1. `Membaca` ➔ **Lexer** memindai Teks kode menjadi Tanda Baca (Token).
2. `Memahamkan` ➔ **Pratt Parser** menyusun Token menjadi *Abstract Syntax Tree (AST)* murni dan memvalidasi sintaksis.
3. `Mengkompilasi` ➔ **Kompilator (Kompilator)** menerjemahkan seluruh AST menjadi intruksi perakitan biner satu dimensi (*Opcodes/Bytecode*).
4. `Mengeksekusi` ➔ **Virtual Machine (VM)** membaca *Bytecode*, mengelola *Call Frames*, memanipulasi *Stack* (Tumpukan), dan menjalankan *Garbage Collector* secara periodik di belakang layar!

---

## 📅 Riwayat Rilis Khusus
- **v1.0.0 (17 April 2026) "The Bytecode Engine"**: Taji telah bermigrasi 100% ke arsitektur Bytecode VM. Dilengkapi dengan manajemen memori **Mark-and-Sweep GC**, sistem *sandbox instruction limit* untuk Fuzzing, dan perbaikan penanganan kesalahan *stack unwinding*. Taji resmi mencapai status siap-produksi!
- **v0.5.0 (9 April 2026)**: Integrasi pustaka standar Jaringan Web HTTP (`ambil_web`) dan fungsional utilitas ekstensif.
- **v0.4.0 (7 April 2026)**: Integrasi tipe data JSON eksternal dan sistem `coba / tangkap / lemparkan`.
- **v0.2.0 (4 April 2026)**: Penambahan kendali alur (`untuk`, `selama`) dan sistem muat modul (`masukkan`).
- **v0.1.0 (4 April 2026)**: Rilis purwarupa awal dengan *Tree-Walking Evaluator*.

---

## 🛡️ Pengujian Kekuatan (Fuzzing & Memory-Safe)
Infrastruktur Taji telah lolos pengujian *Memory Isolation* tingkat kernel melalui `miri` (Rust Miri test) dan *Fuzz Testing* menggunakan `cargo-fuzz`.  
Selain itu, kompilator Taji selalu tervalidasi melalui **129 pengujian unit (Unit Tests) otomatis** setiap ada fitur baru:
```bash
cargo test
```

<br>
<p align="center">
  Arsitektur Taji didesain tanpa kompromi demi pengalaman bahasa tingkat tinggi.<br>
  <b>Dikembangkan secara ambisius oleh Dika.</b>
</p>
