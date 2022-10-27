use crate::create_graph::node::Node;
use crate::{create_graph::block::Block, tools::utils::remove_value};
use std::collections::{HashMap, HashSet};

pub struct Divergences<'a> {
    // (node A, neighbouring node B of A)=> { blocks where B diverges from A, OR dead end, when we start a DFS from A }
    data: HashMap<(Node<'a>, Node<'a>), HashSet<Block<'a>>>,
}

impl<'a> Divergences<'a> {
    pub fn new(blocks: &HashSet<Block<'a>>) -> Self {
        let mut data: HashMap<(Node<'a>, Node<'a>), HashSet<Block<'a>>> = HashMap::new();
        for block in blocks {
            for node_0 in block.get_nodes() {
                for node_1 in block.get_nodes() {
                    data.insert((node_0.clone(), node_1), HashSet::new());
                }
            }
        }
        return Divergences { data };
    }

    pub fn add_divergence_block(
        &mut self,
        from_node: &Node<'a>,
        neighboring_node: &Node<'a>,
        divergence_block: Block<'a>,
    ) {
        assert!(from_node.get_block() == neighboring_node.get_block());
        self.data
            .get_mut(&(from_node.clone(), neighboring_node.clone()))
            .unwrap()
            .insert(divergence_block);
    }

    pub fn add_many_divergence_blocks(
        &mut self,
        from_node: &Node<'a>,
        neighboring_node: &Node<'a>,
        divergence_blocks: HashSet<Block<'a>>,
    ) {
        assert!(from_node.get_block() == neighboring_node.get_block());
        self.data
            .get_mut(&(from_node.clone(), neighboring_node.clone()))
            .unwrap()
            .extend(divergence_blocks);
    }

    pub fn get_divergence_blocks(
        &self,
        from_node: &Node<'a>,
        neighboring_node: &Node<'a>,
    ) -> &HashSet<Block<'a>> {
        assert!(from_node.get_block() == neighboring_node.get_block());
        return &self.data[&(from_node.clone(), neighboring_node.clone())];
    }
}

pub fn compute_divergences<'a>(blocks: &HashSet<Block<'a>>) -> Divergences<'a> {
    let mut divergences: Divergences<'a> = Divergences::new(blocks);
    let mut visited_nodes: HashSet<Node<'a>> = HashSet::new();
    for node in Block::get_all_orphan_nodes(blocks) {
        _dfs(&mut visited_nodes, &mut divergences, &node);
    }
    return divergences;
}

fn _dfs<'a>(
    visited_nodes: &mut HashSet<Node<'a>>,
    divergences: &mut Divergences<'a>,
    node: &Node<'a>,
) {
    if !visited_nodes.contains(node) {
        let mut neighboring_nodes: Vec<Node<'a>> = node.get_block().get_nodes(); // TODO neighboring_nodes only ?
        remove_value(&mut neighboring_nodes, node);

        if node.get_block().is_dead_end() {
            for neighboring_node in node.get_block().get_nodes() {
                divergences.add_divergence_block(node, &neighboring_node, node.get_block());
            }
        } else {
            assert!(node.get_children().len() <= 2);
            for child_index in 0..node.get_children().len() {
                let next_node: &Node = &node.get_children()[child_index];
                _dfs(visited_nodes, divergences, next_node);
                for neighboring_node in &neighboring_nodes {
                    let next_neighboring_node: &Node =
                        &neighboring_node.get_children()[child_index];
                    if next_neighboring_node.get_block() == next_node.get_block() {
                        let next_divergences: HashSet<Block> = divergences
                            .get_divergence_blocks(next_node, next_neighboring_node)
                            .clone();
                        divergences.add_many_divergence_blocks(
                            node,
                            neighboring_node,
                            next_divergences,
                        );
                    } else {
                        divergences.add_divergence_block(node, neighboring_node, node.get_block());
                    }
                }
            }
        }
        visited_nodes.insert(node.clone());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        bytecode_reader::bytecode::Bytecode, create_graph::graph::Graph,
        detect_cycles::acyclic_graph::AcyclicGraph,
    };
    use std::fs;

    #[test]
    pub fn test_divergence_symetry() {
        for path in crate::tools::utils::tests::get_all_bytecode_paths() {
            // let path: String =
            //     String::from("./contracts/simple/contract_0/bytecode.txt");

            println!("Loading {}", path);
            let bytecode_string: String = fs::read_to_string(path).expect("Unable to read file.");
            let bytecode: Bytecode = Bytecode::from(&bytecode_string).unwrap();
            let mut graph: Graph = Graph::from(&bytecode);
            let a_graph: AcyclicGraph = AcyclicGraph::from(&mut graph);
            let divergences: Divergences = compute_divergences(&a_graph.get_all_blocks());
            for block in a_graph.get_all_blocks() {
                for node_0 in block.get_nodes() {
                    for node_1 in block.get_nodes() {
                        if divergences.get_divergence_blocks(&node_0, &node_1)
                            != divergences.get_divergence_blocks(&node_1, &node_0)
                        {
                            panic!();
                        }
                    }
                }
            }
        }
    }
}
