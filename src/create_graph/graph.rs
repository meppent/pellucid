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
        let first_block: BlockRef = graph.get_block(0);
        let initial_node: NodeRef = NodeRef::create_and_attach(first_block, SimpleContext::new());
        graph.explore_from(initial_node);
        return graph;
    }

    //we may want to create RC for SimpleContext knowing they are owned by 2 nodes
    pub fn explore_from(&self, node_origin: NodeRef<'a>) {
        let block_origin: BlockRef = node_origin.get_block();
        let current_final_context: SimpleContext = node_origin.clone_final_context();
        
        let next_dests: Vec<usize> = match &current_final_context.state {
            State::RUNNING => vec![block_origin.get_next_pc_start()],
            State::STOP => vec![],
            State::JUMP(next_dests) => next_dests.clone(),
        };

        let mut next_initial_context: SimpleContext = current_final_context;
        next_initial_context.state = State::RUNNING;
        
        for dest in next_dests {
            if let Some(block_dest) = self.blocks.get(&dest) {
                if !block_dest.contains_initial_context(&next_initial_context) {
                    let node_dest: NodeRef = NodeRef::create_and_attach(BlockRef::clone(block_dest), next_initial_context.clone());
                    node_origin.add_child(NodeRef::clone(&node_dest));
                    self.explore_from(node_dest);
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
            for node_ref_origin in block_ref_origin.get_nodes() {
                for node_ref_dest in node_ref_origin.get_children() {
                    let block_ref_dest: BlockRef = node_ref_dest.get_block();
                    let pc_start_dest: usize = block_ref_dest.get_pc_start();
                    edges.push((*pc_start_origin, pc_start_dest));
                }
            }
        }

        return edges;
    }

    pub fn get_pc_end_of_block(&self, block_pc_start: usize) -> usize {
        return self.blocks[&block_pc_start].get_pc_end();
    }
}

#[cfg(test)]
mod tests {

    use serde::{Deserialize, Serialize};

    

    use super::*;
    use crate::bytecode_reader::bytecode::Bytecode;
    use crate::tools::utils::{read_file, write_file};
    use itertools::Itertools;
    use std::fs;
    use std::iter::FromIterator;

    #[test]
    pub fn small_test() {
        let bytecode_string: String =
            fs::read_to_string("./assets/contracts/simple_contract/bytecode.txt")
                .expect("Unable to read file.");
        let bytecode_test: Bytecode = Bytecode::from(&bytecode_string);

        let graph = Graph::from(&bytecode_test);
        // dbg!(&graph);
        // println!("{}", draw(&graph, &bytecode_test));
    }

    #[test]
    pub fn test_graph_snapshot() {
        #[derive(PartialEq, Deserialize, Serialize)]
        struct SerializableGraph {
            pc_starts: HashSet<usize>,
            pc_ends: HashSet<usize>,
            contexts: HashSet<Vec<(SimpleContext, SimpleContext)>>,
            edges: HashSet<(usize, usize)>,
        }
        let bytecode_string: String =
            fs::read_to_string("./assets/contracts/simple_contract/bytecode.txt")
                .expect("Unable to read file.");
        let bytecode: Bytecode = Bytecode::from(&bytecode_string);
        let graph: Graph = Graph::from(&bytecode);

        let serializable_graph: SerializableGraph = SerializableGraph {
            pc_starts: HashSet::from_iter(graph.blocks.keys().into_iter().cloned().collect_vec()),
            pc_ends: HashSet::from_iter(
                graph
                    .blocks
                    .iter()
                    .map(|(_, block)| block.get_pc_end())
                    .collect_vec(),
            ),
            contexts: HashSet::from_iter(
                graph
                    .blocks
                    .iter()
                    .map(|(_, block)| {
                        block
                            .get_nodes()
                            .iter()
                            .map(|node| {
                                (
                                    node.clone_initial_context().clone(),
                                    node.clone_final_context().clone(),
                                )
                            })
                            .collect_vec()
                    })
                    .collect_vec(),
            ),
            edges: HashSet::from_iter(graph.get_edges()),
        };
        
        let target_serializable_graph: SerializableGraph =
            serde_json::from_str(&read_file("./assets/contracts/simple_contract/graph.json"))
                .unwrap();
        //assert!(target_serializable_graph == serializable_graph);

        //to overwrite the dest json:

        // write_file(
        //     "./assets/contracts/simple_contract/graph.json",
        //     &serde_json::to_string(&serializable_graph).unwrap(),
        // );

    }
}
