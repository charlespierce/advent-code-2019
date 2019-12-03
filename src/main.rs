use std::collections::BTreeSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input.txt").unwrap();
    let read = BufReader::new(file);
    let mut lines = read.lines();

    let first_set = set_from_moves(lines.next().unwrap().unwrap());
    let second_set = set_from_moves(lines.next().unwrap().unwrap());

    let mut smallest = i32::max_value();
    let intersection = first_set.intersection(&second_set);

    for point in intersection {
        if point.distance() < smallest {
            smallest = point.distance();
        }
    }

    println!("Smallest Distance: {}", smallest);
}

fn set_from_moves(input: String) -> BTreeSet<Point> {
    let mut x = 0;
    let mut y = 0;

    let mut set: BTreeSet<Point> = BTreeSet::new();
    let moves = input.split(',').map(Shift::from);

    for shift in moves {
        match shift {
            Shift::Up(distance) => {
                for _ in 0..distance {
                    y += 1;
                    set.insert(Point::new(x, y));
                }
            },
            Shift::Down(distance) => {
                for _ in 0..distance {
                    y -= 1;
                    set.insert(Point::new(x, y));
                }
            },
            Shift::Left(distance) => {
                for _ in 0..distance {
                    x -= 1;
                    set.insert(Point::new(x, y));
                }
            },
            Shift::Right(distance) => {
                for _ in 0..distance {
                    x += 1;
                    set.insert(Point::new(x, y));
                }
            },
        }
    }

    set
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    fn distance(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

enum Shift {
    Up(i32),
    Down(i32),
    Left(i32),
    Right(i32),
}

impl From<&str> for Shift {
    fn from(val: &str) -> Shift {
        let mut chars = val.chars();
        let direction = chars.next().unwrap();
        let distance: i32 = chars.as_str().parse().unwrap();

        match direction {
            'U' => Shift::Up(distance),
            'D' => Shift::Down(distance),
            'L' => Shift::Left(distance),
            'R' => Shift::Right(distance),
            _ => panic!("Unexpected input"),
        }
    }
}