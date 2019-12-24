use std::collections::VecDeque;
use std::fs::read_to_string;

fn main() {
    let mut board = Board::from(read_to_string("input.txt").unwrap());

    for _ in 0..200 {
        board.step();
    }

    println!("Total Bugs: {}", board.bug_count());
}

const OUTER_MASK: u32 = 33_080_895;
const INNER_MASK: u32 = 70_720;

struct Board {
    maps: VecDeque<u32>,
}

impl Board {
    fn step(&mut self) {
        if *self.maps.front().unwrap() & OUTER_MASK > 0 {
            self.maps.push_front(0);
        }

        if *self.maps.back().unwrap() & INNER_MASK > 0 {
            self.maps.push_back(0);
        }

        let new_maps = self
            .maps
            .iter()
            .enumerate()
            .map(|(i, map)| {
                let mut new_map = 0;
                for y in 0..5 {
                    for x in 0..5 {
                        if x == 2 && y == 2 {
                            continue;
                        }

                        let count = self.neighbor_bug_count(x, y, i);
                        let mask = index(x, y);
                        if map & mask > 0 {
                            if count == 1 {
                                new_map |= mask;
                            }
                        } else if 1 <= count && count <= 2 {
                            new_map |= mask;
                        }
                    }
                }
                new_map
            })
            .collect();

        self.maps = new_maps;
    }

    fn neighbor_bug_count(&self, x: usize, y: usize, i: usize) -> u32 {
        let mut count = 0;
        if x == 0 {
            if i > 0 && self.maps[i - 1] & index(1, 2) > 0 {
                count += 1;
            }
        } else if self.maps[i] & index(x - 1, y) > 0 {
            count += 1;
        }

        if x == 4 {
            if i > 0 && self.maps[i - 1] & index(3, 2) > 0 {
                count += 1;
            }
        } else if self.maps[i] & index(x + 1, y) > 0 {
            count += 1;
        }

        if y == 0 {
            if i > 0 && self.maps[i - 1] & index(2, 1) > 0 {
                count += 1;
            }
        } else if self.maps[i] & index(x, y - 1) > 0 {
            count += 1;
        }

        if y == 4 {
            if i > 0 && self.maps[i - 1] & index(2, 3) > 0 {
                count += 1;
            }
        } else if self.maps[i] & index(x, y + 1) > 0 {
            count += 1;
        }

        if x == 2 && y == 1 && i < self.maps.len() - 1 {
            for n in 0..5 {
                if self.maps[i + 1] & index(n, 0) > 0 {
                    count += 1;
                }
            }
        }

        if x == 2 && y == 3 && i < self.maps.len() - 1 {
            for n in 0..5 {
                if self.maps[i + 1] & index(n, 4) > 0 {
                    count += 1;
                }
            }
        }

        if x == 1 && y == 2 && i < self.maps.len() - 1 {
            for n in 0..5 {
                if self.maps[i + 1] & index(0, n) > 0 {
                    count += 1;
                }
            }
        }

        if x == 3 && y == 2 && i < self.maps.len() - 1 {
            for n in 0..5 {
                if self.maps[i + 1] & index(4, n) > 0 {
                    count += 1;
                }
            }
        }

        count
    }

    fn bug_count(&self) -> u32 {
        let mut count = 0;

        for map in self.maps.iter() {
            let mut work = *map;
            while work > 0 {
                if (work & 1) > 0 {
                    count += 1;
                }
                work >>= 1;
            }
        }

        count
    }
}

impl From<String> for Board {
    fn from(value: String) -> Self {
        let map = value
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars().enumerate().map(
                    move |(x, chr)| {
                        if chr == '#' {
                            index(x, y)
                        } else {
                            0
                        }
                    },
                )
            })
            .fold(0, |map, point| map | point);

        Board {
            maps: VecDeque::from(vec![map]),
        }
    }
}

fn index(x: usize, y: usize) -> u32 {
    1 << (y * 5 + x)
}
