use minifb::{Key, Window};

use crate::color::Color;
use crate::logic::{render, update};
use crate::rectangle::Rectangle;

pub const TITLE: &str = "MiniFB Example";

pub const WIDTH: usize = 768;
pub const HEIGHT: usize = 576;
pub const FPS: usize = 60;
pub const STEP: f32 = 1.0 / FPS as f32;

pub const GRAVITY: f32 = 30.0;
pub const SPEED: f32 = 300.0;
pub const JUMP: f32 = 600.0;

pub const CONTENT: Color = Color {
    red: 255,
    green: 63,
    blue: 0,
    alpha: 255,
};

pub const FOREGROUND: Color = Color {
    red: 223,
    green: 255,
    blue: 0,
    alpha: 255,
};

pub const BACKGROUND: Color = Color {
    red: 0,
    green: 63,
    blue: 95,
    alpha: 255,
};

pub struct State {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub vx: f32, // velocity x
    pub vy: f32, // velocity y
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub horizontal: f32,
    pub vertical: f32,
    pub step: f32,
    pub delta: f32,
    pub fps: usize,
    pub speed: f32,
    pub gravity: f32,
    pub jump: f32,
    pub field: Vec<Rectangle>,
}

impl State {
    pub fn update(&mut self, delta: f32, window: &Window) {
        self.delta = delta;
        self.fps = (1.0 / self.delta) as usize;

        // println!("{}, {}", self.delta, self.fps);

        self.left = window.is_key_down(Key::S);
        self.right = window.is_key_down(Key::F);
        self.up = window.is_key_down(Key::E);
        self.down = window.is_key_down(Key::D);

        update(self);
    }
    pub fn render(&mut self, buffer: &mut Vec<u32>, scale: f64, width: u32, height: u32) {
        render(self, buffer, scale, width, height);
    }
}

impl State {
    pub fn new() -> Self {
        Self {
            x: 32.0,
            y: 32.0,
            width: 16.0,
            height: 16.0,
            vx: 8.0,
            vy: 8.0,
            up: false,
            right: false,
            down: false,
            left: false,
            horizontal: 0.0,
            vertical: 0.0,
            delta: 0.0,
            fps: FPS,
            step: STEP,
            speed: 0.0,
            gravity: 0.0,
            jump: 0.0,
            field: get_field().iter().map(Rectangle::from_array).collect(),
        }
    }
}

pub fn get_field() -> Vec<[i32; 4]> {
    vec![
        // borders
        [0, 0, 768, 16],
        [0, 560, 768, 16],
        [0, 0, 16, 576],
        [752, 0, 16, 576],
        // floors
        [336, 144, 16, 288],
        [352, 144, 336, 16],
        [418, 236, 336, 16],
        [352, 326, 336, 16],
        [464, 416, 112, 16],
        [640, 416, 112, 16],
        [576, 486, 64, 16],
        // platforms
        [80, 486, 64, 16],
        [208, 416, 64, 16],
        [80, 348, 64, 16],
        [208, 280, 64, 16],
        [80, 212, 64, 16],
        [208, 144, 64, 16],
        // stairs
        [448, 432, 16, 16],
        [432, 448, 16, 16],
        [416, 464, 16, 16],
        [400, 480, 16, 16],
        [384, 496, 16, 16],
        [368, 512, 16, 16],
        [352, 528, 16, 16],
        [336, 544, 16, 16],
        // walls
        [420, 80, 16, 64],
        [588, 80, 16, 64],
        [504, 16, 16, 64],
    ]
}
