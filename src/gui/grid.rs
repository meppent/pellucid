#![allow(dead_code)]
pub const EMPTY_CHAR: char = ' ';
pub const VERTICAL_CHAR: char = '│';
pub const HORYZONTAL_CHAR: char = '─';
pub const HORYZONTAL_DOWN_CHAR: char = '┬';
pub const DOWN_LEFT_CHAR: char = '┐';
pub const HORYZONTAL_UP_CHAR: char = '┴';
pub const UP_LEFT_CHAR: char = '┘';
pub const DOWN_RIGHT_CHAR: char = '┌';
pub const UP_RIGHT_CHAR: char = '└';

pub const TARGET_LEFT_CHAR: char = '<';
pub const TARGET_RIGHT_CHAR: char = '>';

#[derive(Clone, Copy)]
pub enum LineDirection {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl LineDirection {
    fn to_index(&self) -> (isize, isize) {
        match self {
            LineDirection::UP => (-1, 0),
            LineDirection::DOWN => (1, 0),
            LineDirection::LEFT => (0, -1),
            LineDirection::RIGHT => (0, 1),
        }
    }

    fn to_char(&self) -> char {
        match self {
            LineDirection::UP | LineDirection::DOWN => VERTICAL_CHAR,
            LineDirection::RIGHT | LineDirection::LEFT => HORYZONTAL_CHAR,
        }
    }
}

/*
(0,0) ――> column
  |
  |
  V
  line
*/
pub struct Grid {
    content: Vec<Vec<char>>, // [line][column] starting from upper left corner (0,0)
}

impl Grid {
    pub fn new() -> Self {
        return Grid { content: vec![] };
    }
    pub fn new_with_size(n_lines: usize, n_columns: usize) -> Self {
        let mut grid: Grid = Grid { content: vec![] };
        grid.ensure_size(n_lines - 1, n_columns - 1);
        return grid;
    }

    pub fn to_string(&self) -> String {
        let mut res: String = String::new();
        for line in 0..self.content.len() {
            for column in 0..self.content[line].len() {
                res += &self.content[line][column].to_string();
            }
            if line != self.content.len() - 1 {
                res += &String::from("\n");
            }
        }
        return res;
    }

    pub fn get_n_lines(&self) -> usize {
        return self.content.len();
    }
    pub fn get_n_columns(&self) -> usize {
        return if self.content.len() == 0 {
            0
        } else {
            self.content[0].len()
        };
    }

    pub fn ensure_size(&mut self, line: usize, column: usize) {
        // add empty lines and columns until the coordinate (line, column) exists
        let n_lines_to_add: usize = if self.get_n_lines() <= line {
            1 + line - self.get_n_lines()
        } else {
            0
        };
        self.content.append(&mut vec![
            vec![EMPTY_CHAR; self.get_n_columns()];
            n_lines_to_add
        ]);

        let n_columns_to_add: usize = if self.get_n_columns() <= column {
            1 + column - self.get_n_columns()
        } else {
            0
        };
        for line in 0..self.get_n_lines() {
            self.content[line].append(&mut vec![EMPTY_CHAR; n_columns_to_add]);
        }
    }

    pub fn get_char(&self, line: usize, column: usize) -> char {
        return self.content[line][column];
    }

    pub fn set_char(&mut self, line: usize, column: usize, moved_char: char) {
        self.ensure_size(line, column);
        self.content[line][column] = moved_char;
    }

    pub fn append_sub_grid(&mut self, line: usize, column: usize, grid_to_append: &Self) {
        // the upper left corner of grid_to_append will be located at (line, column)
        self.ensure_size(
            line + grid_to_append.get_n_lines(),
            column + grid_to_append.get_n_columns(),
        );

        for other_line in 0..grid_to_append.get_n_lines() {
            for other_column in 0..grid_to_append.get_n_columns() {
                self.content[line + other_line][column + other_column] =
                    grid_to_append.get_char(other_line, other_column);
            }
        }
    }

    pub fn append_string(&mut self, line: usize, column: usize, string_to_append: &str) {
        // the first character of string_to_append will be located at (line, column)
        self.ensure_size(line, column + string_to_append.len() - 1);
        for (other_column, moved_char) in string_to_append.chars().enumerate() {
            self.content[line][column + other_column] = moved_char;
        }
    }

    pub fn draw_line(
        &mut self,
        initial_line: usize,
        initial_column: usize,
        direction: LineDirection,
        length: usize,
    ) {
        let (mut current_line, mut current_column): (usize, usize) = (initial_line, initial_column);
        for _ in 0..length {
            self.set_char(current_line, current_column, direction.to_char());
            current_line = (current_line as isize + direction.to_index().0) as usize;
            current_column = (current_column as isize + direction.to_index().1) as usize;
        }
    }

    pub fn draw_line_with_coords(
        &mut self,
        initial_line: usize,
        initial_column: usize,
        final_line: usize,
        final_column: usize,
    ) {
        let direction: LineDirection;
        let length: usize;
        if initial_line == final_line {
            if initial_column < final_column {
                direction = LineDirection::RIGHT;
            } else {
                direction = LineDirection::LEFT;
            }
            length = (initial_column as isize - final_column as isize).abs() as usize + 1;
        } else {
            assert!(initial_column == final_column);
            if initial_line < final_line {
                direction = LineDirection::DOWN;
            } else {
                direction = LineDirection::UP;
            }
            length = (initial_line as isize - final_line as isize).abs() as usize + 1;
        }

        self.draw_line(initial_line, initial_column, direction, length);
    }

    pub fn draw_outside_box(&mut self) {
        /*
        Draw a frame surrounding the grid.
                            ..............
        ............        .┌――――――――――┐.
        . some     .        .| some     |.
        . content  .        .| content  |.
        .  xxx     .   ――>  .|  xxx     |.
        .          .        .|          |.
        ............        .└――――――――――┘.
                            ..............
        */

        self.content
            .insert(0, vec![HORYZONTAL_CHAR; self.get_n_columns()]);
        self.content
            .push(vec![HORYZONTAL_CHAR; self.get_n_columns()]);
        for line in 0..self.get_n_lines() {
            self.content[line].insert(0, VERTICAL_CHAR);
            self.content[line].push(VERTICAL_CHAR);
        }
        for (line, column, corner) in [
            (0, 0, DOWN_RIGHT_CHAR),
            (0, self.get_n_columns() - 1, DOWN_LEFT_CHAR),
            (self.get_n_lines() - 1, 0, UP_RIGHT_CHAR),
            (
                self.get_n_lines() - 1,
                self.get_n_columns() - 1,
                UP_LEFT_CHAR,
            ),
        ] {
            self.content[line][column] = corner;
        }
    }
}
