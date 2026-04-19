//! Pustaka inti bahasa pemrograman Taji.
//!
//! Mengekspos modul token, lexer, parser, object, repl,
//! dan infrastruktur bytecode VM (code, compiler, vm) untuk digunakan
//! oleh binary utama atau pengujian.

pub mod ast;
pub mod bawaan;
pub mod code;
pub mod compiler;
pub mod lexer;
pub mod object;
pub mod parser;
pub mod repl;
pub mod token;
pub mod vm;
