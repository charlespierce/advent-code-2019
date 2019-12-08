pub trait IO {
    fn next_input(&mut self) -> i32;
    fn next_output(&mut self, value: i32);
}

pub struct Computer {
    memory: Vec<i32>,
    pointer: usize,
}

impl Computer {
    pub fn new() -> Self {
        Computer {
            memory: vec![
                3, 8, 1001, 8, 10, 8, 105, 1, 0, 0, 21, 46, 67, 76, 97, 118, 199, 280, 361, 442,
                99999, 3, 9, 1002, 9, 3, 9, 101, 4, 9, 9, 102, 3, 9, 9, 1001, 9, 3, 9, 1002, 9, 2,
                9, 4, 9, 99, 3, 9, 102, 2, 9, 9, 101, 5, 9, 9, 1002, 9, 2, 9, 101, 2, 9, 9, 4, 9,
                99, 3, 9, 101, 4, 9, 9, 4, 9, 99, 3, 9, 1001, 9, 4, 9, 102, 2, 9, 9, 1001, 9, 4, 9,
                1002, 9, 5, 9, 4, 9, 99, 3, 9, 102, 3, 9, 9, 1001, 9, 2, 9, 1002, 9, 3, 9, 1001, 9,
                3, 9, 4, 9, 99, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 1001, 9,
                1, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 102, 2, 9,
                9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1001, 9, 2, 9,
                4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 99, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 101, 2, 9, 9,
                4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4,
                9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9,
                3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 99, 3, 9, 1002, 9, 2, 9, 4, 9,
                3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3,
                9, 102, 2, 9, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9,
                101, 1, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 99, 3, 9,
                1001, 9, 2, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9,
                101, 2, 9, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9,
                1001, 9, 2, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9,
                1001, 9, 1, 9, 4, 9, 99, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9,
                1002, 9, 2, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9,
                101, 1, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1001,
                9, 2, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 99,
            ],
            pointer: 0,
        }
    }

    fn read_parameters(&self, count: usize, instruction: i32) -> Vec<i32> {
        let mut params = Vec::new();
        let mut modes = instruction / 100;

        for i in 0..count {
            let mode = modes % 10;
            match mode {
                0 => {
                    let read_pos = self.memory[self.pointer + 1 + i];
                    params.push(self.memory[read_pos as usize]);
                }
                1 => {
                    params.push(self.memory[self.pointer + 1 + i]);
                }
                _ => panic!("Unexpected mode: {}", mode),
            }
            modes /= 10;
        }

        params
    }

    pub fn run(mut self, io: &mut impl IO) {
        loop {
            let instruction = self.memory[self.pointer];
            match instruction % 100 {
                1 => {
                    let params = self.read_parameters(2, instruction);
                    let write_pos = self.memory[self.pointer + 3] as usize;
                    self.memory[write_pos] = params[0] + params[1];
                    self.pointer += 4;
                }
                2 => {
                    let params = self.read_parameters(3, instruction);
                    let write_pos = self.memory[self.pointer + 3] as usize;
                    self.memory[write_pos] = params[0] * params[1];
                    self.pointer += 4;
                }
                3 => {
                    let write_pos = self.memory[self.pointer + 1] as usize;
                    self.memory[write_pos] = io.next_input();
                    self.pointer += 2;
                }
                4 => {
                    let params = self.read_parameters(1, instruction);
                    io.next_output(params[0]);
                    self.pointer += 2;
                }
                5 => {
                    let params = self.read_parameters(2, instruction);
                    if params[0] != 0 {
                        self.pointer = params[1] as usize;
                    } else {
                        self.pointer += 3;
                    }
                }
                6 => {
                    let params = self.read_parameters(2, instruction);
                    if params[0] == 0 {
                        self.pointer = params[1] as usize;
                    } else {
                        self.pointer += 3;
                    }
                }
                7 => {
                    let params = self.read_parameters(2, instruction);
                    let write_pos = self.memory[self.pointer + 3] as usize;
                    self.memory[write_pos] = if params[0] < params[1] { 1 } else { 0 };
                    self.pointer += 4;
                }
                8 => {
                    let params = self.read_parameters(2, instruction);
                    let write_pos = self.memory[self.pointer + 3] as usize;
                    self.memory[write_pos] = if params[0] == params[1] { 1 } else { 0 };
                    self.pointer += 4;
                }
                99 => break,
                _ => panic!("Unexpected instruction: {}", instruction),
            }
        }
    }
}
