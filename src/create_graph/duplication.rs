use crate::{
    create_graph::{block::Block, node::Node},
    detect_cycles::acyclic_graph::AcyclicGraph,
    tools::graph_tools::{clear_orphan_nodes, node_dfs},
};
use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    rc::Rc,
};

pub fn duplicate_block_connection<'a, 'b>(
    a_graph: &mut AcyclicGraph<'a, 'b>,
    parent_block: &Block<'a>,
    child_block: &Block<'a>,
) {
    // create another 'child block', move all the connections comming from 'parent_block'
    // there, and and generate following blocks starting from these nodes
    assert!(parent_block.get_child_blocks().contains(child_block));

    let mut initial_nodes: HashSet<Node<'a>> = HashSet::new();
    for parent_node in parent_block.get_nodes() {
        for child_node in parent_node.get_children() {
            if &child_node.get_block() == child_block {
                initial_nodes.insert(child_node);
            }
        }
    }
    let offspring_blocks: HashSet<Block<'a>> = get_offspring_blocks(&initial_nodes);
    let mut duplicated_blocks: HashMap<Block<'a>, Block<'a>> = HashMap::new(); // existing block => new (duplicated) block
    for block in offspring_blocks {
        duplicated_blocks.insert(block.clone(), a_graph.duplicate_block(&block));
    }

    let mut duplicated_nodes: HashMap<Node<'a>, Node<'a>> = HashMap::new(); // existing node => new (duplicated) node

    /*
    ┌──────────────┐      V
    │              │      │
    │     ┌――――――――┴―┐  ┌―┴――――――――┐
    │     |  other   |  |  parent  |
    │     |  block   |  |  block   |
    │     └――――――――┬―┘  └―┬――――――――┘
    │              │      │
    │            ┌―┴――――――┴―┐
    │            |  child   |
    │            |  block   |
    │            └―┬――――――┬―┘
    └──────────────│──────┘
                   │
                   V

    When duplicating node connections in the dfs, 'child_block' is duplicated a second time in case of cycling
    branches. The first duplicate 'child_block' only concerns the incoming connections that originally came from
    'parent_block'.

    */

    let mut re_duplicated_child_block: Option<Block<'a>> = None;
    let mut re_duplicated_nodes_in_child_blocks: HashMap<Node<'a>, Node<'a>> = HashMap::new(); // all (node) keys are in 'child_block'

    let dfs_beginning: Rc<RefCell<bool>> = Rc::new(RefCell::new(true));

    let mut dup_connection = |parent_node: &Node<'a>, child_node: &Node<'a>| {
        if *RefCell::borrow(&dfs_beginning) {
            assert!(initial_nodes.contains(parent_node));
            assert!(!initial_nodes.contains(child_node));
        }
        let (mut duplicated_parent_node, mut duplicated_child_node): (
            Option<Node<'a>>,
            Option<Node<'a>>,
        ) = (None, None);
        for (node, duplicated_node) in [
            (parent_node, &mut duplicated_parent_node),
            (child_node, &mut duplicated_child_node),
        ] {
            let on_again_duplicated_child_block: bool =
                !*RefCell::borrow(&dfs_beginning) && node.get_block() == *child_block;
            if on_again_duplicated_child_block && re_duplicated_child_block == None {
                re_duplicated_child_block =
                    Some(a_graph.duplicate_block(&duplicated_blocks[child_block]));
                log::debug!("Child block :{:?} has been duplicated again.", child_block);
            }

            let duplicated_block: &Block<'a> = if on_again_duplicated_child_block {
                re_duplicated_child_block.as_ref().unwrap()
            } else {
                &duplicated_blocks[&node.get_block()]
            };

            let dupplication_table: &mut HashMap<Node, Node> = if on_again_duplicated_child_block {
                &mut re_duplicated_nodes_in_child_blocks
            } else {
                &mut duplicated_nodes
            };
            if !dupplication_table.contains_key(node) {
                dupplication_table.insert(
                    node.clone(),
                    Node::create_and_attach(duplicated_block.clone(), node.clone_initial_context()),
                );
            }
            *duplicated_node = Some(dupplication_table[node].clone());
        }
        let (duplicated_parent_node, duplicated_child_node): (Node<'a>, Node<'a>) = (
            duplicated_parent_node.unwrap(),
            duplicated_child_node.unwrap(),
        );

        if !duplicated_parent_node
            .get_children()
            .contains(&duplicated_child_node)
        {
            assert!(duplicated_child_node.get_block() != duplicated_blocks[child_block]);
            duplicated_parent_node.add_child(duplicated_child_node.clone());
        }
        RefCell::replace_with(&dfs_beginning, |_| false);
    };

    for node in &initial_nodes {
        RefCell::replace_with(&dfs_beginning, |_| true);
        node_dfs(node, |_| false, &mut dup_connection);
    }

    // replace all connections parent_block -> child_block by the corresponding one: parent_block -> duplicated_child_block
    for parent_node in parent_block.get_nodes() {
        for child_node in parent_node.get_children() {
            if &child_node.get_block() == child_block {
                let duplicated_child_node: Node<'a> = duplicated_nodes[&child_node].clone();
                a_graph.disconnect_nodes(parent_node.clone(), child_node.clone());
                parent_node.add_child(duplicated_child_node.clone());
            }
        }
    }

    // remove all the nodes that have no parents left
    let mut deleted_nodes: HashSet<Node> = HashSet::new();
    for node in &initial_nodes {
        if node.get_block().get_nodes().contains(node) {
            deleted_nodes.extend(clear_orphan_nodes(node));
        }
        // else: node was removed in a previous loop iteration, TODO find a better way
    }

    assert!(!parent_block.get_child_blocks().contains(child_block));
    assert!(
        duplicated_blocks[child_block].get_parent_blocks() == HashSet::from([parent_block.clone()])
    );
}

pub fn get_offspring_blocks<'a>(nodes: &HashSet<Node<'a>>) -> HashSet<Block<'a>> {
    let mut offspring_blocks: HashSet<Block<'a>> = HashSet::new();
    for node in nodes {
        let node_offspring: HashSet<Node> = node_dfs(node, |_| false, &mut |_, _| ()).visited_nodes;
        for n in &node_offspring {
            offspring_blocks.insert(n.get_block());
        }
    }
    return offspring_blocks;
}
