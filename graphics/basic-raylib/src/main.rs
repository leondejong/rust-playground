mod logic;
mod state;

use crate::logic::{draw, events, update};
use crate::state::{State, FPS, HEIGHT, TITLE, WIDTH};

fn main() {
    let mut state = State::new();

    let (mut handle, thread) = raylib::init().size(WIDTH, HEIGHT).title(TITLE).build();

    handle.set_target_fps(FPS);

    while !handle.window_should_close() {
        events(&mut handle, &mut state);
        update(&mut handle, &mut state);
        draw(&mut handle.begin_drawing(&thread), &state);
    }
}
