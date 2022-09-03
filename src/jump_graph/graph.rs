use crate::bytecode_reader::opcode::Opcode;
use crate::bytecode_reader::{bytecode::Bytecode, vopcode::Vopcode};
use crate::evm::stack::Stack;
use std::collections::{HashMap, HashSet};
use std::{cell::RefCell, rc::Rc};
extern crate queues;
use super::block::{Block, BlockRef};
use super::node::{Node, NodeRef};
use queues::*;

struct Graph<'a> {
    pub blocks: HashMap<usize, BlockRef<'a>>,
}

impl<'a> Graph<'a> {
    pub fn new() -> Self {
        return Graph {
            blocks: HashMap::new(),
        };
    }

    pub fn add_block(&mut self, block: BlockRef<'a>) {
        self.blocks.insert(block.get_pc_start(), block);
    }

    pub fn get_block(&self, index: usize) -> BlockRef<'a> {
        return self.blocks[&index].clone();
    }

    pub fn DFS_search(&self, fun_before: &dyn Fn(&NodeRef<'a>), fun_after: &dyn Fn(&NodeRef<'a>)) {
        let mut visited: HashSet<NodeRef> = HashSet::new();
        //first node access every node
        self.explore_DFS(
            &(self.blocks)[&0].get_nodes()[0],
            &mut visited,
            fun_before,
            fun_after,
        );
    }

    pub fn explore_DFS(
        self,
        node: NodeRef,
        &mut visited: HashSet<NodeRef>,
        fun_before: &dyn Fn(&NodeRef<'a>),
        fun_after: &dyn Fn(&NodeRef<'a>),
    ) {
        if !visited.contains(&node) {
            visited.insert(node);
            fun_before(&node);
            for child in node.get_children() {
                self.explore_DFS(child, &mut visited, fun_before, fun_after);
            }
            fun_after(&node);
        }
    }

    pub fn BFS_search(&self, fun: &dyn Fn(&NodeRef<'a>)) {
        let mut visited: HashSet<NodeRef> = HashSet::new();
        let mut queue: Queue<isize> = queue![];
        queue.add(&(self.blocks)[0].get_nodes()[0]);

        while !queue.is_empty() {
            let node = queue.pop().unwrap();

            if !visited.contains(&node) {
                visited.insert(node);
                fun(&node);
                for child in node.get_children() {
                    queue.add(child);
                }
            }
        }
    }
}
