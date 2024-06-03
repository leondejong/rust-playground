use crate::color::Color;
use crate::graphic::Graphic;
use crate::image::Image;
use crate::rectangle::Rectangle;

const ALPHA_MASK: u32 = 0x000000ff;

pub fn render(buffer: &mut [u8], scale: f64, width: u32, height: u32, graphics: &Vec<Graphic>) {
    for graphic in graphics.iter() {
        match graphic {
            Graphic::Background { color } => {
                render_background(buffer, color);
            }
            Graphic::Rectangle { rectangle, color } => {
                render_rectangle(buffer, scale, width, height, *rectangle, *color);
            }
            Graphic::Image { x, y, image } => {
                render_image(buffer, scale, width, height, image, *x, *y);
            }
        }
    }
}

pub fn render_background(frame: &mut [u8], color: &Color) {
    let value = color.to_array();

    for (_index, pixel) in frame.chunks_exact_mut(4).enumerate() {
        pixel.copy_from_slice(&value);
    }
}

pub fn render_rectangle(
    frame: &mut [u8],
    scale: f64,
    width: u32,
    _height: u32,
    rectangle: Rectangle,
    color: Color,
) {
    let value = color.to_array();
    let sw = width / scale as u32;

    let x = rectangle.x as u32;
    let y = rectangle.y as u32;
    let w = x + rectangle.width as u32;
    let h = y + rectangle.height as u32;

    for (index, pixel) in frame.chunks_exact_mut(4).enumerate() {
        let sx = index as u32 % sw;
        let sy = index as u32 / sw;

        if sx >= x && sx < w && sy >= y && sy < h {
            pixel.copy_from_slice(&value);
        }
    }
}

pub fn render_image(
    frame: &mut [u8],
    scale: f64,
    width: u32,
    _height: u32,
    image: &Image,
    image_x: f32,
    image_y: f32,
) {
    let sw = width / scale as u32;

    let x = image_x as u32;
    let y = image_y as u32;
    let w = x + image.width as u32;
    let h = y + image.height as u32;

    let mut iter = image.data.iter();

    for (index, pixel) in frame.chunks_exact_mut(4).enumerate() {
        let sx = index as u32 % sw;
        let sy = index as u32 / sw;

        if sx >= x && sx < w && sy >= y && sy < h {
            if let Some(color) = iter.next() {
                if color & ALPHA_MASK > 0 {
                    let color = Color::from_u32(*color);
                    pixel.copy_from_slice(&[color.red, color.green, color.blue, color.alpha]);
                }
            }
        }
    }
}
