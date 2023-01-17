use ggez::{
    self,
    conf::{WindowMode, WindowSetup},
    event, ContextBuilder, GameResult,
};

use crate::state::State;

mod logic;
mod state;

const TITLE: &str = "Nano";
const WIDTH: f32 = 768.0;
const HEIGHT: f32 = 576.0;

fn main() -> GameResult {
    let cb = ContextBuilder::new("game", "leon")
        .window_setup(WindowSetup::default().title(TITLE))
        .window_mode(WindowMode::default().dimensions(WIDTH, HEIGHT));
    let (ctx, evl) = cb.build()?;
    let state = State::new()?;

    event::run(ctx, evl, state)
}
