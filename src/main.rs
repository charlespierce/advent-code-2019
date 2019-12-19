mod computer;

use computer::{Computer, IO};

fn main() {
    for x in 900..1000 {
        if is_point_in_beam(x, 844) {
            println!("Lower Edge: {}, 50", x);
            break;
        }
    }

    for y in 700..800 {
        if is_point_in_beam(1020, y) {
            println!("Upper Edge: {}", y);
            break;
        }
    }
}

fn is_point_in_beam(x: i64, y: i64) -> bool {
    let computer = Computer::new();
    let mut drone = Drone::new(x, y);
    computer.run(&mut drone);

    drone.pulled
}

struct Drone {
    position: (i64, i64),
    pulled: bool,
    x_sent: bool,
}

impl Drone {
    fn new(x: i64, y: i64) -> Self {
        Drone {
            position: (x, y),
            pulled: false,
            x_sent: false,
        }
    }
}

impl IO for Drone {
    fn next_input(&mut self) -> i64 {
        if self.x_sent {
            self.position.1
        } else {
            self.x_sent = true;
            self.position.0
        }
    }

    fn next_output(&mut self, value: i64) {
        self.pulled = match value {
            1 => true,
            _ => false,
        };
    }
}
