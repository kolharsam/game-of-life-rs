use crate::utils;
use std::collections::HashSet;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
// notation is (x, y)
pub struct Coordinate(pub u32, pub u32);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Grid {
    pub rows: u32,
    pub columns: u32,
    // HashSet - because we can have a lower memory
    // footprint as we'll be storing alive cells only
    // and also that operations in the game become easier
    pub cells: HashSet<Coordinate>,
}

impl Coordinate {
    pub fn get_neighbours(&self, rows: u32, cols: u32) -> HashSet<Coordinate> {
        let coord_map: [i32; 3] = [-1, 0, 1];
        let mut neighbours: HashSet<Coordinate> = HashSet::new();

        for rr in coord_map.iter() {
            for cc in coord_map.iter() {
                if *rr == 0 && *cc == 0 {
                    continue;
                }
                // FIXME: unsafe type conversion
                let cell_x = self.0 as i32 + *rr;
                let cell_y = self.1 as i32 + *cc;
                // FIXME: unsafe type conversion
                if cell_x < 0 || cell_x >= (rows as i32) || cell_y < 0 || cell_y >= (cols as i32) {
                    continue;
                }
                // FIXME: unsafe type conversion
                neighbours.insert(Coordinate(cell_x as u32, cell_y as u32));
            }
        }

        neighbours
    }
}

impl Grid {
    pub fn new(rows: u32, columns: u32) -> Self {
        Grid {
            rows,
            columns,
            cells: utils::randomize_state(rows, columns),
        }
    }

    fn calculate_next_state(&mut self) {
        let mut new_set_alive: HashSet<Coordinate> = HashSet::new();

        for rr in 0..self.rows {
            for cc in 0..self.columns {
                let coord = Coordinate(rr, cc);
                let neighbours = coord.get_neighbours(self.rows, self.columns);
                // NOTE: This is the magic trick
                let alive_neighbours = neighbours.intersection(&self.cells);
                let alive_count = alive_neighbours.count();
                let is_current_cell_alive = self.cells.contains(&coord);

                if alive_count == 3 || is_current_cell_alive && alive_count == 2 {
                    new_set_alive.insert(coord);
                }
            }
        }

        // NOTE: This is a mutation! of the original instance, beware!
        self.cells = new_set_alive;
    }

    pub fn print_grid_state(&mut self) -> String {
        self.calculate_next_state();

        let mut final_state_str: String = String::new();

        for rr in 0..self.rows {
            let mut row_str: String = String::new();
            // FIXME: my formatting skills need hella improvement
            row_str += "\t\t\t\t";
            for cc in 0..self.columns {
                let coord = Coordinate(rr, cc);
                let is_cell_alive = self.cells.contains(&coord);
                if is_cell_alive {
                    row_str += "#";
                } else {
                    row_str += ".";
                }
            }
            row_str += "\n";
            final_state_str += &row_str;
        }

        final_state_str
    }
}
