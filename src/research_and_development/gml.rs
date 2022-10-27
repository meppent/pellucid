use crate::{
    create_graph::{block::Block, graph::Graph, node::Node},
    tools::utils::usize_to_hex,
};
use std::collections::HashMap;

pub fn blocks_to_gml<'a>(graph: &Graph<'a>, tags: HashMap<Block<'a>, String>) -> String {
    let mut gml: String = String::new();
    gml += "graph\n[\n";

    //nodes
    let mut blocks: Vec<Block> = graph.get_all_blocks().iter().cloned().collect();
    blocks.sort_by_key(|block: &Block| block.get_pc_start());

    for block in blocks {
        gml += "  node\n  [\n  id ";
        gml += &block.get_pc_start().to_string();
        gml += "\n  label \"pc_start: ";
        gml += &usize_to_hex(block.get_pc_start());
        gml += "\"";
        if let Some(tag) = tags.get(&block) {
            gml += &format!("\n  tag \"{}\"\n", tag);
        }
        gml += "\n  ]\n";
    }

    // edges
    let mut edges: Vec<(usize, usize)> = graph.get_edges();
    edges.sort_by_key(|(pc_start_origin, pc_start_dest): &(usize, usize)| {
        *pc_start_origin * 100_000 + pc_start_dest
    });
    for (origin_pc_start, dest_pc_start) in edges {
        gml += "  edge\n  [\n  source ";
        gml += &origin_pc_start.to_string();
        gml += "\n  target ";
        gml += &dest_pc_start.to_string();
        gml += "\n  ]\n";
    }
    gml += "]";
    return gml;
}

pub fn nodes_to_gml<'a>(graph: &'a Graph<'a>, tags: HashMap<Node<'a>, String>) -> String {
    let mut nodes_str: String = String::new();
    let mut edges_str: String = String::new();

    let get_node_label = |node: Node| -> String {
        let mut res: String = String::new();
        res += "pc start: ";
        res += &usize_to_hex(node.get_block().get_pc_start());
        res += ", index ";
        res += &node.get_index_in_block().to_string();
        return res;
    };

    let mut node_ids: HashMap<Node, usize> = HashMap::new();
    let mut current_id: usize = 0;
    let mut get_node_id = |node: Node<'a>| -> String {
        if !node_ids.contains_key(&node) {
            node_ids.insert(node.clone(), current_id);
            current_id += 1;
        }
        return node_ids[&node].to_string();
    };

    for block in &graph.get_all_blocks() {
        for node in block.get_nodes() {
            let node_id: String = get_node_id(node.clone());
            nodes_str += &format!(
                "  node\n  [\n  id {}\n  label \"{}\"\n",
                node_id,
                get_node_label(node.clone())
            );
            if let Some(tag) = tags.get(&node) {
                nodes_str += &format!("  tag \"{}\"\n", tag);
            }
            nodes_str += "  ]\n";
            for child in node.get_children() {
                let child_id: String = get_node_id(child.clone());
                edges_str += &format!(
                    "  edge\n  [\n  source {} \n  target {}\n  ]\n",
                    node_id, child_id
                );
            }
        }
    }

    let mut gml: String = String::from("graph\n[\n");
    gml += &nodes_str;
    gml += &edges_str;
    gml += "]";
    return gml;
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::bytecode_reader::bytecode::Bytecode;

    use std::fs;

    #[test]
    pub fn small_test() {
        let bytecode_string: String =
            fs::read_to_string("./contracts/simple/contract_0/bytecode.txt")
                .expect("Unable to read file.");
        let bytecode_test: Bytecode = Bytecode::from(&bytecode_string).unwrap();
        let graph = Graph::from(&bytecode_test);
        let gml = nodes_to_gml(&graph, HashMap::new());
        let _ = gml;
        //crate::tools::utils::write_file("node_graph.gml", &gml);
    }
}
