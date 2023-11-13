mod astar;
mod field;

use field::*;
use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

pub struct Model {
    field: Vec<Vec<i32>>,
    field_scale: u32,
    start: (u32, u32),
    goal: (u32, u32),
}

fn model(app: &App) -> Model {
    let WIDTH: u32 = 10;
    let HEIGHT: u32 = 10;
    let mut field = vec![vec![-1; WIDTH as usize]; HEIGHT as usize];
    let start: (u32, u32) = (8, 9);
    let goal: (u32, u32) = (3, 4);

    // setting colors for start/goal nodes
    field[start.0 as usize][start.1 as usize] = 0;
    field[goal.0 as usize][goal.1 as usize] = 1;

    let scale = 10;

    app.new_window()
        // fit the window to the grid
        .size(WIDTH * scale, HEIGHT * scale)
        // .size(512, 512)
        .view(field::draw)
        .build()
        .unwrap();

    Model {
        field: field,
        field_scale: scale,
        start: start,
        goal: goal,
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    // update your model here
}
