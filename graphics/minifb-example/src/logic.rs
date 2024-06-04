use minifb::{Key, Window, WindowOptions};

use crate::state::State;

pub const FPS: usize = 60;
pub const WIDTH: usize = 576;
pub const HEIGHT: usize = 512;
pub const TITLE: &str = "MiniFB";

pub fn run(state: &mut State) {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut window = Window::new(TITLE, WIDTH, HEIGHT, WindowOptions::default()).unwrap();
    window.set_target_fps(FPS);
    while window.is_open() && !window.is_key_down(Key::Escape) {
        state.render(&mut buffer, 1.0, WIDTH as u32, HEIGHT as u32);
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
