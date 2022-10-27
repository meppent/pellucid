use super::node_loops::NodeLoops;
use crate::create_graph::{block::Block, graph::Graph, node::Node};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub struct BlockLoops {
    // TODO consider the duplications due to looping blocks (at this point one pc start can represent >= 2 blocks, even though it's unlikely)
    pub pc_start_to_labels: HashMap<usize, HashSet<usize>>,
    pub label_to_pc_starts: HashMap<usize, HashSet<usize>>,
    pub label_to_entry_pc_start: HashMap<usize, usize>,
    pub entry_pc_start_to_label: HashMap<usize, usize>, // the blocks that are not the initial block of a loop have no entries
}

impl<'a> BlockLoops {
    fn new() -> BlockLoops {
        return BlockLoops {
            pc_start_to_labels: HashMap::new(),
            label_to_pc_starts: HashMap::new(),
            label_to_entry_pc_start: HashMap::new(),
            entry_pc_start_to_label: HashMap::new(),
        };
    }

    pub fn from<'b>(graph: &'b Graph<'a>) -> BlockLoops {
        let node_loops: NodeLoops<'a, 'b> = NodeLoops::from(&graph);
        let mut block_loops: BlockLoops = BlockLoops::new();
        for node in node_loops.graph.get_all_nodes() {
            for label in &node_loops.labels[&node] {
                block_loops.add_label_to_pc_start(node.get_block().get_pc_start(), *label);
            }
        }
        for label in 0..node_loops.free_label {
            block_loops.add_loop_entry(Self::compute_loop_entry_block(&node_loops, label), label);
        }
        block_loops.merge_duplicates();
        return block_loops;
    }

    pub fn get_labels(&self) -> Vec<usize> {
        return self.label_to_pc_starts.keys().cloned().collect_vec();
    }

    fn compute_loop_entry_block<'b>(node_loops: &NodeLoops<'a, 'b>, label: usize) -> usize {
        let mut moving_node: Node = node_loops.loop_entries[&label].clone();
        loop {
            let parent_block: Block = node_loops.parent_of[&moving_node]
                .as_ref()
                .unwrap()
                .get_block();
            if !node_loops
                .get_labels_at_block(&parent_block)
                .contains(&label)
            {
                return moving_node.get_block().get_pc_start();
            }
            moving_node = node_loops.get_parent(&moving_node);
        }
    }

    pub fn add_label_to_pc_start(&mut self, pc_start: usize, label: usize) {
        if !self.label_to_pc_starts.contains_key(&label) {
            self.label_to_pc_starts.insert(label, HashSet::new());
        }
        if !self.pc_start_to_labels.contains_key(&pc_start) {
            self.pc_start_to_labels.insert(pc_start, HashSet::new());
        }
        self.pc_start_to_labels
            .get_mut(&pc_start)
            .unwrap()
            .insert(label);
        self.label_to_pc_starts
            .get_mut(&label)
            .unwrap()
            .insert(pc_start);
    }

    pub fn add_loop_entry(&mut self, pc_start: usize, label: usize) {
        assert!(!self.label_to_entry_pc_start.contains_key(&label));
        self.label_to_entry_pc_start.insert(label, pc_start);
        self.entry_pc_start_to_label.insert(pc_start, label);
    }

    fn merge_2_loops(&mut self, label_from: usize, label_to: usize) {
        assert!(
            self.label_to_entry_pc_start[&label_from] == self.label_to_entry_pc_start[&label_to],
            "Can only merge loops that start at the same block"
        );
        for pc_start in self.label_to_pc_starts[&label_from].clone() {
            self.pc_start_to_labels
                .get_mut(&pc_start)
                .unwrap()
                .remove(&label_from);
            self.pc_start_to_labels
                .get_mut(&pc_start)
                .unwrap()
                .insert(label_to);
            self.label_to_pc_starts
                .get_mut(&label_to)
                .unwrap()
                .insert(pc_start);
        }
        self.label_to_pc_starts.remove(&label_from);
        self.label_to_entry_pc_start.remove(&label_from);
        self.entry_pc_start_to_label
            .insert(self.label_to_entry_pc_start[&label_to], label_to); // TODO (ugly + difficult to understand)
    }

    fn merge_duplicates(&mut self) {
        // merge the loops that start at the same block
        let mut labels: Vec<usize> = self.get_labels();
        labels.sort();
        let mut i = 0;
        while i < labels.len() {
            let mut j = i + 1;
            while j < labels.len() {
                if self.label_to_entry_pc_start[&labels[i]]
                    == self.label_to_entry_pc_start[&labels[j]]
                {
                    // loops share the same entry block
                    if !self.label_to_pc_starts[&labels[i]]
                        .is_superset(&self.label_to_pc_starts[&labels[j]])
                    {
                        log::warn!("WARNING: strange loop");
                    }
                    self.merge_2_loops(labels[j], labels[i]);
                    labels.remove(j);
                } else {
                    j += 1;
                }
            }
            i += 1;
        }
    }

    pub fn get_labels_at_pc_start(&self, pc_start: usize) -> HashSet<usize> {
        if let Some(labels) = self.pc_start_to_labels.get(&pc_start) {
            return labels.clone();
        } else {
            return HashSet::new();
        }
    }

    pub fn get_free_label(&self) -> usize {
        let labels: Vec<usize> = self.get_labels();
        if labels.is_empty() {
            return 0;
        } else {
            return labels.iter().max().unwrap() + 1;
        }
    }

    pub fn get_labels_at_block(&self, block: &Block) -> HashSet<usize> {
        if let Some(labels) = self.pc_start_to_labels.get(&block.get_pc_start()) {
            return labels.clone();
        } else {
            return HashSet::new();
        }
    }

    pub fn block_is_in_loop(&self, block: &Block, label: usize) -> bool {
        return self.label_to_pc_starts[&label].contains(&block.get_pc_start());
    }

    pub fn has_loop_starting_at(&self, block: &Block) -> bool {
        return self
            .entry_pc_start_to_label
            .contains_key(&block.get_pc_start());
    }

    pub fn get_label_of_entry(&self, entry: &Block) -> Option<usize> {
        return self
            .entry_pc_start_to_label
            .get(&entry.get_pc_start())
            .cloned();
    }

    pub fn get_pc_start_entry_for_label(&self, label: usize) -> usize {
        return self.label_to_entry_pc_start[&label];
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::create_graph::graph::Graph;
    use crate::{bytecode_reader::bytecode::Bytecode, tools::utils::read_file};

    #[test]
    pub fn test_loop_detection() {
        for (contract_index, n_loops) in [(0, 1), (1, 2), (2, 3)] {
            let path: String = format!(
                "./contracts/loop/contract_{}/bytecode.txt",
                contract_index
            );
            let bytecode: Bytecode = Bytecode::from(&read_file(&path)).unwrap();
            let graph: Graph = Graph::from(&bytecode);
            let block_loops: BlockLoops = BlockLoops::from(&graph);
            assert!(block_loops.get_labels().len() == n_loops);
        }
    }
}
