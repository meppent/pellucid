use std::collections::{HashMap, HashSet};

use crate::bytecode_reader::bytecode::Bytecode;
use crate::create_blocks::parser;
use crate::create_graph::simple_evm::{State};
use super::block::BlockRef;
use super::node::NodeRef;
use super::simple_evm::{SimpleStack, SimpleContext};

pub struct Graph<'a> {
    pub blocks: HashMap<usize, BlockRef<'a>>,
}

impl<'a> Graph<'a> {
    pub fn new() -> Self {
        return Graph {
            blocks: HashMap::new(),
        };
    }



    pub fn from(bytecode: &'a Bytecode) -> Self {
        let blocks: HashMap<usize, BlockRef<'a>> = parser::find_blocks(&bytecode);

        let graph: Graph =  Graph { blocks };
        return graph;
    }

    pub fn explore_from(&self, block_ref: BlockRef<'a>, initial_context: SimpleContext){
        assert!(!block_ref.contains_initial_context(&initial_context));
        let final_context: SimpleContext = block_ref.apply_on_simple_context(&initial_context);
        let next_dests: Vec<usize> = match &final_context.state {
            State::RUNNING => vec![block_ref.get_next_pc_start()],
            State::STOP => vec![],
            State::JUMP(next_dests) => next_dests.clone()
        };
        for dest in next_dests {
            if let Some(block_dest) = self.blocks.get(&dest){
                if !block_dest.contains_initial_context(&final_context){
                    // block_ref -> block_dest
                    let node_dest = NodeRef::new(block_dest.clone(), initial_context.clone(), final_context.clone());
                    self.explore_from(BlockRef::clone(&self.blocks[&dest]), final_context.clone());
                }
            }
        }
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
