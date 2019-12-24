use std::collections::HashSet;
use std::fs::read_to_string;

fn main() {
    let mut board = Board::from(read_to_string("input.txt").unwrap());

    while !board.step() {}

    board.print();

    println!("Biodiversity Rating: {}", board.map);
}

struct Board {
    map: u32,
    history: HashSet<u32>,
}

impl Board {
    fn has_bug(&self, x: u32, y: u32) -> bool {
        self.map & index(x, y) > 0
    }

    fn step(&mut self) -> bool {
        let mut new_map = 0;

        for y in 0..5 {
            for x in 0..5 {
                let bug_count = neighbors(x, y)
                    .into_iter()
                    .filter(|(nx, ny)| self.has_bug(*nx, *ny))
                    .count();
                if self.has_bug(x, y) {
                    if bug_count == 1 {
                        new_map |= index(x, y);
                    }
                } else if 1 <= bug_count && bug_count <= 2 {
                    new_map |= index(x, y);
                }
            }
        }

        self.history.insert(self.map);
        self.map = new_map;

        self.history.contains(&new_map)
    }

    fn print(&self) {
        for y in 0..5 {
            for x in 0..5 {
                if self.has_bug(x, y) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
}

impl From<String> for Board {
    fn from(value: String) -> Self {
        let map = value
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars().enumerate().map(move |(x, chr)| {
                    if chr == '#' {
                        index(x as u32, y as u32)
                    } else {
                        0
                    }
                })
            })
            .fold(0, |map, point| map | point);

        Board {
            map,
            history: HashSet::new(),
        }
    }
}

fn index(x: u32, y: u32) -> u32 {
    1 << (y * 5 + x)
}

fn neighbors(x: u32, y: u32) -> Vec<(u32, u32)> {
    let mut output = Vec::new();

    if x > 0 {
        output.push((x - 1, y));
    }

    if x < 4 {
        output.push((x + 1, y));
    }

    if y > 0 {
        output.push((x, y - 1));
    }

    if y < 4 {
        output.push((x, y + 1));
    }

    output
}
