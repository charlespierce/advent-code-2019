use std::fs::read_to_string;

fn main() {
    let mut digits = repeat(
        read_to_string("input.txt")
            .unwrap()
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect(),
        5_975_483,
        6_500_000,
    );

    for _ in 0..100 {
        digits = phase(digits);
    }

    print!("Result: {}", digits[0]);
    print!("{}", digits[1]);
    print!("{}", digits[2]);
    print!("{}", digits[3]);
    print!("{}", digits[4]);
    print!("{}", digits[5]);
    print!("{}", digits[6]);
    print!("{}", digits[7]);
    println!();
}

fn repeat(input: Vec<u32>, from: usize, to: usize) -> Vec<u32> {
    let mut output = Vec::new();

    for i in from..to {
        let index = i % input.len();
        output.push(input[index as usize]);
    }

    output
}

fn phase(input: Vec<u32>) -> Vec<u32> {
    let mut total: u32 = input.iter().sum();
    let mut output = Vec::new();

    for value in input.iter() {
        output.push(total % 10);
        total -= *value;
    }

    output
}
