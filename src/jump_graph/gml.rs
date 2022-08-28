use crate::bytecode_reader::bytecode::stringify_vopcodes;

use super::graph::BlockSet;

pub fn to_gml(block_set: &BlockSet) -> String {
    let mut gml: String = String::new();
    gml.push_str("graph\n[\n");

    //nodes
    for block in block_set.get_blocks() {
        gml.push_str("\tnode\n\t[\n\tid ");
        gml.push_str(&block.get_pc_start().to_string());
        gml.push_str("\n\tlabel \"");
        let bytecode_str = stringify_vopcodes(block.code);
        gml.push_str(&bytecode_str.replace("\n", "\\n"));
        gml.push_str("\"\n\t]\n");
    }

    // edges
    for (origin_pc_start, dest_pc_start) in block_set.get_edges().iter() {
        gml.push_str("\tedge\n\t[\n\tsource ");
        gml.push_str(&origin_pc_start.to_string());
        gml.push_str("\n\ttarget ");
        gml.push_str(&dest_pc_start.to_string());
        gml.push_str("\n\tlabel \"edge label (TO DO)\"\n\t]\n");
    }
    gml.push_str("]");
    return gml;
}
