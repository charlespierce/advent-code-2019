use std::fmt;
use std::hash::Hash;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Location {
    pub x: i64,
    pub y: i64,
}

impl Location {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    pub fn move_north(&self, distance: i64) -> Self {
        Self::new(self.x, self.y - distance)
    }

    pub fn move_south(&self, distance: i64) -> Self {
        Self::new(self.x, self.y + distance)
    }

    pub fn move_west(&self, distance: i64) -> Self {
        Self::new(self.x - distance, self.y)
    }

    pub fn move_east(&self, distance: i64) -> Self {
        Self::new(self.x + distance, self.y)
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Point {
    pub loc: Location,
    pub level: i64,
}

impl Point {
    pub fn new(loc: Location, level: i64) -> Point {
        Point { loc, level }
    }

    pub fn move_north(&self, distance: i64) -> Point {
        Point::new(self.loc.move_north(distance), self.level)
    }

    pub fn move_south(&self, distance: i64) -> Point {
        Point::new(self.loc.move_south(distance), self.level)
    }

    pub fn move_west(&self, distance: i64) -> Point {
        Point::new(self.loc.move_west(distance), self.level)
    }

    pub fn move_east(&self, distance: i64) -> Point {
        Point::new(self.loc.move_east(distance), self.level)
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} - {}", self.loc, self.level)
    }
}
