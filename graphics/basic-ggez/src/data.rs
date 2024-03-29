use crate::logic::tuple_to_rect;
use crate::state::State;

pub const GRAVITY: f32 = 30.0;
pub const SPEED: f32 = 300.0;
pub const JUMP: f32 = 600.0;

pub const CONTENT: [f32; 4] = [1.0, 1.0, 0.0, 1.0];
pub const BACKGROUND: [f32; 4] = [0.0, 0.2, 0.2, 1.0];
pub const FOREGROUND: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

pub fn get_state() -> State {
    State {
        x: 32.0,
        y: 32.0,
        w: 16.0,
        h: 16.0,
        vx: 8.0,
        vy: 8.0,
        up: false,
        right: false,
        down: false,
        left: false,
        horizontal: 0.0,
        vertical: 0.0,
        field: get_field().iter().map(tuple_to_rect).collect(),
    }
}

pub fn get_field() -> Vec<(i32, i32, i32, i32)> {
    let field = vec![
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
    ];

    field
}
