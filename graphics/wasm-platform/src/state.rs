use web_sys::CanvasRenderingContext2d;

use crate::wasm::{setup, Update};

pub const TITLE: &str = "WASM";

pub const WIDTH: u32 = 768;
pub const HEIGHT: u32 = 576;

pub const GRAVITY: f64 = 30.0;
pub const SPEED: f64 = 300.0;
pub const JUMP: f64 = 600.0;

pub const TEXT: &str = "rgba(255, 255, 255, 255)";
pub const CONTENT: &str = "rgba(255, 63, 0, 255)";
pub const FOREGROUND: &str = "rgba(223, 255, 0, 255)";
pub const BACKGROUND: &str = "rgba(0, 63, 95, 255)";

use crate::logic::{render, update, update_direction, update_orientation};

#[derive(Debug, Copy, Clone)]
pub struct Rectangle {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl Rectangle {
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
    pub fn from_array(r: &[i32; 4]) -> Rectangle {
        Rectangle::new(r[0] as f64, r[1] as f64, r[2] as f64, r[3] as f64)
    }
}

pub struct State {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub vx: f64, // velocity x
    pub vy: f64, // velocity y
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub horizontal: f64,
    pub vertical: f64,
    pub fps: u32,
    pub time: f64,
    pub delta: f64,
    pub speed: f64,
    pub gravity: f64,
    pub jump: f64,
    pub field: Vec<Rectangle>,
    pub context: CanvasRenderingContext2d,
}

impl State {
    pub fn new() -> Self {
        let (_, context) = setup();
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
            fps: 0,
            time: 0.0,
            delta: 0.0,
            speed: 0.0,
            gravity: 0.0,
            jump: 0.0,
            field: get_field().iter().map(Rectangle::from_array).collect(),
            context,
        }
    }
    pub fn events(&mut self, kind: &str, key: &str) {
        update_direction(self, kind, key);
        update_orientation(self);
    }
}

impl Update for State {
    fn update(&mut self, time: f64) {
        let delta = (time - self.time) / 1000.0;
        self.time = time;
        self.delta = delta;
        self.fps = (1.0 / delta) as u32;
        update(self);
        render(self);
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
