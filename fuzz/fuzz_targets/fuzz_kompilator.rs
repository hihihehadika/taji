#![no_main]
use libfuzzer_sys::fuzz_target;
use taji::compiler::Kompilator;
use taji::lexer::Lexer;
use taji::parser::Parser;
use taji::vm::VM;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        let lexer = Lexer::new(s);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        
        let tabel = taji::bawaan::bikin_tabel_awal();
        if let Ok(hasil) = Kompilator::new_dengan_state(tabel, Vec::new()).kompilasi(&program) {
            let mut vm = VM::new_dengan_globals(hasil, taji::bawaan::bikin_globals_awal());
            vm.batas_instruksi = Some(100_000); // Mencegah infinite loop saat fuzzing
            let _ = vm.jalankan();
        }
    }
});
