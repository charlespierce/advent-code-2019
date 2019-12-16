use std::fs::read_to_string;

mod repeater;

use repeater::Repeater;

fn main() {
    let mut digits = read_to_string("input.txt")
        .unwrap()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i64)
        .collect();

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

fn phase(input: Vec<i64>) -> Vec<i64> {
    (0..input.len())
        .map(|i| {
            input
                .iter()
                .zip(Repeater::new((i + 1) as i64))
                .map(|(a, b)| a * b)
                .sum::<i64>()
                .abs()
                % 10
        })
        .collect()
}
