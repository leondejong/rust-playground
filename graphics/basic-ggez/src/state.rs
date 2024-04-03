use ggez::{
    self, event::EventHandler, graphics::Rect, input::keyboard::KeyInput, Context, GameError,
    GameResult,
};

use crate::data::*;
use crate::logic::*;

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
        Ok(get_state())
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
