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
    let mut game = game::Game::new(GRID_COLS, GRID_ROWS, STEP_INTERVAL);

    // Game Loop
    loop {
        // -------- Listening to events
        #[cfg(not(target_arch = "wasm32"))]
        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        game.handle_input();
        game.update(get_frame_time().min(MAX_FRAME_TIME));
        game.draw(MARGIN);

        next_frame().await;
    }
}
