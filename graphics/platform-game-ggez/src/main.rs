use ggez::{
    self,
    conf::{WindowMode, WindowSetup},
    event, ContextBuilder, GameResult,
};

mod models;
mod utils;

use models::{State, HEIGHT, TITLE, WIDTH};

fn main() -> GameResult {
    let cb = ContextBuilder::new("game", "leon")
        .window_setup(WindowSetup::default().title(TITLE))
        .window_mode(WindowMode::default().dimensions(WIDTH, HEIGHT));
    let (ctx, evl) = &mut cb.build()?;
    let state = &mut State::new(ctx)?;

    event::run(ctx, evl, state)
}
