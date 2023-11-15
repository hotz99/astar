use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet, HashMap};

use crate::Model;

pub fn search(model: &mut Model) {
    let field = &model.field;
    let start = field[model.start.0 as usize][model.start.1 as usize];
    let goal = field[model.goal.0 as usize][model.goal.1 as usize];

    println!("{start:?}");
    println!("{goal:?}");

    let mut g_score: HashMap<Node, u32> = HashMap::new();
    let mut f_score: HashMap<Node, f64> = HashMap::new();

    // 2d vec to 1d vec, probably inefficient ?
    let nodes: Vec<Node> = field.iter().flat_map(|row| row.iter().cloned()).collect();

    for node in nodes {
        g_score.insert(node, u32::MAX);
        f_score.insert(node, f64::MAX);
    }

    g_score.insert(start, 0);
    f_score.insert(start, euclidian_distance(&start, &goal));

    
    let mut open: BinaryHeap<Node> = BinaryHeap::new();
    // no need to order closed nodes, so we use a (hash)set
    let mut closed: HashSet<Node> = HashSet::new();

    while open.len() > 0 {
        // open is a min-heap, peek() returns the root of the heap
        // checking if peek() does not return None
        if let Some(&current) = open.peek() {
            open.pop();
            closed.insert(current);

            if current == goal {
                print!("DONE");
            }

            for neighbor in get_neighbors(field, &current) {
                println!("{neighbor:?}");

                if closed.contains(&neighbor) {
                    continue;
                }
            }
        }
    }
}

// heuristic
fn euclidian_distance(node: &Node, goal: &Node) -> f64 {
    let dx = ((node.col - goal.col) as f64).abs();
    let dy = ((node.row - goal.row) as f64).abs();
    ((dx * dx) + (dy * dy)).sqrt()
}

fn get_neighbors(field: &Vec<Vec<Node>>, node: &Node) -> Vec<Node> {
    let mut neighbors = Vec::new();

    let dr = [-1, -1, -1, 0, 0, 1, 1, 1];
    let dc = [-1, 0, 1, -1, 1, -1, 0, 1];

    for i in 0..8 {
        let new_row = node.row as i32 + dr[i];
        let new_col = node.col as i32 + dc[i];

        if new_row >= 0
            && new_row < field.len() as i32
            && new_col >= 0
            && new_col < field[0].len() as i32
        {
            neighbors.push(field[new_row as usize][new_col as usize]);
        }
    }

    neighbors
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct Node {
    pub row: u32,
    pub col: u32,
    pub f_cost: i32,
    pub g_cost: i32,
    pub h_cost: i32,
    pub color_id: i32,
}

impl Node {
    pub fn new(row: u32, col: u32) -> Self {
        Node {
            row: row,
            col: col,
            f_cost: -1,
            g_cost: -1,
            h_cost: -1,
            color_id: -1,
        }
    }
}

// implementation of the Ord trait +
// defining how the cmp function should work for Node
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        // ordering by lowest fcost makes our heap a min-heap
        other.f_cost.cmp(&self.f_cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
