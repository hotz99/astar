mod astar;
mod field;

use astar::Node;
use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

pub struct Model {
    field: Vec<Vec<Node>>,
    field_scale: u32,
    start: (u32, u32),
    goal: (u32, u32),
}

// app state, basically
// shared by reference to search() and others
fn model(app: &App) -> Model {
    let width: u32 = 10;
    let height: u32 = 10;
    
    let mut field = Vec::new();
    
    for row in 0..height {
        let mut row_nodes = Vec::new();
        for col in 0..width {
            row_nodes.push(Node::new(row, col));
        }
        field.push(row_nodes);
    }

    let start: (u32, u32) = (8, 9);
    let goal: (u32, u32) = (3, 4);

    // setting start/goal node colors to blue/green
    field[start.0 as usize][start.1 as usize].color_id = 0;
    field[goal.0 as usize][goal.1 as usize].color_id = 1;
    

    let scale = 50;

    app.new_window()
        // fit the window to the grid
        .size(width * scale, height * scale)
        // .size(512, 512)
        .view(field::draw)
        .build()
        .unwrap();

    let mut model = Model {
        field: field,
        field_scale: scale,
        start: start,
        goal: goal,
    };

    astar::search(&mut model);

    return model;
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    // update your model here
}
