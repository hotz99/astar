use crate::Model;
use nannou::prelude::*;

#[derive(Clone, Eq, PartialEq, Hash, Debug, Copy)]
pub struct Node {
    pub row: u32,
    pub col: u32,
    pub color_id: i32,
}

impl Node {
    pub fn new(row: u32, col: u32) -> Self {
        Node {
            row: row,
            col: col,
            color_id: -1,
        }
    }
}

pub fn draw(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(LIGHTGRAY);

    let win = app.window_rect();

    let grid = &model.grid;
    let scale = &model.grid_scale;
    
    // draw nodes
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            let node = &grid[i][j];

            let side = (scale - 1) as f32;
            
            // node origin will be top left corner
            let node_visual = Rect::from_w_h(side, side).top_left_of(win);

            // todo: display node fcost, gcost, hcost

            draw.rect()
                .x(node_visual.x() + (j as u32 * scale) as f32)
                .y(node_visual.y() - (i as u32 * scale) as f32)
                .wh(node_visual.wh())
                .color(get_color_of_id(node.color_id));
        }
    }

    if let Some(path) = &model.path {
        let side = (scale - 1) as f32;

        for (row, col) in path {
            // skip coloring start/goal nodes
            if model.grid[*row as usize][*col as usize].color_id != -1 {
                continue;
            }

            let node_visual = Rect::from_w_h(side, side).top_left_of(win);

            // todo: display node fcost, gcost, hcost

            draw.rect()
                .x(node_visual.x() + (col * scale) as f32)
                .y(node_visual.y() - (row * scale) as f32)
                .wh(node_visual.wh())
                .color(get_color_of_id(2));
        }
    }

    let color = BLACK;

    // draw rows
    for i in 0..=grid.len() {
        let width = win.w();
        let height = win.h();

        let start = pt2(
            -width / 2.0,
            ((i as u32 * scale) as f32 - height / 2.0) as f32,
        );
        let end = pt2(
            (grid.len() as u32 * scale) as f32 - width / 2.0,
            ((i as u32 * scale) as f32 - height / 2.0) as f32,
        );
        draw.line()
            .start(start)
            .end(end)
            .stroke_weight(4.0)
            .color(color);
    }

    // draw cols
    for i in 0..=grid[0].len() {
        let width = win.w();
        let height = win.h();

        let start = pt2(
            ((i as u32 * scale) as f32 - width / 2.0) as f32,
            -height / 2.0,
        );
        let end = pt2(
            ((i as u32 * scale) as f32 - width / 2.0) as f32,
            (grid[0].len() as u32 * scale) as f32 - height / 2.0,
        );
        draw.line()
            .start(start)
            .end(end)
            .stroke_weight(4.0)
            .color(color);

    }

    draw.to_frame(app, &frame).unwrap();
}

fn get_color_of_id(id: i32) -> Srgb<u8> {
    match id {
        -1 => LIGHTGREY,
        // start node
        0 => DODGERBLUE,
        // finish node
        1 => LIMEGREEN,
        2 => YELLOW,
        _ => LIGHTGREY,
    }
}
