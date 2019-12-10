use std::cmp;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines().map(|l| l.unwrap());
    let field = parse_asteroids(lines);

    let max = maximum_visible(field);

    println!("Maximum Visible: {}", max);
}

fn maximum_visible(asteroids: Vec<Point>) -> i32 {
    let mut max_visible = 0;
    let mut point = None;

    for p1 in asteroids.iter() {
        let mut count = 0;
        for p2 in asteroids.iter() {
            if p1 != p2 && can_see(p1, p2, &asteroids) {
                count += 1;
            }
        }
        if count > max_visible {
            max_visible = count;
            point = Some(p1);
        }
    }

    println!("{:?}", point.unwrap());
    max_visible
}

fn can_see(p1: &Point, p2: &Point, asteroids: &[Point]) -> bool {
    for p3 in asteroids.iter() {
        if p1 != p3 && p2 != p3 && three_between(p1, p2, p3) {
            return false;
        }
    }
    true
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

#[derive(Debug, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}

fn three_between(p1: &Point, p2: &Point, p3: &Point) -> bool {
    if are_collinear(p1, p2, p3) {
        let min_x = cmp::min(p1.x, p2.x);
        let max_x = cmp::max(p1.x, p2.x);
        let min_y = cmp::min(p1.y, p2.y);
        let max_y = cmp::max(p1.y, p2.y);

        if min_x <= p3.x && p3.x <= max_x && min_y <= p3.y && p3.y <= max_y {
            return true;
        }
    }
    false
}

fn are_collinear(p1: &Point, p2: &Point, p3: &Point) -> bool {
    let first = (p1.x - p2.x) * (p2.y - p3.y);
    let second = (p2.x - p3.x) * (p1.y - p2.y);

    first - second == 0
}
