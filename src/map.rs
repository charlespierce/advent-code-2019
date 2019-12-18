use super::keys::Keys;
use std::collections::HashMap;

pub type Point = (i64, i64);

pub struct Map {
    pub data: HashMap<Point, char>,
    pub keys: HashMap<Point, char>,
    pub all_keys: Keys,
    pub start: Vec<Point>,
}

impl Map {
    pub fn can_access(&self, pos: Point, keys: Keys) -> bool {
        match self.data.get(&pos) {
            Some('.') | Some('@') => true,
            Some(key) if 'a' <= *key && *key <= 'z' => true,
            Some(door)
                if 'A' <= *door
                    && *door <= 'Z'
                    && keys.contains_key(door.to_lowercase().next().unwrap()) =>
            {
                true
            }
            _ => false,
        }
    }
}

impl From<String> for Map {
    fn from(value: String) -> Self {
        let mut data = HashMap::new();
        let mut keys = HashMap::new();
        let mut start = Vec::new();

        for (y, line) in value.lines().enumerate() {
            for (x, chr) in line.chars().enumerate() {
                data.insert((x as i64, y as i64), chr);
                if 'a' <= chr && chr <= 'z' {
                    keys.insert((x as i64, y as i64), chr);
                } else if chr == '@' {
                    start.push((x as i64, y as i64));
                }
            }
        }

        let all_keys = keys.values().cloned().into();

        Map {
            data,
            keys,
            all_keys,
            start,
        }
    }
}
