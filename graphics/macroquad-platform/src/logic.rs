use macroquad::prelude::*;

use crate::state::*;

pub fn update(state: &mut State) {
    let delta = get_frame_time();
    let fps = get_fps();

    let speed = SPEED * delta;
    let gravity = GRAVITY * delta;
    let jump = JUMP * (1.0 / fps as f32); // corrects instable jump

    events(state);

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
}

pub fn render(state: &State) {
    clear_background(tuple_to_color(BACKGROUND));

    for object in state.field.iter() {
        draw_rectangle(
            object.x,
            object.y,
            object.width,
            object.height,
            tuple_to_color(FOREGROUND),
        );
    }

    draw_rectangle(
        state.x,
        state.y,
        state.width,
        state.height,
        tuple_to_color(CONTENT),
    );
}

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

pub fn events(state: &mut State) {
    update_direction(state);
    update_orientation(state);
}

pub fn update_direction(state: &mut State) {
    state.up = is_key_down(KeyCode::E);
    state.down = is_key_down(KeyCode::D);
    state.left = is_key_down(KeyCode::S);
    state.right = is_key_down(KeyCode::F);
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
