use std::collections::HashMap;

mod computer;

use computer::{Computer, IO};

fn main() {
    let mut robot = Robot::new();
    let computer = Computer::new();

    computer.run(&mut robot);

    println!("Panels Painted: {}", robot.grid.colors.keys().len());
}

type Point = (i32, i32);

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

enum NextOutput {
    Paint,
    Turn,
}

#[derive(Copy, Clone)]
enum Color {
    Black = 0,
    White = 1,
}

impl From<i64> for Color {
    fn from(value: i64) -> Self {
        match value {
            0 => Color::Black,
            1 => Color::White,
            _ => panic!("Unexpected color: {}", value),
        }
    }
}

struct Grid {
    colors: HashMap<Point, Color>,
}

impl Grid {
    fn new() -> Self {
        Grid {
            colors: HashMap::new(),
        }
    }

    fn get_color(&self, point: Point) -> Color {
        match self.colors.get(&point) {
            Some(color) => *color,
            None => Color::Black,
        }
    }

    fn paint(&mut self, point: Point, color: Color) {
        self.colors.insert(point, color);
    }
}

struct Robot {
    grid: Grid,
    pointing_dir: Direction,
    position: Point,
    next: NextOutput,
}

impl Robot {
    fn new() -> Self {
        Robot {
            grid: Grid::new(),
            pointing_dir: Direction::Up,
            position: (0, 0),
            next: NextOutput::Paint,
        }
    }

    fn move_step(&mut self) {
        self.position = match self.pointing_dir {
            Direction::Up => (self.position.0, self.position.1 + 1),
            Direction::Down => (self.position.0, self.position.1 - 1),
            Direction::Left => (self.position.0 - 1, self.position.1),
            Direction::Right => (self.position.0 + 1, self.position.1),
        };
    }

    fn turn_left(&mut self) {
        self.pointing_dir = match self.pointing_dir {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        }
    }

    fn turn_right(&mut self) {
        self.pointing_dir = match self.pointing_dir {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

impl IO for Robot {
    fn next_input(&mut self) -> i64 {
        self.grid.get_color(self.position) as i64
    }

    fn next_output(&mut self, value: i64) {
        match self.next {
            NextOutput::Paint => {
                self.grid.paint(self.position, value.into());
                self.next = NextOutput::Turn;
            }
            NextOutput::Turn => {
                if value == 0 {
                    self.turn_left();
                } else {
                    self.turn_right();
                }
                self.move_step();
                self.next = NextOutput::Paint;
            }
        }
    }
}
