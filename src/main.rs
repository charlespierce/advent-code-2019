use std::convert::TryFrom;
use std::str::Chars;

mod computer;

use computer::{Computer, IO};

fn main() {
    let computer = Computer::new();
    let mut ascii = Ascii::new();

    computer.run(&mut ascii);
}

const INPUT: &str = "NOT A J
NOT B T
OR T J
NOT C T
OR J T
AND D T
NOT E J
NOT J J
OR H J
AND T J
RUN
";

struct Ascii {
    chars: Chars<'static>,
}

impl Ascii {
    fn new() -> Self {
        Ascii {
            chars: INPUT.chars(),
        }
    }
}

impl IO for Ascii {
    fn next_input(&mut self) -> i64 {
        self.chars.next().unwrap() as i64
    }

    fn next_output(&mut self, value: i64) {
        match char::try_from(value as u32) {
            Ok(chr) => print!("{}", chr),
            Err(_) => println!("{}", value),
        }
    }
}
