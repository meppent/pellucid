use std::collections::{HashMap, HashSet};
use std::{cell::RefCell, rc::Rc};
use crate::bytecode_reader::opcode::Opcode;
use crate::bytecode_reader::{bytecode::Bytecode, vopcode::Vopcode};
use crate::evm::context::Context;
use crate::evm::stack::Stack;

use super::node::{Node, NodeRef};
use super::block::{Block, BlockRef};

struct Graph<'a> {
    pub blocks: HashMap<usize, BlockRef<'a>>,
}

impl <'a> Graph<'a> {
    pub fn new() -> Self {
        return Graph{blocks: HashMap::new()};
    }

    pub fn add_block(&mut self, block: BlockRef<'a>) {
        self.blocks.insert(block.get_pc_start(), block);
    }

    pub fn get_block(&self, index: usize) -> BlockRef<'a> {
        return self.blocks[&index].clone();
    }

    pub fn in_depth_search(&self){
        let mut visited: HashSet<NodeRef> = HashSet::new();
        for (index_block, block) in &(self.blocks){
            for node in block.get_nodes(){
                if !visited.contains(&node){
                    self.explore(node, &mut visited);
                }
            }
        }
    }

    pub fn explore(self){

    }
}
