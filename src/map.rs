use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};

use crate::point::Point;

#[derive(Debug)]
pub struct Map {
    pub tunnels: HashSet<Point>,
    pub portals: HashMap<Point, Point>,
    pub start: Point,
    pub end: Point,
}

impl Map {
    pub fn get_neighbors(&self, from: Point) -> Vec<Point> {
        let mut output = Vec::new();

        for point in &[
            from.move_north(1),
            from.move_east(1),
            from.move_west(1),
            from.move_south(1),
        ] {
            if self.tunnels.contains(point) {
                output.push(*point);
            }
        }

        if let Some(other) = self.portals.get(&from) {
            output.push(*other);
        }

        output
    }
}

impl From<String> for Map {
    fn from(value: String) -> Self {
        let map: HashMap<Point, char> = value
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(move |(x, chr)| (Point::new(x as i64, y as i64), chr))
            })
            .collect();

        let mut tunnels = HashSet::new();
        let mut portals = HashMap::new();
        let mut portal_temp: HashMap<String, Point> = HashMap::new();

        for (point, chr) in map.iter() {
            match chr {
                '.' => {
                    tunnels.insert(*point);
                }
                label if 'A' <= *label && *label <= 'Z' => {
                    if let Some((label, portal)) = find_label_tunnel(point, &map) {
                        match portal_temp.entry(label) {
                            Entry::Vacant(e) => {
                                e.insert(portal);
                            }
                            Entry::Occupied(e) => {
                                let other = e.remove();
                                portals.insert(portal, other);
                                portals.insert(other, portal);
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        Map {
            tunnels,
            portals,
            start: portal_temp.remove("AA").unwrap(),
            end: portal_temp.remove("ZZ").unwrap(),
        }
    }
}

fn find_label_tunnel(label_start: &Point, map: &HashMap<Point, char>) -> Option<(String, Point)> {
    let mut label = String::new();
    label.push(*map.get(&label_start).unwrap());

    for trial in &[label_start.move_west(1), label_start.move_east(2)] {
        if let Some('.') = map.get(trial) {
            let other = *map.get(&label_start.move_east(1)).unwrap();
            if 'A' <= other && other <= 'Z' {
                label.push(other);
                return Some((label, *trial));
            }
        }
    }

    for trial in &[label_start.move_north(1), label_start.move_south(2)] {
        if let Some('.') = map.get(trial) {
            let other = *map.get(&label_start.move_south(1)).unwrap();
            if 'A' <= other && other <= 'Z' {
                label.push(other);
                return Some((label, *trial));
            }
        }
    }

    None
}
