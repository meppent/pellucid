use crate::{
    create_graph::{block::Block, graph::Graph, node::Node},
    tools::graph_tools::clear_orphan_nodes,
};
use std::collections::HashSet;

pub fn remove_looping_blocks<'a>(graph: &mut Graph<'a>) {
    'main: loop {
        for mut parent_node in graph.get_all_nodes() {
            for child_node in parent_node.get_children() {
                if child_node.get_block() == parent_node.get_block() {
                    // we found a looping block
                    let block: Block<'a> = parent_node.get_block();
                    let mut visited_nodes: HashSet<Node<'a>> = HashSet::new();
                    'find_entry: while parent_node.get_block_parents().contains(&block) {
                        // in case there are multiple loops in the block
                        for grand_parent_node in parent_node.get_parents() {
                            if grand_parent_node.get_block() == block {
                                if visited_nodes.contains(&parent_node) {
                                    handle_infinite_loop_block(&block);
                                    break 'find_entry;
                                }
                                visited_nodes.insert(parent_node.clone());
                                parent_node = grand_parent_node;
                                continue 'find_entry;
                            }
                        }
                        unreachable!();
                    }
                    let duplicated_block: Block<'a> = graph.duplicate_block(&block);
                    parent_node.switch_block(&duplicated_block);
                    log::debug!("Duplicated a looping block");
                    continue 'main;
                }
            }
        }
        break 'main;
    }
}

fn handle_infinite_loop_block(block: &Block) {
    // TODO find a better way
    log::warn!("Infinite looping block");
    for parent_node in block.get_nodes() {
        for child_node in parent_node.get_children() {
            if &child_node.get_block() == block {
                parent_node.remove_child(child_node.clone());
                child_node.remove_parent(parent_node.clone());
                clear_orphan_nodes(&child_node);
            }
        }
    }
}
