pub struct Permuter {
    data: [i32; 5],
    first: bool,
}

impl Permuter {
    pub fn new() -> Self {
        Permuter {
            data: [0, 1, 2, 3, 4],
            first: true,
        }
    }
}

impl Iterator for Permuter {
    type Item = Vec<i32>;

    fn next(&mut self) -> Option<Vec<i32>> {
        if !self.first {
            let mut i = 3;
            let pivot = loop {
                if self.data[i] < self.data[i + 1] {
                    break i;
                }

                if i == 0 {
                    return None;
                }

                i -= 1;
            };

            i = 4;
            let target = loop {
                if self.data[i] > self.data[pivot] {
                    break i;
                }

                i -= 1;
            };

            self.data.swap(pivot, target);
            self.data[pivot + 1..].reverse();
        }

        self.first = false;
        Some(self.data.to_vec())
    }
}
