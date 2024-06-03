mod logic;
mod state;

use macroquad::prelude::*;

use logic::{render, update};
use state::{State, HEIGHT, TITLE, WIDTH};

// Web build: `cargo build --target wasm32-unknown-unknown`
// Run server: `python3 -m http.server` or `php -S localhost:8000`

fn window_conf() -> Conf {
    Conf {
        window_title: TITLE.to_owned(),
        window_width: WIDTH,
        window_height: HEIGHT,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let state = &mut State::new();
    loop {
        update(state);
        render(state);
        next_frame().await
    }
}
