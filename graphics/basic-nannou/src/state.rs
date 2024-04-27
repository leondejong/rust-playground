pub const TITLE: &str = "Nannou";
pub const WIDTH: u32 = 768;
pub const HEIGHT: u32 = 576;

pub const GRAVITY: f32 = 30.0;
pub const SPEED: f32 = 300.0;
pub const JUMP: f32 = 600.0;

pub const CONTENT: (f32, f32, f32, f32) = (1.0, 0.25, 0.0, 1.0);
pub const FOREGROUND: (f32, f32, f32, f32) = (0.875, 1.0, 0.0, 1.0);
pub const BACKGROUND: (f32, f32, f32, f32) = (0.0, 0.25, 0.375, 1.0);

pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Rectangle {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
    pub fn from_tuple(tuple: &(i32, i32, i32, i32)) -> Self {
        Self {
            x: tuple.0 as f32,
            y: tuple.1 as f32,
            width: tuple.2 as f32,
            height: tuple.3 as f32,
        }
    }
}

pub struct Model {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub vx: f32,
    pub vy: f32,
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub horizontal: f32,
    pub vertical: f32,
    pub field: Vec<Rectangle>,
}

impl Model {
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
            field: get_field().iter().map(Rectangle::from_tuple).collect(),
        }
    }
}

pub fn get_field() -> Vec<(i32, i32, i32, i32)> {
    vec![
        // borders
        (0, 0, 768, 16),
        (0, 560, 768, 16),
        (0, 0, 16, 576),
        (752, 0, 16, 576),
        // floors
        (336, 144, 16, 288),
        (352, 144, 336, 16),
        (418, 236, 336, 16),
        (352, 326, 336, 16),
        (464, 416, 112, 16),
        (640, 416, 112, 16),
        (576, 486, 64, 16),
        // platforms
        (80, 486, 64, 16),
        (208, 416, 64, 16),
        (80, 348, 64, 16),
        (208, 280, 64, 16),
        (80, 212, 64, 16),
        (208, 144, 64, 16),
        // stairs
        (448, 432, 16, 16),
        (432, 448, 16, 16),
        (416, 464, 16, 16),
        (400, 480, 16, 16),
        (384, 496, 16, 16),
        (368, 512, 16, 16),
        (352, 528, 16, 16),
        (336, 544, 16, 16),
        // walls
        (420, 80, 16, 64),
        (588, 80, 16, 64),
        (504, 16, 16, 64),
    ]
}
