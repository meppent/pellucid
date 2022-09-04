use std::collections::{HashMap, HashSet};
use itertools::Itertools;

use crate::bytecode_reader::bytecode::Bytecode;
use crate::create_blocks::parser;

use super::block::{BlockRef, Block, self};
use super::node::NodeRef;

pub struct Graph<'a> {
    pub blocks: HashMap<usize, BlockRef<'a>>,
}

impl<'a> Graph<'a> {
    pub fn new()->Self{
        return Graph { blocks: HashMap::new() };
    }
    pub fn from(bytecode: &'a Bytecode) -> Self {
        let blocks: HashMap<usize, BlockRef<'a>> = parser::find_blocks(&bytecode);


        return Graph {blocks};
    }

    pub fn add_block(&mut self, block: BlockRef<'a>) {
        self.blocks.insert(block.get_pc_start(), block);
    }

    pub fn get_block(&self, index: usize) -> BlockRef<'a> {
        return self.blocks[&index].clone();
    }

    pub fn DFS_search(&self, fun_before: &dyn Fn(NodeRef<'a>), fun_after: &dyn Fn(NodeRef<'a>)) {
        let mut visited: HashSet<NodeRef> = HashSet::new();
        //first node access every node
        self.explore_DFS(
            (&self.blocks)[&0].get_nodes()[0].clone(),
            &mut visited,
            fun_before,
            fun_after,
        );
    }

    pub fn explore_DFS(
        &self,
        node: NodeRef<'a>,
        visited: &mut HashSet<NodeRef<'a>>,
        fun_before: &dyn Fn(NodeRef<'a>),
        fun_after: &dyn Fn(NodeRef<'a>),
    ) {
        if !visited.contains(&node.clone()) {
            visited.insert(node.clone());
            fun_before(node.clone());
            for child in node.get_children() {
                (&self).explore_DFS(child.clone(), visited, &fun_before, &fun_after);
            }
            fun_after(node.clone());
        }
    }
}
