use crate::utils;
use std::collections::HashSet;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
// notation is (x, y);
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

impl Grid {
    pub fn new(rows: u32, columns: u32) -> Self {
        Grid {
            rows,
            columns,
            cells: utils::randomize_state(rows, columns),
        }
    }
    pub fn print_grid_state(&self, r: u32) -> String {
        let mut final_str: String = String::new();
        let row = format!(
            "\t\t\t\tYou have {} rows and {} columns\n",
            self.rows * r,
            self.columns * r
        );

        for _i in 0..=10 {
            final_str += &row;
        }

        final_str
    }
}
