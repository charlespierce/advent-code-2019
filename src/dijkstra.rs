use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::hash::Hash;
use std::rc::Rc;

pub struct Dijkstra<N, FS, FN, I>
where
    N: Eq + Hash,
    FS: FnMut(&N) -> bool,
    FN: FnMut(&N) -> I,
    I: IntoIterator<Item = (N, u64)>,
{
    unvisited: BinaryHeap<UnvisitedNode<N>>,
    visited: HashSet<Rc<N>>,
    success: FS,
    neighbors: FN,
}

impl<N, FS, FN, I> Dijkstra<N, FS, FN, I>
where
    N: Eq + Hash,
    FS: FnMut(&N) -> bool,
    FN: FnMut(&N) -> I,
    I: IntoIterator<Item = (N, u64)>,
{
    pub fn new(start: N, success: FS, neighbors: FN) -> Self {
        let first_node = Rc::new(start);
        let mut unvisited = BinaryHeap::new();

        unvisited.push(UnvisitedNode {
            value: Rc::clone(&first_node),
            cost: 0,
        });

        Dijkstra {
            unvisited,
            visited: HashSet::new(),
            success,
            neighbors,
        }
    }
}

impl<N, FS, FN, I> Iterator for Dijkstra<N, FS, FN, I>
where
    N: Eq + Hash,
    FS: FnMut(&N) -> bool,
    FN: FnMut(&N) -> I,
    I: IntoIterator<Item = (N, u64)>,
{
    type Item = (Rc<N>, u64);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(UnvisitedNode { value, cost }) = self.unvisited.pop() {
            if self.visited.contains(&value) {
                continue;
            }

            self.visited.insert(Rc::clone(&value));

            if (self.success)(&value) {
                return Some((value, cost));
            }

            for (node, move_cost) in (self.neighbors)(&value) {
                let node = Rc::new(node);
                if !self.visited.contains(&node) {
                    let new_cost = cost + move_cost;
                    self.unvisited.push(UnvisitedNode {
                        value: Rc::clone(&node),
                        cost: new_cost,
                    });
                }
            }
        }

        None
    }
}

struct UnvisitedNode<N> {
    value: Rc<N>,
    cost: u64,
}

impl<N> PartialEq for UnvisitedNode<N> {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl<N> Eq for UnvisitedNode<N> {}

impl<N> PartialOrd for UnvisitedNode<N> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<N> Ord for UnvisitedNode<N> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}
