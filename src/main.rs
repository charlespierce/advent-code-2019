use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut total_fuel = 0;
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    for mass in reader.lines() {
        total_fuel += fuel_for_module(mass.unwrap().parse().unwrap());
    }

    println!("Fuel: {}", total_fuel);
}

fn fuel_for_module(mass: i32) -> i32 {
    let mut fuel_needed = 0;
    let mut current_fuel = fuel_for_mass(mass);

    loop {
        fuel_needed += current_fuel;
        current_fuel = fuel_for_mass(current_fuel);

        if current_fuel == 0 {
            break;
        }
    }

    fuel_needed
}

fn fuel_for_mass(mass: i32) -> i32 {
    let fuel = (mass / 3) - 2;

    if fuel < 0 {
        0
    } else {
        fuel
    }
}