use std::collections::{HashMap, VecDeque};
use std::fs::read_to_string;

fn main() {
    let equations: HashMap<String, Equation> = read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|line| {
            let eq: Equation = line.into();
            (eq.output.name.clone(), eq)
        })
        .collect();
    let mut excess: HashMap<String, i64> = HashMap::new();
    let mut ore_count = 0;
    let mut queue = VecDeque::new();
    queue.push_back(Reactant {
        quantity: 1,
        name: "FUEL".into(),
    });

    while !queue.is_empty() {
        let reactant = queue.pop_front().unwrap();
        if reactant.name == "ORE" {
            ore_count += reactant.quantity;
        } else {
            let excess_quantity = *excess.get(&reactant.name).unwrap_or(&0);
            if excess_quantity >= reactant.quantity {
                excess.insert(reactant.name.clone(), excess_quantity - reactant.quantity);
            } else {
                let needed = reactant.quantity - excess_quantity;
                let source = equations.get(&reactant.name).unwrap();
                let multiplier = (needed + source.output.quantity - 1) / source.output.quantity;

                for mat in source.inputs.iter() {
                    queue.push_back(Reactant {
                        quantity: mat.quantity * multiplier,
                        name: mat.name.clone(),
                    });
                }

                excess.insert(
                    reactant.name.clone(),
                    source.output.quantity * multiplier - needed,
                );
            }
        }
    }

    println!("Total Ore: {}", ore_count);
}

struct Reactant {
    quantity: i64,
    name: String,
}

impl From<&str> for Reactant {
    fn from(val: &str) -> Self {
        let mut parts = val.split(' ');
        let quantity = parts.next().unwrap().parse().unwrap();
        let name = parts.next().unwrap().into();

        Reactant { quantity, name }
    }
}

struct Equation {
    inputs: Vec<Reactant>,
    output: Reactant,
}

impl From<&str> for Equation {
    fn from(val: &str) -> Self {
        let mut halves = val.split(" => ");
        let inputs = halves
            .next()
            .unwrap()
            .split(", ")
            .map(Reactant::from)
            .collect();
        let output = halves.next().unwrap().into();

        Equation { inputs, output }
    }
}
