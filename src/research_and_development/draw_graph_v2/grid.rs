use std::{
    collections::{HashMap, HashSet},
    ops::Range,
};

use itertools::{sorted, Itertools};

// a grid representing the coordinates of the blocks in the graph we display
pub struct Grid {
    index_to_coord: HashMap<usize, (usize, usize)>, // index => (x, y)
    coord_to_index: HashMap<(usize, usize), usize>, // (x, y) => index
    y_to_indexes: HashMap<usize, HashSet<usize>>,
    width: usize,
    height: usize,
    n_indexes: usize,
}

impl Grid {
    pub fn from(indexes_matrix: Vec<Vec<usize>>) -> Self {
        let mut index_to_coord: HashMap<usize, (usize, usize)> = HashMap::new();
        let mut coord_to_index: HashMap<(usize, usize), usize> = HashMap::new();
        let mut y_to_indexes: HashMap<usize, HashSet<usize>> = HashMap::new();
        let mut n_indexes: usize = 0;
        for x in 0..indexes_matrix.len() {
            for y in 0..indexes_matrix[x].len() {
                n_indexes += 1;
                let index: usize = indexes_matrix[x][y];
                index_to_coord.insert(index, (x, y));
                coord_to_index.insert((x, y), index);
                if !y_to_indexes.contains_key(&y) {
                    y_to_indexes.insert(y, HashSet::new());
                }
                y_to_indexes.get_mut(&y).unwrap().insert(index);
            }
        }

        let width: usize = 1 + *index_to_coord.values().map(|(x, _)| x).max().unwrap();
        let height: usize = 1 + *index_to_coord.values().map(|(_, y)| y).max().unwrap();

        return Grid {
            index_to_coord,
            coord_to_index,
            y_to_indexes,
            width,
            height,
            n_indexes,
        };
    }

    pub fn get_width(&self) -> usize {
        return self.width;
    }

    pub fn get_height(&self) -> usize {
        return self.height;
    }

    pub fn remove_empty_bands(&mut self) {
        self.remove_empty_bands_on_x();
        self.remove_empty_bands_on_y();
    }
    pub fn remove_empty_bands_on_x(&mut self) {
        let sorted_x: Vec<usize> = sorted(
            self.index_to_coord
                .values()
                .map(|(x, _)| *x)
                .collect::<HashSet<usize>>()
                .iter()
                .cloned()
                .collect_vec(),
        )
        .collect();
        for (best_x, moving_x) in sorted_x.iter().enumerate() {
            if best_x != *moving_x {
                assert!(best_x < *moving_x);
                let delta: usize = moving_x - best_x;
                for (index, (prev_x, prev_y)) in self.index_to_coord.clone() {
                    let mut new_x: usize = prev_x;
                    let new_y: usize = prev_y;
                    if best_x < prev_x {
                        new_x -= delta;
                    }
                    self.index_to_coord.insert(index, (new_x, new_y));
                    self.coord_to_index.insert((new_x, new_y), index);
                }
                self.remove_empty_bands_on_x();
                return;
            }
        }
    }

    pub fn remove_empty_bands_on_y(&mut self) {
        let sorted_y: Vec<usize> = sorted(
            self.index_to_coord
                .values()
                .map(|(_, y)| *y)
                .collect::<HashSet<usize>>()
                .iter()
                .cloned()
                .collect_vec(),
        )
        .collect();
        for (best_y, moving_y) in sorted_y.iter().enumerate() {
            if best_y != *moving_y {
                assert!(best_y < *moving_y);
                let delta: usize = moving_y - best_y;
                for (index, (prev_x, prev_y)) in self.index_to_coord.clone() {
                    let new_x: usize = prev_x;
                    let mut new_y: usize = prev_y;
                    if best_y < prev_x {
                        new_y -= delta;
                    }
                    self.index_to_coord.insert(index, (new_x, new_y));
                    self.coord_to_index.insert((new_x, new_y), index);
                }
                self.remove_empty_bands_on_y();
                return;
            }
        }
    }

    pub fn update_dims(&mut self) {
        self.width = 1 + self.index_to_coord.values().map(|(x, _)| x).max().unwrap();
        self.height = 1 + self.index_to_coord.values().map(|(_, y)| y).max().unwrap();
    }

    pub fn get_indexes_at_y(&self, y: usize) -> HashSet<usize> {
        return self.y_to_indexes[&y].clone();
    }

    pub fn get_sorted_indexes_at_y(&self, y: usize) -> Vec<(usize, (usize, usize))> {
        return sorted(self.get_indexes_at_y(y).iter().cloned())
            .map(|index| (index, self.index_to_coord[&index]))
            .collect_vec();
    }

    pub fn move_index(&mut self, index: usize, new_position: (usize, usize)) {
        self.width = self.width.max(1 + new_position.0);
        self.height = self.height.max(1 + new_position.1);

        self.coord_to_index.remove(&self.index_to_coord[&index]);
        self.y_to_indexes
            .get_mut(&self.index_to_coord[&index].1)
            .unwrap()
            .remove(&index);
        self.coord_to_index.insert(new_position, index);
        self.y_to_indexes
            .get_mut(&new_position.1)
            .unwrap()
            .insert(index);
        self.index_to_coord.insert(index, new_position);
    }

    pub fn move_up(&mut self, index: usize) {
        let y: usize = self.index_to_coord[&index].1;
        let mut moving_x: usize = 0;
        loop {
            if !self.coord_to_index.contains_key(&(moving_x, y - 1)) {
                self.move_index(index, (moving_x, y - 1));
                return;
            }
            moving_x += 1;
        }
    }

    pub fn get_x_of_index(&self, index: usize) -> usize {
        return self.index_to_coord[&index].0;
    }

    pub fn iter_indexes(&self) -> Range<usize> {
        return 0..self.n_indexes;
    }
}
