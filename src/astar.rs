use std::cmp::Ordering;
use std::collections::BinaryHeap;

use nannou::App;
use nannou::geom::Rect;

use crate::field;

// pub fn search(start: (usize, usize), goal: (usize, usize), field: Field) {
//     let mut openNodes: BinaryHeap<Node> = BinaryHeap::new();

//     while openNodes.len() > 0 {
//         let current = openNodes.peek();
//     }
// }

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Node {
    pub f_cost: i32,
    pub g_cost: i32,
    pub h_cost: i32,
    pub colorId: i32
}

impl Node {
    pub fn new() -> Self {
        Node {
            f_cost: -1,
            g_cost: -1,
            h_cost: -1,
            colorId: -1
        }
    }
}

// implementation of the Ord trait +
// defining how the cmp function should work for Node
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        // ordering by lowest fcost makes our heap a min-heap
        other
            .f_cost
            .cmp(&self.f_cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
