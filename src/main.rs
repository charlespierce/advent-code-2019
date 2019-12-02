fn main() {
    let computer = Computer::new(vec![1,0,0,3,1,1,2,3,1,3,4,3,1,5,0,3,2,6,1,19,1,19,10,23,2,13,23,27,1,5,27,31,2,6,31,35,1,6,35,39,2,39,9,43,1,5,43,47,1,13,47,51,1,10,51,55,2,55,10,59,2,10,59,63,1,9,63,67,2,67,13,71,1,71,6,75,2,6,75,79,1,5,79,83,2,83,9,87,1,6,87,91,2,91,6,95,1,95,6,99,2,99,13,103,1,6,103,107,1,2,107,111,1,111,9,0,99,2,14,0,0]);

    println!("66, 35: {}", computer.run(66, 35));
}

struct Computer {
    program: Vec<usize>,
}

impl Computer {
    fn new(program: Vec<usize>) -> Self {
        Computer { program }
    }

    fn run(&self, input_one: usize, input_two: usize) -> usize {
        let mut memory = self.program.clone();
        memory[1] = input_one;
        memory[2] = input_two;
        let mut pos = 0;
    
        loop {
            match memory[pos] {
                1 => {
                    let pos1 = memory[pos + 1];
                    let pos2 = memory[pos + 2];
                    let pos_result = memory[pos + 3];
                    memory[pos_result] = memory[pos1] + memory[pos2];
                },
                2 => {
                    let pos1 = memory[pos + 1];
                    let pos2 = memory[pos + 2];
                    let pos_result = memory[pos + 3];
                    memory[pos_result] = memory[pos1] * memory[pos2];
                },
                99 => break,
                code => panic!("Unknown Opcode: {}", code),
            };
            pos += 4;
        }
    
        memory[0]
    }
}
