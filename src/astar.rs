use std::cmp::Ordering;
use std::collections::BinaryHeap;

// pub fn search(start: (usize, usize), goal: (usize, usize), field: Field) {
//     let mut openNodes: BinaryHeap<Node> = BinaryHeap::new();

//     while openNodes.len() > 0 {
//         let current = openNodes.peek();
//     }
// }

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Node {
    f_cost: i32,
    g_cost: i32,
    h_cost: i32,
    position: (i32, i32),
}

impl Node {
    pub fn new(x: i32, y: i32) -> Self {
        Node {
            f_cost: -1,
            g_cost: -1,
            h_cost: -1,
            position: (x, y),
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
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
