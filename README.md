# Taji (v1.1.1)

<p align="center">
  <img src="https://img.shields.io/badge/bahasa-Rust-orange?style=for-the-badge&logo=rust&logoColor=white" alt="Rust">
  <img src="https://img.shields.io/badge/versi-1.1.1-blue?style=for-the-badge" alt="Version">
  <img src="https://img.shields.io/badge/lisensi-MIT-green?style=for-the-badge" alt="License">
  <img src="https://img.shields.io/badge/tests-44%20passed-brightgreen?style=for-the-badge&logo=checkmarx&logoColor=white" alt="Tests">
  <img src="https://img.shields.io/badge/arsitektur-Bytecode%20VM-purple?style=for-the-badge" alt="Bytecode VM">
  <img src="https://img.shields.io/badge/sintaks-Bahasa%20Indonesia-red?style=for-the-badge" alt="Bahasa Indonesia">
</p>

**Bahasa pemrograman dengan sintaks Bahasa Indonesia Baku, dieksekusi oleh mesin virtual bytecode (TVM) yang ditulis sepenuhnya dalam Rust.**

Taji adalah bahasa pemrograman yang dirancang agar terasa akrab bagi penutur Bahasa Indonesia, tanpa mengorbankan kemampuan atau keandalan. Pada versi 1.1.1, Taji memperkenalkan **Taji Package Manager (TPM)** untuk distribusi modul eksternal, melengkapi arsitektur **100% Bytecode Virtual Machine** yang telah dimurnikan.

---

## Uji Coba Taji di Peramban

Ingin mencoba Taji tanpa instalasi? Gunakan **Web Playground** resmi kami yang interaktif, lengkap dengan dokumentasi dan contoh kode:

**[tajicode.vercel.app](https://tajicode.vercel.app)**

---

## Fitur Utama v1.1.1

| Fitur | Deskripsi |
| --- | --- |
| **Sintaks Bahasa Indonesia** | Kata kunci baku: `misalkan`, `fungsi`, `jika`, `lainnya`, `selama`, `untuk`, `kembalikan`, `berhenti`, `lanjut`, `coba`, `tangkap`, `lemparkan`, `kosong`. |
| **100% Bytecode VM (TVM)** | Mesin virtual tumpukan murni tanpa fallback ke tree-walking evaluator. Pipeline: Lexer -> Parser -> Kompilator -> TVM. |
| **Pelaporan Galat Akurat** | Setiap galat parser menampilkan `[baris X, kolom Y]`. Galat runtime VM menampilkan `[baris X]` dari kode sumber asli. |
| **Operator Logika Kata** | Mendukung `dan`, `atau`, `bukan` sebagai alias natural untuk `&&`, `\|\|`, `!` dengan evaluasi short-circuit. |
| **Komentar Satu & Multi-Baris** | `// komentar` untuk satu baris, `/* komentar */` untuk blok multi-baris. |
| **Manajemen Memori** | Skema hibrida Reference Counting + Mark-and-Sweep GC untuk mendeteksi dan menyelesaikan referensi siklik. |
| **Pemrograman Fungsional** | Fungsi kelas pertama, closure, upvalue, dan fungsi bawaan tingkat tinggi: `petakan`, `saring`, `dorong`. |
| **Sistem Modul** | Resolusi modul otomatis dengan prioritas: Jalur Eksplisit -> Direktori Lokal -> `taji_modul/`. |
| **Taji Package Manager (TPM)** | Kelola modul eksternal dengan `taji pasang <URL>`. Pustaka disimpan secara lokal di folder `taji_modul/`. |
| **Pustaka Standar** | Utilitas HTTP (`ambil_web`), manipulasi teks (`potong`, `format`, `panjang`), JSON (`ke_json`, `dari_json`), waktu, dan pembangkit acak. |

---

## Instalasi dan Kompilasi

### Prasyarat
Sistem harus memiliki [Rust & Cargo](https://rustup.rs/) versi 1.75.0 atau lebih baru.

### Kompilasi dari Kode Sumber
```bash
git clone https://github.com/hihihehadika/taji.git
cd taji
cargo build --release
```
Berkas biner `taji` akan tersedia di `target/release/taji`.

---

## Panduan Penggunaan

### 1. Mode Interaktif (REPL)
```bash
cargo run
```
```
  ======================================================
        TAJI - Bahasa Pemrograman Indonesia
        Versi 1.1.1 [TVM-murni]
        Ketik 'keluar' untuk berhenti.
  ======================================================

taji >> misalkan data = [1, 2, 3];
taji >> petakan(data, (x) => x * x)
  -> [1, 4, 9]
```

### 2. Eksekusi Berkas `.tj`
```bash
cargo run -- contoh/program.tj
```

### 3. Manajemen Paket (TPM)
Pasang modul dari URL publik ke folder `taji_modul/` proyek Anda:
```bash
cargo run -- pasang https://raw.githubusercontent.com/user/repo/main/modul.tj
```

### 4. Menjalankan Pengujian
```bash
cargo test
```

---

## Referensi Sintaks

### Variabel dan Tipe Data
```taji
misalkan bilangan = 42;
misalkan desimal  = 3.14;
misalkan teks     = "Halo, dunia!";
misalkan kondisi  = benar;
misalkan nihil    = kosong;
misalkan daftar   = [1, 2, 3];
misalkan kamus    = {"nama": "Taji", "versi": 1};
```

### Operator Logika
```taji
// Simbol maupun kata keduanya berlaku
jika (x > 0 && y > 0) { cetak("simbol"); };
jika (x > 0 dan y > 0) { cetak("kata");   };

jika (a || b) { cetak("simbol"); };
jika (a atau b) { cetak("kata");   };

jika (!aktif)       { cetak("simbol"); };
jika (bukan aktif)  { cetak("kata");   };
```

### Komentar
```taji
// Ini komentar satu baris

/*
   Ini komentar
   multi-baris
*/
```

### Pengondisian
```taji
misalkan nilai = 75;
jika (nilai >= 80) {
    cetak("Lulus dengan pujian");
} lainnya {
    cetak("Lulus standar");
};
```

### Pengulangan
```taji
// Selama (while)
misalkan i = 0;
selama (i < 5) {
    cetak(i);
    i += 1;
};

// Untuk (for)
untuk (misalkan j = 0; j < 3; j += 1) {
    jika (j == 1) { lanjut; };
    cetak(j);
};
```

### Fungsi dan Closure
```taji
misalkan tambah = fungsi(a, b) {
    kembalikan a + b;
};

// Fungsi anonim (lambda)
misalkan kali_dua = (x) => x * 2;

// Fungsi tingkat tinggi
misalkan angka  = [1, 2, 3, 4, 5];
misalkan genap  = saring(angka, (x) => x % 2 == 0);
misalkan kuadrat = petakan(genap, (x) => x * x);
cetak(kuadrat); // [4, 16]
```

### Penanganan Kesalahan
```taji
misalkan bagi = fungsi(a, b) {
    jika (b == 0) {
        lemparkan "Pembagian dengan nol tidak diizinkan";
    };
    kembalikan a / b;
};

coba {
    cetak(bagi(10, 0));
} tangkap(err) {
    cetak("Galat tertangkap: " + err);
};
```

### Sistem Modul
```taji
// berkas: utilitas.tj
misalkan sapa = fungsi(nama) {
    kembalikan "Halo, " + nama + "!";
};

// berkas: utama.tj
// Resolusi mencari: ./utilitas.tj -> ./taji_modul/utilitas.tj
misalkan utilitas = masukkan("utilitas"); 
cetak(utilitas["sapa"]("Dika"));
```

### JSON
```taji
misalkan data   = {"kode": 200, "status": "OK"};
misalkan teks   = ke_json(data);
misalkan parsed = dari_json(teks);
cetak(parsed["status"]); // OK
```

---

## Struktur Proyek

```
taji/
|- Cargo.toml
|- README.md
|- src/
|   |- main.rs              -- Titik masuk: mode file & REPL
|   |- lib.rs               -- Deklarasi modul publik
|   |- bawaan.rs            -- Fungsi bawaan (built-in functions) & Resolusi Modul
|   |- tpm.rs               -- Taji Package Manager: Mesin pengunduh modul
|   |- token/
|   |   `- mod.rs           -- Definisi Token (type, literal, baris, kolom)
|   |- lexer/
|   |   `- mod.rs           -- Analisis leksikal, pelacakan posisi baris/kolom
|   |- ast/
|   |   `- mod.rs           -- Definisi node Abstract Syntax Tree
|   |- parser/
|   |   `- mod.rs           -- Pratt Parser: AST dari token stream
|   |- compiler/
|   |   |- mod.rs           -- Kompilator AST -> Bytecode, tabel_baris
|   |   |- galat.rs         -- Tipe galat kompilasi
|   |   `- tabel_simbol.rs  -- Resolusi simbol lokal, global, dan upvalue
|   |- code/
|   |   |- definisi.rs      -- Definisi OpCode
|   |   `- mod.rs           -- Encoder/Decoder bytecode
|   |- object/
|   |   `- mod.rs           -- Sistem tipe runtime (Integer, Float, Str, dsb.)
|   |- vm/
|   |   |- mod.rs           -- Taji Virtual Machine (TVM), Mark-and-Sweep GC
|   |   `- galat.rs         -- Tipe galat runtime VM
|   `- repl/
|       `- mod.rs           -- Read-Eval-Print Loop interaktif
`- tests/
    |- lexer_tests.rs       -- Pengujian unit Lexer
    |- parser_tests.rs      -- Pengujian unit Parser
    |- stress_tests.rs      -- Pengujian beban VM (memori, closure, rekursi)
    `- encoder_tests.rs     -- (terintegrasi di src/code/mod.rs)
```

---

## Arsitektur Pipeline Eksekusi

```
Kode Sumber (.tj)
       |
       v
  [ Lexer ]  -- Menghasilkan token dengan metadata baris & kolom
       |
       v
  [ Parser ]  -- Membangun AST, pesan galat menyertakan [baris X, kolom Y]
       |
       v
  [ Kompilator ]  -- AST -> Bytecode + tabel_baris
       |
       v
  [ TVM (Bytecode VM) ]  -- Eksekusi stack-based, galat runtime menyertakan [baris X]
```

---

## Riwayat Pembaruan

- **v1.1.1 (3 Mei 2026)**: Penambahan **Web Playground Taji** berbasis WebAssembly (WASM), memungkinkan eksekusi kode Taji secara interaktif langsung di peramban. Pembaruan sistem pelaporan galat (jejak balik, deteksi salah ketik, pelacakan baris/kolom yang lebih presisi). Perbaikan arsitektur kritis sistem modul: instruksi `masukkan()` direfaktor menjadi **opcode VM native `OpMasukkan`**, memastikan stabilitas rekursi dan merger konstanta. Penambahan Manajer Paket Taji (MPT/TPM) di antarmuka baris perintah (CLI).
- **v1.1.0 (19 April 2026)**: Pemurnian ke 100% TVM. Implementasi pelacakan baris/kolom di Lexer & Token. Pelaporan galat parser (`[baris X, kolom Y]`) dan galat runtime VM (`[baris X]`). Penambahan operator logika kata `dan`, `atau`, `bukan`. Dukungan komentar multi-baris `/* ... */`.
- **v1.0.0 (17 April 2026)**: Migrasi arsitektur ke Bytecode VM. Implementasi Mark-and-Sweep GC dan sandbox instruksi.
- **v0.5.0 (9 April 2026)**: Penambahan HTTP bawaan (`ambil_web`) dan modul utilitas teks.
- **v0.4.0 (7 April 2026)**: Integrasi JSON dan sistem eksepsi `coba / tangkap / lemparkan`.
- **v0.2.0 (4 April 2026)**: Dukungan perulangan (`untuk`, `selama`) dan impor modul (`masukkan`).
- **v0.1.0 (4 April 2026)**: Kerangka dasar Lexer, Parser, dan Tree-Walking Evaluator eksperimental.

---

<br>
<p align="center">
  Bahasa pemrograman Taji<br>
  <b>Oleh Dika</b>
</p>
