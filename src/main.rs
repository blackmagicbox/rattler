use macroquad::prelude::*;

const COL_WIDTH: u32 = 20;
const COL_HEIGHT: u32 = 20;

#[macroquad::main("Rattler")]
async fn main() {
    loop {
        let cell_size:f32 = screen_width()/COL_WIDTH as f32;
        let x0 = screen_width() / 2.0 - 10.0 * cell_size;
        let y0 = screen_height()/2.0 - 10.0 * cell_size;
        clear_background(WHITE);
        for n in 0..=COL_WIDTH {
            draw_line(
                x0 + (cell_size * n as f32),
                y0,
                x0 + (cell_size * n as f32),
                y0 + (cell_size * COL_HEIGHT as f32),
                1.0,
                LIGHTGRAY,
            );
        }
        for n in 0..=COL_WIDTH {
            draw_line(
                x0,
                y0 + (cell_size * n as f32),
                x0 + (cell_size * COL_WIDTH as f32),
                y0 + (cell_size * n as f32),
                1.0,
                LIGHTGRAY,
            );
        }
        next_frame().await
    }
}
