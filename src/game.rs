use std::collections::HashSet;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Coordinate {
    x: u32,
    y: u32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Grid {
    width: u32,
    height: u32,
    // HashSet - because we can have a lower memory
    // footprint as we'll be storing alive cells only
    // and also that operations in the game become easier
    cells: HashSet<Coordinate>,
}
