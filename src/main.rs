use std::str::Chars;

mod computer;

use computer::{Computer, IO};

fn main() {
    let computer = Computer::new();
    let mut ascii = Ascii::new();

    computer.run(&mut ascii);

    println!("Dust Collected: {}", ascii.dust);
}

const INPUT: &str = "A,B,A,B,C,C,B,A,B,C
L,4,R,8,L,6,L,10
L,6,R,8,R,10,L,6,L,6
L,4,L,4,L,10
n
";

struct Ascii {
    chars: Chars<'static>,
    dust: i64,
}

impl Ascii {
    fn new() -> Self {
        Ascii {
            chars: INPUT.chars(),
            dust: 0,
        }
    }
}

impl IO for Ascii {
    fn next_input(&mut self) -> i64 {
        self.chars.next().unwrap() as i64
    }

    fn next_output(&mut self, value: i64) {
        self.dust = value;
    }
}
