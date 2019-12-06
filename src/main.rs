use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let data = reader.lines().map(Result::unwrap);
    let map = Map::new(data);

    println!("Minimum Transfers: {}", map.orbital_transfers("YOU", "SAN"));
}

struct Map {
    data: HashMap<String, Body>,
}

impl Map {
    fn new(data: impl Iterator<Item = String>) -> Self {
        let mut map = Map {
            data: HashMap::new(),
        };

        for orbit in data {
            let mut elements = orbit.split(')');
            let com_id = elements.next().unwrap();
            let satellite_id = elements.next().unwrap();

            match map.data.get_mut(satellite_id) {
                Some(ref mut satellite) => {
                    if satellite.parent.is_some() {
                        panic!("Body Orbits Multiple Others: {}", satellite_id);
                    }

                    satellite.parent = Some(com_id.to_string());
                }
                None => {
                    map.data.insert(
                        satellite_id.to_string(),
                        Body::new(Some(com_id.to_string())),
                    );
                }
            }

            match map.data.get_mut(com_id) {
                Some(ref mut com) => {
                    com.satellites.push(satellite_id.to_string());
                }
                None => {
                    let mut com = Body::new(None);
                    com.satellites.push(satellite_id.to_string());
                    map.data.insert(com_id.to_string(), com);
                }
            }
        }

        map
    }

    fn orbital_transfers(&self, from: &str, to: &str) -> usize {
        let from_path = self.path(from);
        let to_path = self.path(to);

        let mut from_iter = from_path.iter();
        let mut to_iter = to_path.iter();

        loop {
            match (from_iter.next(), to_iter.next()) {
                (Some(a), Some(b)) if a == b => {}
                _ => break,
            }
        }

        from_iter.count() + to_iter.count()
    }

    fn path(&self, to: &str) -> Vec<String> {
        let mut cur = self.data.get(to).unwrap();
        let mut full_path = vec![to.to_string()];

        while let Some(ref parent_id) = cur.parent {
            full_path.insert(0, parent_id.to_string());
            cur = self.data.get(parent_id).unwrap();
        }

        full_path
    }
}

struct Body {
    parent: Option<String>,
    satellites: Vec<String>,
}

impl Body {
    fn new(parent: Option<String>) -> Self {
        Body {
            parent,
            satellites: Vec::new(),
        }
    }
}
