use std::collections::BTreeMap;
use std::convert::TryFrom;

mod computer;

use computer::{Computer, IO};

fn main() {
    let computer = Computer::new();
    let mut ascii = Ascii::new();

    computer.run(&mut ascii);

    print_view(&ascii.map);
    println!("Calibration: {}", calculate_calibration(&ascii.map));
}

fn print_view(map: &BTreeMap<Point, char>) {
    let mut curr_y = 0;
    for ((y, _), chr) in map.iter() {
        if curr_y != *y {
            println!();
            curr_y = *y;
        }
        print!("{}", chr);
    }
    println!();
}

fn calculate_calibration(map: &BTreeMap<Point, char>) -> i64 {
    let mut output = 0;

    for (point, chr) in map.iter() {
        if *chr == '#' {
            if map.get(&(point.0, point.1 - 1)) != Some(&'#') {
                continue;
            }

            if map.get(&(point.0, point.1 + 1)) != Some(&'#') {
                continue;
            }

            if map.get(&(point.0 - 1, point.1)) != Some(&'#') {
                continue;
            }

            if map.get(&(point.0 + 1, point.1)) != Some(&'#') {
                continue;
            }

            output += point.0 * point.1;
        }
    }

    output
}

type Point = (i64, i64);

struct Ascii {
    map: BTreeMap<Point, char>,
    position: Point,
}

impl Ascii {
    fn new() -> Self {
        Ascii {
            map: BTreeMap::new(),
            position: (0, 0),
        }
    }
}

impl IO for Ascii {
    fn next_input(&mut self) -> i64 {
        unimplemented!();
    }

    fn next_output(&mut self, value: i64) {
        match value {
            10 => {
                self.position = (self.position.0 + 1, 0);
            }
            c => {
                self.map
                    .insert(self.position, char::try_from(c as u32).unwrap());
                self.position.1 += 1;
            }
        }
    }
}
