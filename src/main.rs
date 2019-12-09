use std::io::{stdin, stdout, Write};

mod computer;

use computer::{Computer, IO};

fn main() {
    let computer = Computer::new();
    computer.run(&mut ComputerIO);
}

struct ComputerIO;

impl IO for ComputerIO {
    fn next_input(&mut self) -> i64 {
        print!("Input value: ");
        stdout().flush().unwrap();
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        input.trim().parse().unwrap()
    }

    fn next_output(&mut self, output: i64) {
        println!("{}", output);
    }
}
