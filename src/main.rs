mod computer;

use computer::{Computer, IO};

fn main() {
    let mut affected = 0;

    for x in 0..50 {
        for y in 0..50 {
            let computer = Computer::new();
            let mut drone = Drone::new(x, y);
            computer.run(&mut drone);

            if drone.pulled {
                affected += 1;
            }
        }
    }

    println!("Total Points Affected: {}", affected);
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
