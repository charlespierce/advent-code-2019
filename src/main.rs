use std::collections::btree_map::Entry;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines().map(|l| l.unwrap());
    let field = parse_asteroids(lines);
    let points = point_map(&Point::new(23, 20), &field);
    let point = points.values().nth(199).unwrap();
    println!("200th Point: {}", point.x * 100 + point.y);
}

fn point_map<'a>(reference: &Point, asteroids: &'a [Point]) -> BTreeMap<String, Point> {
    let mut map = BTreeMap::new();
    for point in asteroids {
        if point != reference {
            let angle = angle_between(reference, point).to_string();

            match map.entry(angle) {
                Entry::Vacant(vacant) => {
                    vacant.insert(point.clone());
                }
                Entry::Occupied(mut occupied) => {
                    let existing = occupied.get();
                    if radius(reference, point) < radius(reference, existing) {
                        occupied.insert(point.clone());
                    }
                }
            }
        }
    }
    map
}

fn angle_between(p1: &Point, p2: &Point) -> f64 {
    let angle = f64::atan2((p2.x - p1.x) as f64, (p1.y - p2.y) as f64);
    if angle < 0.0 {
        angle + (2.0 * std::f64::consts::PI)
    } else {
        angle
    }
}

fn radius(p1: &Point, p2: &Point) -> f64 {
    f64::sqrt(f64::powi((p2.x - p1.x) as f64, 2) + f64::powi((p2.y - p1.y) as f64, 2))
}

fn parse_asteroids<I: Iterator<Item = String>>(data: I) -> Vec<Point> {
    let mut asteroids = Vec::new();
    for (y, row) in data.enumerate() {
        for (x, chr) in row.trim().chars().enumerate() {
            if chr == '#' {
                asteroids.push(Point::new(x as i32, y as i32));
            }
        }
    }

    asteroids
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}
