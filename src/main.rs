use macroquad::prelude::*;

mod game;
mod palette;

const GRID_COLS: u32 = 20;
const GRID_ROWS: u32 = 20;

const MARGIN: f32 = 0.9;

const _: () = assert!(
    MARGIN > 0.0 && MARGIN <= 1.0,
    "MARGIN should be between 0 and 1"
);

#[macroquad::main("Rattler")]
async fn main() {
    let head = game::Vector2d { x: 5, y: 5 };
    let direction = game::Direction::Left;

    loop {
        let layout = game::Layout::from_screen(
            GRID_COLS,
            GRID_ROWS,
            MARGIN,
            screen_width(),
            screen_height(),
        );
        clear_background(palette::BACKGROUND);
        layout.draw_grid(GRID_COLS, GRID_ROWS, palette::GRID_LINE);
        let snake = game::Snake::new(head, 2, direction);
        // Draw snake
        let snake_body = snake.get_body();
        for (i, item) in snake_body.iter().enumerate() {
            if i == 0 {
                layout.draw_cell(*item, palette::SNAKE_HEAD);
            } else {
                layout.draw_cell(*item, palette::SNAKE_BODY);
            }
        }

        next_frame().await;
    }
}
