use std::collections::BTreeMap;

mod computer;

use computer::{Computer, IO};

fn main() {
    let computer = Computer::new();
    let mut searcher = Searcher::new();

    computer.run(&mut searcher);
}

type Point = (i64, i64);

#[derive(Clone)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn move_point(&self, point: Point) -> Point {
        match self {
            Direction::North => (point.0 - 1, point.1),
            Direction::South => (point.0 + 1, point.1),
            Direction::West => (point.0, point.1 - 1),
            Direction::East => (point.0, point.1 + 1),
        }
    }

    fn reverse(self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }

    fn turn_right(self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}

impl From<Direction> for i64 {
    fn from(dir: Direction) -> i64 {
        match dir {
            Direction::North => 1,
            Direction::South => 2,
            Direction::West => 3,
            Direction::East => 4,
        }
    }
}

enum Reply {
    Wall,
    Success,
    Oxygen,
}

impl From<i64> for Reply {
    fn from(val: i64) -> Self {
        match val {
            0 => Reply::Wall,
            1 => Reply::Success,
            2 => Reply::Oxygen,
            _ => panic!("Unknown Reply: {}", val),
        }
    }
}

struct Searcher {
    position: Point,
    map: BTreeMap<Point, char>,
    moves: Vec<Direction>,
    prev: Option<Direction>,
    backtrack: bool,
}

impl Searcher {
    fn new() -> Self {
        let mut searcher = Searcher {
            position: (0, 0),
            map: BTreeMap::new(),
            moves: Vec::new(),
            prev: None,
            backtrack: false,
        };

        searcher.map.insert((0, 0), '*');

        searcher
    }

    fn unmapped_adjacent(&self) -> Option<Direction> {
        let mut direction = Direction::North;

        for _ in 0..4 {
            if !self.map.contains_key(&direction.move_point(self.position)) {
                return Some(direction);
            }

            direction = direction.turn_right();
        }

        None
    }
}

impl IO for Searcher {
    fn next_input(&mut self) -> i64 {
        let dir = match self.unmapped_adjacent() {
            Some(dir) => dir,
            None => match self.moves.pop() {
                Some(prev) => {
                    self.backtrack = true;
                    prev.reverse()
                }
                None => {
                    fill(&self.map);
                    panic!("Done!");
                }
            },
        };

        self.prev = Some(dir.clone());
        dir.into()
    }

    fn next_output(&mut self, value: i64) {
        let direction = self.prev.take().unwrap();
        match value.into() {
            Reply::Wall => {
                let wall_position = direction.move_point(self.position);
                self.map.entry(wall_position).or_insert('#');
            }
            Reply::Success => {
                self.position = direction.move_point(self.position);
                if self.backtrack {
                    self.backtrack = false;
                } else {
                    self.map.entry(self.position).or_insert('.');
                    self.moves.push(direction);
                }
            }
            Reply::Oxygen => {
                self.position = direction.move_point(self.position);
                self.map.entry(self.position).or_insert('X');
                self.moves.push(direction);
            }
        }
    }
}

fn fill(map: &BTreeMap<Point, char>) {
    let mut map = map.clone();
    let points = map.values().filter(|c| **c == '.' || **c == '*').count();
    let mut time = 0;

    println!("Filling {} Areas...", points);

    loop {
        time += 1;
        let filled = map
            .iter()
            .filter(|(_, c)| **c == 'X')
            .map(|(&point, _)| point)
            .collect::<Vec<_>>();
        if filled.len() == points {
            println!("Total Time: {}", time);
            break;
        }

        for point in filled {
            expand(&mut map, Direction::North.move_point(point));
            expand(&mut map, Direction::South.move_point(point));
            expand(&mut map, Direction::East.move_point(point));
            expand(&mut map, Direction::West.move_point(point));
        }
    }
}

fn expand(map: &mut BTreeMap<Point, char>, point: Point) {
    if let Some(c) = map.get(&point) {
        if *c == '.' || *c == '*' {
            map.insert(point, 'X');
        }
    }
}
