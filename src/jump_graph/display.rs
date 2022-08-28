use super::jump_graph::BlockSet;
use crate::{
    bytecode_reader::bytecode::Bytecode,
    gui::grid::{
        Grid, LineDirection, DOWN_LEFT_CHAR, DOWN_RIGHT_CHAR, HORYZONTAL_DOWN_CHAR,
        HORYZONTAL_UP_CHAR, TARGET_LEFT_CHAR, TARGET_RIGHT_CHAR, UP_LEFT_CHAR, UP_RIGHT_CHAR,
    },
    utils::{get_max_key, get_sorted_keys, iter_int, map_values_to_index, max_mapped_value},
};
use std::{
    collections::{HashMap, HashSet},
    usize,
};

pub const BLOCK_WIDTH: usize = 40;

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
enum Direction {
    RISING,
    DESCENDING,
}

fn rectangle_to_grid(pc_start: usize, pc_end: usize, bytecode: &Bytecode, label: &str) -> Grid {
    let mut grid: Grid = Grid::new_with_size(1, BLOCK_WIDTH - 2);
    grid.append_string(0, 0, label);

    let mut current_line: usize = 1;
    for vopcode in bytecode.iter(pc_start, pc_end) {
        let vopcode_str: String = vopcode.to_string();
        grid.append_string(current_line, 0, &vopcode_str[0..20.min(vopcode_str.len())]);
        current_line += 1;
    }
    grid.draw_outside_box();
    return grid;
}

/*
Understand the Link object:
┌――――――――┐
|        |
|index 0 |――――――┐
|        |―――┐  |
└――――――――┘   |  |
             |  |
┌――――――――┐   |  |
|        |<――┘  |
|index 1 |      |
|        |―――┐  |
└――――――――┘   |  |
             |  |
┌――――――――┐   |  |
|        |<――┘  |
|index 2 |   :  |
|        |<――:――┘
└――――――――┘   :  :
             :  :
    depth:   0  1

starting_links:  0 => { (1, 0),       (0 goes to 1 with depth 0)
                        (2, 1) }      (0 goes to 2 with depth 1)
                 1 => { (2, 0) }      (1 goes to 2 with depth 0)

starting_links:  1 => { (0, 0) }      (arrives to 1 from 0 with depth 0)
                 2 => { (1, 0),       (arrives to 2 from 1 with depth 0)
                        (0, 1) }      (arrives to 2 from 0 with depth 1)

depth_to_links:  0 => { (0, 1),       (at depth 0, 0 goes to 1)
                        (1, 2) }      (at depth 0, 1 goes to 2)
                 1 => { (0, 2) }      (at depth 1, 0 goes to 2)

In this example, direction == Direction::DESCENDING.
Al links must share the same direction, ie all links are descending, or all links are rising.

*/
struct Links {
    pub starting_links: HashMap<usize, HashSet<(usize, usize)>>,
    pub arriving_links: HashMap<usize, HashSet<(usize, usize)>>,
    pub depth_to_links: HashMap<usize, HashSet<(usize, usize)>>,
    pub direction: Direction,
}

impl Links {
    fn new(n_indexes: usize, direction: Direction) -> Self {
        return Links {
            starting_links: (0..n_indexes)
                .map(|i| (i, HashSet::<(usize, usize)>::new()))
                .collect(),
            arriving_links: (0..n_indexes)
                .map(|i| (i, HashSet::<(usize, usize)>::new()))
                .collect(),
            depth_to_links: HashMap::new(),
            direction,
        };
    }

    fn connect(&mut self, index_from: usize, index_to: usize) {
        match self.direction {
            Direction::DESCENDING => assert!(index_from < index_to),
            Direction::RISING => assert!(index_to < index_from),
        }
        let depth: usize = self.find_minimum_depth(index_from, index_to);
        self.starting_links
            .get_mut(&index_from)
            .unwrap()
            .insert((index_to, depth));
        self.arriving_links
            .get_mut(&index_to)
            .unwrap()
            .insert((index_from, depth));

        if !self.depth_to_links.contains_key(&depth) {
            self.depth_to_links.insert(depth, HashSet::new());
        }

        self.depth_to_links
            .get_mut(&depth)
            .unwrap()
            .insert((index_from, index_to));
    }

    fn find_minimum_depth(&self, index_from: usize, index_to: usize) -> usize {
        let mut max_depth: usize = 0;
        let mut used_depths: HashSet<usize> = HashSet::new();
        for index in iter_int(index_from, index_to) {
            for (_, depth) in &self.starting_links[&index] {
                max_depth = max_depth.max(*depth);
                used_depths.insert(*depth);
            }
        }
        for index in iter_int(index_to, index_from) {
            for (_, depth) in &self.arriving_links[&index] {
                max_depth = max_depth.max(*depth);
                used_depths.insert(*depth);
            }
        }
        for depth in 0..max_depth + 1 {
            if !used_depths.contains(&depth) {
                return depth;
            }
        }
        return max_depth + 1;
    }

    fn get_global_max_depth(&self) -> usize {
        return get_max_key(&self.depth_to_links).unwrap();
    }
}

struct RectangleSet<'a> {
    pub block_set: &'a BlockSet<'a>,
    pub sorted_pc_starts: Vec<usize>,
    pub links: HashMap<Direction, Links>,
}

impl<'a> RectangleSet<'a> {
    fn new(block_set: &'a BlockSet) -> Self {
        let mut sorted_pc_starts: Vec<usize> = block_set.get_all_pc_starts().into_iter().collect();
        sorted_pc_starts.sort();

        let links: HashMap<Direction, Links> = HashMap::from([
            (
                Direction::DESCENDING,
                Links::new(sorted_pc_starts.len(), Direction::DESCENDING),
            ),
            (
                Direction::RISING,
                Links::new(sorted_pc_starts.len(), Direction::RISING),
            ),
        ]);

        let mut rectangle_set: RectangleSet = RectangleSet {
            block_set,
            sorted_pc_starts,
            links,
        };
        rectangle_set.connect_all();
        return rectangle_set;
    }

    fn connect_all(&mut self) {
        for direction in [Direction::RISING, Direction::DESCENDING] {
            for (index_from, index_to) in self.get_sorted_edges_by_index(direction) {
                self.links
                    .get_mut(&direction)
                    .unwrap()
                    .connect(index_from, index_to);
            }
        }
    }

    fn get_n_indexes(&self) -> usize {
        return self.sorted_pc_starts.len();
    }

    fn get_rectangle_left_column(&self) -> usize {
        let max_rising_depth: usize = self.links[&Direction::DESCENDING].get_global_max_depth();
        return Self::get_offset(max_rising_depth) + 1;
    }

    fn get_offset(depth: usize) -> usize {
        return 2 + depth * 2;
    }

    fn get_external_column(&self, direction: Direction, offset: usize) -> usize {
        match direction {
            Direction::RISING => self.get_rectangle_left_column() - 1 - offset,
            Direction::DESCENDING => self.get_rectangle_left_column() + BLOCK_WIDTH + offset,
        }
    }

    fn get_internal_column(&self, direction: Direction) -> usize {
        match direction {
            Direction::RISING => self.get_rectangle_left_column() - 1,
            Direction::DESCENDING => self.get_rectangle_left_column() + BLOCK_WIDTH,
        }
    }

    fn get_sorted_edges_by_index(&self, direction: Direction) -> Vec<(usize, usize)> {
        let edges_by_pc_start: Vec<(usize, usize)> = self.block_set.get_edges();

        let direction_filer = |(pc_start_from, pc_start_to): &&(usize, usize)| match direction {
            Direction::DESCENDING => pc_start_from < pc_start_to,
            Direction::RISING => pc_start_to < pc_start_from,
        };

        let unidirectional_edges_by_pc_start: Vec<(usize, usize)> = edges_by_pc_start
            .iter()
            .filter(direction_filer)
            .map(|(pc_start_from, pc_start_to)| (*pc_start_from, *pc_start_to))
            .collect();

        let pc_start_to_index: HashMap<usize, usize> = map_values_to_index(&self.sorted_pc_starts);

        let mut edges_by_indexes: Vec<(usize, usize)> = vec![];
        for (pc_start_from, pc_start_to) in unidirectional_edges_by_pc_start {
            let index_from: usize = pc_start_to_index[&pc_start_from];
            let index_to: usize = pc_start_to_index[&pc_start_to];
            edges_by_indexes.push((index_from, index_to));
        }

        let mut sorted_edges_by_indexes: Vec<(usize, usize)> = edges_by_indexes.clone();
        sorted_edges_by_indexes.sort_by_key(|(index_from, index_to)| {
            ((*index_to as isize - *index_from as isize).abs()) * 10000 + *index_from as isize
        }); // TODO enlever

        return sorted_edges_by_indexes;
    }

    fn draw_rectangles(
        &self,
        grid: &mut Grid,
        bytecode: &Bytecode,
    ) -> HashMap<usize, (usize, usize)> {
        // rectangle index => (first line, last line) of the representation of the rectangle in the main grid
        let mut rectangle_to_grid_lines: HashMap<usize, (usize, usize)> = HashMap::new();

        let mut current_available_line: usize = 0;

        for (index, pc_start) in self.sorted_pc_starts.iter().enumerate() {
            let rectangle_grid: Grid = rectangle_to_grid(
                *pc_start,
                self.block_set.get_pc_end_of_block(*pc_start),
                bytecode,
                &(String::from("label ") + &index.to_string()),
            );

            grid.append_sub_grid(
                current_available_line,
                self.get_rectangle_left_column(),
                &rectangle_grid,
            );

            rectangle_to_grid_lines.insert(
                index,
                (
                    current_available_line,
                    current_available_line + rectangle_grid.get_n_lines() - 1,
                ),
            );

            current_available_line += rectangle_grid.get_n_lines() + 1;
        }
        return rectangle_to_grid_lines;
    }

    fn draw_crossings(
        &self,
        grid: &mut Grid,
        direction: Direction,
        rectangle_index: usize,
        link_start_line: usize,
        link_end_line: usize,
    ) {
        #[derive(PartialEq)]
        enum ConnectionType {
            ARRIVING,
            STARTING,
        }
        for (connections, line, connection_type) in [
            (
                &self.links[&direction].starting_links[&rectangle_index],
                link_start_line,
                ConnectionType::STARTING,
            ),
            (
                &self.links[&direction].arriving_links[&rectangle_index],
                link_end_line,
                ConnectionType::ARRIVING,
            ),
        ] {
            if let Some(max_starting_depth) =
                max_mapped_value(&connections, &|(_, depth): (usize, usize)| -> usize {
                    depth
                })
            {
                for (_, depth) in connections {
                    let symbol: char =
                        match (depth < &max_starting_depth, direction, &connection_type) {
                            (true, Direction::DESCENDING, ConnectionType::ARRIVING)
                            | (true, Direction::RISING, ConnectionType::STARTING) => {
                                HORYZONTAL_UP_CHAR
                            }
                            (true, Direction::DESCENDING, ConnectionType::STARTING)
                            | (true, Direction::RISING, ConnectionType::ARRIVING) => {
                                HORYZONTAL_DOWN_CHAR
                            }
                            (false, Direction::RISING, ConnectionType::STARTING) => UP_RIGHT_CHAR,
                            (false, Direction::DESCENDING, ConnectionType::STARTING) => {
                                DOWN_LEFT_CHAR
                            }
                            (false, Direction::RISING, ConnectionType::ARRIVING) => DOWN_RIGHT_CHAR,
                            (false, Direction::DESCENDING, ConnectionType::ARRIVING) => {
                                UP_LEFT_CHAR
                            }
                        };
                    grid.set_char(
                        line,
                        self.get_external_column(direction, Self::get_offset(*depth)),
                        symbol,
                    );
                }
                if connection_type == ConnectionType::ARRIVING {
                    grid.set_char(
                        line,
                        self.get_internal_column(direction),
                        match direction {
                            Direction::RISING => TARGET_RIGHT_CHAR,
                            Direction::DESCENDING => TARGET_LEFT_CHAR,
                        },
                    );
                }
            }
        }
    }

    fn draw_links(
        &self,
        grid: &mut Grid,
        direction: Direction,
        rectangle_to_grid_lines: &HashMap<usize, (usize, usize)>,
    ) {
        let get_link_start_line = |index_from: usize| -> usize {
            match direction {
                Direction::DESCENDING => rectangle_to_grid_lines[&index_from].1 - 1,
                Direction::RISING => rectangle_to_grid_lines[&index_from].0 + 1,
            }
        };

        let get_link_end_line = |index_to: usize| -> usize {
            match direction {
                Direction::DESCENDING => rectangle_to_grid_lines[&index_to].0 + 1,
                Direction::RISING => rectangle_to_grid_lines[&index_to].1 - 1,
            }
        };

        for depth in get_sorted_keys(&self.links[&direction].depth_to_links) {
            let mut link_index: usize = 0;
            for (index_from, index_to) in &self.links[&direction].depth_to_links[&depth] {
                link_index += 1;
                println!(
                    "direction: {:?}, depth: {} / {}, link {} / {}",
                    direction,
                    depth,
                    &self.links[&direction].find_minimum_depth(0, self.get_n_indexes() - 1) - 1,
                    link_index,
                    &self.links[&direction].depth_to_links[&depth].len()
                );

                let line_from: usize = get_link_start_line(*index_from);
                let line_to: usize = get_link_end_line(*index_to);
                let offset: usize = Self::get_offset(depth);

                let vertical_line_dir: LineDirection = match direction {
                    Direction::RISING => LineDirection::UP,
                    Direction::DESCENDING => LineDirection::DOWN,
                };

                let horyzontal_line_dir: LineDirection = match direction {
                    Direction::RISING => LineDirection::LEFT,
                    Direction::DESCENDING => LineDirection::RIGHT,
                };

                grid.draw_line(
                    line_from,
                    self.get_external_column(direction, offset),
                    vertical_line_dir,
                    (line_to as isize - line_from as isize).abs() as usize + 1,
                );

                for line in [line_from, line_to] {
                    grid.draw_line(
                        line,
                        self.get_internal_column(direction),
                        horyzontal_line_dir,
                        offset,
                    );
                }
            }
        }

        for rectangle_index in 0..self.get_n_indexes() {
            self.draw_crossings(
                grid,
                direction,
                rectangle_index,
                get_link_start_line(rectangle_index),
                get_link_end_line(rectangle_index),
            );
        }
    }

    fn to_grid(&self, bytecode: &Bytecode) -> Grid {
        let mut grid: Grid = Grid::new_with_size(1, 120);

        let rectangle_to_grid_lines: HashMap<usize, (usize, usize)> =
            self.draw_rectangles(&mut grid, bytecode);
        println!("rectangles drawn");
        for direction in [Direction::RISING, Direction::DESCENDING] {
            self.draw_links(&mut grid, direction, &rectangle_to_grid_lines)
        }
        println!("links drawn");
        return grid;
    }
}

pub fn draw(block_set: &BlockSet, bytecode: &Bytecode) -> String {
    let rectangle_set: RectangleSet = RectangleSet::new(block_set);
    let final_grid: Grid = rectangle_set.to_grid(bytecode);
    return final_grid.to_string();
}
