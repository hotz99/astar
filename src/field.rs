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

        let state = vec![vec![-1; cols]; rows];
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

                let side = (field.scale - 1) as f32;

                // cell origin will be top left corner
                let cell = Rect::from_w_h(side, side).top_left_of(win);

                draw.rect()
                    .x(cell.x() + (j * field.scale) as f32)
                    .y(cell.y() - (i * field.scale) as f32)
                    .wh(cell.wh())
                    .color(CORNFLOWERBLUE);
            }
        }

        let color = BLACK;

        // draw rows
        for i in 0..=field.state.len() {
            let width = win.w();
            let height = win.h();

            let start = pt2(
                -width / 2.0,
                ((i * field.scale) as f32 - height / 2.0) as f32,
            );
            let end = pt2(
                (field.state.len() * field.scale) as f32 - width / 2.0,
                ((i * field.scale) as f32 - height / 2.0) as f32,
            );
            draw.line()
                .start(start)
                .end(end)
                .stroke_weight(2.0)
                .color(color);
        }

        // draw cols
        for i in 0..=field.state[0].len() {
            let width = win.w();
            let height = win.h();

            let start = pt2(
                ((i * field.scale) as f32 - width / 2.0) as f32,
                -height / 2.0,
            );
            let end = pt2(
                ((i * field.scale) as f32 - width / 2.0) as f32,
                (field.state[0].len() * field.scale) as f32 - height / 2.0,
            );
            draw.line()
                .start(start)
                .end(end)
                .stroke_weight(2.0)
                .color(color);
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
