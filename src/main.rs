use std::collections::{BTreeMap, BTreeSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input.txt").unwrap();
    let read = BufReader::new(file);
    let mut lines = read.lines();

    let (first_set, first_map) = set_from_moves(lines.next().unwrap().unwrap());
    let (second_set, second_map) = set_from_moves(lines.next().unwrap().unwrap());

    let mut smallest = i32::max_value();
    let intersection = first_set.intersection(&second_set);

    for point in intersection {
        let distance = first_map.get(&point).unwrap() + second_map.get(&point).unwrap();
        if distance < smallest {
            smallest = distance;
        }
    }

    println!("Smallest Distance: {}", smallest);
}

fn set_from_moves(input: String) -> (BTreeSet<Point>, BTreeMap<Point, i32>) {
    let mut x = 0;
    let mut y = 0;
    let mut steps = 0;

    let mut set: BTreeSet<Point> = BTreeSet::new();
    let mut map: BTreeMap<Point, i32> = BTreeMap::new();
    let moves = input.split(',').map(Shift::from);

    for shift in moves {
        match shift {
            Shift::Up(distance) => {
                for _ in 0..distance {
                    steps += 1;
                    y += 1;
                    let point = Point::new(x, y);
                    set.insert(point.clone());
                    map.entry(point).or_insert(steps);
                }
            },
            Shift::Down(distance) => {
                for _ in 0..distance {
                    steps += 1;
                    y -= 1;
                    let point = Point::new(x, y);
                    set.insert(point.clone());
                    map.entry(point).or_insert(steps);
                }
            },
            Shift::Left(distance) => {
                for _ in 0..distance {
                    steps += 1;
                    x -= 1;
                    let point = Point::new(x, y);
                    set.insert(point.clone());
                    map.entry(point).or_insert(steps);
                }
            },
            Shift::Right(distance) => {
                for _ in 0..distance {
                    steps += 1;
                    x += 1;
                    let point = Point::new(x, y);
                    set.insert(point.clone());
                    map.entry(point).or_insert(steps);
                }
            },
        }
    }

    (set, map)
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
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