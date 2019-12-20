use std::fmt;
use std::hash::Hash;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Point {
    x: i64,
    y: i64,
}

impl Point {
    pub fn new(x: i64, y: i64) -> Point {
        Point { x, y }
    }

    pub fn move_north(&self, distance: i64) -> Point {
        Point::new(self.x, self.y - distance)
    }

    pub fn move_south(&self, distance: i64) -> Point {
        Point::new(self.x, self.y + distance)
    }

    pub fn move_west(&self, distance: i64) -> Point {
        Point::new(self.x - distance, self.y)
    }

    pub fn move_east(&self, distance: i64) -> Point {
        Point::new(self.x + distance, self.y)
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
