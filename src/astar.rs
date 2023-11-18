use std::cmp::{Ordering, self};
use std::collections::{BinaryHeap, HashSet, HashMap};

use ordered_float::OrderedFloat;

use crate::grid::Node;
use crate::Model;

pub fn search(model: &mut Model) -> Option<Vec<(u32, u32)>> {
    let grid = &model.grid;
    let start = grid[model.start.0 as usize][model.start.1 as usize];
    let goal = grid[model.goal.0 as usize][model.goal.1 as usize];

    println!("{start:?}");
    println!("{goal:?}");
    
    // nodes to be evaluated
    let mut open: BinaryHeap<NodeState> = BinaryHeap::new();
    open.push(NodeState::new(start, OrderedFloat::from(0.0), euclidean(&start, &goal)));

    // nodes already evaluated
    // no need to order closed nodes, so we use a (hash)set
    let mut closed: HashSet<Node> = HashSet::new();

    // children nodes come from parent nodes
    let mut parents: HashMap<Node, Node> = HashMap::new();

    while open.len() > 0 {
        // open is a min-heap, peek() returns the root of the heap
        // checking if peek() does not return None
        if let Some(node_state) = open.pop() {
            let current = node_state.node;
            
            if current == goal {
                return Some(path(&parents, current));
            }
            
            closed.insert(current);

            for neighbor in get_neighbors(grid, &current) {
                if closed.contains(&neighbor) {
                    continue;
                }

                // tentative_g_score is the distance from start to the neighbor through current
                let tentative_g_score = node_state.g_cost + euclidean(&current, &neighbor);

                let mut neighbor_state = NodeState::new(neighbor, euclidean(&start, &neighbor), euclidean(&neighbor, &goal));

                // if this path to neighbor is better than previous one
                if tentative_g_score < neighbor_state.g_cost {
                    neighbor_state.g_cost = tentative_g_score;
                    neighbor_state.f_cost = tentative_g_score + neighbor_state.h_cost;

                        // binary heaps dont have .contains()
                } else if !open.iter().any(|x| x.node == neighbor) {
                    open.push(neighbor_state);
                }

                parents.insert(neighbor, current);
            }
        }
    }

    None
}

// heuristic
fn euclidean(node: &Node, goal: &Node) -> OrderedFloat<f32> {
    let dx = (node.col as f32 - goal.col as f32).abs();
    let dy = (node.row as f32 - goal.row as f32).abs();

    OrderedFloat::from((dx * dx + dy * dy).sqrt())
}

fn path(parents: &HashMap<Node, Node>, node: Node) -> Vec<(u32, u32)> {
    let mut current = node;
    let mut path = vec![(current.row, current.col)];

    while let Some(parent) = parents.get(&current) {
        path.insert(0, (parent.row, parent.col));
        current = *parent;
    }

    path
}


fn get_neighbors(grid: &Vec<Vec<Node>>, node: &Node) -> Vec<Node> {
    let mut neighbors = Vec::new();

    let dr = [-1, -1, -1, 0, 0, 1, 1, 1];
    let dc = [-1, 0, 1, -1, 1, -1, 0, 1];

    for i in 0..8 {
        let new_row = node.row as i32 + dr[i];
        let new_col = node.col as i32 + dc[i];

        if new_row >= 0
            && new_row < grid.len() as i32
            && new_col >= 0
            && new_col < grid[0].len() as i32
        {
            neighbors.push(grid[new_row as usize][new_col as usize]);
        }
    }

    neighbors
}

// binary heap -> priority queue
// priority queue depends on 'Ord'
// implement the trait such that the bin heap becomes a min-heap

#[derive(Clone, PartialEq, Eq)]
struct NodeState {
    node: Node,
    g_cost: OrderedFloat<f32>,
    h_cost: OrderedFloat<f32>,
    // f(n) = g(n) + h(n)
    f_cost: OrderedFloat<f32>
}

impl NodeState {
    fn new(node: Node, g_cost: OrderedFloat<f32>, h_cost: OrderedFloat<f32>) -> NodeState {
        NodeState { node: (node), g_cost: (g_cost), h_cost: (h_cost), f_cost: (g_cost + h_cost) }
    }
}

impl Ord for NodeState {
    fn cmp(&self, other: &Self) -> Ordering {
        // Use f_cost for ordering (min-heap)
        self.f_cost.partial_cmp(&other.f_cost).unwrap_or(Ordering::Equal).reverse()
    }
}

impl PartialOrd for NodeState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}