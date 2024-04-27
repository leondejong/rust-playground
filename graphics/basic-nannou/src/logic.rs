use nannou::prelude::*;

use crate::state::*;

pub fn update(app: &App, model: &mut Model, update: Update) {
    let delta = update.since_last.as_secs_f32();
    let fps = app.fps();

    let speed = SPEED * delta;
    let gravity = GRAVITY * delta;
    let jump = JUMP * (1.0 / fps as f32); // corrects instable jump

    events(model, app);

    model.vx = speed * model.horizontal;
    model.vy += gravity;

    let (dx, dy, cx, cy) = get_translation(model);

    model.x += dx;
    model.y += dy;
    model.vx = if cx == 0.0 { dx } else { 0.0 };
    model.vy = if cy == 0.0 { dy } else { 0.0 };

    if model.up && cy > 0.1 && model.vy == 0.0 {
        model.vy = -jump;
    }
}

pub fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app
        .draw()
        .scale_y(-1.0)
        .x_y(-(WIDTH as f32) * 0.5, -(HEIGHT as f32) * 0.5);

    draw.background().color(tuple_to_color(&BACKGROUND));

    for object in model.field.iter() {
        draw_rectangle(&draw, object, &FOREGROUND);
    }

    draw_rectangle(
        &draw,
        &Rectangle::new(model.x, model.y, model.width, model.height),
        &CONTENT,
    );

    draw.to_frame(app, &frame).unwrap();
}

pub fn tuple_to_color(r: &(f32, f32, f32, f32)) -> Rgba {
    Rgba::new(r.0 as f32, r.1 as f32, r.2 as f32, r.3 as f32)
}

pub fn draw_rectangle(draw: &Draw, rectangle: &Rectangle, color: &(f32, f32, f32, f32)) {
    draw.rect()
        .x_y(
            rectangle.x + rectangle.width * 0.5,
            rectangle.y + rectangle.height * 0.5,
        )
        .w_h(rectangle.width, rectangle.height)
        .color(tuple_to_color(color));
}

pub fn intersects(a: &Rectangle, b: &Rectangle) -> bool {
    return a.x < b.x + b.width
        && b.x < a.x + a.width
        && a.y < b.y + b.height
        && b.y < a.y + a.height;
}

pub fn get_translation(model: &Model) -> (f32, f32, f32, f32) {
    let mut cx = 0.0;
    let mut cy = 0.0;
    let mut dx = model.vx;
    let mut dy = model.vy;

    let subject = &Rectangle::new(model.x, model.y, model.width, model.height);
    let horizontal = &Rectangle::new(model.x + dx, model.y, model.width, model.height);
    let vertical = &Rectangle::new(model.x, model.y + dy, model.width, model.height);

    for object in model.field.iter() {
        if intersects(object, horizontal) {
            if dx < 0.0 {
                dx = object.x + object.width - subject.x;
            } else if dx > 0.0 {
                dx = object.x - subject.x - subject.width;
            }
            cx = model.vx - dx;
        }
        if intersects(object, vertical) {
            if dy < 0.0 {
                dy = object.y + object.height - subject.y;
            } else if dy > 0.0 {
                dy = object.y - subject.y - subject.height;
            }
            cy = model.vy - dy;
        }
    }

    // delta x and y, correction x and y
    (dx, dy, cx, cy)
}

pub fn events(model: &mut Model, app: &App) {
    update_direction(model, app);
    update_orientation(model);
}

pub fn update_direction(model: &mut Model, app: &App) {
    model.up = app.keys.down.contains(&Key::E);
    model.down = app.keys.down.contains(&Key::D);
    model.left = app.keys.down.contains(&Key::S);
    model.right = app.keys.down.contains(&Key::F);
}

pub fn update_orientation(model: &mut Model) {
    model.horizontal = 0.0;
    model.vertical = 0.0;

    if model.left && !model.right {
        model.horizontal = -1.0;
    }
    if model.right && !model.left {
        model.horizontal = 1.0;
    }
    if model.up && !model.down {
        model.vertical = -1.0;
    }
    if model.down && !model.up {
        model.vertical = 1.0;
    }
}
