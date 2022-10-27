use crate::create_graph::{block::Block, graph::Graph, node::Node};
use std::{
    collections::{HashMap, HashSet},
    ops::{Deref, DerefMut},
};

use super::block_loops::BlockLoops;

pub struct AcyclicGraph<'a, 'b> {
    // TODO improve structure
    pub graph: &'b mut Graph<'a>,
    pub loops: BlockLoops,
    pub disconnected_connections: HashMap<usize, usize>, // final block pc_start with a 'continue' -> loop label
}

impl<'a, 'b> Deref for AcyclicGraph<'a, 'b> {
    type Target = Graph<'a>;
    fn deref(&self) -> &Graph<'a> {
        self.graph
    }
}

impl<'a, 'b> DerefMut for AcyclicGraph<'a, 'b> {
    fn deref_mut(&mut self) -> &mut Graph<'a> {
        self.graph
    }
}
impl<'a, 'b> AcyclicGraph<'a, 'b> {
    pub fn from(graph: &'b mut Graph<'a>) -> AcyclicGraph<'a, 'b> {
        let loops: BlockLoops = BlockLoops::from(graph);
        // cut loops:
        let mut disconnected_connections: HashMap<usize, usize> = HashMap::new();

        for parent_block in graph.get_all_blocks() {
            let parent_labels: HashSet<usize> = loops.get_labels_at_block(&parent_block);
            for parent_node in parent_block.get_nodes() {
                for child_node in parent_node.get_children() {
                    let child_block: Block<'a> = child_node.get_block();
                    if let Some(entry_label) = loops.get_label_of_entry(&child_block) {
                        if parent_labels.contains(&entry_label) {
                            graph.disconnect_nodes(parent_node.clone(), child_node.clone());
                            if let Some(already_existing_label) =
                                disconnected_connections.get(&parent_block.get_pc_start())
                            {
                                assert!(*already_existing_label == entry_label);
                            } else {
                                disconnected_connections
                                    .insert(parent_block.get_pc_start(), entry_label);
                            }
                        }
                    }
                }
            }
        }
        let acyclic_graph = AcyclicGraph {
            graph,
            loops,
            disconnected_connections,
        };
        acyclic_graph.assert_is_really_acyclic();
        return acyclic_graph;
    }

    fn assert_is_really_acyclic(&self) {
        let mut visited: HashSet<Node> = HashSet::new();
        let mut current_parents: HashSet<Node> = HashSet::new();
        let initial_node: Node = self.graph.get_block(0).get_nodes()[0].clone();
        Self::_explore_dfs_to_check_acyclic(&mut visited, &mut current_parents, initial_node);
    }

    fn _explore_dfs_to_check_acyclic(
        visited: &mut HashSet<Node<'a>>,
        current_parents: &mut HashSet<Node<'a>>,
        node: Node<'a>,
    ) where
        'a: 'b,
    {
        if visited.contains(&node) {
            assert!(!current_parents.contains(&node));
        } else {
            visited.insert(node.clone());
            current_parents.insert(node.clone());

            for child in node.get_children() {
                Self::_explore_dfs_to_check_acyclic(visited, current_parents, child);
            }
            current_parents.remove(&node);
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::bytecode_reader::bytecode::Bytecode;
    use crate::create_graph::graph::tests::SerializableGraph;
    use crate::create_graph::graph::Graph;
    use crate::tools::utils::read_file;
    use crate::tools::utils::tests::get_all_bytecode_paths;
    use std::fs;

    #[test]
    pub fn test_no_node_cycles() {
        //let path: String = String::from("./contracts/simple/contract_0/");
        for path in get_all_bytecode_paths() {
            let bytecode_string: String = fs::read_to_string(path).expect("Unable to read file.");
            let bytecode: Bytecode = Bytecode::from(&bytecode_string).unwrap();

            let mut graph: Graph = Graph::from(&bytecode);
            let _: AcyclicGraph = AcyclicGraph::from(&mut graph);
        }
    }

    #[test]
    pub fn test_snapshot_acyclic_graph() {
        let path: String = String::from("./contracts/loop/contract_2/");
        let bytecode_string: String =
            fs::read_to_string(path.clone() + "bytecode.txt").expect("Unable to read file.");
        let bytecode: Bytecode = Bytecode::from(&bytecode_string).unwrap();
        let mut graph: Graph = Graph::from(&bytecode);
        let a_graph: AcyclicGraph = AcyclicGraph::from(&mut graph);

        let serializable_acyclic_graph: SerializableGraph = SerializableGraph::from(a_graph.graph);

        let target_serialized_acyclic_graph: SerializableGraph = serde_json::from_str(&read_file(
            &(path.clone() + "acyclic_serialized_graph.json"),
        ))
        .unwrap();
        assert!(target_serialized_acyclic_graph == serializable_acyclic_graph);

        //to overwrite the dest json:
        // crate::tools::utils::write_file(
        //     &(path.clone() + "acyclic_serialized_graph.json"),
        //     &serde_json::to_string(&serializable_acyclic_graph).unwrap(),
        // );
    }
}
