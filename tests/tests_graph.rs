mod utils;
use pellucid::bytecode_reader::bytecode;
use pellucid::jump_graph::gml::to_gml;
use pellucid::jump_graph::{display::draw, graph::BlockSet};
use pellucid::utils::write_file;

#[test]
fn test_graph_drawing() {
    let contract: utils::Contract = utils::Contract::SIMPLE_CONTRACT;
    let bytecode: bytecode::Bytecode = bytecode::Bytecode::from(&contract.get_bytecode());
    let block_set: BlockSet = BlockSet::new(&bytecode);
    let graph_drawing: String = draw(&block_set, &bytecode);
    assert!(graph_drawing == contract.get_graph_drawing());
}

#[test]
fn test_gml() {
    let contract: utils::Contract = utils::Contract::SIMPLE_CONTRACT;
    let bytecode: bytecode::Bytecode = bytecode::Bytecode::from(&contract.get_bytecode());
    let block_set: BlockSet = BlockSet::new(&bytecode);
    let gml: String = to_gml(&block_set);
    assert!(gml == contract.get_gml());
}
