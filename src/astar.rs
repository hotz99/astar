use std::collections::{BinaryHeap, HashSet, HashMap};

use crate::grid::Node;
use crate::Model;

pub fn search(model: &mut Model) -> Option<Vec<Node>> {
    let field = &model.field;
    let start = field[model.start.0 as usize][model.start.1 as usize];
    let goal = field[model.goal.0 as usize][model.goal.1 as usize];

    println!("{start:?}");
    println!("{goal:?}");
    
    // nodes to be evaluated
    let mut open: BinaryHeap<NodeState> = BinaryHeap::new();
    open.push(NodeState::new(start, 0, manhattan(&start, &goal)));

    // nodes already evaluated
    // no need to order closed nodes, so we use a (hash)set
    let mut closed: HashSet<Node> = HashSet::new();

    // children nodes come from parent nodes
    let mut parents: HashMap<Node, Node> = HashMap::new();

    while open.len() > 0 {
        // open is a min-heap, peek() returns the root of the heap
        // checking if peek() does not return None
        if let Some(&node_state) = open.peek() {
            let current = node_state.node;
            
            if current == goal {
                return Some(path(&parents, current));
            }
            
            open.pop();
            closed.insert(current);

            for neighbor in get_neighbors(field, &current) {
                if closed.contains(&neighbor) {
                    continue;
                }

                // tentative_g_score is the distance from start to the neighbor through current
                let tentative_g_score = node_state.g_cost + manhattan(&current, &neighbor);

                let mut neighbor_state = NodeState::new(neighbor, manhattan(&start, &neighbor), manhattan(&neighbor, &goal));

                // if this path to neighbor is better than previous one
                if tentative_g_score < neighbor_state.g_cost {
                    parents.insert(neighbor, current);
                    neighbor_state.g_cost = tentative_g_score;
                    neighbor_state.f_cost = tentative_g_score + neighbor_state.h_cost;

                    // binary heaps dont have .contains()
                    if !open.iter().any(|x| x.node == neighbor) {
                        open.push(neighbor_state);
                    }
                }
            }
        }
    }

    None
}

// our heuristic is the manhattan distance formula
fn manhattan(node: &Node, goal: &Node) -> u32 {
    let dx = node.col.abs_diff(goal.col);
    let dy = node.row.abs_diff(goal.row);

    dx + dy
}

fn path(parents: &HashMap<Node, Node>, node: Node) -> Vec<Node> {
    let mut current = node;
    let mut path = vec![node];

    while let Some(parent) = parents.get(&current) {
        path.insert(0, *parent);
        current = *parent;
    }

    path
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

// binary heap -> priority queue
// priority queue depends on 'Ord'
// implement the trait such that the bin heap becomes a min-heap

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct NodeState {
    node: Node,
    g_cost: u32,
    h_cost: u32,
    // f(n) = g(n) + h(n)
    f_cost: u32
}

impl NodeState {
    fn new(node: Node, g_cost: u32, h_cost: u32) -> NodeState {
        NodeState { node: (node), g_cost: (g_cost), h_cost: (h_cost), f_cost: (g_cost + h_cost) }
    }
}

impl PartialOrd for NodeState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.f_cost.partial_cmp(&self.f_cost)
    }
}

impl Ord for NodeState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.f_cost.cmp(&self.f_cost)
    }
}