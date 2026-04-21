//! Pustaka inti bahasa pemrograman Taji.
//!
//! Mengekspos modul token, lexer, parser, object, repl,
//! infrastruktur bytecode VM (code, compiler, vm), dan
//! Taji Package Manager (tpm) untuk digunakan oleh binary
//! utama maupun pengujian.

pub mod ast;
pub mod bawaan;
pub mod code;
pub mod compiler;
pub mod lexer;
pub mod object;
pub mod parser;
pub mod repl;
pub mod token;
pub mod tpm;
pub mod vm;
