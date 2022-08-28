use crate::bytecode_reader::opcode::init_opcodes;
use std::env;
mod bytecode_reader;
mod evm;
mod jump_graph;
mod tests;
mod utils;

fn init() {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "debug")
    };
    env_logger::init();
    init_opcodes();
}
fn main() {
    init();
    tests::test_gml();
}
