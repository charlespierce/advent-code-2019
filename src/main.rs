use std::cmp::Ordering;
use std::fs::read_to_string;
use std::str::FromStr;

fn main() {
    let mut system: System = read_to_string("input.txt").unwrap().parse().unwrap();

    for _ in 0..1000 {
        system.step();
    }

    println!("Total Energy: {}", system.total_energy());
}

struct System {
    moons: Vec<Moon>,
}

impl System {
    fn step(&mut self) {
        let compare = self.moons.clone();
        for (index, moon) in self.moons.iter_mut().enumerate() {
            for (compare_index, compare_to) in compare.iter().enumerate() {
                if compare_index != index {
                    moon.step_velocity(compare_to);
                }
            }
        }

        for moon in self.moons.iter_mut() {
            moon.step_position();
        }
    }

    fn total_energy(&self) -> i64 {
        self.moons.iter().map(Moon::energy).sum()
    }
}

impl FromStr for System {
    type Err = String;

    fn from_str(value: &str) -> Result<System, String> {
        let moons = value.lines().map(|l| l.parse().unwrap()).collect();

        Ok(System { moons })
    }
}

type Vector = (i64, i64, i64);

#[derive(Clone)]
struct Moon {
    position: Vector,
    velocity: Vector,
}

impl Moon {
    fn step_position(&mut self) {
        self.position.0 += self.velocity.0;
        self.position.1 += self.velocity.1;
        self.position.2 += self.velocity.2;
    }

    fn step_velocity(&mut self, other: &Moon) {
        self.velocity.0 += gravity(self.position.0, other.position.0);
        self.velocity.1 += gravity(self.position.1, other.position.1);
        self.velocity.2 += gravity(self.position.2, other.position.2);
    }

    fn energy(&self) -> i64 {
        (self.position.0.abs() + self.position.1.abs() + self.position.2.abs())
            * (self.velocity.0.abs() + self.velocity.1.abs() + self.velocity.2.abs())
    }
}

impl FromStr for Moon {
    type Err = String;

    fn from_str(value: &str) -> Result<Moon, String> {
        let mut data = value.split(',');
        let position = (
            data.next().unwrap().parse().unwrap(),
            data.next().unwrap().parse().unwrap(),
            data.next().unwrap().parse().unwrap(),
        );

        Ok(Moon {
            position,
            velocity: (0, 0, 0),
        })
    }
}

fn gravity(reference: i64, compare: i64) -> i64 {
    match reference.cmp(&compare) {
        Ordering::Greater => -1,
        Ordering::Equal => 0,
        Ordering::Less => 1,
    }
}
