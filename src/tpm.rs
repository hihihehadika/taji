//! Modul `tpm`: Taji Package Manager.
//!
//! Bertanggung jawab mengunduh modul `.tj` dari URL publik dan
//! menyimpannya ke dalam direktori `taji_modul/` di folder kerja saat ini.
//! Setiap proyek Taji bersifat sandboxed — pustaka yang dipasang hanya
//! berlaku untuk proyek di direktori tersebut.

use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};

/// Nama direktori tempat modul yang dipasang akan disimpan.
pub const DIREKTORI_MODUL: &str = "taji_modul";

/// Mengunduh sebuah modul dari URL yang diberikan dan menyimpannya
/// ke dalam folder `taji_modul/` di direktori kerja saat ini.
///
/// Mengembalikan jalur lengkap berkas yang disimpan jika berhasil.
pub fn pasang_modul(url: &str) -> Result<PathBuf, String> {
    // Pastikan direktori `taji_modul/` tersedia
    let direktori_tujuan = Path::new(DIREKTORI_MODUL);
    if !direktori_tujuan.exists() {
        fs::create_dir_all(direktori_tujuan)
            .map_err(|e| format!("tpm: gagal membuat direktori '{}': {}", DIREKTORI_MODUL, e))?;
    }

    // Ekstrak nama berkas dari ujung URL
    // Contoh: "https://example.com/modul/matematika.tj" → "matematika.tj"
    let nama_berkas = ekstrak_nama_berkas(url)
        .ok_or_else(|| format!("tpm: tidak dapat menentukan nama berkas dari URL: {}", url))?;

    // Validasi ekstensi — hanya berkas `.tj` yang diizinkan
    if !nama_berkas.ends_with(".tj") {
        return Err(format!(
            "tpm: berkas '{}' bukan modul Taji yang valid (.tj)",
            nama_berkas
        ));
    }

    eprintln!("tpm: mengunduh '{}'...", url);

    // Kirim permintaan HTTP GET
    let agen = ureq::Agent::new_with_defaults();
    let mut respons = agen
        .get(url)
        .call()
        .map_err(|e| format!("tpm: gagal mengunduh dari URL: {}", e))?;

    // Baca isi bodi respons sebagai teks
    let mut isi_kode = String::new();
    respons
        .body_mut()
        .as_reader()
        .read_to_string(&mut isi_kode)
        .map_err(|e| format!("tpm: gagal membaca respons unduhan: {}", e))?;

    // Simpan ke direktori tujuan
    let jalur_tujuan = direktori_tujuan.join(&nama_berkas);
    fs::write(&jalur_tujuan, &isi_kode)
        .map_err(|e| format!("tpm: gagal menyimpan modul '{}': {}", nama_berkas, e))?;

    eprintln!(
        "tpm: modul '{}' berhasil dipasang ke '{}'.",
        nama_berkas,
        jalur_tujuan.display()
    );

    Ok(jalur_tujuan)
}

/// Mengekstrak nama berkas dari sebuah URL.
///
/// Mengambil segmen terakhir dari path URL.
/// Contoh: `https://example.com/modul/matematika.tj` → `Some("matematika.tj")`
fn ekstrak_nama_berkas(url: &str) -> Option<String> {
    // Hilangkan parameter query jika ada (contoh: ?versi=1)
    let url_bersih = url.split('?').next().unwrap_or(url);

    url_bersih
        .split('/')
        .filter(|s| !s.is_empty())
        .last()
        .map(|s| s.to_string())
}

/// Menampilkan panduan penggunaan perintah `tpm` ke terminal.
pub fn tampilkan_bantuan_tpm() {
    eprintln!("Taji Package Manager (tpm)");
    eprintln!();
    eprintln!("Penggunaan:");
    eprintln!("  taji pasang <URL>   Unduh dan pasang modul dari URL");
    eprintln!();
    eprintln!("Contoh:");
    eprintln!("  taji pasang https://example.com/modul/matematika.tj");
    eprintln!();
    eprintln!("Modul yang dipasang disimpan di folder '{}/'", DIREKTORI_MODUL);
    eprintln!("dan dapat digunakan via: masukkan(\"matematika\")");
}
