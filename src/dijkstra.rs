use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::hash::Hash;

pub trait Node {
    type Id: Eq + Hash;

    fn id(&self) -> Self::Id;
}

pub struct Dijkstra<N, FS, FN>
where
    N: Node,
{
    unvisited: BinaryHeap<UnvisitedNode<N>>,
    visited: HashSet<N::Id>,
    success: FS,
    neighbors: FN,
}

impl<N, FS, FN, I> Dijkstra<N, FS, FN>
where
    N: Node,
    FS: FnMut(&N) -> bool,
    FN: FnMut(&N) -> I,
    I: IntoIterator<Item = (N, usize)>,
{
    pub fn new(start: N, success: FS, neighbors: FN) -> Self {
        let mut unvisited = BinaryHeap::new();
        unvisited.push(UnvisitedNode {
            value: start,
            cost: 0,
        });

        Self {
            unvisited,
            success,
            neighbors,
            visited: HashSet::new(),
        }
    }
}

impl<N, FS, FN, I> Iterator for Dijkstra<N, FS, FN>
where
    N: Node,
    FS: FnMut(&N) -> bool,
    FN: FnMut(&N) -> I,
    I: IntoIterator<Item = (N, usize)>,
{
    type Item = (N, usize);

    fn next(&mut self) -> Option<(N, usize)> {
        while let Some(UnvisitedNode { value, cost }) = self.unvisited.pop() {
            let id = value.id();
            if self.visited.contains(&id) {
                continue;
            }

            self.visited.insert(id);

            if (self.success)(&value) {
                return Some((value, cost));
            }

            for (node, move_cost) in (self.neighbors)(&value) {
                if !self.visited.contains(&node.id()) {
                    let new_cost = cost + move_cost;
                    self.unvisited.push(UnvisitedNode {
                        value: node,
                        cost: new_cost,
                    });
                }
            }
        }

        None
    }
}

struct UnvisitedNode<N> {
    value: N,
    cost: usize,
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
