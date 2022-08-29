use pellucid::bytecode_reader::bytecode;

fn main() {
    let pattern = std::env::args().nth(1).expect("No action precised");
    let source_code = std::env::args().nth(2).expect("No source code given");

    assert!(pattern == "decompile", "Only decompile available for now");

    let bytecode : bytecode::Bytecode = bytecode::Bytecode::from(&source_code);

    println!("{}", bytecode);
}
