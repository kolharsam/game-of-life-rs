use std::collections::HashSet;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
// notation is (x, y);
pub struct Coordinate(pub u32, pub u32);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Grid {
    pub width: u32,
    pub height: u32,
    // HashSet - because we can have a lower memory
    // footprint as we'll be storing alive cells only
    // and also that operations in the game become easier
    pub cells: HashSet<Coordinate>,
}
