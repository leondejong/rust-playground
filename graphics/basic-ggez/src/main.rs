use ggez::{
    self,
    conf::{WindowMode, WindowSetup},
    event, ContextBuilder, GameResult,
};

use crate::state::State;

mod data;
mod logic;
mod state;

const TITLE: &str = "Basic";
const WIDTH: f32 = 768.0;
const HEIGHT: f32 = 576.0;

fn main() -> GameResult {
    let title = WindowSetup::default().title(TITLE);
    let dimensions = WindowMode::default().dimensions(WIDTH, HEIGHT);
    let cb = ContextBuilder::new("basic", "autor")
        .window_setup(title)
        .window_mode(dimensions);
    let (ctx, evl) = cb.build()?;
    let state = State::new()?;

    event::run(ctx, evl, state)
}
