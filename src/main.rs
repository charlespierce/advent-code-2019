use std::collections::BTreeMap;

mod computer;

use computer::{Computer, IO};

fn main() {
    let computer = Computer::new();
    let mut io = Output::new();

    computer.run(&mut io);

    println!("Final Score: {}", io.score);
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
    tiles: BTreeMap<Point, TileType>,
    score: i64,
}

impl Output {
    fn new() -> Self {
        Output {
            next: NextOutput::X,
            x: 0,
            y: 0,
            tiles: BTreeMap::new(),
            score: 0,
        }
    }
}

impl IO for Output {
    fn next_input(&mut self) -> i64 {
        let (ball_x, paddle_x) = self.tiles.iter().fold((0, 0), |acc, ((_, x), t)| match t {
            TileType::Ball => (*x, acc.1),
            TileType::Paddle => (acc.0, *x),
            _ => acc,
        });

        if ball_x < paddle_x {
            -1
        } else if ball_x > paddle_x {
            1
        } else {
            0
        }
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
                let point = (self.y, self.x);

                if point == (0, -1) {
                    self.score = value;
                } else {
                    self.tiles.insert(point, value.into());
                }
                NextOutput::X
            }
        }
    }
}
