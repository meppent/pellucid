#![allow(dead_code)]
use std::fs;

use crate::{
    bytecode_reader::bytecode::Bytecode,
    jump_graph::{display::draw, gml::to_gml, graph::BlockSet},
};

pub const BYTE_CODE : &str = "608060405234801561001057600080fd5b50600436106100415760003560e01c80630dbe671f1461004657806326121ff014610064578063e2179b8e14610082575b600080fd5b61004e6100a0565b60405161005b9190610173565b60405180910390f35b61006c6100a6565b6040516100799190610173565b60405180910390f35b61008a6100f5565b6040516100979190610158565b60405180910390f35b60005481565b600080600c905060005b818110156100ec5760006100c38261012d565b9050806100d057816100d2565b815b6000819055505080806100e4906101a4565b9150506100b0565b50604391505090565b6000640f85a49aaa60008190555060005b61010f8161012d565b61011e57806001019050610106565b6101278161012d565b91505090565b6000600682119050919050565b6101438161018e565b82525050565b6101528161019a565b82525050565b600060208201905061016d600083018461013a565b92915050565b60006020820190506101886000830184610149565b92915050565b60008115159050919050565b6000819050919050565b60006101af8261019a565b91507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8214156101e2576101e16101ed565b5b600182019050919050565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052601160045260246000fdfea264697066735822122089e2522dad178ca7ec9d442000419cd24591a9a757b7b2605a9cfcf1cb7752f864736f6c63430008070033";

pub fn test_jump_graph() {
    let bytecode: Bytecode = Bytecode::from(BYTE_CODE);
    // let block_set: BlockSet = BlockSet::new(&bytecode);
    // let graph_drawing: String = draw(&block_set, &bytecode);
    // println!("{}", graph_drawing);
}

pub fn test_gml() {
    let bytecode: Bytecode = Bytecode::from(BYTE_CODE);
    // let block_set: BlockSet = BlockSet::new(&bytecode);
    // let gml: String = to_gml(&block_set);
    // println!("{}", gml);
}

pub fn test_bytecode_reader() {
    let bytecode: Bytecode = Bytecode::from(BYTE_CODE);
    println!("{}", bytecode)
}
