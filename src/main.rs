use std::fs::read_to_string;
use std::hash::Hash;

mod dijkstra;
mod keys;
mod map;

use dijkstra::Dijkstra;
use keys::Keys;
use map::{Map, Point};

fn main() {
    let map = Map::from(read_to_string("input.txt").unwrap());

    let mut start = map.start.iter();
    let start = Node {
        pos: (
            *start.next().unwrap(),
            *start.next().unwrap(),
            *start.next().unwrap(),
            *start.next().unwrap(),
        ),
        keys: Keys::new(),
    };

    let maybe_distance = Dijkstra::new(
        start,
        |node| node.keys == map.all_keys,
        |node| find_neighbors_outer(&map, node),
    )
    .next();

    match maybe_distance {
        Some((_, distance)) => println!("Shortest Distance: {}", distance),
        None => println!("Nothing found!"),
    }
}

type Position = (Point, Point, Point, Point);

#[derive(Debug, PartialEq, Eq, Hash)]
struct Node {
    pos: Position,
    keys: Keys,
}

fn find_neighbors_outer(map: &Map, node: &Node) -> Vec<(Node, u64)> {
    Dijkstra::new(
        node.pos.0,
        |point| match map.keys.get(point) {
            Some(key) => !node.keys.contains_key(*key),
            None => false,
        },
        |point| {
            possible_neighbors(*point)
                .into_iter()
                .filter_map(|p| {
                    if map.can_access(p, node.keys) {
                        Some((p, 1))
                    } else {
                        None
                    }
                })
                .collect::<Vec<(Point, u64)>>()
        },
    )
    .map(|(point, cost)| {
        (
            Node {
                pos: (*point, node.pos.1, node.pos.2, node.pos.3),
                keys: node.keys.add_key(*map.keys.get(&point).unwrap()),
            },
            cost,
        )
    })
    .chain(
        Dijkstra::new(
            node.pos.1,
            |point| match map.keys.get(point) {
                Some(key) => !node.keys.contains_key(*key),
                None => false,
            },
            |point| {
                possible_neighbors(*point)
                    .into_iter()
                    .filter_map(|p| {
                        if map.can_access(p, node.keys) {
                            Some((p, 1))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<(Point, u64)>>()
            },
        )
        .map(|(point, cost)| {
            (
                Node {
                    pos: (node.pos.0, *point, node.pos.2, node.pos.3),
                    keys: node.keys.add_key(*map.keys.get(&point).unwrap()),
                },
                cost,
            )
        }),
    )
    .chain(
        Dijkstra::new(
            node.pos.2,
            |point| match map.keys.get(point) {
                Some(key) => !node.keys.contains_key(*key),
                None => false,
            },
            |point| {
                possible_neighbors(*point)
                    .into_iter()
                    .filter_map(|p| {
                        if map.can_access(p, node.keys) {
                            Some((p, 1))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<(Point, u64)>>()
            },
        )
        .map(|(point, cost)| {
            (
                Node {
                    pos: (node.pos.0, node.pos.1, *point, node.pos.3),
                    keys: node.keys.add_key(*map.keys.get(&point).unwrap()),
                },
                cost,
            )
        }),
    )
    .chain(
        Dijkstra::new(
            node.pos.3,
            |point| match map.keys.get(point) {
                Some(key) => !node.keys.contains_key(*key),
                None => false,
            },
            |point| {
                possible_neighbors(*point)
                    .into_iter()
                    .filter_map(|p| {
                        if map.can_access(p, node.keys) {
                            Some((p, 1))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<(Point, u64)>>()
            },
        )
        .map(|(point, cost)| {
            (
                Node {
                    pos: (node.pos.0, node.pos.1, node.pos.2, *point),
                    keys: node.keys.add_key(*map.keys.get(&point).unwrap()),
                },
                cost,
            )
        }),
    )
    .collect()
}

fn possible_neighbors(pos: Point) -> Vec<Point> {
    vec![
        (pos.0 - 1, pos.1),
        (pos.0 + 1, pos.1),
        (pos.0, pos.1 - 1),
        (pos.0, pos.1 + 1),
    ]
}
