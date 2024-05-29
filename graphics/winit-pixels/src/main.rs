mod color;
mod graphic;
mod image;
mod rectangle;
mod render;
mod state;
mod window;

use color::Color;
use graphic::Graphic;
use rectangle::Rectangle;
use state::State;
use window::run;

fn main() {
    let mut state = State::new();

    let background_color = Color::new(15, 23, 31, 255);
    let rectangle_color = Color::new(7, 15, 23, 255);

    let background = Graphic::background(background_color);

    let dimensions = Rectangle::new(64.0, 64.0, 448.0, 384.0);
    let rectangle = Graphic::rectangle(dimensions, rectangle_color);

    let green_uri = "graphics/green.png";
    let green_image = Graphic::image(128.0, 128.0, green_uri);

    let purple_uri = "graphics/purple.png";
    let purple_rectangle = Rectangle::new(64.0, 64.0, 256.0, 192.0);
    let purple_image = Graphic::cropped_image(192.0, 192.0, purple_uri, purple_rectangle);

    let orange_uri = "graphics/orange.png";
    let orange_rectangle = Rectangle::new(128.0, 128.0, 192.0, 128.0);
    let orange_image = Graphic::cropped_image(256.0, 256.0, orange_uri, orange_rectangle);

    let graphics = vec![
        background,
        rectangle,
        green_image,
        purple_image,
        orange_image,
    ];

    state.graphics = graphics;

    run(state);
}
