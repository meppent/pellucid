mod utils;
use std::fs;

use pellucid::bytecode_reader::bytecode;
use pellucid::jump_graph::gml::to_gml;
use pellucid::jump_graph::{display::draw, graph::BlockSet};

#[test]
fn test_graph_drawing() {
    let contract: utils::Contract = utils::Contract::SIMPLE_CONTRACT;
    let bytecode: bytecode::Bytecode = bytecode::Bytecode::from(&contract.get_bytecode());
    let block_set: BlockSet = BlockSet::new(&bytecode);
    let graph_drawing: String = draw(&block_set, &bytecode);
    let graph_drawing_ref : String = contract.get_graph_drawing();
    println!("{}", graph_drawing.len());
    println!("{}", graph_drawing_ref.len());

    // We now write at the offset 10.
    fs::write("draw1.txt", contract.get_graph_drawing()).expect("Unable to write file");
    fs::write("draw2.txt", &graph_drawing).expect("Unable to write file");

    assert!(graph_drawing.eq(&contract.get_graph_drawing()), "graphs differ");
}

#[test]
fn test_gml() {
    let contract: utils::Contract = utils::Contract::SIMPLE_CONTRACT;
    let bytecode: bytecode::Bytecode = bytecode::Bytecode::from(&contract.get_bytecode());
    let block_set: BlockSet = BlockSet::new(&bytecode);
    let gml: String = to_gml(&block_set);
    println!("{}", gml);

    assert!(gml == contract.get_gml());
}
