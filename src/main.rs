use std::collections::HashMap;

mod computer;

use computer::{Computer, IO};

fn main() {
    let computer = Computer::new();
    let mut io = Output::new();

    computer.run(&mut io);

    let blocks: i64 = io
        .tiles
        .values()
        .map(|t| match t {
            TileType::Block => 1,
            _ => 0,
        })
        .sum();

    println!("Total Blocks: {}", blocks);
}

type Point = (i64, i64);

enum NextOutput {
    X,
    Y,
    TileId,
}

enum TileType {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
}

impl From<i64> for TileType {
    fn from(val: i64) -> TileType {
        match val {
            0 => TileType::Empty,
            1 => TileType::Wall,
            2 => TileType::Block,
            3 => TileType::Paddle,
            4 => TileType::Ball,
            _ => panic!("Unknown tile type: {}", val),
        }
    }
}

struct Output {
    next: NextOutput,
    x: i64,
    y: i64,
    tiles: HashMap<Point, TileType>,
}

impl Output {
    fn new() -> Self {
        Output {
            next: NextOutput::X,
            x: 0,
            y: 0,
            tiles: HashMap::new(),
        }
    }
}

impl IO for Output {
    fn next_input(&mut self) -> i64 {
        unimplemented!();
    }

    fn next_output(&mut self, value: i64) {
        self.next = match self.next {
            NextOutput::X => {
                self.x = value;
                NextOutput::Y
            }
            NextOutput::Y => {
                self.y = value;
                NextOutput::TileId
            }
            NextOutput::TileId => {
                let point = (self.x, self.y);
                self.tiles.insert(point, value.into());
                NextOutput::X
            }
        }
    }
}
