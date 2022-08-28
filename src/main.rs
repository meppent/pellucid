use crate::bytecode_reader::opcode::init_opcodes;
use std::env;
mod bytecode_reader;
mod evm;
mod gui;
mod jump_graph;
mod tests;
mod utils;

fn main() {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "debug")
    };
    env_logger::init();
    init_opcodes();
    tests::test_jump_graph();
}
