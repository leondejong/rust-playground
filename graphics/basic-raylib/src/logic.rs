use raylib::prelude::*;

use crate::state::*;

pub fn intersects(a: &Rectangle, b: &Rectangle) -> bool {
    return a.x < b.x + b.width
        && b.x < a.x + a.width
        && a.y < b.y + b.height
        && b.y < a.y + a.height;
}

pub fn get_translation(state: &State) -> (f32, f32, f32, f32) {
    let mut cx = 0.0;
    let mut cy = 0.0;
    let mut dx = state.vx;
    let mut dy = state.vy;

    let subject = &Rectangle::new(state.x, state.y, state.width, state.height);
    let horizontal = &Rectangle::new(state.x + dx, state.y, state.width, state.height);
    let vertical = &Rectangle::new(state.x, state.y + dy, state.width, state.height);

    for object in state.field.iter() {
        if intersects(object, horizontal) {
            if dx < 0.0 {
                dx = object.x + object.width - subject.x;
            } else if dx > 0.0 {
                dx = object.x - subject.x - subject.width;
            }
            cx = state.vx - dx;
        }
        if intersects(object, vertical) {
            if dy < 0.0 {
                dy = object.y + object.height - subject.y;
            } else if dy > 0.0 {
                dy = object.y - subject.y - subject.height;
            }
            cy = state.vy - dy;
        }
    }

    // delta x and y, correction x and y
    (dx, dy, cx, cy)
}

pub fn update_direction(handle: &mut RaylibHandle, state: &mut State) {
    state.up = handle.is_key_down(KeyboardKey::KEY_E);
    state.down = handle.is_key_down(KeyboardKey::KEY_D);
    state.left = handle.is_key_down(KeyboardKey::KEY_S);
    state.right = handle.is_key_down(KeyboardKey::KEY_F);
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

pub fn events(handle: &mut RaylibHandle, state: &mut State) {
    update_direction(handle, state);
    update_orientation(state);
}

pub fn update(handle: &mut RaylibHandle, state: &mut State) {
    state.delta = handle.get_frame_time();
    state.fps = handle.get_fps();

    state.speed = SPEED * state.delta;
    state.gravity = GRAVITY * state.delta;
    state.jump = JUMP * (1.0 / state.fps as f32); // corrects instable jump

    state.vx = state.speed * state.horizontal;
    state.vy += state.gravity;

    let (dx, dy, cx, cy) = get_translation(state);

    state.x += dx;
    state.y += dy;
    state.vx = if cx == 0.0 { dx } else { 0.0 };
    state.vy = if cy == 0.0 { dy } else { 0.0 };

    if state.up && cy > 0.1 && state.vy == 0.0 {
        state.vy = -state.jump;
    }
}

pub fn draw(handle: &mut RaylibDrawHandle, state: &State) {
    let text = format!("fps: {}", state.fps);

    handle.clear_background(BACKGROUND);
    handle.draw_text(&text, 28, 24, 20, TEXT);
    handle.draw_rectangle(
        state.x as i32,
        state.y as i32,
        state.width as i32,
        state.height as i32,
        CONTENT,
    );

    for rectangle in state.field.iter() {
        handle.draw_rectangle(
            rectangle.x as i32,
            rectangle.y as i32,
            rectangle.width as i32,
            rectangle.height as i32,
            FOREGROUND,
        );
    }
}
