use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};

use crate::point::{Location, Point};

#[derive(Debug)]
pub struct Map {
    pub tunnels: HashSet<Location>,
    pub inner_portals: HashMap<Location, Location>,
    pub outer_portals: HashMap<Location, Location>,
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
            if self.tunnels.contains(&point.loc) {
                output.push(*point);
            }
        }

        if from.level > 0 {
            if let Some(other) = self.outer_portals.get(&from.loc) {
                output.push(Point::new(*other, from.level - 1));
            }
        }

        if let Some(other) = self.inner_portals.get(&from.loc) {
            output.push(Point::new(*other, from.level + 1));
        }

        output
    }
}

impl From<String> for Map {
    fn from(value: String) -> Self {
        let map: HashMap<Location, char> = value
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(move |(x, chr)| (Location::new(x as i64, y as i64), chr))
            })
            .collect();

        let (max_x, max_y) = map.keys().fold((0, 0), |(mx, my), Location { x, y }| {
            let x_val = mx.max(*x);
            let y_val = my.max(*y);
            (x_val, y_val)
        });

        let mut tunnels = HashSet::new();
        let mut inner_portals = HashMap::new();
        let mut outer_portals = HashMap::new();
        let mut portal_temp: HashMap<String, Location> = HashMap::new();

        for (loc, chr) in map.iter() {
            match chr {
                '.' => {
                    tunnels.insert(*loc);
                }
                label if 'A' <= *label && *label <= 'Z' => {
                    if let Some((label, portal)) = find_label_tunnel(loc, &map) {
                        match portal_temp.entry(label) {
                            Entry::Vacant(e) => {
                                e.insert(portal);
                            }
                            Entry::Occupied(e) => {
                                let other = e.remove();

                                if portal.x == 2
                                    || portal.x == max_x - 2
                                    || portal.y == 2
                                    || portal.y == max_y - 2
                                {
                                    // Outer
                                    outer_portals.insert(portal, other);
                                    inner_portals.insert(other, portal);
                                } else {
                                    // Inner
                                    inner_portals.insert(portal, other);
                                    outer_portals.insert(other, portal);
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        Map {
            tunnels,
            inner_portals,
            outer_portals,
            start: Point::new(portal_temp.remove("AA").unwrap(), 0),
            end: Point::new(portal_temp.remove("ZZ").unwrap(), 0),
        }
    }
}

fn find_label_tunnel(
    label_start: &Location,
    map: &HashMap<Location, char>,
) -> Option<(String, Location)> {
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
