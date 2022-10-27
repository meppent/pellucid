use crate::create_graph::{block::Block, node::Node};
use std::collections::{HashMap, HashSet};

use super::{
    compute_block_locations::compute_blocks_organisation,
    drawing_tools::{Color, Coord, Dims, Drawing},
    grid::Grid,
    specs::{
        BLOCK_WIDTH, DEFAULT_BLOCK_COLOR, DEFAULT_CONNECTIONS_COLOR, DISPLAY_BYTECODE,
        EXTERNAL_CONNECTIONS_COUNT_COLOR, LEFT_PADDING, TEXT_SIZE, TOP_PADDING,
        X_SPACE_BETWEEN_BLOCKS, Y_SPACE_BETWEEN_BLOCKS,
    },
};

fn precompute_image_dims<'a>(
    grid: &Grid,
    index_to_block: &HashMap<usize, Block<'a>>,
) -> (i32, i32) {
    let mut height: i32 = TOP_PADDING as i32;
    for y in 0..grid.get_height() {
        let max_n_lines: i32 = if DISPLAY_BYTECODE {
            grid.get_indexes_at_y(y)
                .iter()
                .map(|index| index_to_block[&index].get_code().len())
                .max()
                .unwrap() as i32
        } else {
            1
        };
        height += Y_SPACE_BETWEEN_BLOCKS as i32 + TEXT_SIZE as i32 * max_n_lines;
    }
    return (
        grid.get_width() as i32 * (X_SPACE_BETWEEN_BLOCKS + BLOCK_WIDTH) as i32,
        (height * 10) / 9,
    );
}

fn draw_block<'a>(
    drawing: &mut Drawing,
    block: &Block<'a>,
    start: Coord,
    node_up_coords: &mut HashMap<Node<'a>, Coord>,
    node_down_coords: &mut HashMap<Node<'a>, Coord>,
    color: Color,
) -> Dims {
    let bytecode_lines: Vec<String> = block
        .get_code()
        .iter()
        .map(|vopcode| vopcode.to_string())
        .collect();
    let block_content: String = if DISPLAY_BYTECODE {
        bytecode_lines.join("\n")
    } else {
        bytecode_lines[0].to_owned()
    };
    let block_dims: Dims =
        drawing.draw_boxed_text(&block_content, start, BLOCK_WIDTH, color, TEXT_SIZE);
    for (index, node) in block.get_nodes().iter().enumerate() {
        let x: f32 =
            start.x + block_dims.width * (1. + index as f32) / (block.nodes_count() + 1) as f32;

        let up_y: f32 = start.y;
        let down_y: f32 = start.y + block_dims.height;
        node_up_coords.insert(node.clone(), Coord { x, y: up_y });
        node_down_coords.insert(node.clone(), Coord { x, y: down_y });
    }
    return block_dims;
}

pub fn draw_blocks<'a>(
    blocks: &HashSet<Block<'a>>,
    path: &str,
    block_colors: HashMap<Block<'a>, Color>,
    focus_on_block: Option<Block<'a>>,
) {
    let (grid, index_to_block): (Grid, HashMap<usize, Block<'a>>) =
        compute_blocks_organisation(blocks);
    let (image_width, image_height): (i32, i32) = precompute_image_dims(&grid, &index_to_block);

    let mut drawing: Drawing = Drawing::new(image_width, image_height);

    let mut node_up_coords: HashMap<Node, Coord> = HashMap::new();
    let mut node_down_coords: HashMap<Node, Coord> = HashMap::new();
    let mut current_y: f32 = TOP_PADDING;
    for y in 0..grid.get_height() {
        let mut next_y: f32 = current_y;
        for (index, (x, _)) in grid.get_sorted_indexes_at_y(y) {
            let block: &Block = &index_to_block[&index];
            let start = Coord {
                x: LEFT_PADDING + x as f32 * X_SPACE_BETWEEN_BLOCKS,
                y: current_y,
            };
            let color: Color = if let Some(_color) = block_colors.get(block) {
                *_color
            } else {
                DEFAULT_BLOCK_COLOR
            };
            let dims: Dims = draw_block(
                &mut drawing,
                block,
                start,
                &mut node_up_coords,
                &mut node_down_coords,
                color,
            );
            let down_right_corner: Coord = start + dims;
            next_y = next_y.max(Y_SPACE_BETWEEN_BLOCKS + down_right_corner.y);
        }

        current_y = next_y;
    }
    draw_connections(
        &mut drawing,
        blocks,
        &node_up_coords,
        &node_down_coords,
        focus_on_block,
    );
    drawing.save_image(path);
}

fn draw_connections<'a>(
    drawing: &mut Drawing,
    blocks: &HashSet<Block<'a>>,
    node_up_coords: &HashMap<Node<'a>, Coord>,
    node_down_coords: &HashMap<Node<'a>, Coord>,
    focus_on_block: Option<Block<'a>>,
) {
    // the color we use to connect 2 nodes
    let mut special_nodes_to_color: HashMap<Node<'a>, Color> = HashMap::new();
    if let Some(block) = focus_on_block.as_ref() {
        for node in block.get_nodes() {
            special_nodes_to_color.insert(node, Color::new_random(255));
        }
    }

    // drawconnections
    for parent_block in blocks {
        for parent_node in parent_block.get_nodes() {
            for child_node in parent_node.get_children() {
                if blocks.contains(&child_node.get_block()) {
                    let down_coord: Coord = node_down_coords[&parent_node];
                    let up_coord: Coord = node_up_coords[&child_node];
                    let mut connection_color: Color;
                    if let Some(color) = special_nodes_to_color.get(&parent_node) {
                        connection_color = *color;
                    } else if let Some(color) = special_nodes_to_color.get(&child_node) {
                        connection_color = *color;
                    } else {
                        connection_color = DEFAULT_CONNECTIONS_COLOR;
                    }
                    if up_coord.y < down_coord.y {
                        // ascending connection
                        connection_color = connection_color.dim();
                    }
                    drawing.draw_line(down_coord, up_coord, connection_color);
                }
            }
        }
    }

    // display the number of external connections
    let mut all_nodes: HashSet<Node> = HashSet::new();
    for block in blocks {
        all_nodes.extend(block.get_nodes());
    }
    for block in blocks {
        for node in block.get_nodes() {
            let n_external_parents: usize = HashSet::from_iter(node.get_parents())
                .difference(&all_nodes)
                .count();
            let n_external_children: usize = HashSet::from_iter(node.get_children())
                .difference(&all_nodes)
                .count();

            if n_external_parents > 0 {
                let mut start: Coord = node_up_coords[&node];
                start.x += 20.;
                start.y -= 10. + TEXT_SIZE;
                drawing.draw_text(
                    &format!("+ {}", n_external_parents),
                    start,
                    usize::MAX,
                    EXTERNAL_CONNECTIONS_COUNT_COLOR,
                    TEXT_SIZE,
                )
            }

            if n_external_children > 0 {
                let mut start: Coord = node_down_coords[&node];
                start.x += 20.;
                start.y += 10.;
                drawing.draw_text(
                    &format!("+ {}", n_external_children),
                    start,
                    usize::MAX,
                    EXTERNAL_CONNECTIONS_COUNT_COLOR,
                    TEXT_SIZE,
                )
            }
        }
    }
}
