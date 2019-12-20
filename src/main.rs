use std::fs::read_to_string;

mod dijkstra;
mod map;
mod point;

use dijkstra::Dijkstra;
use map::Map;

fn main() {
    let map = Map::from(read_to_string("input.txt").unwrap());

    let (_, distance) = Dijkstra::new(
        map.start,
        |point| *point == map.end,
        |point| {
            map.get_neighbors(*point)
                .into_iter()
                .map(|neighbor| (neighbor, 1))
        },
    )
    .next()
    .unwrap();

    println!("Shortest Path: {}", distance);
}
