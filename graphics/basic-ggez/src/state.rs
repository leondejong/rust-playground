use ggez::{
    self, event::EventHandler, graphics::Rect, input::keyboard::KeyInput, Context, GameError,
    GameResult,
};

use crate::logic::*;

pub const TITLE: &str = "GGEZ";
pub const WIDTH: f32 = 768.0;
pub const HEIGHT: f32 = 576.0;

pub const GRAVITY: f32 = 30.0;
pub const SPEED: f32 = 300.0;
pub const JUMP: f32 = 600.0;

pub const CONTENT: [f32; 4] = [1.0, 0.25, 0.0, 1.0];
pub const FOREGROUND: [f32; 4] = [0.875, 1.0, 0.0, 1.0];
pub const BACKGROUND: [f32; 4] = [0.0, 0.25, 0.375, 1.0];

pub struct State {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub vx: f32,
    pub vy: f32,
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub horizontal: f32,
    pub vertical: f32,
    pub field: Vec<Rect>,
}

impl State {
    pub fn new() -> GameResult<State> {
        Ok(Self {
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
        })
    }
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        update(self, ctx)
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        draw(self, ctx)
    }
    fn events(&mut self, input: KeyInput, value: bool) -> GameResult {
        update_direction(self, input, value);
        update_orientation(self);

        Ok(())
    }
}

impl EventHandler<GameError> for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.update(ctx)
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        self.draw(ctx)
    }
    fn key_down_event(&mut self, _ctx: &mut Context, input: KeyInput, _repeat: bool) -> GameResult {
        self.events(input, true)
    }
    fn key_up_event(&mut self, _ctx: &mut Context, input: KeyInput) -> GameResult {
        self.events(input, false)
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

pub fn tuple_to_rect(r: &(i32, i32, i32, i32)) -> Rect {
    Rect::new(r.0 as f32, r.1 as f32, r.2 as f32, r.3 as f32)
}
