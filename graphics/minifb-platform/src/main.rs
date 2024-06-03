mod color;
mod graphic;
mod image;
mod logic;
mod rectangle;
mod render;
mod state;

use std::{
    thread,
    time::{Duration, Instant},
};

use minifb::{Key, Window, WindowOptions};

use state::*;

fn main() {
    let mut state = State::new();
    run(&mut state);
}

// Custom game loop (more stable)
fn run(state: &mut State) {
    let duration = Duration::from_secs_f32(state.step as f32);
    let mut now = Instant::now();
    let mut elapsed;
    let mut delta;
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut window = Window::new(TITLE, WIDTH, HEIGHT, WindowOptions::default()).unwrap();

    window.set_target_fps(0);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        elapsed = now.elapsed();

        if duration > elapsed {
            delta = duration - elapsed;
        } else {
            delta = Duration::new(0, 0);
        }

        thread::sleep(delta);

        let delta = (delta + elapsed).as_secs_f32();

        now = Instant::now();

        state.update(delta, &window);
        state.render(&mut buffer, 1.0, WIDTH as u32, HEIGHT as u32);

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}

// MiniFB game loop
// pub fn run(state: &mut State) {
//     let mut now = Instant::now();
//     let mut elapsed;
//     let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
//     let mut window = Window::new(TITLE, WIDTH, HEIGHT, WindowOptions::default()).unwrap();
//     window.set_target_fps(FPS);
//     while window.is_open() && !window.is_key_down(Key::Escape) {
//         elapsed = now.elapsed();
//         now = Instant::now();

//         state.update((elapsed).as_secs_f32(), &window);
//         state.render(&mut buffer, 1.0, WIDTH as u32, HEIGHT as u32);

//         window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
//     }
// }
