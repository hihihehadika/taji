/* tslint:disable */
/* eslint-disable */

/**
 * Mengatur antrian masukan untuk fungsi `tanya()` di lingkungan WASM.
 *
 * Frontend TypeScript memanggil fungsi ini sebelum `jalankan_taji()`
 * untuk menyediakan baris-baris input yang akan dikonsumsi oleh
 * setiap pemanggilan `tanya()` di dalam skrip Taji.
 */
export function atur_antrian_masukan(input_mentah: string): void;

/**
 * Titik masuk utama untuk eksekusi kode Taji dari lingkungan WebAssembly.
 *
 * Fungsi ini menerima kode sumber Taji sebagai string, menjalankan
 * seluruh pipeline (Lexer -> Parser -> Kompilator -> VM), dan
 * mengembalikan output sebagai string tunggal.
 */
export function jalankan_taji(kode_sumber: string): string;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly atur_antrian_masukan: (a: number, b: number) => void;
    readonly jalankan_taji: (a: number, b: number) => [number, number];
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
    readonly __wbindgen_exn_store: (a: number) => void;
    readonly __externref_table_alloc: () => number;
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __wbindgen_free: (a: number, b: number, c: number) => void;
    readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
 * Instantiates the given `module`, which can either be bytes or
 * a precompiled `WebAssembly.Module`.
 *
 * @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
 *
 * @returns {InitOutput}
 */
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
 * If `module_or_path` is {RequestInfo} or {URL}, makes a request and
 * for everything else, calls `WebAssembly.instantiate` directly.
 *
 * @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
 *
 * @returns {Promise<InitOutput>}
 */
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
