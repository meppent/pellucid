use pellucid::bytecode_reader::bytecode::Bytecode;
use pellucid::bytecode_reader::metadata;

fn main() {
    let pattern = std::env::args().nth(1).expect("No action precised");
    let source_code = std::env::args().nth(2).expect("No source code given");

    match pattern.as_str() {
        "decompile" => {
            let bytecode : Bytecode = Bytecode::from(&source_code);

            println!("{}", bytecode);
        }
        "metadata" => {
            let metadata = metadata::get_metadata(&source_code);
            let metadata_length = metadata.len();
            println!("{} metadata found", metadata_length);
            for m in metadata {
                println!("  {}", m);
            }
        }
        _ => {}
    }

}
