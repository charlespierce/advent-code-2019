mod computer;
mod permuter;

use computer::{Computer, IO};
use permuter::Permuter;

fn main() {
    let mut maximum = 0;

    for v in Permuter::new() {
        let output = run_amplifiers(&v[..]);
        if output > maximum {
            maximum = output;
        }
    }

    println!("Maximum Output: {}", maximum);
}

fn run_amplifiers(phases: &[i32]) -> i32 {
    let mut register = 0;

    for phase in phases.iter() {
        let computer = Computer::new();
        let mut io = AmplifierIO::new(*phase, register);
        computer.run(&mut io);
        register = io.output;
    }

    register
}

struct AmplifierIO {
    inputs: std::vec::IntoIter<i32>,
    output: i32,
}

impl AmplifierIO {
    fn new(phase: i32, input: i32) -> Self {
        AmplifierIO {
            inputs: vec![phase, input].into_iter(),
            output: 0,
        }
    }
}

impl IO for AmplifierIO {
    fn next_input(&mut self) -> i32 {
        self.inputs.next().unwrap()
    }

    fn next_output(&mut self, output: i32) {
        self.output = output;
    }
}
