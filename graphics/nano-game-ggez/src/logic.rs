use ggez::{
    self,
    glam::Vec2,
    graphics::{Canvas, Color, DrawMode, Mesh, Rect},
    Context, GameResult,
};

use crate::state::State;

pub fn tuple_to_rect(r: &(i32, i32, i32, i32)) -> Rect {
    Rect::new(r.0 as f32, r.1 as f32, r.2 as f32, r.3 as f32)
}

pub fn intersects(a: &Rect, b: &Rect) -> bool {
    return a.x < b.x + b.w && b.x < a.x + a.w && a.y < b.y + b.h && b.y < a.y + a.h;
}

pub fn draw_rectangle(
    ctx: &mut Context,
    canvas: &mut Canvas,
    rectangle: &Rect,
    color: Color,
) -> GameResult {
    let rect = Rect::new(rectangle.x, rectangle.y, rectangle.w, rectangle.h);
    let shape = Mesh::new_rectangle(ctx, DrawMode::fill(), rect, color)?;

    canvas.draw(&shape, Vec2::new(0.0, 0.0));

    Ok(())
}

pub fn get_translation(state: &State) -> (f32, f32, f32, f32) {
    let mut cx = 0.0;
    let mut cy = 0.0;
    let mut dx = state.vx;
    let mut dy = state.vy;

    let subject = &Rect::new(state.x, state.y, state.w, state.h);
    let horizontal = &Rect::new(state.x + dx, state.y, state.w, state.h);
    let vertical = &Rect::new(state.x, state.y + dy, state.w, state.h);

    for object in get_field().iter() {
        if intersects(object, horizontal) {
            if dx < 0.0 {
                dx = object.x + object.w - subject.x;
            } else if dx > 0.0 {
                dx = object.x - subject.x - subject.w;
            }
            cx = state.vx - dx;
        }
        if intersects(object, vertical) {
            if dy < 0.0 {
                dy = object.y + object.h - subject.y;
            } else if dy > 0.0 {
                dy = object.y - subject.y - subject.h;
            }
            cy = state.vy - dy;
        }
    }

    // delta x and y, correction x and y
    (dx, dy, cx, cy)
}

pub fn get_field() -> Vec<Rect> {
    let field = vec![
        // borders
        (0, 0, 768, 16),
        (0, 560, 768, 16),
        (0, 0, 16, 576),
        (752, 0, 16, 576),
        // floors
        (336, 144, 16, 288),
        (352, 144, 336, 16),
        (418, 236, 336, 16),
        (352, 326, 336, 16),
        (464, 416, 112, 16),
        (640, 416, 112, 16),
        (576, 486, 64, 16),
        // platforms
        (80, 486, 64, 16),
        (208, 416, 64, 16),
        (80, 348, 64, 16),
        (208, 280, 64, 16),
        (80, 212, 64, 16),
        (208, 144, 64, 16),
        // stairs
        (448, 432, 16, 16),
        (432, 448, 16, 16),
        (416, 464, 16, 16),
        (400, 480, 16, 16),
        (384, 496, 16, 16),
        (368, 512, 16, 16),
        (352, 528, 16, 16),
        (336, 544, 16, 16),
        // walls
        (420, 80, 16, 64),
        (588, 80, 16, 64),
        (504, 16, 16, 64),
    ];

    field.iter().map(tuple_to_rect).collect()
}
