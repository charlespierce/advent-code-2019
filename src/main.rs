use std::fs::read_to_string;

use mod_exp::mod_exp;

fn main() {
    let single = read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(Move::from)
        .fold(Deck::new(119_315_717_514_047), |deck, mv| {
            deck.apply_move(mv)
        });

    let shuffled = single.iterate(101_741_582_076_661);

    println!("Position 2020: {}", shuffled.value(2020));
}

struct Deck {
    offset: i128,
    increment: i128,
    size: i128,
}

impl Deck {
    fn new(size: i128) -> Self {
        Deck {
            offset: 0,
            increment: 1,
            size,
        }
    }

    fn iterate(self, i: i128) -> Self {
        let increment = mod_exp(self.increment, i, self.size);
        let inv = mod_exp(
            i128::rem_euclid(1 - self.increment, self.size),
            self.size - 2,
            self.size,
        );
        let product = i128::rem_euclid(1 - mod_exp(self.increment, i, self.size), self.size);
        let product = i128::rem_euclid(product * inv, self.size);
        let offset = i128::rem_euclid(self.offset * product, self.size);

        Deck {
            offset,
            increment,
            size: self.size,
        }
    }

    fn value(&self, index: i128) -> i128 {
        let step = i128::rem_euclid(self.increment * index, self.size);
        i128::rem_euclid(self.offset + step, self.size)
    }

    fn apply_move(self, mv: Move) -> Self {
        match mv {
            Move::NewStack => {
                let increment = i128::rem_euclid(-self.increment, self.size);
                Deck {
                    offset: i128::rem_euclid(self.offset + increment, self.size),
                    increment,
                    size: self.size,
                }
            }
            Move::Cut(loc) => {
                let actual_cut = i128::rem_euclid(loc, self.size);
                let step = i128::rem_euclid(actual_cut * self.increment, self.size);
                Deck {
                    offset: i128::rem_euclid(self.offset + step, self.size),
                    increment: self.increment,
                    size: self.size,
                }
            }
            Move::Increment(step) => {
                let inv = mod_exp(step, self.size - 2, self.size);
                Deck {
                    offset: self.offset,
                    increment: i128::rem_euclid(self.increment * inv, self.size),
                    size: self.size,
                }
            }
        }
    }
}

enum Move {
    NewStack,
    Cut(i128),
    Increment(i128),
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
