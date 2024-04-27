use ggez::{
    self,
    glam::Vec2,
    graphics::{Canvas, Color, DrawMode, Mesh, Rect},
    input::keyboard::{KeyCode, KeyInput},
    Context, GameResult,
};

use crate::state::*;

pub fn intersects(a: &Rect, b: &Rect) -> bool {
    return a.x < b.x + b.w && b.x < a.x + a.w && a.y < b.y + b.h && b.y < a.y + a.h;
}

pub fn draw_rectangle(
    ctx: &mut Context,
    canvas: &mut Canvas,
    rectangle: &Rect,
    color: Color,
) -> GameResult {
    let shape = Mesh::new_rectangle(ctx, DrawMode::fill(), rectangle.clone(), color)?;

    canvas.draw(&shape, Vec2::new(0.0, 0.0));

    Ok(())
}

pub fn update_direction(state: &mut State, input: KeyInput, value: bool) {
    match input.keycode {
        Some(KeyCode::E) => state.up = value,
        Some(KeyCode::D) => state.down = value,
        Some(KeyCode::S) => state.left = value,
        Some(KeyCode::F) => state.right = value,
        _ => (),
    }
}

pub fn update_orientation(state: &mut State) {
    state.horizontal = 0.0;
    state.vertical = 0.0;

    if state.left && !state.right {
        state.horizontal = -1.0;
    }
    if state.right && !state.left {
        state.horizontal = 1.0;
    }
    if state.up && !state.down {
        state.vertical = -1.0;
    }
    if state.down && !state.up {
        state.vertical = 1.0;
    }
}

pub fn get_translation(state: &State) -> (f32, f32, f32, f32) {
    let mut cx = 0.0;
    let mut cy = 0.0;
    let mut dx = state.vx;
    let mut dy = state.vy;

    let subject = &Rect::new(state.x, state.y, state.w, state.h);
    let horizontal = &Rect::new(state.x + dx, state.y, state.w, state.h);
    let vertical = &Rect::new(state.x, state.y + dy, state.w, state.h);

    for object in state.field.iter() {
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

pub fn update(state: &mut State, ctx: &mut Context) -> GameResult {
    let delta = ctx.time.delta().as_secs_f32();
    let fps = ctx.time.fps();

    let speed = SPEED * delta;
    let gravity = GRAVITY * delta;
    let jump = JUMP * (1.0 / fps as f32); // corrects instable jump

    state.vx = speed * state.horizontal;
    state.vy += gravity;

    let (dx, dy, cx, cy) = get_translation(state);

    state.x += dx;
    state.y += dy;
    state.vx = if cx == 0.0 { dx } else { 0.0 };
    state.vy = if cy == 0.0 { dy } else { 0.0 };

    if state.up && cy > 0.1 && state.vy == 0.0 {
        state.vy = -jump;
    }

    Ok(())
}

pub fn draw(state: &mut State, ctx: &mut Context) -> GameResult {
    let mut canvas = Canvas::from_frame(ctx, Color::from(BACKGROUND));

    for object in state.field.iter() {
        draw_rectangle(ctx, &mut canvas, object, FOREGROUND.into())?;
    }

    draw_rectangle(
        ctx,
        &mut canvas,
        &Rect::new(state.x, state.y, state.w, state.h),
        CONTENT.into(),
    )?;

    canvas.finish(ctx)?;

    Ok(())
}
