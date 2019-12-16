const PATTERN: [i64; 4] = [0, 1, 0, -1];

pub struct Repeater {
    current: i64,
    count: i64,
}

impl Repeater {
    pub fn new(count: i64) -> Self {
        Repeater { current: 0, count }
    }
}

impl Iterator for Repeater {
    type Item = i64;

    fn next(&mut self) -> Option<i64> {
        self.current += 1;
        let index = (self.current / self.count) % 4;
        Some(PATTERN[index as usize])
    }
}
