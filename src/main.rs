use macroquad::prelude::*;

#[macroquad::main("Snake")]
async fn main() {
    loop {
        clear_background(WHITE);
        let mut x = 0.0;
        while x < screen_width() {
            draw_line(x, 0.0, x, screen_height(), 3.0, LIGHTGRAY);
            x += 20.0;
        }
        let mut y = 0.0;
        while y < screen_height() {
            draw_line(0.0, y, screen_width(), y, 3.0, LIGHTGRAY);
            y += 20.0;
        }
        next_frame().await
    }
}
