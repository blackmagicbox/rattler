use macroquad::prelude::*;

const COL_WIDTH: u32 = 20;
const COL_HEIGHT: u32 = 20;
const CELL_SIZE: f32 = 20.0;

#[macroquad::main("Rattler")]
async fn main() {
    loop {
        let x0 = screen_width() / 2.0 - 10.0 * CELL_SIZE;
        let y0 = screen_height()/2.0 - 10.0 * CELL_SIZE;
        clear_background(WHITE);
        for n in 0..=COL_WIDTH {
            draw_line(
                x0 + (CELL_SIZE * n as f32),
                y0,
                x0 + (CELL_SIZE * n as f32),
                y0 + (CELL_SIZE * COL_HEIGHT as f32),
                1.0,
                LIGHTGRAY,
            );
        }
        for n in 0..=COL_WIDTH {
            draw_line(
                x0,
                y0 + (CELL_SIZE * n as f32),
                x0 + (CELL_SIZE * COL_WIDTH as f32),
                y0 + (CELL_SIZE * n as f32),
                1.0,
                LIGHTGRAY,
            );
        }
        next_frame().await
    }
}
