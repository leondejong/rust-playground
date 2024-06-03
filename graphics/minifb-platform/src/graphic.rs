use crate::color::Color;
use crate::image::Image;
use crate::rectangle::Rectangle;

#[derive(Debug, Clone)]
pub enum Graphic {
    Background {
        color: Color,
    },
    Rectangle {
        rectangle: Rectangle,
        color: Color,
    },
    #[allow(unused)]
    Image {
        x: f32,
        y: f32,
        image: Image,
    },
}

impl Graphic {
    pub fn background(color: Color) -> Self {
        Graphic::Background { color }
    }
    pub fn rectangle(rectangle: Rectangle, color: Color) -> Self {
        Graphic::Rectangle { rectangle, color }
    }
    #[allow(unused)]
    pub fn image(x: f32, y: f32, uri: &str) -> Self {
        let message = "Fetching image failed";
        let image = Image::load(uri).expect(message);

        Graphic::Image { x, y, image }
    }
    #[allow(unused)]
    pub fn cropped_image(x: f32, y: f32, uri: &str, rectangle: Rectangle) -> Self {
        let mut graphic = Graphic::image(x, y, uri);

        if let Graphic::Image {
            x: _,
            y: _,
            ref mut image,
        } = graphic
        {
            image.crop(rectangle);
        }

        graphic
    }
}
