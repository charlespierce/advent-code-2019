use std::io::stdin;

mod computer;

use computer::{Computer, IO};

fn main() {
    let computer = Computer::new();

    computer.run(&mut Ascii::new());
}

struct Ascii {
    input: String,
}

impl Ascii {
    fn new() -> Self {
        Ascii {
            input: "".to_string(),
        }
    }
}

impl IO for Ascii {
    fn next_input(&mut self) -> i64 {
        if self.input == "" {
            let mut s = String::new();
            stdin().read_line(&mut s).unwrap();
            // Deal with Windows end of line
            s.pop();
            s.pop();
            s.push('\n');
            self.input = s;
        }

        self.input.remove(0) as i64
    }

    fn next_output(&mut self, value: i64) {
        print!("{}", std::char::from_u32(value as u32).unwrap());
    }
}
