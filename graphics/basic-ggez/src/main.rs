mod data;
mod logic;
mod state;

use ggez::{
    self,
    conf::{WindowMode, WindowSetup},
    event, ContextBuilder, GameResult,
};

use crate::data::{HEIGHT, TITLE, WIDTH};
use crate::state::State;

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
