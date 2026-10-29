use macroquad::prelude::*;

const GRID_COLS: u32 = 20;
const GRID_ROWS: u32 = 20;

const MARGIN: f32 = 0.9;

const _: () = assert!(
    MARGIN > 0.0 && MARGIN <= 1.0,
    "MARGIN should be between 0 and 1"
);

#[macroquad::main("Rattler")]
async fn main() {
    loop {
        let cell_w = screen_width() / GRID_COLS as f32;
        let cell_h = screen_height() / GRID_ROWS as f32;
        let cell_size = cell_w.min(cell_h) * MARGIN;

        let grid_w = cell_size * GRID_COLS as f32;
        let grid_h = cell_size * GRID_ROWS as f32;

        let x0 = (screen_width() - grid_w) / 2.0;
        let y0 = (screen_height() - grid_h) / 2.0;

        clear_background(DARKBLUE);

        for col in 0..=GRID_COLS {
            let x = x0 + cell_size * col as f32;
            draw_line(x, y0, x, y0 + grid_h, 1.0, LIGHTGRAY);
        }

        for row in 0..=GRID_ROWS {
            let y = y0 + cell_size * row as f32;
            draw_line(x0, y, x0 + grid_w, y, 1.0, LIGHTGRAY);
        }

        next_frame().await;
    }
}
