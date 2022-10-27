use std::collections::{HashMap, HashSet};

use crate::create_graph::{block::Block, graph::Graph, node::Node};

pub struct NodeLoops<'a, 'b> {
    pub graph: &'b Graph<'a>,
    pub visited: HashSet<Node<'a>>,
    pub parent_of: HashMap<Node<'a>, Option<Node<'a>>>,
    pub current_parents: HashSet<Node<'a>>, // TODO ? use HashMap node => bool to opti
    pub current_loop_origins: HashSet<usize>, // the loops we come from
    pub labels: HashMap<Node<'a>, HashSet<usize>>, // node => all the labels of the loops it is in
    pub free_label: usize,
    pub loop_entries: HashMap<usize, Node<'a>>, // label => first node of the corresponding loop
    pub loop_starting_at: HashMap<Node<'a>, usize>, // node => label of the loops starting at that node, if it exists
}

impl<'a, 'b> NodeLoops<'a, 'b> {
    pub fn new(graph: &'b Graph<'a>) -> NodeLoops<'a, 'b> {
        let mut labels: HashMap<Node<'a>, HashSet<usize>> = HashMap::new();
        for node in graph.get_all_nodes() {
            labels.insert(node.clone(), HashSet::new());
        }
        return NodeLoops {
            graph: graph,
            visited: HashSet::new(),
            parent_of: HashMap::new(),
            current_parents: HashSet::new(),
            current_loop_origins: HashSet::new(),
            labels: labels,
            free_label: 0,
            loop_entries: HashMap::new(),
            loop_starting_at: HashMap::new(),
        };
    }

    pub fn from(graph: &'b Graph<'a>) -> NodeLoops<'a, 'b> {
        let mut node_loops: NodeLoops = NodeLoops::new(graph);
        let initial_node: Node = graph.get_block(0).get_nodes()[0].clone();
        node_loops.explore_dfs(None, initial_node);
        return node_loops;
    }

    pub fn explore_dfs(&mut self, prev_node: Option<Node<'a>>, current_node: Node<'a>) {
        if self.visited.contains(&current_node) {
            let _prev_node: Node = prev_node.unwrap();
            if self.current_parents.contains(&current_node) {
                self.on_loop_found(_prev_node.clone(), current_node.clone());
            }
            self.on_junction_found(_prev_node.clone(), current_node.clone());
        } else {
            self.visited.insert(current_node.clone());
            self.parent_of.insert(current_node.clone(), prev_node);
            self.current_parents.insert(current_node.clone());

            for child in current_node.get_children() {
                self.explore_dfs(Some(current_node.clone()), child);
            }
            self.current_parents.remove(&current_node);
            if let Some(label) = self.loop_starting_at.get(&current_node) {
                self.current_loop_origins.remove(label);
            }
        }
    }

    pub fn on_loop_found(&mut self, last_node: Node<'a>, first_node: Node<'a>) {
        if let Some(label_already_starting_at_first_node) = self.loop_starting_at.get(&first_node) {
            if self
                .current_loop_origins
                .contains(&label_already_starting_at_first_node)
            {
                return;
            }
        }

        let label: usize = self.free_label;
        self.free_label += 1;

        self.loop_entries.insert(label, first_node.clone());
        assert!(!self.loop_starting_at.contains_key(&first_node));
        self.loop_starting_at.insert(first_node.clone(), label);
        self.current_loop_origins.insert(label);

        let mut moving_node: Node = last_node;
        loop {
            if moving_node == first_node {
                self.add_label(&moving_node, label);
                break;
            }
            if let Some(other_label) = self.loop_starting_at.get(&moving_node) {
                for other_node in self.get_nodes_with_label(*other_label) {
                    self.add_label(&other_node, label);
                }
            } else {
                self.add_label(&moving_node, label);
            }
            moving_node = self.get_parent(&moving_node);
        }
    }

    pub fn on_junction_found(&mut self, prev_node: Node<'a>, common_node: Node<'a>) {
        let joining_labels: HashSet<usize> = self
            .current_loop_origins
            .intersection(&self.labels[&common_node])
            .cloned()
            .collect();

        for label in joining_labels {
            let mut moving_node: Node = prev_node.clone();
            loop {
                if self.labels[&moving_node].contains(&label) {
                    break;
                }
                self.add_label(&moving_node, label);
                moving_node = self.get_parent(&moving_node);
            }
        }
    }

    fn get_nodes_with_label(&self, label: usize) -> HashSet<Node<'a>> {
        // return all the nodes of the graph with label `label`
        // we do a DFS with imperative programming because closures do not allow recursion

        let mut matching_nodes: HashSet<Node<'a>> = HashSet::new();
        let _first_node: Node<'a> = self.loop_entries[&label].clone();
        let mut nodes_to_explore: Vec<Node> = vec![_first_node];
        while nodes_to_explore.len() > 0 {
            let current_node: Node = nodes_to_explore.pop().unwrap();
            matching_nodes.insert(current_node.clone());
            for child in current_node.get_children() {
                if !matching_nodes.contains(&child) && self.labels[&child].contains(&label) {
                    nodes_to_explore.push(child);
                }
            }
        }
        return matching_nodes;
    }

    pub fn get_parent(&self, node: &Node<'a>) -> Node<'a> {
        return self.parent_of[&node].as_ref().unwrap().clone();
    }

    pub fn add_label(&mut self, node: &Node<'a>, label: usize) {
        assert!(!self.labels.get_mut(node).unwrap().contains(&label));
        self.labels.get_mut(node).unwrap().insert(label);
    }

    pub fn get_labels_at_block(&self, block: &Block<'a>) -> HashSet<usize> {
        let mut labels: HashSet<usize> = HashSet::new();
        for node in block.get_nodes() {
            labels.extend(&self.labels[&node]);
        }
        return labels;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{bytecode_reader::bytecode::Bytecode, research_and_development::gml::nodes_to_gml};
    use std::fs;

    #[test]
    pub fn draw_node_loops_graph() {
        // R&D
        let path: String = String::from("./contracts/simple/contract_0/");
        let bytecode_string: String =
            fs::read_to_string(path.clone() + "bytecode.txt").expect("Unable to read file.");
        let bytecode: Bytecode = Bytecode::from(&bytecode_string).unwrap();
        let graph: Graph = Graph::from(&bytecode);
        let node_loops: NodeLoops = NodeLoops::from(&graph);
        let mut tags: HashMap<Node, String> = HashMap::new();
        for (node, loop_labels) in &node_loops.labels {
            let mut sorted_loop_labels: Vec<usize> = loop_labels.iter().cloned().collect();
            sorted_loop_labels.sort();
            let mut tag: String = String::new();
            tag += "[";
            for label in sorted_loop_labels {
                tag += &format!("{} ", label);
            }
            tag += "]";
            tags.insert(node.clone(), tag);
        }
        let node_loops_gml: String = nodes_to_gml(&graph, tags);
        let _ = node_loops_gml;
        //crate::tools::utils::write_file("node_loops_graph.gml", &node_loops_gml);
    }
}
