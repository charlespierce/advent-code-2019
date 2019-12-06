use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let data = reader.lines().map(Result::unwrap);
    let map = Map::new(data);

    println!("Orbits: {}", map.total_orbits());
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

    fn total_orbits(&self) -> u32 {
        let total_orbit_helper = |leaf_id| {
            let mut cur = self.data.get(leaf_id).unwrap();
            let mut orbit_count = 0;

            while let Some(ref parent_id) = cur.parent {
                orbit_count += 1;
                cur = self.data.get(parent_id).unwrap();
            }

            orbit_count
        };

        self.data
            .keys()
            .fold(0, |acc, id| acc + total_orbit_helper(id))
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
