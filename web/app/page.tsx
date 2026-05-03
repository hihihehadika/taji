"use client";

import { useState, useEffect, useRef, useCallback } from "react";
import Editor from "@monaco-editor/react";
import { useTheme } from "next-themes";
import {
  Play, Moon, Sun, Code2, Download, Upload, Share2,
  Trash2, Copy, Check, Maximize2, Minimize2, BookOpen, ChevronDown, FolderOpen, FilePlus, X, HelpCircle,
} from "lucide-react";
import { Group, Panel, Separator } from "react-resizable-panels";
import init, { jalankan_taji } from "@/public/taji-wasm/taji_lib.js";

// Fungsi atur_antrian_masukan diakses secara dinamis karena
// mungkin belum tersedia di binary WASM lama. Jika belum ada,
// fungsi ini tidak melakukan apa-apa (graceful fallback).
let atur_antrian_masukan: (input: string) => void = () => { };
async function muatFungsiTambahan() {
  try {
    const mod = await import("@/public/taji-wasm/taji_lib.js") as any;
    if (typeof mod.atur_antrian_masukan === "function") {
      atur_antrian_masukan = mod.atur_antrian_masukan;
    }
  } catch { /* Abaikan jika belum tersedia */ }
}

// Contoh kode default
const DEFAULT_CODE = `// Selamat datang di Taji Web Playground!
// Taji berjalan langsung di peramban Anda melalui WebAssembly.

misalkan sapaan = "Halo, Dunia Taji!";
cetak(sapaan);

misalkan faktorial = fungsi(n) {
  jika (n <= 1) {
    kembalikan 1;
  } lainnya {
    kembalikan n * faktorial(n - 1);
  }
};

cetak("Faktorial dari 5 adalah:", faktorial(5));
`;

// Poin 22: Daftar snippet kode contoh siap pakai
const DAFTAR_SNIPPET = [
  {
    nama: "Halo, Dunia",
    kode: `misalkan sapaan = "Halo, Dunia Taji!";
cetak(sapaan);`,
  },
  {
    nama: "Faktorial Rekursif",
    kode: `misalkan faktorial = fungsi(n) {
  jika (n <= 1) { kembalikan 1; }
  kembalikan n * faktorial(n - 1);
};

cetak("5! =", faktorial(5));
cetak("10! =", faktorial(10));`,
  },
  {
    nama: "FizzBuzz",
    kode: `misalkan i = 1;
selama (i <= 20) {
  jika (i % 15 == 0) {
    cetak("FizzBuzz");
  } lainnya jika (i % 3 == 0) {
    cetak("Fizz");
  } lainnya jika (i % 5 == 0) {
    cetak("Buzz");
  } lainnya {
    cetak(i);
  }
  i = i + 1;
}`,
  },
  {
    nama: "Manipulasi Larik",
    kode: `misalkan buah = ["apel", "mangga", "jeruk"];
dorong(buah, "durian");

cetak("Jumlah buah:", panjang(buah));
cetak("Buah pertama:", pertama(buah));
cetak("Buah terakhir:", terakhir(buah));

misalkan gabungan = gabung(buah, ", ");
cetak("Semua buah:", gabungan);`,
  },
  {
    nama: "Kamus (Kunci-Nilai)",
    kode: `misalkan profil = {
  "nama": "Budi Santoso",
  "usia": 28,
  "kota": "Bandung",
};

cetak("Nama:", profil["nama"]);
cetak("Usia:", profil["usia"]);
cetak("Kota:", profil["kota"]);`,
  },
  {
    nama: "Fungsi Tingkat Tinggi",
    kode: `misalkan terapkan = fungsi(f, x) {
  kembalikan f(x);
};

misalkan kuadrat = fungsi(x) { kembalikan x * x; };
misalkan ganda   = fungsi(x) { kembalikan x * 2; };

cetak("Kuadrat 7 :", terapkan(kuadrat, 7));
cetak("Ganda 7   :", terapkan(ganda, 7));`,
  },
  {
    nama: "Penanganan Galat",
    kode: `misalkan hasilBagi = fungsi(a, b) {
  jika (b == 0) {
    lemparkan("Pembagi tidak boleh nol!");
  }
  kembalikan a / b;
};

coba {
  cetak("10 / 2 =", hasilBagi(10, 2));
  cetak("10 / 0 =", hasilBagi(10, 0));
} tangkap (e) {
  cetak("Galat tertangkap:", e);
}`,
  },
  {
    nama: "Barisan Fibonacci",
    kode: `misalkan fibonacci = fungsi(n) {
  misalkan a = 0;
  misalkan b = 1;
  misalkan i = 0;
  selama (i < n) {
    cetak(a);
    misalkan tmp = a + b;
    a = b;
    b = tmp;
    i = i + 1;
  }
};

cetak("10 suku pertama Fibonacci:");
fibonacci(10);`,
  },
];



// Antarmuka untuk entri riwayat eksekusi
interface EntriRiwayat {
  waktu: string;
  hasil: string;
  durasi: string;
}

export default function TajiPlayground() {
  const [code, setCode] = useState(DEFAULT_CODE);
  const [inputTanya, setInputTanya] = useState("");
  const [riwayat, setRiwayat] = useState<EntriRiwayat[]>([]);
  const [pesanAwal, setPesanAwal] = useState("Memuat modul WebAssembly...");
  const [isWasmLoaded, setIsWasmLoaded] = useState(false);
  const [disalin, setDisalin] = useState(false);
  const { theme, resolvedTheme, setTheme } = useTheme();
  const [mounted, setMounted] = useState(false);
  // Poin 12: Ukuran font editor, dapat diubah via tombol zoom
  const [ukuranFont, setUkuranFont] = useState(14);
  // Poin 21: Mode layar penuh
  const [layarPenuh, setLayarPenuh] = useState(false);
  // Poin 22: Visibilitas dropdown Snippet Library
  const [snippetTerbuka, setSnippetTerbuka] = useState(false);
  // Poin 23: Sidebar VFS Explorer
  const [vfsTerbuka, setVfsTerbuka] = useState(false);
  const [vfsBerkas, setVfsBerkas] = useState<string[]>([]);
  const [namaVfsBaru, setNamaVfsBaru] = useState("");
  // Poin 25: Multi-Tab Editor
  const [daftarTab, setDaftarTab] = useState<string[]>(["utama.tj"]);
  const [tabAktif, setTabAktif] = useState<string>("utama.tj");
  const [kodeTab, setKodeTab] = useState<Record<string, string>>({ "utama.tj": DEFAULT_CODE });
  // Poin 26: Dokumentasi Terintegrasi
  const [docTerbuka, setDocTerbuka] = useState(false);

  // Referensi untuk elemen input file tersembunyi (Poin 6: Impor Berkas)
  const fileInputRef = useRef<HTMLInputElement>(null);
  // Referensi untuk auto-scroll panel output ke bawah
  const outputRef = useRef<HTMLDivElement>(null);
  // Referensi untuk instansi editor Monaco (Poin 11: Penanda Baris Galat)
  const editorRef = useRef<any>(null);
  const monacoRef = useRef<any>(null);

  // Mencegah hydration mismatch
  useEffect(() => {
    setMounted(true);
  }, []);

  // Poin 23: Muat daftar berkas VFS dari localStorage saat pertama kali
  useEffect(() => {
    if (typeof window === "undefined") return;
    const kunci = Object.keys(localStorage)
      .filter(k => k.startsWith("vfs:"))
      .map(k => k.slice(4));
    setVfsBerkas(kunci);
  }, []);

  // Muat sesi terakhir dari localStorage (Poin 24: Persistensi antar sesi)
  // Mencakup: kode sumber, ukuran font, dan masukan program
  useEffect(() => {
    if (typeof window === "undefined") return;
    const simpan = localStorage.getItem("taji_sesi");
    if (simpan) {
      try {
        const { kode, font, masukan, dTab, tAktif, kTab } = JSON.parse(simpan);
        if (kTab) setKodeTab(kTab);
        else if (kode) setKodeTab({ "utama.tj": kode });

        if (dTab) setDaftarTab(dTab);
        if (tAktif) setTabAktif(tAktif);

        if (kTab && tAktif) setCode(kTab[tAktif]);
        else if (kode) setCode(kode);

        if (font && typeof font === "number") setUkuranFont(font);
        if (masukan) setInputTanya(masukan);
      } catch {
        // Data lama tidak valid, abaikan
      }
    }
  }, []);

  // Auto-save sesi ke localStorage setiap perubahan (debounce 800ms)
  useEffect(() => {
    if (typeof window === "undefined") return;
    const timer = setTimeout(() => {
      localStorage.setItem("taji_sesi", JSON.stringify({
        kode: code,
        font: ukuranFont,
        masukan: inputTanya,
        dTab: daftarTab,
        tAktif: tabAktif,
        kTab: kodeTab,
      }));
    }, 800);
    return () => clearTimeout(timer);
  }, [code, ukuranFont, inputTanya, daftarTab, tabAktif, kodeTab]);

  // Cek parameter URL ?kode= untuk fitur bagikan kode (Poin 7)
  // Dijalankan setelah muat sesi agar URL override sesi tersimpan
  useEffect(() => {
    if (typeof window === "undefined") return;
    const params = new URLSearchParams(window.location.search);
    const kodeParam = params.get("kode");
    if (kodeParam) {
      try {
        setCode(decodeURIComponent(atob(kodeParam)));
      } catch { /* Parameter tidak valid, abaikan */ }
    }
  }, []);

  // Auto-scroll panel output ke bawah saat ada riwayat baru
  useEffect(() => {
    if (outputRef.current) {
      outputRef.current.scrollTop = outputRef.current.scrollHeight;
    }
  }, [riwayat]);

  // Inisialisasi WebAssembly
  useEffect(() => {
    async function loadWasm() {
      try {
        const cacheBuster = new Date().getTime();
        await init(`/taji-wasm/taji_lib_bg.wasm?v=${cacheBuster}`);
        setIsWasmLoaded(true);
        await muatFungsiTambahan();
        setPesanAwal("Taji WebAssembly siap.\nTekan tombol 'Jalankan' atau Ctrl+Enter untuk mengeksekusi kode!");
      } catch (err) {
        console.error("Gagal memuat WASM:", err);
        setPesanAwal(`Gagal memuat modul WebAssembly: ${err}`);
      }
    }
    loadWasm();
  }, []);

  // Konfigurasi Monaco Editor untuk bahasa Taji
  const handleEditorWillMount = (monaco: any) => {
    monaco.languages.register({ id: "taji" });

    monaco.languages.setMonarchTokensProvider("taji", {
      keywords: [
        "jika", "lainnya", "selama", "untuk", "kembalikan", "berhenti", "lanjut",
        "coba", "tangkap", "lemparkan", "dan", "atau", "bukan", "misalkan", "fungsi"
      ],
      constants: ["benar", "salah", "kosong"],
      builtins: [
        "cetak", "panjang", "tipe", "dorong", "pertama", "terakhir", "sisa",
        "tanya", "waktu", "teks", "angka", "pisah", "gabung",
        "baca_berkas", "tulis_berkas", "format", "dari_json", "ke_json",
        "potong", "ganti", "huruf_besar", "huruf_kecil", "berisi",
        "jeda", "acak", "masukkan", "ambil_web"
      ],
      operators: [
        "+", "-", "*", "/", "%", "==", "!=", "<", ">", "<=", ">=", "=", "=>", "+=", "-=", "*=", "/="
      ],
      symbols: /[=><~?:&|+\-*\/\^%]+/,

      tokenizer: {
        root: [
          // Identifiers and keywords
          [/[a-zA-Z_]\w*/, {
            cases: {
              "@keywords": "keyword",
              "@constants": "constant",
              "@builtins": "type.identifier",
              "@default": "identifier"
            }
          }],

          // Whitespace
          { include: "@whitespace" },

          // Delimiters and operators
          [/[{}()\[\]]/, "@brackets"],
          [/[<>](?!@symbols)/, "@brackets"],
          [/@symbols/, {
            cases: {
              "@operators": "operator",
              "@default": ""
            }
          }],

          // Numbers
          [/\d*\.\d+([eE][\-+]?\d+)?/, "number.float"],
          [/\d+/, "number"],

          // Strings
          [/"([^"\\]|\\.)*$/, "string.invalid"],  // non-terminated string
          [/"/, { token: "string.quote", bracket: "@open", next: "@string" }],
        ],

        string: [
          [/[^\\"]+/, "string"],
          [/\\./, "string.escape"],
          [/"/, { token: "string.quote", bracket: "@close", next: "@pop" }]
        ],

        whitespace: [
          [/[ \t\r\n]+/, "white"],
          [/\/\*/, "comment", "@comment"],
          [/\/\/.*$/, "comment"],
        ],

        comment: [
          [/[^\/*]+/, "comment"],
          [/\*\//, "comment", "@pop"],
          [/[\/*]/, "comment"]
        ],
      }
    });

    // Tema Kustom Taji (mengikuti identitas warna)
    monaco.editor.defineTheme("taji-dark", {
      base: "vs-dark",
      inherit: true,
      rules: [
        { token: "keyword", foreground: "10b981", fontStyle: "bold" },
        { token: "type.identifier", foreground: "34d399" },
        { token: "constant", foreground: "f59e0b" },
        { token: "string", foreground: "fcd34d" },
        { token: "number", foreground: "60a5fa" },
        { token: "comment", foreground: "64748b", fontStyle: "italic" },
      ],
      colors: {
        "editor.background": "#0f172a",
        "editor.lineHighlightBackground": "#1e293b",
        "editorLineNumber.foreground": "#475569",
      }
    });

    monaco.editor.defineTheme("taji-light", {
      base: "vs",
      inherit: true,
      rules: [
        { token: "keyword", foreground: "059669", fontStyle: "bold" },
        { token: "type.identifier", foreground: "047857" },
        { token: "constant", foreground: "d97706" },
        { token: "string", foreground: "b45309" },
        { token: "number", foreground: "2563eb" },
        { token: "comment", foreground: "94a3b8", fontStyle: "italic" },
      ],
      colors: {
        "editor.background": "#ffffff",
        "editor.lineHighlightBackground": "#f1f5f9",
        "editorLineNumber.foreground": "#94a3b8",
      }
    });

    // Poin 9: IntelliSense — Daftar lengkap 25 fungsi bawaan Taji
    const daftarFungsiBawaan = [
      {
        label: "cetak",
        insertText: "cetak(${1:nilai})",
        dokumentasi: "Mencetak nilai ke konsol output.",
      },
      {
        label: "panjang",
        insertText: "panjang(${1:larik_atau_teks})",
        dokumentasi: "Mengembalikan jumlah elemen dalam larik atau panjang teks.",
      },
      {
        label: "tipe",
        insertText: "tipe(${1:nilai})",
        dokumentasi: "Mengembalikan tipe data suatu nilai sebagai teks.",
      },
      {
        label: "dorong",
        insertText: "dorong(${1:larik}, ${2:nilai})",
        dokumentasi: "Menambahkan elemen baru ke akhir larik.",
      },
      {
        label: "pertama",
        insertText: "pertama(${1:larik})",
        dokumentasi: "Mengembalikan elemen pertama dari sebuah larik.",
      },
      {
        label: "terakhir",
        insertText: "terakhir(${1:larik})",
        dokumentasi: "Mengembalikan elemen terakhir dari sebuah larik.",
      },
      {
        label: "sisa",
        insertText: "sisa(${1:larik})",
        dokumentasi: "Mengembalikan larik baru tanpa elemen pertama.",
      },
      {
        label: "tanya",
        insertText: "tanya(${1:\"Pertanyaan:\"})",
        dokumentasi: "Meminta input dari pengguna dan mengembalikan teks jawaban.",
      },
      {
        label: "waktu",
        insertText: "waktu()",
        dokumentasi: "Mengembalikan waktu Unix saat ini dalam milidetik.",
      },
      {
        label: "teks",
        insertText: "teks(${1:nilai})",
        dokumentasi: "Mengonversi nilai ke representasi teks (String).",
      },
      {
        label: "angka",
        insertText: "angka(${1:teks})",
        dokumentasi: "Mengonversi teks ke nilai angka bulat.",
      },
      {
        label: "pisah",
        insertText: "pisah(${1:teks}, ${2:\"pemisah\"})",
        dokumentasi: "Memecah teks menjadi larik berdasarkan karakter pemisah.",
      },
      {
        label: "gabung",
        insertText: "gabung(${1:larik}, ${2:\"pemisah\"})",
        dokumentasi: "Menggabungkan elemen larik menjadi satu teks.",
      },
      {
        label: "baca_berkas",
        insertText: "baca_berkas(${1:\"jalur/berkas.txt\"})",
        dokumentasi: "Membaca isi berkas dan mengembalikannya sebagai teks.",
      },
      {
        label: "tulis_berkas",
        insertText: "tulis_berkas(${1:\"jalur/berkas.txt\"}, ${2:isi})",
        dokumentasi: "Menulis teks ke sebuah berkas di sistem atau VFS.",
      },
      {
        label: "format",
        insertText: "format(${1:\"Template {}\", nilai})",
        dokumentasi: "Memformat teks dengan menggantikan placeholder {} dengan nilai.",
      },
      {
        label: "dari_json",
        insertText: "dari_json(${1:teks_json})",
        dokumentasi: "Mengurai teks JSON menjadi nilai Taji (Kamus/Larik).",
      },
      {
        label: "ke_json",
        insertText: "ke_json(${1:nilai})",
        dokumentasi: "Mengubah nilai Taji menjadi representasi teks JSON.",
      },
      {
        label: "potong",
        insertText: "potong(${1:teks}, ${2:mulai}, ${3:akhir})",
        dokumentasi: "Mengambil sebagian teks dari indeks mulai hingga akhir.",
      },
      {
        label: "ganti",
        insertText: "ganti(${1:teks}, ${2:\"lama\"}, ${3:\"baru\"})",
        dokumentasi: "Mengganti semua kemunculan substring lama dengan yang baru.",
      },
      {
        label: "huruf_besar",
        insertText: "huruf_besar(${1:teks})",
        dokumentasi: "Mengubah seluruh karakter teks menjadi huruf besar.",
      },
      {
        label: "huruf_kecil",
        insertText: "huruf_kecil(${1:teks})",
        dokumentasi: "Mengubah seluruh karakter teks menjadi huruf kecil.",
      },
      {
        label: "berisi",
        insertText: "berisi(${1:teks_atau_larik}, ${2:nilai})",
        dokumentasi: "Memeriksa apakah teks atau larik mengandung nilai tertentu.",
      },
      {
        label: "jeda",
        insertText: "jeda(${1:1000})",
        dokumentasi: "Menghentikan eksekusi selama durasi milidetik yang ditentukan.",
      },
      {
        label: "acak",
        insertText: "acak(${1:min}, ${2:max})",
        dokumentasi: "Menghasilkan angka acak antara min (inklusif) dan max (eksklusif).",
      },
      {
        label: "masukkan",
        insertText: "masukkan(${1:\"nama_modul\"})",
        dokumentasi: "Memuat dan mengeksekusi modul .tj lain, mengembalikan ekspornya.",
      },
      {
        label: "ambil_web",
        insertText: "ambil_web(${1:\"https://\"})",
        dokumentasi: "Mengambil konten dari URL melalui permintaan HTTP GET.",
      },
    ];

    monaco.languages.registerCompletionItemProvider("taji", {
      provideCompletionItems: (model: any, position: any) => {
        const kataSebelumKursor = model.getWordUntilPosition(position);
        const rentang = {
          startLineNumber: position.lineNumber,
          endLineNumber: position.lineNumber,
          startColumn: kataSebelumKursor.startColumn,
          endColumn: kataSebelumKursor.endColumn,
        };
        const saran = daftarFungsiBawaan.map((fn) => ({
          label: fn.label,
          kind: monaco.languages.CompletionItemKind.Function,
          documentation: fn.dokumentasi,
          insertText: fn.insertText,
          insertTextRules:
            monaco.languages.CompletionItemInsertTextRule.InsertAsSnippet,
          range: rentang,
        }));
        return { suggestions: saran };
      },
    });

    // Poin 10: Auto-close brackets dan string untuk bahasa Taji
    monaco.languages.setLanguageConfiguration("taji", {
      autoClosingPairs: [
        { open: "{", close: "}" },
        { open: "[", close: "]" },
        { open: "(", close: ")" },
        { open: '"', close: '"', notIn: ["string", "comment"] },
      ],
      brackets: [
        ["{", "}"],
        ["[", "]"],
        ["(", ")"],
      ],
      surroundingPairs: [
        { open: "{", close: "}" },
        { open: "[", close: "]" },
        { open: "(", close: ")" },
        { open: '"', close: '"' },
      ],
      comments: {
        lineComment: "//",
        blockComment: ["/*", "*/"],
      },
      indentationRules: {
        increaseIndentPattern: /^.*\{[^}]*$/,
        decreaseIndentPattern: /^.*\}/,
      },
    });

    // Poin 13: Formatter kode otomatis berbasis kedalaman indentasi kurung kurawal
    monaco.languages.registerDocumentFormattingEditProvider("taji", {
      provideDocumentFormattingEdits: (model: any) => {
        const teksAsli = model.getValue();
        const daftarBaris = teksAsli.split("\n");
        let kedalaman = 0;
        const SPASI = "  "; // 2 spasi per level indentasi
        const hasilFormat = daftarBaris.map((baris: string) => {
          const dipotong = baris.trim();
          // Kurangi kedalaman dulu jika baris dimulai dengan kurung tutup
          if (dipotong.startsWith("}")) kedalaman = Math.max(0, kedalaman - 1);
          const indentasi = SPASI.repeat(kedalaman);
          // Tambah kedalaman jika baris diakhiri kurung buka
          if (dipotong.endsWith("{")) kedalaman++;
          return dipotong === "" ? "" : indentasi + dipotong;
        });
        return [{
          range: model.getFullModelRange(),
          text: hasilFormat.join("\n"),
        }];
      },
    });
  };

  // Pintasan Ctrl+Enter untuk menjalankan kode (Poin 15)
  // Simpan referensi editor dan monaco untuk penanda baris galat (Poin 11)
  const handleEditorMount = (editor: any, monaco: any) => {
    editorRef.current = editor;
    monacoRef.current = monaco;
    editor.addCommand(
      monaco.KeyMod.CtrlCmd | monaco.KeyCode.Enter,
      () => runCode(code)
    );
  };

  // Eksekusi kode Taji melalui mesin WASM
  const runCode = useCallback((source: string) => {
    if (!isWasmLoaded) return;

    try {
      // Kirim antrian masukan ke WASM sebelum eksekusi
      atur_antrian_masukan(inputTanya);

      // Catat waktu mulai untuk indikator durasi eksekusi (Poin 17)
      const waktuMulai = performance.now();
      const hasil = jalankan_taji(source);
      const waktuSelesai = performance.now();
      const durasi = (waktuSelesai - waktuMulai).toFixed(2);

      // Hapus kode ANSI escape (warna terminal) karena tidak didukung peramban
      const hasilBersih = hasil.replace(/\x1b\[[0-9;]*m/g, "");

      // Poin 11: Hapus semua marker lama, lalu pasang marker baru jika ada galat
      if (editorRef.current && monacoRef.current) {
        const model = editorRef.current.getModel();
        if (model) {
          // Selalu bersihkan marker lama setiap kali dijalankan
          monacoRef.current.editor.setModelMarkers(model, "taji", []);

          // Cari pola [Baris N] di output galat
          const polaBaris = /\[Baris (\d+)\]/g;
          const markers: any[] = [];
          let cocok;
          while ((cocok = polaBaris.exec(hasilBersih)) !== null) {
            const nomorBaris = parseInt(cocok[1], 10);
            markers.push({
              startLineNumber: nomorBaris,
              endLineNumber: nomorBaris,
              startColumn: 1,
              endColumn: model.getLineMaxColumn(nomorBaris),
              message: hasilBersih,
              severity: monacoRef.current.MarkerSeverity.Error,
            });
          }
          if (markers.length > 0) {
            monacoRef.current.editor.setModelMarkers(model, "taji", markers);
          }
        }
      }

      // Catat waktu eksekusi untuk label riwayat
      const sekarang = new Date();
      const labelWaktu = sekarang.toLocaleTimeString("id-ID", {
        hour: "2-digit",
        minute: "2-digit",
        second: "2-digit",
      });

      // Tambahkan ke riwayat eksekusi (Poin 8)
      setRiwayat((prev) => [
        ...prev,
        { waktu: labelWaktu, hasil: hasilBersih, durasi },
      ]);
    } catch (err) {
      const sekarang = new Date();
      const labelWaktu = sekarang.toLocaleTimeString("id-ID", {
        hour: "2-digit",
        minute: "2-digit",
        second: "2-digit",
      });
      setRiwayat((prev) => [
        ...prev,
        {
          waktu: labelWaktu,
          hasil: `GALAT INTERNAL WASM: ${err}`,
          durasi: "0",
        },
      ]);
    }
  }, [isWasmLoaded, inputTanya]);

  // Simpan kode sebagai berkas .tj ke disk pengguna (Poin 5)
  const simpanBerkas = () => {
    const blob = new Blob([code], { type: "text/plain" });
    const url = URL.createObjectURL(blob);
    const a = document.createElement("a");
    a.href = url;
    a.download = "skrip_taji.tj";
    a.click();
    URL.revokeObjectURL(url);
  };

  // Buka berkas .tj dari disk pengguna (Poin 6)
  const imporBerkas = (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    if (!file) return;
    const reader = new FileReader();
    reader.onload = (ev) => {
      const isi = ev.target?.result as string;
      const namaFile = file.name;

      // Sinkronisasikan dengan sistem Multi-Tab
      setKodeTab(prev => ({ ...prev, [namaFile]: isi }));
      if (!daftarTab.includes(namaFile)) {
        setDaftarTab(prev => [...prev, namaFile]);
      }
      setTabAktif(namaFile);
      setCode(isi);
    };
    reader.readAsText(file);
    // Reset value agar file yang sama bisa di-impor ulang
    e.target.value = "";
  };

  // Bagikan kode via URL (Poin 7)
  const bagikanKode = () => {
    const kodeBase64 = btoa(encodeURIComponent(code));
    const urlBaru = `${window.location.origin}${window.location.pathname}?kode=${kodeBase64}`;
    navigator.clipboard.writeText(urlBaru).then(() => {
      setDisalin(true);
      setTimeout(() => setDisalin(false), 2000);
    });
  };

  // Salin seluruh output ke clipboard (Poin 18)
  const salinOutput = () => {
    const semuaOutput = riwayat.map((r) => r.hasil).join("\n---\n");
    navigator.clipboard.writeText(semuaOutput).then(() => {
      setDisalin(true);
      setTimeout(() => setDisalin(false), 2000);
    });
  };

  // Bersihkan seluruh riwayat output (Poin 16)
  const bersihkanOutput = () => {
    setRiwayat([]);
  };

  // Poin 23: Simpan kode aktif ke VFS dengan nama tertentu
  const simpanKeVfs = () => {
    const nama = namaVfsBaru.trim();
    if (!nama) return;
    const kunciLengkap = nama.endsWith(".tj") ? nama : `${nama}.tj`;
    localStorage.setItem(`vfs:${kunciLengkap}`, code);
    setVfsBerkas(prev =>
      prev.includes(kunciLengkap) ? prev : [...prev, kunciLengkap]
    );
    setNamaVfsBaru("");
  };

  // Poin 23: Buka berkas dari VFS ke editor (Poin 25: Integrasi Multi-Tab)
  const bukaDariVfs = (nama: string) => {
    const isi = localStorage.getItem(`vfs:${nama}`);
    if (isi !== null) {
      setKodeTab(prev => ({ ...prev, [nama]: isi }));
      if (!daftarTab.includes(nama)) setDaftarTab(prev => [...prev, nama]);
      setTabAktif(nama);
      setCode(isi);
    }
  };

  // Poin 23: Hapus berkas dari VFS
  const hapusDariVfs = (nama: string) => {
    localStorage.removeItem(`vfs:${nama}`);
    setVfsBerkas(prev => prev.filter(n => n !== nama));
  };

  // Poin 21: Toggle mode layar penuh
  const toggleLayarPenuh = () => {
    if (!document.fullscreenElement) {
      document.documentElement.requestFullscreen().catch(() => { });
    } else {
      document.exitFullscreen().catch(() => { });
    }
  };

  useEffect(() => {
    const handlePerubahanLayarPenuh = () => {
      setLayarPenuh(!!document.fullscreenElement);
    };
    document.addEventListener("fullscreenchange", handlePerubahanLayarPenuh);
    return () => document.removeEventListener("fullscreenchange", handlePerubahanLayarPenuh);
  }, []);

  // Poin 25: Handle perubahan kode di Editor untuk tab aktif
  const handleKodeChange = (value: string | undefined) => {
    const val = value || "";
    setCode(val);
    setKodeTab(prev => ({ ...prev, [tabAktif]: val }));
  };

  // Poin 25: Logika Multi-Tab
  const gantiTab = (nama: string) => {
    setTabAktif(nama);
    setCode(kodeTab[nama] || "");
  };

  const tutupTab = (e: React.MouseEvent, nama: string) => {
    e.stopPropagation();
    const tabBaru = daftarTab.filter(t => t !== nama);
    if (tabBaru.length === 0) {
      tabBaru.push("baru.tj");
      setKodeTab(prev => ({ ...prev, "baru.tj": "" }));
    }
    setDaftarTab(tabBaru);
    if (tabAktif === nama) {
      const tabSebelah = tabBaru[tabBaru.length - 1];
      setTabAktif(tabSebelah);
      setCode(kodeTab[tabSebelah] || "");
    }
  };

  if (!mounted) return null;

  const currentTheme = resolvedTheme === "dark" ? "taji-dark" : "taji-light";

  return (
    <div className="flex flex-col h-screen overflow-hidden text-sm transition-colors duration-300">
      {/* Input file tersembunyi untuk impor berkas (Poin 6) */}
      <input
        type="file"
        accept=".tj"
        ref={fileInputRef}
        className="hidden"
        onChange={imporBerkas}
      />

      {/* Header Panel */}
      <header className="glassmorphism flex items-center justify-between px-6 py-4 shadow-sm z-10 border-b">
        <div className="flex items-center gap-3">
          <div className="bg-emerald-500 p-2 rounded-lg text-white shadow-lg shadow-emerald-500/20">
            <Code2 size={24} />
          </div>
          <div>
            <h1 className="font-bold text-xl tracking-tight flex items-center gap-2">
              Taji <span className="text-emerald-500">Playground</span>
            </h1>
            <p className="text-xs opacity-70">
              Ruang interaktif bahasa Taji
            </p>
          </div>
        </div>

        <div className="flex items-center gap-2">
          {/* Tombol Simpan Berkas (Poin 5) */}
          <button
            onClick={simpanBerkas}
            className="p-2 rounded-md hover:bg-black/5 dark:hover:bg-white/10 transition-colors"
            title="Simpan sebagai .tj"
          >
            <Download size={18} />
          </button>

          {/* Tombol Impor Berkas (Poin 6) */}
          <button
            onClick={() => fileInputRef.current?.click()}
            className="p-2 rounded-md hover:bg-black/5 dark:hover:bg-white/10 transition-colors"
            title="Buka berkas .tj"
          >
            <Upload size={18} />
          </button>

          {/* Tombol Bagikan Kode via URL (Poin 7) */}
          <button
            onClick={bagikanKode}
            className="p-2 rounded-md hover:bg-black/5 dark:hover:bg-white/10 transition-colors"
            title="Salin tautan bagikan"
          >
            {disalin ? <Check size={18} className="text-emerald-500" /> : <Share2 size={18} />}
          </button>

          {/* Tombol Toggle VFS Explorer (Poin 23) */}
          <button
            onClick={() => setVfsTerbuka(b => !b)}
            className={`p-2 rounded-md transition-colors ${vfsTerbuka ? "bg-emerald-500/20 text-emerald-500" : "hover:bg-black/5 dark:hover:bg-white/10"}`}
            title="Penjelajah Berkas VFS"
          >
            <FolderOpen size={18} />
          </button>

          {/* Pemisah vertikal */}
          <div className="w-px h-6 bg-current opacity-20" />

          {/* Dropdown Snippet Library (Poin 22) */}
          <div className="relative">
            <button
              onClick={() => setSnippetTerbuka(b => !b)}
              className={`flex items-center gap-1.5 p-2 rounded-md transition-colors ${snippetTerbuka ? "bg-emerald-500/20 text-emerald-500" : "hover:bg-black/5 dark:hover:bg-white/10"}`}
              title="Contoh kode"
            >
              <BookOpen size={18} />
              <ChevronDown size={12} className={`transition-transform ${snippetTerbuka ? "rotate-180" : ""}`} />
            </button>
            {snippetTerbuka && (
              <div
                className="absolute right-0 top-full mt-2 z-50 w-64 rounded-xl shadow-2xl border border-slate-200 dark:border-slate-800 bg-white dark:bg-slate-900 overflow-hidden animate-in fade-in slide-in-from-top-2 duration-200"
              >
                <div className="px-4 py-3 text-[10px] font-bold uppercase tracking-widest text-slate-400 dark:text-slate-500 border-b border-slate-100 dark:border-slate-800 bg-slate-50 dark:bg-slate-900/50">
                  Koleksi Contoh Kode
                </div>
                <div className="max-h-[60vh] overflow-y-auto py-1 custom-scrollbar">
                  {DAFTAR_SNIPPET.map((s, i) => (
                    <button
                      key={i}
                      onClick={() => {
                        setCode(s.kode);
                        setKodeTab(prev => ({ ...prev, [tabAktif]: s.kode }));
                        setSnippetTerbuka(false);
                      }}
                      className="w-full text-left px-4 py-2.5 text-sm text-slate-700 dark:text-slate-300 hover:bg-emerald-50 hover:text-emerald-600 dark:hover:bg-emerald-500/10 dark:hover:text-emerald-400 transition-colors flex items-center justify-between group"
                    >
                      <span>{s.nama}</span>
                      <ChevronDown size={14} className="opacity-0 group-hover:opacity-100 -rotate-90 transition-opacity" />
                    </button>
                  ))}
                </div>
              </div>
            )}
          </div>

          {/* Tombol Dokumentasi Terintegrasi (Poin 26) */}
          <button
            onClick={() => setDocTerbuka(true)}
            className="p-2 rounded-md hover:bg-black/5 dark:hover:bg-white/10 transition-colors"
            title="Dokumentasi Taji"
          >
            <HelpCircle size={18} />
          </button>

          {/* Pemisah vertikal */}
          <div className="w-px h-6 bg-current opacity-20" />

          {/* Tombol Jalankan */}
          <button
            onClick={() => runCode(code)}
            disabled={!isWasmLoaded}
            className="flex items-center gap-2 bg-emerald-500 hover:bg-emerald-600 text-white px-4 py-2 rounded-md transition-colors disabled:opacity-50 font-medium"
          >
            <Play size={16} />
            <span>Jalankan</span>
          </button>

          {/* Tombol Layar Penuh (Poin 21) */}
          <button
            onClick={toggleLayarPenuh}
            className="p-2 rounded-full hover:bg-black/5 dark:hover:bg-white/10 transition-colors"
            title={layarPenuh ? "Keluar layar penuh" : "Layar penuh"}
            aria-label="Toggle Layar Penuh"
          >
            {layarPenuh ? <Minimize2 size={20} /> : <Maximize2 size={20} />}
          </button>

          {/* Tombol Tema */}
          <button
            onClick={() => setTheme(theme === "dark" ? "light" : "dark")}
            className="p-2 rounded-full hover:bg-black/5 dark:hover:bg-white/10 transition-colors"
            aria-label="Toggle Theme"
          >
            {theme === "dark" ? <Sun size={20} /> : <Moon size={20} />}
          </button>
        </div>
      </header>

      {/* Main Content Split — Panel yang bisa digeser (Poin 20) */}
      <main className="flex-1 overflow-hidden flex flex-row">

        {/* Panel Sidebar VFS Explorer (Poin 23) */}
        {vfsTerbuka && (
          <div className="w-64 flex-shrink-0 flex flex-col border-r border-(--border) bg-[var(--panel)] transition-colors duration-300">
            <div className="px-3 py-2 font-mono text-xs uppercase tracking-widest opacity-60 border-b border-(--border) flex items-center justify-between">
              <span>Berkas VFS</span>
              <button onClick={() => setVfsTerbuka(false)} className="opacity-60 hover:opacity-100"><X size={13} /></button>
            </div>

            {/* Form simpan berkas baru ke VFS */}
            <div className="px-3 pt-3 pb-2 border-b border-(--border) flex gap-1">
              <input
                value={namaVfsBaru}
                onChange={e => setNamaVfsBaru(e.target.value)}
                onKeyDown={e => e.key === "Enter" && simpanKeVfs()}
                placeholder="nama.tj"
                className="flex-1 min-w-0 bg-transparent border border-(--border) rounded px-2 py-1 text-xs font-mono focus:outline-none focus:border-emerald-500"
              />
              <button
                onClick={simpanKeVfs}
                title="Simpan ke VFS"
                className="flex-shrink-0 p-1.5 rounded hover:bg-emerald-500/20 text-emerald-500 transition-colors"
              >
                <FilePlus size={14} />
              </button>
            </div>

            {/* Daftar berkas VFS */}
            <div className="flex-1 overflow-y-auto py-1">
              {vfsBerkas.length === 0 ? (
                <p className="px-3 py-4 text-xs opacity-40 text-center">Belum ada berkas tersimpan</p>
              ) : (
                vfsBerkas.map(nama => (
                  <div key={nama} className="flex items-center gap-1 px-2 py-1.5 hover:bg-emerald-500/10 group rounded mx-1">
                    <button
                      onClick={() => bukaDariVfs(nama)}
                      className="flex-1 text-left text-xs font-mono truncate"
                      title={nama}
                    >
                      {nama}
                    </button>
                    <button
                      onClick={() => hapusDariVfs(nama)}
                      className="opacity-0 group-hover:opacity-60 hover:!opacity-100 transition-opacity p-0.5 rounded hover:text-red-500 flex-shrink-0"
                      title="Hapus"
                    >
                      <Trash2 size={11} />
                    </button>
                  </div>
                ))
              )}
            </div>
          </div>
        )}

        {/* Pembungkus Editor dan Output */}
        <div className="flex-1 min-w-0 h-full">
          <Group id="taji-layout" orientation="horizontal" className="h-full">
            {/* Panel Editor */}
            <Panel id="editor" defaultSize={50} minSize={25} className="flex flex-col border-r border-(--border) bg-[var(--panel)] transition-colors duration-300">
              {/* Poin 25: Tab Bar */}
              <div className="flex items-center overflow-x-auto border-b border-(--border) bg-[var(--bg)] custom-scrollbar">
                {daftarTab.map(tab => (
                  <div
                    key={tab}
                    onClick={() => gantiTab(tab)}
                    className={`flex items-center gap-2 px-4 py-2 border-r border-(--border) cursor-pointer font-mono text-xs transition-colors whitespace-nowrap
                    ${tabAktif === tab ? "bg-[var(--panel)] text-emerald-500 border-t-2 border-t-emerald-500" : "opacity-60 hover:opacity-100 hover:bg-[var(--panel)] border-t-2 border-t-transparent"}
                  `}
                  >
                    <span>{tab}</span>
                    {daftarTab.length > 1 && (
                      <button onClick={(e) => tutupTab(e, tab)} className="hover:text-red-500 p-0.5 rounded"><X size={12} /></button>
                    )}
                  </div>
                ))}
              </div>

              {/* Header Control Editor */}
              <div className="px-4 py-1 font-mono text-[10px] uppercase tracking-widest opacity-60 border-b border-(--border) flex items-center justify-between">
                <span>Editor</span>
                <div className="flex items-center gap-2">
                  {!isWasmLoaded && <span className="text-emerald-500 animate-pulse">Memuat Mesin Taji...</span>}
                  {/* Tombol Zoom In/Out ukuran font (Poin 12) */}
                  <button
                    onClick={() => setUkuranFont(f => Math.min(22, f + 2))}
                    className="opacity-60 hover:opacity-100 transition-opacity px-1"
                    title="Perbesar font"
                  >A+</button>
                  <span className="opacity-40">{ukuranFont}px</span>
                  <button
                    onClick={() => setUkuranFont(f => Math.max(10, f - 2))}
                    className="opacity-60 hover:opacity-100 transition-opacity px-1"
                    title="Perkecil font"
                  >A-</button>
                </div>
              </div>
              <div className="flex-1 relative">
                <Editor
                  height="100%"
                  defaultLanguage="taji"
                  language="taji"
                  theme={currentTheme}
                  value={code}
                  beforeMount={handleEditorWillMount}
                  onMount={handleEditorMount}
                  onChange={handleKodeChange}
                  options={{
                    minimap: { enabled: false },
                    fontSize: ukuranFont,
                    fontFamily: "'JetBrains Mono', 'Fira Code', 'Courier New', monospace",
                    lineHeight: 24,
                    padding: { top: 16, bottom: 16 },
                    scrollBeyondLastLine: false,
                    smoothScrolling: true,
                    cursorBlinking: "smooth",
                    cursorSmoothCaretAnimation: "on",
                    formatOnPaste: true,
                  }}
                />
              </div>
            </Panel>

            {/* Pegangan pemisah yang bisa digeser */}
            <Separator className="w-1.5 bg-transparent hover:bg-emerald-500/30 transition-colors cursor-col-resize" />

            {/* Panel Output & Input */}
            <Panel id="output" defaultSize={50} minSize={20} className="flex flex-col bg-[var(--bg)] transition-colors duration-300">
              {/* Header Panel Output dengan tombol aksi */}
              <div className="px-4 py-2 font-mono text-xs uppercase tracking-widest opacity-60 border-b border-(--border) bg-[var(--panel)] transition-colors duration-300 flex items-center justify-between">
                <span>Keluaran Virtual (Standar Output)</span>
                <div className="flex items-center gap-1">
                  {/* Tombol Salin Output (Poin 18) */}
                  <button
                    onClick={salinOutput}
                    className="p-1 rounded hover:bg-black/10 dark:hover:bg-white/10 transition-colors"
                    title="Salin semua output"
                  >
                    <Copy size={14} />
                  </button>
                  {/* Tombol Bersihkan Output (Poin 16) */}
                  <button
                    onClick={bersihkanOutput}
                    className="p-1 rounded hover:bg-black/10 dark:hover:bg-white/10 transition-colors"
                    title="Bersihkan output"
                  >
                    <Trash2 size={14} />
                  </button>
                </div>
              </div>

              {/* Panel Output — Riwayat Eksekusi (Poin 8) */}
              <div
                ref={outputRef}
                className="flex-1 p-6 overflow-auto font-mono text-[13px] leading-relaxed whitespace-pre-wrap"
              >
                {riwayat.length === 0 ? (
                  <span className="opacity-50">{pesanAwal}</span>
                ) : (
                  riwayat.map((entri, idx) => (
                    <div key={idx} className="mb-4">
                      <div className="text-xs opacity-40 mb-1">
                        [{entri.waktu}] Dieksekusi dalam {entri.durasi}ms
                      </div>
                      {entri.hasil.split("\n").map((baris, i) => (
                        <div
                          key={i}
                          className={
                            baris.startsWith("GALAT")
                              ? "text-red-500 font-bold bg-red-500/10 px-1 rounded"
                              : ""
                          }
                        >
                          {baris}
                        </div>
                      ))}
                      {idx < riwayat.length - 1 && (
                        <hr className="border-current opacity-10 mt-4" />
                      )}
                    </div>
                  ))
                )}
              </div>

              {/* Panel Antrian Masukan untuk tanya() (Poin 4) */}
              <div className="border-t border-(--border) bg-[var(--panel)] transition-colors duration-300">
                <div className="px-4 py-1 font-mono text-xs uppercase tracking-widest opacity-40">
                  Masukan Program (satu input per baris)
                </div>
                <textarea
                  value={inputTanya}
                  onChange={(e) => setInputTanya(e.target.value)}
                  placeholder="Ketik input di sini, satu per baris..."
                  className="w-full px-4 py-2 bg-transparent resize-none font-mono text-[13px] leading-relaxed focus:outline-none placeholder:opacity-30"
                  rows={3}
                  spellCheck={false}
                />
              </div>
            </Panel>
          </Group>
        </div>
      </main>

      {/* Modal Dokumentasi Terintegrasi (Poin 26) */}
      {docTerbuka && (
        <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/60 backdrop-blur-md p-4 sm:p-6 transition-all">
          <div className="bg-white dark:bg-slate-900 border border-slate-200 dark:border-slate-800 rounded-2xl shadow-2xl w-full max-w-5xl max-h-[90vh] flex flex-col overflow-hidden animate-in fade-in zoom-in-95 duration-200">
            {/* Header */}
            <div className="px-6 py-4 border-b border-slate-100 dark:border-slate-800 flex items-center justify-between bg-slate-50/50 dark:bg-slate-900/50">
              <div className="flex items-center gap-3">
                <div className="p-2 bg-emerald-100 dark:bg-emerald-500/20 text-emerald-600 dark:text-emerald-400 rounded-lg">
                  <BookOpen size={20} />
                </div>
                <div>
                  <h2 className="font-bold text-lg text-slate-800 dark:text-slate-100">Buku Panduan Taji</h2>
                  <p className="text-xs text-slate-500 dark:text-slate-400">Dokumentasi resmi bahasa pemrograman Taji</p>
                </div>
              </div>
              <button
                onClick={() => setDocTerbuka(false)}
                className="p-2 hover:bg-slate-200 dark:hover:bg-slate-800 text-slate-500 rounded-full transition-colors"
              >
                <X size={20} />
              </button>
            </div>

            {/* Body */}
            <div className="flex flex-1 overflow-hidden">
              {/* Sidebar Nav */}
              <div className="w-56 flex-shrink-0 border-r border-slate-100 dark:border-slate-800 bg-slate-50 dark:bg-slate-900/50 overflow-y-auto p-4 hidden md:block custom-scrollbar">
                <h3 className="text-[10px] font-bold text-slate-400 dark:text-slate-500 uppercase tracking-widest mb-3 px-2">Daftar Isi</h3>
                <nav className="space-y-1">
                  {['Pendahuluan', 'Variabel', 'Tipe Data', 'Struktur Kontrol', 'Fungsi', 'Penanganan Galat', 'Operator', 'Fungsi Bawaan', 'Manipulasi Koleksi', 'Sistem & Web'].map((item) => (
                    <a key={item} href={`#doc-${item.replace(/ /g, '-')}`} className="block px-3 py-2 text-sm font-medium text-slate-600 dark:text-slate-300 hover:bg-emerald-50 dark:hover:bg-emerald-500/10 hover:text-emerald-600 dark:hover:text-emerald-400 rounded-md transition-colors">
                      {item}
                    </a>
                  ))}
                </nav>
              </div>

              {/* Content */}
              <div className="flex-1 overflow-y-auto p-6 md:p-8 space-y-12 bg-white dark:bg-slate-900 custom-scrollbar scroll-smooth">

                {/* Section 1: Pendahuluan */}
                <section id="doc-Pendahuluan">
                  <div className="flex items-center gap-3 mb-4">
                    <span className="flex items-center justify-center w-7 h-7 rounded-full bg-emerald-100 dark:bg-emerald-500/20 text-emerald-600 dark:text-emerald-400 font-bold text-sm">1</span>
                    <h3 className="text-xl font-bold text-slate-800 dark:text-slate-100">Pendahuluan</h3>
                  </div>
                  <p className="text-slate-600 dark:text-slate-300 text-sm leading-relaxed mb-4">
                    Taji adalah bahasa pemrograman modern berorientasi prosedural yang menggunakan sintaks dan kosakata bahasa Indonesia.
                    Desainnya mengambil inspirasi dari JavaScript dan Rust, menawarkan kemudahan pembacaan sekaligus performa tinggi melalui mesin eksekusi berbasis WebAssembly.
                  </p>
                  <div className="bg-blue-50 dark:bg-blue-900/20 border border-blue-100 dark:border-blue-800/50 rounded-xl p-4 flex gap-3 text-sm text-blue-800 dark:text-blue-200">
                    <HelpCircle size={18} className="flex-shrink-0 mt-0.5" />
                    <p><strong>Penting:</strong> Semua perintah diakhiri dengan titik koma (<code>;</code>). Taji membedakan huruf besar dan huruf kecil (case-sensitive).</p>
                  </div>
                </section>

                {/* Section 2: Variabel */}
                <section id="doc-Variabel">
                  <div className="flex items-center gap-3 mb-4">
                    <span className="flex items-center justify-center w-7 h-7 rounded-full bg-emerald-100 dark:bg-emerald-500/20 text-emerald-600 dark:text-emerald-400 font-bold text-sm">2</span>
                    <h3 className="text-xl font-bold text-slate-800 dark:text-slate-100">Variabel & Konstanta</h3>
                  </div>
                  <p className="text-slate-600 dark:text-slate-300 text-sm mb-4">Gunakan kata kunci <code className="text-emerald-600 dark:text-emerald-400">misalkan</code> untuk variabel yang nilainya bisa diubah, dan <code className="text-emerald-600 dark:text-emerald-400">konstanta</code> untuk nilai mutlak yang tidak bisa diubah setelah dideklarasikan.</p>
                  <div className="bg-slate-50 dark:bg-slate-950 rounded-xl overflow-hidden border border-slate-200 dark:border-slate-800">
                    <div className="px-4 py-2 border-b border-slate-200 dark:border-slate-800 bg-slate-100/50 dark:bg-slate-900 font-mono text-xs text-slate-500 flex items-center gap-2">
                      <div className="flex gap-1.5"><div className="w-2.5 h-2.5 rounded-full bg-red-400"></div><div className="w-2.5 h-2.5 rounded-full bg-amber-400"></div><div className="w-2.5 h-2.5 rounded-full bg-green-400"></div></div>
                      <span className="ml-2">contoh_variabel.tj</span>
                    </div>
                    <pre className="p-4 text-sm font-mono overflow-x-auto text-slate-800 dark:text-slate-200 leading-relaxed">
                      <span className="text-emerald-600 dark:text-emerald-400 font-bold">misalkan</span> nama = <span className="text-amber-600 dark:text-amber-400">"Taji"</span>;<br />
                      nama = <span className="text-amber-600 dark:text-amber-400">"Taji Modern"</span>; <span className="text-slate-400 dark:text-slate-500 italic">// Valid, nilai diubah</span><br /><br />
                      <span className="text-emerald-600 dark:text-emerald-400 font-bold">konstanta</span> PI = <span className="text-blue-600 dark:text-blue-400">3.14</span>;<br />
                      PI = <span className="text-blue-600 dark:text-blue-400">3.14159</span>; <span className="text-red-500 dark:text-red-400 italic">// GALAT: Tidak bisa mengubah konstanta</span>
                    </pre>
                  </div>
                </section>

                {/* Section 3: Tipe Data */}
                <section id="doc-Tipe-Data">
                  <div className="flex items-center gap-3 mb-4">
                    <span className="flex items-center justify-center w-7 h-7 rounded-full bg-emerald-100 dark:bg-emerald-500/20 text-emerald-600 dark:text-emerald-400 font-bold text-sm">3</span>
                    <h3 className="text-xl font-bold text-slate-800 dark:text-slate-100">Tipe Data Inti</h3>
                  </div>
                  <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                    <div className="border border-slate-200 dark:border-slate-800 rounded-xl p-5 bg-slate-50/50 dark:bg-slate-900/50">
                      <h4 className="font-bold text-slate-700 dark:text-slate-200 mb-3 flex items-center gap-2"><Code2 size={16} className="text-emerald-500" /> Primitif</h4>
                      <div className="space-y-3">
                        <div>
                          <code className="text-sm bg-slate-200 dark:bg-slate-800 px-2 py-0.5 rounded text-blue-600 dark:text-blue-400 font-bold">123</code> <span className="text-slate-400 mx-1">|</span> <code className="text-sm bg-slate-200 dark:bg-slate-800 px-2 py-0.5 rounded text-blue-600 dark:text-blue-400 font-bold">3.14</code><br />
                          <span className="text-xs text-slate-500 dark:text-slate-400 mt-1 block">Angka (Bulat & Pecahan)</span>
                        </div>
                        <div>
                          <code className="text-sm bg-slate-200 dark:bg-slate-800 px-2 py-0.5 rounded text-amber-600 dark:text-amber-400 font-bold">"Halo Dunia"</code><br />
                          <span className="text-xs text-slate-500 dark:text-slate-400 mt-1 block">Teks (String)</span>
                        </div>
                        <div>
                          <code className="text-sm bg-slate-200 dark:bg-slate-800 px-2 py-0.5 rounded text-emerald-600 dark:text-emerald-400 font-bold">benar</code> <span className="text-slate-400 mx-1">|</span> <code className="text-sm bg-slate-200 dark:bg-slate-800 px-2 py-0.5 rounded text-red-600 dark:text-red-400 font-bold">salah</code><br />
                          <span className="text-xs text-slate-500 dark:text-slate-400 mt-1 block">Boolean (Logika)</span>
                        </div>
                        <div>
                          <code className="text-sm bg-slate-200 dark:bg-slate-800 px-2 py-0.5 rounded text-slate-600 dark:text-slate-400 font-bold">kosong</code><br />
                          <span className="text-xs text-slate-500 dark:text-slate-400 mt-1 block">Null (Tidak ada nilai)</span>
                        </div>
                      </div>
                    </div>
                    <div className="border border-slate-200 dark:border-slate-800 rounded-xl p-5 bg-slate-50/50 dark:bg-slate-900/50">
                      <h4 className="font-bold text-slate-700 dark:text-slate-200 mb-3 flex items-center gap-2"><Code2 size={16} className="text-blue-500" /> Struktur Kompleks</h4>
                      <div className="space-y-4">
                        <div>
                          <div className="text-sm font-bold text-slate-700 dark:text-slate-300 mb-1">Larik (Array)</div>
                          <p className="text-xs text-slate-500 dark:text-slate-400 mb-2">Kumpulan nilai berurutan.</p>
                          <pre className="text-sm font-mono bg-white dark:bg-slate-950 p-3 rounded-lg border border-slate-200 dark:border-slate-800 text-slate-800 dark:text-slate-200">
                            [<span className="text-amber-600 dark:text-amber-400">"apel"</span>, <span className="text-blue-600 dark:text-blue-400">42</span>, <span className="text-emerald-600 dark:text-emerald-400 font-bold">benar</span>]
                          </pre>
                        </div>
                        <div>
                          <div className="text-sm font-bold text-slate-700 dark:text-slate-300 mb-1">Kamus (Dictionary / Hash)</div>
                          <p className="text-xs text-slate-500 dark:text-slate-400 mb-2">Pasangan Kunci dan Nilai.</p>
                          <pre className="text-sm font-mono bg-white dark:bg-slate-950 p-3 rounded-lg border border-slate-200 dark:border-slate-800 text-slate-800 dark:text-slate-200">
                            {"{"}<br />
                            {"  "}<span className="text-amber-600 dark:text-amber-400">"nama"</span>: <span className="text-amber-600 dark:text-amber-400">"Andi"</span>,<br />
                            {"  "}<span className="text-amber-600 dark:text-amber-400">"umur"</span>: <span className="text-blue-600 dark:text-blue-400">20</span><br />
                            {"}"}
                          </pre>
                        </div>
                      </div>
                    </div>
                  </div>
                </section>

                {/* Section 4: Struktur Kontrol */}
                <section id="doc-Struktur-Kontrol">
                  <div className="flex items-center gap-3 mb-4">
                    <span className="flex items-center justify-center w-7 h-7 rounded-full bg-emerald-100 dark:bg-emerald-500/20 text-emerald-600 dark:text-emerald-400 font-bold text-sm">4</span>
                    <h3 className="text-xl font-bold text-slate-800 dark:text-slate-100">Struktur Kontrol</h3>
                  </div>

                  <div className="space-y-6">
                    <div>
                      <h4 className="font-bold text-slate-700 dark:text-slate-200 mb-2">Percabangan (Jika / Lainnya Jika / Lainnya)</h4>
                      <div className="bg-slate-50 dark:bg-slate-950 rounded-xl overflow-hidden border border-slate-200 dark:border-slate-800">
                        <pre className="p-4 text-sm font-mono overflow-x-auto text-slate-800 dark:text-slate-200 leading-relaxed">
                          <span className="text-emerald-600 dark:text-emerald-400 font-bold">jika</span> (nilai &gt; <span className="text-blue-600 dark:text-blue-400">80</span>) {"{"}<br />
                          {"  "}<span className="text-teal-600 dark:text-teal-400">cetak</span>(<span className="text-amber-600 dark:text-amber-400">"Lulus"</span>);<br />
                          {"}"} <span className="text-emerald-600 dark:text-emerald-400 font-bold">lainnya jika</span> (nilai &gt; <span className="text-blue-600 dark:text-blue-400">50</span>) {"{"}<br />
                          {"  "}<span className="text-teal-600 dark:text-teal-400">cetak</span>(<span className="text-amber-600 dark:text-amber-400">"Remedial"</span>);<br />
                          {"}"} <span className="text-emerald-600 dark:text-emerald-400 font-bold">lainnya</span> {"{"}<br />
                          {"  "}<span className="text-teal-600 dark:text-teal-400">cetak</span>(<span className="text-amber-600 dark:text-amber-400">"Gagal"</span>);<br />
                          {"}"}
                        </pre>
                      </div>
                    </div>

                    <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                      <div>
                        <h4 className="font-bold text-slate-700 dark:text-slate-200 mb-2">Perulangan Kondisional</h4>
                        <div className="bg-slate-50 dark:bg-slate-950 rounded-xl overflow-hidden border border-slate-200 dark:border-slate-800 h-full">
                          <pre className="p-4 text-sm font-mono overflow-x-auto text-slate-800 dark:text-slate-200 leading-relaxed">
                            <span className="text-slate-400 dark:text-slate-500 italic">// Berjalan selama kondisi benar</span><br />
                            <span className="text-emerald-600 dark:text-emerald-400 font-bold">misalkan</span> i = <span className="text-blue-600 dark:text-blue-400">0</span>;<br />
                            <span className="text-emerald-600 dark:text-emerald-400 font-bold">selama</span> (i &lt; <span className="text-blue-600 dark:text-blue-400">5</span>) {"{"}<br />
                            {"  "}<span className="text-teal-600 dark:text-teal-400">cetak</span>(i);<br />
                            {"  "}i = i + <span className="text-blue-600 dark:text-blue-400">1</span>;<br />
                            {"}"}
                          </pre>
                        </div>
                      </div>
                      <div>
                        <h4 className="font-bold text-slate-700 dark:text-slate-200 mb-2">Perulangan Tradisional</h4>
                        <div className="bg-slate-50 dark:bg-slate-950 rounded-xl overflow-hidden border border-slate-200 dark:border-slate-800 h-full">
                          <pre className="p-4 text-sm font-mono overflow-x-auto text-slate-800 dark:text-slate-200 leading-relaxed">
                            <span className="text-slate-400 dark:text-slate-500 italic">// Inisialisasi; Kondisi; Langkah</span><br />
                            <span className="text-emerald-600 dark:text-emerald-400 font-bold">untuk</span> (<span className="text-emerald-600 dark:text-emerald-400 font-bold">misalkan</span> i = <span className="text-blue-600 dark:text-blue-400">0</span>; i &lt; <span className="text-blue-600 dark:text-blue-400">5</span>; i += <span className="text-blue-600 dark:text-blue-400">1</span>) {"{"}<br />
                            {"  "}<span className="text-teal-600 dark:text-teal-400">cetak</span>(i);<br />
                            {"}"}
                          </pre>
                        </div>
                      </div>
                    </div>
                  </div>
                </section>

                {/* Section 5: Fungsi */}
                <section id="doc-Fungsi">
                  <div className="flex items-center gap-3 mb-4">
                    <span className="flex items-center justify-center w-7 h-7 rounded-full bg-emerald-100 dark:bg-emerald-500/20 text-emerald-600 dark:text-emerald-400 font-bold text-sm">5</span>
                    <h3 className="text-xl font-bold text-slate-800 dark:text-slate-100">Fungsi (Sub-rutin)</h3>
                  </div>
                  <p className="text-slate-600 dark:text-slate-300 text-sm mb-4">
                    Fungsi di Taji bersifat <em>First-Class Citizen</em>. Artinya, fungsi dapat disimpan dalam variabel, diteruskan sebagai argumen, atau dikembalikan dari fungsi lain (mendukung pemrograman fungsional).
                  </p>
                  <div className="bg-slate-50 dark:bg-slate-950 rounded-xl overflow-hidden border border-slate-200 dark:border-slate-800">
                    <pre className="p-4 text-sm font-mono overflow-x-auto text-slate-800 dark:text-slate-200 leading-relaxed">
                      <span className="text-emerald-600 dark:text-emerald-400 font-bold">misalkan</span> kuadrat = <span className="text-emerald-600 dark:text-emerald-400 font-bold">fungsi</span>(x) {"{"}<br />
                      {"  "}<span className="text-emerald-600 dark:text-emerald-400 font-bold">kembalikan</span> x * x;<br />
                      {"}"};<br /><br />
                      <span className="text-teal-600 dark:text-teal-400">cetak</span>(<span className="text-amber-600 dark:text-amber-400">"Hasil:"</span>, kuadrat(<span className="text-blue-600 dark:text-blue-400">7</span>)); <span className="text-slate-400 dark:text-slate-500 italic">// Hasil: 49</span>
                    </pre>
                  </div>
                </section>

                {/* Section 6: Penanganan Galat */}
                <section id="doc-Penanganan-Galat">
                  <div className="flex items-center gap-3 mb-4">
                    <span className="flex items-center justify-center w-7 h-7 rounded-full bg-emerald-100 dark:bg-emerald-500/20 text-emerald-600 dark:text-emerald-400 font-bold text-sm">6</span>
                    <h3 className="text-xl font-bold text-slate-800 dark:text-slate-100">Penanganan Galat (Try-Catch)</h3>
                  </div>
                  <p className="text-slate-600 dark:text-slate-300 text-sm mb-4">
                    Gunakan blok <code>coba - tangkap</code> untuk menangkap *Exception* agar aplikasi tidak *crash*. Anda juga bisa memicu *Exception* sendiri dengan perintah <code>lemparkan</code>.
                  </p>
                  <div className="bg-slate-50 dark:bg-slate-950 rounded-xl overflow-hidden border border-slate-200 dark:border-slate-800">
                    <pre className="p-4 text-sm font-mono overflow-x-auto text-slate-800 dark:text-slate-200 leading-relaxed">
                      <span className="text-emerald-600 dark:text-emerald-400 font-bold">coba</span> {"{"}<br />
                      {"  "}<span className="text-slate-400 dark:text-slate-500 italic">// Kode yang mungkin gagal</span><br />
                      {"  "}<span className="text-emerald-600 dark:text-emerald-400 font-bold">lemparkan</span>(<span className="text-amber-600 dark:text-amber-400">"Koneksi database gagal!"</span>);<br />
                      {"}"} <span className="text-emerald-600 dark:text-emerald-400 font-bold">tangkap</span> (err) {"{"}<br />
                      {"  "}<span className="text-teal-600 dark:text-teal-400">cetak</span>(<span className="text-amber-600 dark:text-amber-400">"Error ditangkap dengan aman:"</span>, err);<br />
                      {"}"}
                    </pre>
                  </div>
                </section>

                {/* Section 7: Operator */}
                <section id="doc-Operator">
                  <div className="flex items-center gap-3 mb-4">
                    <span className="flex items-center justify-center w-7 h-7 rounded-full bg-emerald-100 dark:bg-emerald-500/20 text-emerald-600 dark:text-emerald-400 font-bold text-sm">7</span>
                    <h3 className="text-xl font-bold text-slate-800 dark:text-slate-100">Operator Matematika & Logika</h3>
                  </div>
                  <div className="overflow-x-auto">
                    <table className="w-full text-sm text-left border-collapse">
                      <thead>
                        <tr className="bg-slate-50 dark:bg-slate-900 border-b border-slate-200 dark:border-slate-800 text-slate-500">
                          <th className="px-4 py-3 font-semibold w-1/3">Jenis</th>
                          <th className="px-4 py-3 font-semibold w-1/3">Simbol</th>
                          <th className="px-4 py-3 font-semibold w-1/3">Contoh</th>
                        </tr>
                      </thead>
                      <tbody className="divide-y divide-slate-100 dark:divide-slate-800 text-slate-700 dark:text-slate-300">
                        <tr>
                          <td className="px-4 py-3 font-medium">Aritmatika</td>
                          <td className="px-4 py-3"><code className="text-blue-500 font-bold bg-blue-50 dark:bg-blue-900/20 px-2 py-0.5 rounded">+ - * / %</code></td>
                          <td className="px-4 py-3 font-mono text-xs">10 % 3 == 1</td>
                        </tr>
                        <tr>
                          <td className="px-4 py-3 font-medium">Perbandingan</td>
                          <td className="px-4 py-3"><code className="text-emerald-500 font-bold bg-emerald-50 dark:bg-emerald-900/20 px-2 py-0.5 rounded">== != &lt; &gt; &lt;= &gt;=</code></td>
                          <td className="px-4 py-3 font-mono text-xs">5 != 10</td>
                        </tr>
                        <tr>
                          <td className="px-4 py-3 font-medium">Logika (Kata Kunci)</td>
                          <td className="px-4 py-3"><code className="text-purple-500 font-bold bg-purple-50 dark:bg-purple-900/20 px-2 py-0.5 rounded">dan atau bukan</code></td>
                          <td className="px-4 py-3 font-mono text-xs">benar dan salah</td>
                        </tr>
                        <tr>
                          <td className="px-4 py-3 font-medium">Shorthand Assignment</td>
                          <td className="px-4 py-3"><code className="text-amber-500 font-bold bg-amber-50 dark:bg-amber-900/20 px-2 py-0.5 rounded">+= -= *= /=</code></td>
                          <td className="px-4 py-3 font-mono text-xs">x += 5</td>
                        </tr>
                      </tbody>
                    </table>
                  </div>
                </section>

                {/* Section 8: Fungsi Bawaan */}
                <section id="doc-Fungsi-Bawaan">
                  <div className="flex items-center gap-3 mb-4">
                    <span className="flex items-center justify-center w-7 h-7 rounded-full bg-emerald-100 dark:bg-emerald-500/20 text-emerald-600 dark:text-emerald-400 font-bold text-sm">8</span>
                    <h3 className="text-xl font-bold text-slate-800 dark:text-slate-100">Fungsi Bawaan (Standard Library)</h3>
                  </div>
                  <p className="text-slate-600 dark:text-slate-300 text-sm mb-4">Taji menyediakan modul standar untuk I/O dan manipulasi dasar tanpa perlu mengimpor library.</p>
                  <div className="grid grid-cols-1 md:grid-cols-2 gap-3">
                    {[
                      { nama: 'cetak(nilai, ...)', desc: 'Mencetak data ke layar terminal/output.' },
                      { nama: 'tanya("Prompt: ")', desc: 'Membaca input teks dari pengguna.' },
                      { nama: 'tipe(nilai)', desc: 'Mengembalikan teks jenis tipe data ("ANGKA", "TEKS", dll).' },
                      { nama: 'waktu()', desc: 'Mengembalikan Unix Timestamp (milidetik).' },
                      { nama: 'jeda(ms)', desc: 'Menghentikan eksekusi kode sementara (Sleep).' },
                      { nama: 'acak(min, max)', desc: 'Menghasilkan angka bulat acak.' },
                      { nama: 'teks(nilai)', desc: 'Konversi sembarang nilai menjadi Teks.' },
                      { nama: 'angka(teks)', desc: 'Konversi Teks menjadi Angka Bulat.' },
                      { nama: 'format("Umur: {}", 20)', desc: 'Format string dinamis dengan kurung kurawal.' },
                    ].map(f => (
                      <div key={f.nama} className="p-3 bg-slate-50 dark:bg-slate-900 border border-slate-200 dark:border-slate-800 rounded-lg">
                        <code className="text-emerald-600 dark:text-emerald-400 font-bold text-xs">{f.nama}</code>
                        <p className="text-xs text-slate-500 mt-1">{f.desc}</p>
                      </div>
                    ))}
                  </div>
                </section>

                {/* Section 9: Manipulasi Koleksi */}
                <section id="doc-Manipulasi-Koleksi">
                  <div className="flex items-center gap-3 mb-4">
                    <span className="flex items-center justify-center w-7 h-7 rounded-full bg-emerald-100 dark:bg-emerald-500/20 text-emerald-600 dark:text-emerald-400 font-bold text-sm">9</span>
                    <h3 className="text-xl font-bold text-slate-800 dark:text-slate-100">Manipulasi Koleksi & Teks</h3>
                  </div>
                  <div className="bg-slate-50 dark:bg-slate-950 rounded-xl overflow-hidden border border-slate-200 dark:border-slate-800 p-5">
                    <h4 className="font-bold text-slate-700 dark:text-slate-200 mb-3 text-sm">Larik & Kamus</h4>
                    <ul className="space-y-2 text-sm text-slate-600 dark:text-slate-300 font-mono mb-6">
                      <li><span className="text-teal-500">panjang</span>(koleksi) <span className="text-slate-400 font-sans text-xs">→ Jumlah elemen/panjang teks</span></li>
                      <li><span className="text-teal-500">dorong</span>(larik, nilai) <span className="text-slate-400 font-sans text-xs">→ Tambah ke akhir larik</span></li>
                      <li><span className="text-teal-500">pertama</span>(larik) | <span className="text-teal-500">terakhir</span>(larik) <span className="text-slate-400 font-sans text-xs">→ Akses elemen</span></li>
                      <li><span className="text-teal-500">sisa</span>(larik) <span className="text-slate-400 font-sans text-xs">→ Larik tanpa elemen pertama</span></li>
                      <li><span className="text-teal-500">berisi</span>(koleksi, nilai) <span className="text-slate-400 font-sans text-xs">→ Kembalikan benar/salah</span></li>
                    </ul>

                    <h4 className="font-bold text-slate-700 dark:text-slate-200 mb-3 text-sm">Teks (String)</h4>
                    <ul className="space-y-2 text-sm text-slate-600 dark:text-slate-300 font-mono">
                      <li><span className="text-teal-500">pisah</span>("a,b,c", ",") <span className="text-slate-400 font-sans text-xs">→ ["a", "b", "c"]</span></li>
                      <li><span className="text-teal-500">gabung</span>(["x", "y"], "-") <span className="text-slate-400 font-sans text-xs">→ "x-y"</span></li>
                      <li><span className="text-teal-500">potong</span>("Halo", 0, 2) <span className="text-slate-400 font-sans text-xs">→ "Ha"</span></li>
                      <li><span className="text-teal-500">ganti</span>("Buku", "u", "a") <span className="text-slate-400 font-sans text-xs">→ "Baka"</span></li>
                      <li><span className="text-teal-500">huruf_besar</span>("tes") | <span className="text-teal-500">huruf_kecil</span>("TES")</li>
                    </ul>
                  </div>
                </section>

                {/* Section 10: Sistem & Web */}
                <section id="doc-Sistem-&-Web">
                  <div className="flex items-center gap-3 mb-4">
                    <span className="flex items-center justify-center w-7 h-7 rounded-full bg-emerald-100 dark:bg-emerald-500/20 text-emerald-600 dark:text-emerald-400 font-bold text-sm">10</span>
                    <h3 className="text-xl font-bold text-slate-800 dark:text-slate-100">Sistem, Web, Modul & JSON</h3>
                  </div>
                  <p className="text-slate-600 dark:text-slate-300 text-sm mb-4">Fitur level lanjut untuk manajemen berkas, komunikasi HTTP, dan ekspor/impor JSON.</p>
                  <div className="grid grid-cols-1 gap-4">
                    <div className="border border-slate-200 dark:border-slate-800 rounded-xl p-4 bg-slate-50/50 dark:bg-slate-900/50">
                      <div className="space-y-3 font-mono text-sm">
                        <div className="flex flex-col gap-1">
                          <code className="text-emerald-600 dark:text-emerald-400">baca_berkas("data.txt")</code>
                          <span className="text-xs text-slate-500 font-sans">Membaca isi berkas dari VFS/Sistem. Mengembalikan "ERROR" jika gagal.</span>
                        </div>
                        <div className="flex flex-col gap-1">
                          <code className="text-emerald-600 dark:text-emerald-400">tulis_berkas("data.txt", isi)</code>
                          <span className="text-xs text-slate-500 font-sans">Menulis atau menimpa isi ke dalam berkas.</span>
                        </div>
                        <div className="w-full h-px bg-slate-200 dark:bg-slate-700 my-2"></div>
                        <div className="flex flex-col gap-1">
                          <code className="text-blue-500">ambil_web("https://api.github.com")</code>
                          <span className="text-xs text-slate-500 font-sans">Mengirim permintaan HTTP GET secara sinkron.</span>
                        </div>
                        <div className="flex flex-col gap-1">
                          <code className="text-purple-500">masukkan("modul.tj")</code>
                          <span className="text-xs text-slate-500 font-sans">Mengeksekusi skrip lain dan mengimpor nilai/fungsi kembaliannya.</span>
                        </div>
                        <div className="w-full h-px bg-slate-200 dark:bg-slate-700 my-2"></div>
                        <div className="flex flex-col gap-1">
                          <code className="text-amber-600 dark:text-amber-400">ke_json(kamus_atau_larik)</code>
                          <span className="text-xs text-slate-500 font-sans">Serialisasi struktur data Taji menjadi format teks JSON.</span>
                        </div>
                        <div className="flex flex-col gap-1">
                          <code className="text-amber-600 dark:text-amber-400">{"dari_json('{\"key\":\"value\"}')"}</code>
                          <span className="text-xs text-slate-500 font-sans">Deserialisasi string JSON menjadi Kamus/Larik Taji.</span>
                        </div>
                      </div>
                    </div>
                  </div>
                </section>

              </div>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
