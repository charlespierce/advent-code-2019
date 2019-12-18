use std::hash::Hash;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Keys {
    set: u64,
}

impl Keys {
    pub fn new() -> Self {
        Keys { set: 0 }
    }

    pub fn contains_key(self, key: char) -> bool {
        self.set & bit_mask(key) > 0
    }

    pub fn add_key(self, key: char) -> Self {
        Keys {
            set: self.set | bit_mask(key),
        }
    }
}

impl<I> From<I> for Keys
where
    I: IntoIterator<Item = char>,
{
    fn from(key_list: I) -> Self {
        Keys {
            set: key_list.into_iter().fold(0, |acc, key| acc | bit_mask(key)),
        }
    }
}

fn bit_mask(key: char) -> u64 {
    1 << (key as u64 - 97)
}
