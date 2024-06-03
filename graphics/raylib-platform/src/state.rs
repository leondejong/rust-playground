use raylib::prelude::*;

pub const TITLE: &str = "Raylib";

pub const WIDTH: i32 = 768;
pub const HEIGHT: i32 = 576;
pub const FPS: u32 = 60;

pub const GRAVITY: f32 = 30.0;
pub const SPEED: f32 = 300.0;
pub const JUMP: f32 = 600.0;

pub const TEXT: Color = Color::new(255, 255, 255, 255);
pub const CONTENT: Color = Color::new(255, 63, 0, 255);
pub const FOREGROUND: Color = Color::new(223, 255, 0, 255);
pub const BACKGROUND: Color = Color::new(0, 63, 95, 255);

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
    pub delta: f32,
    pub fps: u32,
    pub speed: f32,
    pub gravity: f32,
    pub jump: f32,
    pub field: Vec<Rectangle>,
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
            speed: 0.0,
            gravity: 0.0,
            jump: 0.0,
            field: get_field().iter().map(tuple_to_rectangle).collect(),
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

pub fn tuple_to_rectangle(r: &(i32, i32, i32, i32)) -> Rectangle {
    Rectangle::new(r.0 as f32, r.1 as f32, r.2 as f32, r.3 as f32)
}
