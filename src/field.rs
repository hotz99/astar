use nannou::prelude::*;

pub struct Field {
    _window: window::Id,
    state: Vec<Vec<i32>>,
    // using usize to avoid casting
    scale: usize,
}

impl Field {
    pub fn new(app: &App) -> Self {
        let rows = 10;
        let cols = 10;

        let state = vec![vec![-1; rows]; cols];
        let scale = 20;

        let _window = app
            .new_window()
            // fit the window to the grid
            .size((cols * scale) as u32, (rows * scale) as u32)
            .view(Self::view)
            .build()
            .unwrap();

        Self {
            _window,
            state,
            scale,
        }
    }

    pub fn update(_app: &App, _Field: &mut Field, _update: Update) {}

    fn view(app: &App, field: &Field, frame: Frame) {
        let draw = app.draw();
        draw.background().color(LIGHTGRAY);

        let win = app.window_rect();

        // draw cells
        for i in 0..field.state.len() {
            for j in 0..field.state[0].len() {
                let color = Self::get_color_of_id(field.state[i][j]);

                // Rect has hella ctors, picked this one bc it's closer to
                // javaswing implementation.
                // the origin is the center, not top left corner.

                let side = (field.scale - 1) as f32;
                let cell = Rect::from_w_h(side, side).top_left_of(win);

                draw.rect()
                    .x(cell.x() + (j * field.scale) as f32)
                    .y(cell.y() - (i * field.scale) as f32)
                    .wh(cell.wh())
                    .color(CORNFLOWERBLUE);
            }
        }

        // TODO make the rows and cols align with the cells

        let color = BLACK;

        // draw rows
        for i in 0..=field.state.len() {
            let start = pt2((i * field.scale) as f32, 0.0);
            let end = pt2(
                (i * field.scale) as f32,
                (field.state[0].len() * field.scale) as f32,
            );
            // draw.line().start(start).end(end).stroke_weight(2.0).color(color);
        }

        // draw columns
        for i in 0..=field.state[0].len() {
            let start = pt2(0.0, (i * field.scale) as f32);
            let end = pt2(
                (field.state.len() * field.scale) as f32,
                (i * field.scale) as f32,
            );
            // draw.line().start(start).end(end).stroke_weight(2.0).color(color);
        }

        draw.to_frame(app, &frame).unwrap();
    }

    fn get_color_of_id(id: i32) -> Srgb<u8> {
        match id {
            -1 => BLACK,
            _ => WHITE,
        }
    }
}
