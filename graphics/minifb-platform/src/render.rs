use crate::color::Color;
use crate::graphic::Graphic;
use crate::image::Image;
use crate::rectangle::Rectangle;

const ALPHA_MASK: u32 = 0x000000ff;

pub fn render_graphics(
    buffer: &mut [u32],
    scale: f64,
    width: u32,
    height: u32,
    graphics: &Vec<Graphic>,
) {
    for graphic in graphics.iter() {
        match graphic {
            Graphic::Background { color } => {
                render_background(buffer, *color);
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

pub fn render_background(buffer: &mut [u32], color: Color) {
    let value = color.to_u32(false);

    for pixel in buffer.iter_mut() {
        *pixel = value;
    }
}

pub fn render_rectangle(
    buffer: &mut [u32],
    scale: f64,
    width: u32,
    height: u32,
    rectangle: Rectangle,
    color: Color,
) {
    let value = color.to_u32(false);

    let s = scale as usize;

    let w = width as usize;
    let h = height as usize;

    let sx = s * rectangle.x as usize;
    let sy = s * rectangle.y as usize;

    let sxw = (sx + rectangle.width as usize * s).clamp(0, w);
    let syh = (sy + rectangle.height as usize * s).clamp(0, h);

    for y in sy..syh {
        for x in sx..sxw {
            buffer[x + y * w] = value;
        }
    }
}

pub fn render_image(
    buffer: &mut [u32],
    scale: f64,
    width: u32,
    height: u32,
    image: &Image,
    image_x: f32,
    image_y: f32,
) {
    let s = scale as i32;

    let w = width as i32;
    let h = height as i32;

    let ix = image_x as i32;
    let iy = image_y as i32;

    let iw = image.width as i32;
    // let ih = image.height as i32;

    let sx = s * image_x as i32;
    let sy = s * image_y as i32;

    let sw = s * image.width as i32;
    let sh = s * image.height as i32;

    let cx = s * (image_x as i32).clamp(0, w);
    let cy = s * (image_y as i32).clamp(0, h);

    let cw = (sx + sw).clamp(0, w);
    let ch = (sy + sh).clamp(0, h);

    for y in cy..ch {
        for x in cx..cw {
            let index = (x + y * w) as usize;
            let point = (x / s - ix + (y / s - iy) * iw) as usize;
            if image.data[point] & ALPHA_MASK > 0 {
                buffer[index] = image.data[point] >> 8;
            }
        }
    }
}
