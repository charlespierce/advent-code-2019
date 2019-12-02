use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut total_fuel = 0;
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    for mass in reader.lines() {
        total_fuel += fuel_for_mass(mass.unwrap().parse().unwrap());
    }

    println!("Fuel: {}", total_fuel);
}

fn fuel_for_mass(mass: u32) -> u32 {
    (mass / 3) -2
}