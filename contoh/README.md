# Kumpulan Contoh Skrip Taji

Direktori ini berisi kumpulan skrip contoh yang ditulis dalam bahasa pemrograman **Taji**. Skrip-skrip ini disediakan untuk mendemonstrasikan berbagai fitur bahasa, kapabilitas mesin virtual (VM), hingga interaksi I/O dengan sistem.

## Cara Menjalankan Skrip

Anda dapat mengeksekusi skrip apa pun di dalam folder ini menggunakan *compiler* Taji dari direktori utama (root) repositori:

```bash
cargo run -- contoh/<nama_berkas>.tj
```

*Contoh:*
```bash
cargo run -- contoh/halo_dunia.tj
```

## Daftar Referensi Skrip

Berikut adalah deskripsi singkat untuk masing-masing skrip yang tersedia:

### 1. Dasar-Dasar Bahasa
- `halo_dunia.tj` : Skrip paling sederhana untuk mencetak teks ke layar. Cocok untuk pengujian awal.
- `sintaks_dasar.tj` : Referensi sintaksis umum bahasa Taji seperti deklarasi variabel, tipe data, dan operasi aritmatika.
- `perulangan.tj` : Contoh kontrol alur program menggunakan blok kondisional dan perulangan (`untuk`, `selama`, `jika`).
- `demo_lengkap.tj` : Skrip komprehensif yang menggabungkan berbagai fitur sintaks dalam satu alur kerja logis.

### 2. Fungsi Bawaan & Pustaka
- `algoritma.tj` : Implementasi algoritma logika atau matematika dasar menggunakan Taji.
- `pustaka_standar.tj` : Uji coba dan demonstrasi berbagai fungsi bawaan (built-in functions) seperti manipulasi string, array, dan konversi tipe data.

### 3. Interaksi Sistem (I/O & Jaringan)
- `interaktif.tj` : Demonstrasi pembacaan input dari antarmuka baris perintah (CLI).
- `crypto_tracker.tj` : Implementasi interaksi HTTP ke API publik eksternal beserta cara mem-parsing struktur data JSON yang dikembalikan oleh server.
- `project_database.tj` : Demonstrasi manipulasi struktur data Kamus (Hash Map) layaknya simulasi operasi database sederhana, termasuk persistensi operasi baca/tulis data ke sistem berkas lokal.

### 4. Pengujian Sistem Modul (Taji Package Manager)
Kumpulan skrip di bawah ini difokuskan untuk menguji fungsionalitas modul dan pemisahan file eksternal:
- `uji_tpm.tj` : Pengujian impor dan ekspor modul dari dalam sub-direktori.
- `uji_taji_modul.tj` : Pengujian resolusi modul yang diunduh ke lingkungan *sandbox* (`taji_modul/`).
- `uji_galat_modul.tj` : Menguji respons VM saat diminta untuk memuat modul yang tidak tersedia.

---
**Catatan:** Beberapa skrip (seperti *Crypto Tracker*) akan menghasilkan *file* output atau log. *File-file* keluaran sementara tersebut akan disimpan secara otomatis ke dalam direktori `contoh/data/`.
