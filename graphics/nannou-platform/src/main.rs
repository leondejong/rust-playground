mod logic;
mod state;

use nannou::prelude::*;

use logic::{update, view};
use state::{Model, HEIGHT, TITLE, WIDTH};

fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(WIDTH, HEIGHT)
        .title(TITLE)
        .view(view)
        .build()
        .unwrap();

    Model::new()
}
