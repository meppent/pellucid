mod utils;

use pellucid::bytecode_reader::bytecode;
use pellucid::utils::write_file;
use pellucid::{
    bytecode_reader::bytecode::Bytecode,
    jump_graph::{display::draw, graph::BlockSet},
};

#[test]
fn test_graph_drawing() {
    let contract: utils::Contract = utils::Contract::SIMPLE_CONTRACT;
    let bytecode: bytecode::Bytecode = bytecode::Bytecode::from(&contract.get_bytecode());
    let block_set: BlockSet = BlockSet::new(&bytecode);
    let graph_drawing: String = draw(&block_set, &bytecode);
    assert!(graph_drawing == contract.get_graph_drawing());
}
