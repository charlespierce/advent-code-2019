pub trait IO {
    fn next_input(&mut self) -> i64;
    fn next_output(&mut self, value: i64);
}

pub struct Computer {
    memory: Vec<i64>,
    pointer: usize,
    relative_base: i64,
}

impl Computer {
    pub fn new() -> Self {
        Computer {
            memory: vec![
                109, 424, 203, 1, 21101, 11, 0, 0, 1106, 0, 282, 21102, 18, 1, 0, 1106, 0, 259,
                2101, 0, 1, 221, 203, 1, 21101, 31, 0, 0, 1106, 0, 282, 21102, 1, 38, 0, 1106, 0,
                259, 21002, 23, 1, 2, 22102, 1, 1, 3, 21102, 1, 1, 1, 21101, 57, 0, 0, 1106, 0,
                303, 2101, 0, 1, 222, 21002, 221, 1, 3, 21001, 221, 0, 2, 21102, 259, 1, 1, 21102,
                80, 1, 0, 1106, 0, 225, 21102, 1, 79, 2, 21101, 0, 91, 0, 1106, 0, 303, 2102, 1, 1,
                223, 21001, 222, 0, 4, 21102, 259, 1, 3, 21101, 225, 0, 2, 21102, 1, 225, 1, 21101,
                0, 118, 0, 1105, 1, 225, 21002, 222, 1, 3, 21101, 118, 0, 2, 21101, 0, 133, 0,
                1106, 0, 303, 21202, 1, -1, 1, 22001, 223, 1, 1, 21102, 1, 148, 0, 1105, 1, 259,
                1202, 1, 1, 223, 20102, 1, 221, 4, 20101, 0, 222, 3, 21102, 1, 22, 2, 1001, 132,
                -2, 224, 1002, 224, 2, 224, 1001, 224, 3, 224, 1002, 132, -1, 132, 1, 224, 132,
                224, 21001, 224, 1, 1, 21102, 1, 195, 0, 105, 1, 109, 20207, 1, 223, 2, 21002, 23,
                1, 1, 21101, -1, 0, 3, 21102, 214, 1, 0, 1106, 0, 303, 22101, 1, 1, 1, 204, 1, 99,
                0, 0, 0, 0, 109, 5, 2101, 0, -4, 249, 22101, 0, -3, 1, 22102, 1, -2, 2, 21201, -1,
                0, 3, 21101, 0, 250, 0, 1105, 1, 225, 22101, 0, 1, -4, 109, -5, 2105, 1, 0, 109, 3,
                22107, 0, -2, -1, 21202, -1, 2, -1, 21201, -1, -1, -1, 22202, -1, -2, -2, 109, -3,
                2106, 0, 0, 109, 3, 21207, -2, 0, -1, 1206, -1, 294, 104, 0, 99, 22102, 1, -2, -2,
                109, -3, 2106, 0, 0, 109, 5, 22207, -3, -4, -1, 1206, -1, 346, 22201, -4, -3, -4,
                21202, -3, -1, -1, 22201, -4, -1, 2, 21202, 2, -1, -1, 22201, -4, -1, 1, 22102, 1,
                -2, 3, 21102, 343, 1, 0, 1106, 0, 303, 1105, 1, 415, 22207, -2, -3, -1, 1206, -1,
                387, 22201, -3, -2, -3, 21202, -2, -1, -1, 22201, -3, -1, 3, 21202, 3, -1, -1,
                22201, -3, -1, 2, 21201, -4, 0, 1, 21102, 384, 1, 0, 1105, 1, 303, 1106, 0, 415,
                21202, -4, -1, -4, 22201, -4, -3, -4, 22202, -3, -2, -2, 22202, -2, -4, -4, 22202,
                -3, -2, -3, 21202, -4, -1, -2, 22201, -3, -2, 1, 22101, 0, 1, -4, 109, -5, 2106, 0,
                0,
            ],
            pointer: 0,
            relative_base: 0,
        }
    }

    fn expand_memory(&mut self, index: usize) {
        if index > self.memory.len() - 1 {
            self.memory.resize(index + 1, 0);
        }
    }

    fn read_parameters(&mut self, count: usize, instruction: i64) -> Vec<i64> {
        let mut params = Vec::new();
        let mut modes = instruction / 100;

        for i in 0..count {
            let mode = modes % 10;
            match mode {
                0 => {
                    let read_pos = self.memory[self.pointer + 1 + i];
                    self.expand_memory(read_pos as usize);
                    params.push(self.memory[read_pos as usize]);
                }
                1 => {
                    params.push(self.memory[self.pointer + 1 + i]);
                }
                2 => {
                    let offset = self.memory[self.pointer + 1 + i];
                    let read_pos = self.relative_base + offset;
                    self.expand_memory(read_pos as usize);
                    params.push(self.memory[read_pos as usize]);
                }
                _ => panic!("Unexpected mode: {}", mode),
            }
            modes /= 10;
        }

        params
    }

    fn write_parameter(&mut self, index: usize, instruction: i64) -> i64 {
        let mode = (instruction / (100 * i64::pow(10, index as u32))) % 10;
        match mode {
            0 => {
                let pos = self.memory[self.pointer + 1 + index];
                self.expand_memory(pos as usize);
                pos
            }
            2 => {
                let pos = self.relative_base + self.memory[self.pointer + 1 + index];
                self.expand_memory(pos as usize);
                pos
            }
            _ => panic!("Unexpected mode: {}", mode),
        }
    }

    pub fn run(mut self, io: &mut impl IO) {
        loop {
            let instruction = self.memory[self.pointer];
            match instruction % 100 {
                1 => {
                    let params = self.read_parameters(2, instruction);
                    let write_pos = self.write_parameter(2, instruction) as usize;
                    self.memory[write_pos] = params[0] + params[1];
                    self.pointer += 4;
                }
                2 => {
                    let params = self.read_parameters(3, instruction);
                    let write_pos = self.write_parameter(2, instruction) as usize;
                    self.memory[write_pos] = params[0] * params[1];
                    self.pointer += 4;
                }
                3 => {
                    let write_pos = self.write_parameter(0, instruction) as usize;
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
                    let write_pos = self.write_parameter(2, instruction) as usize;
                    self.memory[write_pos] = if params[0] < params[1] { 1 } else { 0 };
                    self.pointer += 4;
                }
                8 => {
                    let params = self.read_parameters(2, instruction);
                    let write_pos = self.write_parameter(2, instruction) as usize;
                    self.memory[write_pos] = if params[0] == params[1] { 1 } else { 0 };
                    self.pointer += 4;
                }
                9 => {
                    let params = self.read_parameters(1, instruction);
                    self.relative_base += params[0];
                    self.pointer += 2;
                }
                99 => break,
                _ => panic!("Unexpected instruction: {}", instruction),
            }
        }
    }
}
