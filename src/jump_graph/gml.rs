use crate::bytecode_reader::bytecode::stringify_vopcodes;

use super::{block_set::{ BlockSet}, block::Block};

pub fn to_gml(block_set: & BlockSet) -> String {
    let mut gml: String = String::new();
    gml.push_str("graph\n[\n");

    //nodes
    let mut blocks: Vec<Block> = block_set.get_blocks();
    blocks.sort_by_key(|block: &Block| block.get_pc_start());

    for block in blocks {
        gml.push_str("  node\n  [\n  id ");
        gml.push_str(&block.get_pc_start().to_string());
        gml.push_str("\n  label \"");
        let bytecode_str = stringify_vopcodes(block.code);
        gml.push_str(&bytecode_str.replace("\n", "\\n"));
        gml.push_str("\"\n  ]\n");
    }

    // edges
    let mut edges: Vec<(usize, usize)> = block_set.get_edges();
    edges.sort_by_key(|(pc_start_origin, pc_start_dest): &(usize, usize)| {
        *pc_start_origin * 100_000 + pc_start_dest
    });
    for (origin_pc_start, dest_pc_start) in edges {
        gml.push_str("  edge\n  [\n  source ");
        gml.push_str(&origin_pc_start.to_string());
        gml.push_str("\n  target ");
        gml.push_str(&dest_pc_start.to_string());
        gml.push_str("\n  label \"edge label (TO DO)\"\n  ]\n");
    }
    gml.push_str("]");
    return gml;
}


