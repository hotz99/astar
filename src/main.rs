mod astar;
mod grid;
mod obstacles;

use grid::Node;
use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

pub struct Model {
    grid: Vec<Vec<Node>>,
    grid_scale: u32,
    start: (u32, u32),
    goal: (u32, u32),
    path: Option<Vec<(u32, u32)>>,
}

// app state
// shared as reference to search() and others
fn model(app: &App) -> Model {
    let width: u32 = 30;
    let height: u32 = 30;

    let mut grid = Vec::new();

    for row in 0..height {
        let mut row_nodes = Vec::new();
        for col in 0..width {
            row_nodes.push(Node::new(row, col));
        }
        grid.push(row_nodes);
    }

    let start: (u32, u32) = (0, 0);
    let goal: (u32, u32) = (29, 29);

    obstacles::random(&mut grid, 45);

    // set start/goal node colors to blue/green
    grid[start.0 as usize][start.1 as usize].color_id = 0;
    grid[goal.0 as usize][goal.1 as usize].color_id = 1;

    let scale = 40;

    app.new_window()
        // fit window to grid
        .size(width * scale, height * scale)
        .view(grid::draw)
        .build()
        .unwrap();

    Model {
        grid: grid,
        grid_scale: scale,
        start: start,
        goal: goal,
        path: None,
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    if _model.path.is_none() {
        if let Some(path) = astar::search(_model) {
            _model.path = Some(path);
        } else {
            println!("astar failed");
        }
    }
}
