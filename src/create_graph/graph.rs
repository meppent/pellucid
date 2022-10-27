use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use super::block::Block;
use super::node::Node;
use super::post_processing::remove_looping_blocks;
use super::simple_evm::SimpleContext;
use crate::bytecode_reader::bytecode::Bytecode;
use crate::create_blocks::parser;
use crate::create_graph::simple_evm::State;
#[derive(Debug)]
pub struct Graph<'a> {
    pub origin_blocks: HashMap<usize, Block<'a>>,
    pub all_blocks: HashSet<Block<'a>>, // icnlude duplications
}
pub static mut CURRENT_DUPLICATION_INDEX: usize = 0;

impl<'a> Graph<'a> {
    pub fn from(bytecode: &'a Bytecode) -> Self {
        let origin_blocks: HashMap<usize, Block<'a>> = parser::find_blocks(&bytecode);
        let all_blocks: HashSet<Block> = origin_blocks.values().cloned().collect();
        let mut graph: Graph = Graph {
            origin_blocks,
            all_blocks,
        };
        let first_block: Block = graph.get_block(0);
        let initial_node: Node = Node::create_and_attach(first_block, SimpleContext::new());
        graph.explore_from(initial_node);
        remove_looping_blocks(&mut graph);
        return graph;
    }

    pub fn explore_from(&self, node_origin: Node<'a>) {
        let block_origin: Block = node_origin.get_block();
        let current_final_context: SimpleContext = node_origin.clone_final_context();
        let next_dests: Vec<usize> = match &current_final_context.state {
            State::RUNNING => vec![block_origin.get_next_pc_start()],
            State::STOP => vec![],
            State::JUMP(next_dests) => next_dests.clone(),
        };

        let mut next_initial_context: SimpleContext = current_final_context;
        next_initial_context.state = State::RUNNING;

        for dest in next_dests {
            if let Some(block_dest) = self.origin_blocks.get(&dest) {
                if let Some(node_dest) = block_dest.get_node_starting_with(&next_initial_context) {
                    node_origin.add_child(Node::clone(&node_dest));
                } else {
                    let node_dest: Node = Node::create_and_attach(
                        Block::clone(block_dest),
                        next_initial_context.clone(),
                    );
                    node_origin.add_child(Node::clone(&node_dest));
                    self.explore_from(node_dest);
                }
            }
        }
    }

    pub fn duplicate_block(&mut self, block: &Block<'a>) -> Block<'a> {
        assert!(self.all_blocks.contains(block));
        let duplicated_block: Block = Block::new(
            block.get_code(),
            Some((unsafe { CURRENT_DUPLICATION_INDEX }, block.clone())),
        );
        unsafe { CURRENT_DUPLICATION_INDEX += 1 };
        self.all_blocks.insert(duplicated_block.clone());
        return duplicated_block;
    }

    pub fn get_block(&self, pc_start: usize) -> Block<'a> {
        return self.origin_blocks[&pc_start].clone();
    }

    pub fn get_all_pc_starts(&self) -> Vec<usize> {
        return self.origin_blocks.keys().into_iter().cloned().collect_vec();
    }

    pub fn get_edges(&self) -> Vec<(usize, usize)> {
        // list of all edges: pc_start origin => pc_start dest
        let mut edges: Vec<(usize, usize)> = Vec::new();
        for (pc_start_origin, block_origin) in &self.origin_blocks {
            for node_origin in block_origin.get_nodes() {
                for node_dest in node_origin.get_children() {
                    let block_dest: Block = node_dest.get_block();
                    let pc_start_dest: usize = block_dest.get_pc_start();
                    edges.push((*pc_start_origin, pc_start_dest));
                }
            }
        }
        return edges;
    }

    pub fn get_pc_end_of_block(&self, block_pc_start: usize) -> usize {
        return self.origin_blocks[&block_pc_start].get_pc_end();
    }

    pub fn get_all_nodes(&self) -> HashSet<Node<'a>> {
        let mut all_nodes: HashSet<Node<'a>> = HashSet::new();
        for block in self.get_all_blocks() {
            for node in block.get_nodes() {
                all_nodes.insert(node);
            }
        }
        return all_nodes;
    }

    pub fn disconnect_nodes(&self, parent: Node<'a>, child: Node<'a>) {
        parent.remove_child(child.clone());
        child.remove_parent(parent.clone());
    }

    pub fn get_blocks_of_nodes(nodes: &HashSet<Node<'a>>) -> HashSet<Block<'a>> {
        let mut blocks: HashSet<Block<'a>> = HashSet::new();
        for node in nodes {
            blocks.insert(node.get_block());
        }
        return blocks;
    }

    pub fn get_all_blocks(&self) -> HashSet<Block<'a>> {
        return self.all_blocks.iter().cloned().collect();
    }

    pub fn get_pc_ends(&self) -> HashSet<usize> {
        return self
            .origin_blocks
            .values()
            .map(|block| block.get_pc_end())
            .collect();
    }

    pub fn get_initial_node(&self) -> Node<'a> {
        return self.origin_blocks[&0].get_nodes()[0].clone();
    }

    // pub fn deep_copy(&self) -> Self {
    //     let mut new_blocks: HashMap<usize, Block<'a>> = HashMap::new();
    //     for (pc_start, current_block) in &self.blocks {
    //         new_blocks.insert(*pc_start, Block::new(current_block.get_code()));
    //     }
    //     let mut current_node_to_new_node: HashMap<Node<'a>, Node<'a>> = HashMap::new();
    //     for (pc_start, current_block) in &self.blocks {
    //         let new_block: Block = new_blocks[pc_start].clone();
    //         for current_node in current_block.get_nodes() {
    //             let new_node: Node = Node::create_and_attach(
    //                 new_block.clone(),
    //                 current_node.clone_initial_context(),
    //             );
    //             current_node_to_new_node.insert(current_node, new_node);
    //         }
    //     }
    //     for current_parent in self.get_all_nodes() {
    //         let new_parent: Node = current_node_to_new_node[&current_parent].clone();
    //         for current_child in current_parent.get_children() {
    //             let new_child: Node = current_node_to_new_node[&current_child].clone();
    //             new_parent.add_child(new_child);
    //         }
    //     }
    //     return Graph { blocks: new_blocks };
    // }
}

#[cfg(test)]
pub mod tests {
    use serde::{Deserialize, Serialize};
    use std::collections::HashSet;
    use super::*;
    use crate::bytecode_reader::bytecode::Bytecode;
    use crate::tools::utils::read_file;
    use itertools::Itertools;
    use std::fs;
    use std::iter::FromIterator;

    #[derive(PartialEq, Deserialize, Serialize)]
    pub struct SerializableGraph {
        pc_starts: HashSet<usize>,
        pc_ends: HashSet<usize>,
        contexts: HashSet<Vec<(SimpleContext, SimpleContext)>>,
        edges: HashSet<(usize, usize)>,
    }

    impl SerializableGraph {
        pub fn from<'a>(graph: &Graph<'a>) -> Self {
            return SerializableGraph {
                pc_starts: HashSet::from_iter(
                    graph
                        .origin_blocks
                        .keys()
                        .into_iter()
                        .cloned()
                        .collect_vec(),
                ),
                pc_ends: graph.get_pc_ends(),
                contexts: HashSet::from_iter(
                    graph
                        .origin_blocks
                        .values()
                        .map(|block| {
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
        }
    }

    #[test]
    pub fn test_graph_snapshot() {
        let bytecode_string: String =
            fs::read_to_string("./contracts/simple/contract_0/bytecode.txt")
                .expect("Unable to read file.");
        let bytecode: Bytecode = Bytecode::from(&bytecode_string).unwrap();
        let graph: Graph = Graph::from(&bytecode);

        let target_serializable_graph: SerializableGraph =
            serde_json::from_str(&read_file("./contracts/simple/contract_0/graph.json")).unwrap();
        let serializable_graph: SerializableGraph = SerializableGraph::from(&graph);
        assert!(target_serializable_graph == serializable_graph);

        //to overwrite the dest json:

        // write_file(
        //     "./contracts/simple/contract_0/graph.json",
        //     &serde_json::to_string(&serializable_graph).unwrap(),
        // );
    }
}
