use std::cmp::Ordering;
use std::collections::HashSet;
use std::fs::read_to_string;
use std::str::FromStr;

fn main() {
    let x_period = calculate_period_axis(|v| v.0);
    println!("X Period: {}", x_period);
    let y_period = calculate_period_axis(|v| v.1);
    println!("Y Period: {}", y_period);
    let z_period = calculate_period_axis(|v| v.2);
    println!("Z Period: {}", z_period);

    let mut x = x_period;
    let mut y = y_period;
    let mut z = z_period;
    let mut divisor = 2;
    let mut lcm = 1;

    loop {
        if x == 1 && y == 1 && z == 1 {
            println!("Total Period: {}", lcm);
            break;
        }

        if x % divisor != 0 && y % divisor != 0 && z % divisor != 0 {
            divisor += 1;
            continue;
        }

        if x % divisor == 0 {
            x /= divisor;
        }

        if y % divisor == 0 {
            y /= divisor;
        }

        if z % divisor == 0 {
            z /= divisor;
        }

        lcm *= divisor;
    }
}

fn calculate_period_axis(selector: impl Fn(&Vector) -> i64) -> i64 {
    let mut system: System = read_to_string("input.txt").unwrap().parse().unwrap();

    let mut seen = HashSet::new();
    let mut i = 0;
    loop {
        let state = system.stringify(&selector);
        if seen.contains(&state) {
            break i;
        }
        seen.insert(state);

        i += 1;
        system.step();
    }
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

    fn stringify(&self, selector: &impl Fn(&Vector) -> i64) -> String {
        self.moons
            .iter()
            .map(|m| m.stringify(selector))
            .collect::<Vec<String>>()
            .join(",")
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

    fn stringify(&self, selector: &impl Fn(&Vector) -> i64) -> String {
        format!(
            "({}, {})",
            selector(&self.position),
            selector(&self.velocity)
        )
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
