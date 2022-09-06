use std::borrow::BorrowMut;
use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use super::block::BlockRef;
use super::node::NodeRef;
use super::simple_evm::SimpleContext;
use crate::bytecode_reader::bytecode::Bytecode;
use crate::create_blocks::parser;
use crate::create_graph::simple_evm::State;
#[derive(Debug)]
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
        let graph: Graph = Graph { blocks };
        let first_block = graph.get_block(0);
        let initial_node = NodeRef::new(first_block, SimpleContext::new());
        graph.explore_from(initial_node, SimpleContext::new());
        return graph;
    }

    //we may want to create RC for SimpleContext knowing they are owned by 2 nodes
    pub fn explore_from(&self, node_origin: NodeRef<'a>, initial_context: SimpleContext) {
        let block_origin = node_origin.get_block();
        //assert!(!block_origin.contains_initial_context(&initial_context)); // MODIFICATION TO BE MADE
        let final_context: SimpleContext = block_origin.apply_on_simple_context(&initial_context);

        let next_dests: Vec<usize> = match &final_context.state {
            State::RUNNING => vec![block_origin.get_next_pc_start()],
            State::STOP => vec![],
            State::JUMP(next_dests) => next_dests.clone(),
        };

        node_origin.set_final_context(final_context.clone());

        for dest in next_dests {
            if let Some(block_dest) = self.blocks.get(&dest) {
                if !block_dest.contains_initial_context(&final_context) {
                    let node_dest = NodeRef::new(block_dest.clone(), final_context.clone());
                    node_origin.add_child(node_dest.clone());
                    let mut next_initial_context = final_context.clone();
                    next_initial_context.state = State::RUNNING;
                    self.explore_from(node_dest, next_initial_context);
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

    pub fn get_all_pc_starts(&self) -> Vec<usize> {
        return self.blocks.keys().into_iter().cloned().collect_vec();
    }

    pub fn get_edges(&self) -> Vec<(usize, usize)> {
        // list of all edges: pc_start origin => pc_start dest
        let mut edges: Vec<(usize, usize)> = Vec::new();
        for (pc_start_origin, block_ref_origin) in &self.blocks {
            for node_ref_origin in block_ref_origin.get_nodes(){
                for node_ref_dest in node_ref_origin.get_children(){
                    let block_ref_dest: BlockRef = node_ref_dest.get_block();
                    let pc_start_dest: usize = block_ref_dest.get_pc_start();
                    edges.push((*pc_start_origin, pc_start_dest));
                }
            }
        }

        return edges;
    }

    pub fn get_pc_end_of_block(&self, block_pc_start: usize)->usize{
        return self.blocks[&block_pc_start].get_pc_end();
    }
}

#[cfg(test)]
mod tests {

    use primitive_types::U256;

    use crate::bytecode_reader::vopcode::Vopcode;
    use crate::create_blocks::symbolic_expression::StackExpression;
    use crate::create_graph::display::draw;

    use super::*;
    use crate::bytecode_reader::bytecode::Bytecode;
    use std::fs;

    #[test]
    pub fn small_test() {
        let bytecode_string: String =
            fs::read_to_string("./assets/contracts/simple_contract/bytecode.txt")
                .expect("Unable to read file.");
        let bytecode_test: Bytecode = Bytecode::from(&bytecode_string);

        let graph = Graph::from(&bytecode_test);
        println!("{}", draw(&graph, &bytecode_test));
    }
}
