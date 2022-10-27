use itertools::Itertools;

use super::draw_blocks::draw_blocks;
use super::drawing_tools::Color;
use crate::create_graph::{block::Block, graph::Graph};
use crate::research_and_development::draw_graph_v2::specs::SPECIAL_BLOCK_COLOR;
use std::collections::{HashMap, HashSet};

pub fn draw_graph<'a>(graph: &Graph<'a>, path: &str) {
    draw_blocks(&graph.get_all_blocks(), path, HashMap::new(), None);
}

pub fn draw_block_area<'a>(
    blocks: &HashSet<Block<'a>>,
    area_depth: usize,
    focus_on_block: Option<Block<'a>>,
    path: &str,
) {
    let mut surrounding_blocks: HashSet<Block<'a>> = blocks.iter().cloned().collect();
    for _ in 0..area_depth {
        for block in surrounding_blocks.clone() {
            surrounding_blocks.extend(block.get_parent_blocks());
            surrounding_blocks.extend(block.get_child_blocks());
        }
    }
    draw_blocks(
        &surrounding_blocks,
        path,
        blocks
            .iter()
            .map(|block| (block.clone(), SPECIAL_BLOCK_COLOR))
            .collect(),
        focus_on_block,
    );
}

pub fn draw_block_groups<'a>(
    blocks_groups: &Vec<HashSet<Block<'a>>>,
    focus_on_block: Option<Block<'a>>,
    path: &str,
) {
    for (group_0, group_1) in blocks_groups.iter().tuple_combinations() {
        assert!(group_0.intersection(group_1).collect_vec().is_empty());
    }
    let mut block_colors: HashMap<Block<'a>, Color> = HashMap::new();
    let mut all_blocks: HashSet<Block<'a>> = HashSet::new();
    for group in blocks_groups {
        let color: Color = Color::new_random(255);
        for block in group {
            block_colors.insert(block.clone(), color);
            all_blocks.insert(block.clone());
        }
    }
    draw_blocks(&all_blocks, path, block_colors, focus_on_block);
}

pub fn draw_block_offspring<'a>(
    blocks: &HashSet<Block<'a>>,
    area_depth: usize,
    focus_on_block: Option<Block<'a>>,
    path: &str,
) {
    let mut surrounding_blocks: HashSet<Block<'a>> = blocks.iter().cloned().collect();
    for _ in 0..area_depth {
        for block in surrounding_blocks.clone() {
            surrounding_blocks.extend(block.get_child_blocks());
        }
    }
    draw_blocks(
        &surrounding_blocks,
        path,
        blocks
            .iter()
            .map(|block| (block.clone(), SPECIAL_BLOCK_COLOR))
            .collect(),
        focus_on_block,
    );
}
#[cfg(test)]
mod tests {
    use crate::{
        bytecode_reader::bytecode::Bytecode,
        create_graph::{block::Block, graph::Graph},
        tools::utils::{hex_to_usize, read_file},
    };

    #[test]
    fn test_draw_graph() {
        let raw_bytecode: String =
            read_file("./contracts/simple/contract_0/bytecode.txt");
        let bytecode: Bytecode = Bytecode::from(&raw_bytecode).unwrap();
        let graph: Graph = Graph::from(&bytecode);
        //super::draw_graph(&graph, "temp.png");
        let _ = graph;
    }
    #[test]
    fn test_draw_block_area() {
        let raw_bytecode: String =
            read_file("./contracts/simple/contract_0/bytecode.txt");
        let bytecode: Bytecode = Bytecode::from(&raw_bytecode).unwrap();
        let graph: Graph = Graph::from(&bytecode);
        let block: Block = graph.get_block(hex_to_usize("12d"));
        //super::draw_block_area(&HashSet::from([block.clone()]), 3, Some(block), "temp.png");
        let _ = block;
    }
}
