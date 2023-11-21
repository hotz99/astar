use rand::Rng;

use crate::grid::Node;

pub fn random(field: &mut Vec<Vec<Node>>, num_obstacles: u32) {
    let mut rng = rand::thread_rng();

    for _ in 0..num_obstacles {
        let row = rng.gen_range(0..field.len());
        let col = rng.gen_range(0..field[0].len());

        field[row as usize][col as usize].color_id = 3;
    }
}
