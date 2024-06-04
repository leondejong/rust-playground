use crate::wasm::{draw_rectangle, draw_text};

use crate::state::{
    Rectangle, State, BACKGROUND, CONTENT, FOREGROUND, GRAVITY, HEIGHT, JUMP, SPEED, TEXT, WIDTH,
};

pub fn intersects(a: &Rectangle, b: &Rectangle) -> bool {
    return a.x < b.x + b.width
        && b.x < a.x + a.width
        && a.y < b.y + b.height
        && b.y < a.y + a.height;
}

pub fn get_translation(state: &State) -> (f64, f64, f64, f64) {
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

pub fn update_direction(state: &mut State, kind: &str, key: &str) {
    if kind == "keydown" {
        match key {
            "s" => state.left = true,
            "f" => state.right = true,
            "e" => state.up = true,
            "d" => state.down = true,
            _ => {}
        }
    }
    if kind == "keyup" {
        match key {
            "s" => state.left = false,
            "f" => state.right = false,
            "e" => state.up = false,
            "d" => state.down = false,
            _ => {}
        }
    }
}

pub fn update(state: &mut State) {
    state.speed = SPEED * state.delta;
    state.gravity = GRAVITY * state.delta;
    state.jump = JUMP * (1.0 / state.fps as f64); // corrects instable jump

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

pub fn render(state: &State) {
    let text = format!("fps: {}", state.fps);

    draw_rectangle(
        &state.context,
        0.0,
        0.0,
        WIDTH as f64,
        HEIGHT as f64,
        BACKGROUND,
    );

    for rectangle in state.field.iter() {
        draw_rectangle(
            &state.context,
            rectangle.x,
            rectangle.y,
            rectangle.width,
            rectangle.height,
            FOREGROUND,
        );
    }

    draw_text(&state.context, &text, 32.0, 40.0, "14px sans-serif", TEXT);

    draw_rectangle(
        &state.context,
        state.x,
        state.y,
        state.width,
        state.height,
        CONTENT,
    );
}
