use crate::create_graph::{block::Block, node::Node};
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

pub struct DfsResult<'a> {
    pub visited_nodes: HashSet<Node<'a>>,
    pub stopped_nodes: HashSet<Node<'a>>,
}

pub fn pack_by_block<'a, A: Hash + PartialEq + Eq + Clone, F: Fn(&A) -> Block<'a>>(
    blocks: &HashSet<Block<'a>>,
    objects: HashSet<A>,
    corresponding_block: F,
) -> HashMap<Block<'a>, HashSet<A>> {
    let mut block_to_objects: HashMap<Block<'a>, HashSet<A>> =
        blocks.iter().map(|b| (b.clone(), HashSet::new())).collect();
    for object in objects {
        let block: Block<'a> = corresponding_block(&object);
        block_to_objects.get_mut(&block).unwrap().insert(object);
    }
    return block_to_objects;
}

pub fn node_dfs<'a, S: Fn(&Node<'a>) -> bool, A: FnMut(&Node<'a>, &Node<'a>) -> ()>(
    initial_node: &Node<'a>,
    stop_condition: S,
    on_connection: &mut A,
) -> DfsResult<'a> {
    let mut stopped_nodes: HashSet<Node<'a>> = HashSet::new();
    let mut visited_nodes: HashSet<Node<'a>> = HashSet::new();
    let mut nodes_to_visit: Vec<(Option<Node<'a>>, Node<'a>)> = vec![(None, initial_node.clone())]; // (parent, current)
    while let Some((parent, current)) = nodes_to_visit.pop() {
        if let Some(_parent) = &parent {
            on_connection(_parent, &current);
        }
        if visited_nodes.contains(&current) {
            continue;
        }
        visited_nodes.insert(current.clone());
        if stop_condition(&current) {
            stopped_nodes.insert(current.clone());
            continue;
        }
        for child in current.get_children() {
            nodes_to_visit.push((Some(current.clone()), child));
        }
    }
    return DfsResult {
        visited_nodes,
        stopped_nodes,
    };
}

pub fn clear_orphan_nodes<'a>(initial_node: &Node<'a>) -> HashSet<Node<'a>> {
    let mut deleted_nodes: HashSet<Node<'a>> = HashSet::new();
    let mut visited_nodes: HashSet<Node<'a>> = HashSet::new();
    let mut nodes_to_visit: Vec<Node<'a>> = vec![initial_node.clone()];
    while let Some(current) = nodes_to_visit.pop() {
        if visited_nodes.contains(&current) {
            continue;
        }
        visited_nodes.insert(current.clone());

        let children: Vec<Node> = current.get_children();
        if current.is_orphan() {
            for child in &children {
                child.remove_parent(current.clone());
                current.remove_child(child.clone());
            }
            current.get_block().remove_node(&current);
            deleted_nodes.insert(current);
        }
        for child in children {
            nodes_to_visit.push(child);
        }
    }
    return deleted_nodes;
}
