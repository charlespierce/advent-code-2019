use std::fs::read_to_string;

fn main() {
    let mut deck = Vec::with_capacity(10_007);
    for i in 0..10_007 {
        deck.push(i as u64);
    }

    let shuffled = read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(Move::from)
        .fold(deck, |d, m| m.apply(d));

    let card_index = shuffled
        .iter()
        .enumerate()
        .find_map(|(i, val)| if *val == 2019 { Some(i) } else { None })
        .unwrap();

    println!("Card 2019 is at: {}", card_index);
}

enum Move {
    NewStack,
    Cut(i64),
    Increment(i64),
}

impl Move {
    fn apply(self, mut deck: Vec<u64>) -> Vec<u64> {
        match self {
            Move::NewStack => deck.into_iter().rev().collect(),
            Move::Cut(len) => {
                let index = if len < 0 {
                    let size = deck.len() as i64;
                    (size + len) as usize
                } else {
                    len as usize
                };
                let mut bottom = deck.split_off(index);
                bottom.append(&mut deck);
                bottom
            }
            Move::Increment(step) => {
                let mut result = Vec::from(&*deck);
                let mut index = 0;

                for val in deck {
                    result[index] = val;
                    index += step as usize;
                    index %= result.len();
                }

                result
            }
        }
    }
}

impl From<&str> for Move {
    fn from(s: &str) -> Self {
        let s = s.trim();
        if s == "deal into new stack" {
            return Move::NewStack;
        }

        if s.starts_with("cut ") {
            return Move::Cut(s[4..].parse().unwrap());
        }

        if s.starts_with("deal with increment ") {
            return Move::Increment(s[20..].parse().unwrap());
        }
        panic!("Unexpected move: {}", s);
    }
}
