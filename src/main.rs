use macroquad::prelude::*;

mod game;
mod palette;

const GRID_COLS: u32 = 20;
const GRID_ROWS: u32 = 20;

const MARGIN: f32 = 0.9;

// Time between two steps
const STEP_INTERVAL: f32 = 0.15;

// Biggest time interval per frame, to avoid "fast-forward" effect when the user gets back to the browser window
const MAX_FRAME_TIME: f32 = 0.25;

const _: () = assert!(
    MARGIN > 0.0 && MARGIN <= 1.0,
    "MARGIN should be between 0 and 1"
);

#[macroquad::main("Rattler")]
async fn main() {
    // ----- Initialize
    let head = game::Vector2d { x: 5, y: 5 };
    let mut snake = game::Snake::new(head, 3, game::Direction::Left);
    // ACCUMULATOR
    let mut acc: f32 = 0.0;

    // Game Loop
    loop {
        // -------- Listening to events
        // Keyboard
        if is_key_pressed(KeyCode::Up) {
            snake.set_direction(game::Direction::Up);
        }
        if is_key_pressed(KeyCode::Down) {
            snake.set_direction(game::Direction::Down);
        }
        if is_key_pressed(KeyCode::Left) {
            snake.set_direction(game::Direction::Left);
        }
        if is_key_pressed(KeyCode::Right) {
            snake.set_direction(game::Direction::Right);
        }

        #[cfg(not(target_arch = "wasm32"))]
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        if is_key_pressed(KeyCode::G) {
            snake.step(true);
        }
        // --------- Update
        acc += get_frame_time().min(MAX_FRAME_TIME);
        while acc >= STEP_INTERVAL {
            snake.step(false);
            acc -= STEP_INTERVAL;
        }

        // -------- Drawing
        let layout = game::Layout::from_screen(
            GRID_COLS,
            GRID_ROWS,
            MARGIN,
            screen_width(),
            screen_height(),
        );

        clear_background(palette::BACKGROUND);
        // Draw Grid
        layout.draw_grid(GRID_COLS, GRID_ROWS, palette::GRID_LINE);

        // Draw snake
        let snake_body = snake.body();
        for (i, item) in snake_body.iter().enumerate() {
            if i == 0 {
                layout.draw_cell(*item, palette::SNAKE_HEAD);
            } else {
                layout.draw_cell(*item, palette::SNAKE_BODY);
            }
        }

        // ------ Cleanup
        next_frame().await;
    }
}
