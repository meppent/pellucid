use super::{cut_cycles::remove_cycles, grid::Grid, specs::HORYZONTAL_IMPROVEMENT_N_ITERS};
use crate::create_graph::block::Block;
use std::collections::{HashMap, HashSet};
use toposort_scc::IndexGraph;

pub fn compute_blocks_organisation<'a>(
    blocks: &HashSet<Block<'a>>,
) -> (Grid, HashMap<usize, Block<'a>>) {
    let mut block_to_index: HashMap<Block<'a>, usize> = HashMap::new();
    let mut index_to_block: HashMap<usize, Block<'a>> = HashMap::new();

    for (index, block) in blocks.iter().enumerate() {
        block_to_index.insert(block.clone(), index);
        index_to_block.insert(index, block.clone());
    }

    let mut adjacency_list: Vec<Vec<usize>> = Vec::new();

    for index in 0..blocks.len() {
        let mut child_indexes: Vec<usize> = Vec::new();
        for child in index_to_block[&index].get_child_blocks() {
            if blocks.contains(&child) {
                child_indexes.push(block_to_index[&child]);
            }
        }
        adjacency_list.push(child_indexes);
    }
    remove_cycles(&mut adjacency_list);

    let sorted_indexes = IndexGraph::from_adjacency_list(&adjacency_list)
        .toposort_or_scc()
        .expect("Seems that the graph has some cycles...");

    let mut grid: Grid = Grid::from(vec![sorted_indexes]);

    let mut y: usize = grid.get_height();
    while y > 1 {
        y -= 1;
        for index in grid.get_indexes_at_y(y) {
            if grid.get_indexes_at_y(y - 1).iter().all(|up_index| {
                !adjacency_list[*up_index].contains(&index) // TODO opti with hashset here
            }) {
                grid.move_up(index);
                y = grid.get_height();
                break;
            }
        }
    }

    for _ in 0..HORYZONTAL_IMPROVEMENT_N_ITERS {
        let mut index_to_x_sum: HashMap<usize, f32> = HashMap::new();
        let mut index_to_div_coef: HashMap<usize, f32> = HashMap::new();

        for index in grid.iter_indexes() {
            index_to_x_sum.insert(index, 0.1 * grid.get_x_of_index(index) as f32);
            index_to_div_coef.insert(index, 0.1);
        }

        for index in grid.iter_indexes() {
            for child_index in &adjacency_list[index] {
                for (index_a, index_b, importance) in
                    [(index, *child_index, 1.), (*child_index, index, 2.)]
                {
                    *index_to_x_sum.get_mut(&index_a).unwrap() +=
                        grid.get_x_of_index(index_b) as f32 * importance;
                    *index_to_div_coef.get_mut(&index_a).unwrap() += importance;
                }
            }
        }

        let mut index_to_neighbour_average_x: HashMap<usize, f32> = HashMap::new();
        for index in grid.iter_indexes() {
            index_to_neighbour_average_x.insert(
                index,
                index_to_x_sum[&index] as f32 / index_to_div_coef[&index] as f32,
            );
        }

        for y in 0..grid.get_height() {
            let mut naverage_x_and_indexes: Vec<(f32, usize)> = Vec::new();
            for index in grid.get_indexes_at_y(y) {
                naverage_x_and_indexes.push((index_to_neighbour_average_x[&index], index));
            }
            naverage_x_and_indexes.sort_by_key(|(avg_x, _)| *avg_x as i32);

            let mut left_x: isize = -1;
            for (avg_x, index) in &naverage_x_and_indexes {
                let x: usize = (*avg_x as usize).max((left_x + 1) as usize);
                left_x = x as isize;
                grid.move_index(*index, (x, y)); // TODO avoid this temporary index override
            }
        }
    }

    grid.remove_empty_bands();
    grid.update_dims();
    return (grid, index_to_block);
}
