use crate::graphic::Graphic;
use crate::render::render;

#[derive(Default)]
pub struct State {
    pub graphics: Vec<Graphic>,
}

impl State {
    pub fn new() -> Self {
        State {
            graphics: Vec::new(),
        }
    }
}

impl State {
    pub fn render(&mut self, buffer: &mut Vec<u32>, scale: f64, width: u32, height: u32) {
        render(buffer, scale, width, height, &self.graphics);
    }
}
