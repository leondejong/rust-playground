use ggez::{
    self,
    event::EventHandler,
    graphics::{Canvas, Color, Rect},
    input::keyboard::{KeyCode, KeyInput},
    Context, GameError, GameResult,
};

use crate::logic::*;

const GRAVITY: f32 = 30.0;
const SPEED: f32 = 300.0;
const JUMP: f32 = 600.0;
const CONTENT: [f32; 4] = [1.0, 1.0, 0.0, 1.0];
const BACKGROUND: [f32; 4] = [0.0, 0.2, 0.2, 1.0];
const FOREGROUND: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

pub struct State {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub vx: f32,
    pub vy: f32,
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub horizontal: f32,
    pub vertical: f32,
}

impl State {
    pub fn new() -> GameResult<State> {
        Ok(State {
            x: 32.0,
            y: 32.0,
            w: 16.0,
            h: 16.0,
            vx: 8.0,
            vy: 8.0,
            up: false,
            right: false,
            down: false,
            left: false,
            horizontal: 0.0,
            vertical: 0.0,
        })
    }
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let delta = ctx.time.delta().as_secs_f32();
        let fps = ctx.time.fps();

        let speed = SPEED * delta;
        let gravity = GRAVITY * delta;
        let jump = JUMP * (1.0 / fps as f32); // corrects instable jump

        self.vx = speed * self.horizontal;
        self.vy += gravity;

        let (dx, dy, cx, cy) = get_translation(self);

        self.x += dx;
        self.y += dy;
        self.vx = if cx == 0.0 { dx } else { 0.0 };
        self.vy = if cy == 0.0 { dy } else { 0.0 };

        if self.up && cy > 0.1 && self.vy == 0.0 {
            self.vy = -jump;
        }

        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(ctx, Color::from(BACKGROUND));

        for rectangle in get_field().iter() {
            draw_rectangle(ctx, &mut canvas, rectangle, FOREGROUND.into())?;
        }

        draw_rectangle(
            ctx,
            &mut canvas,
            &Rect::new(self.x, self.y, self.w, self.h),
            CONTENT.into(),
        )?;

        canvas.finish(ctx)?;

        Ok(())
    }
    fn direction(&mut self) -> GameResult {
        self.horizontal = 0.0;
        self.vertical = 0.0;

        if self.left && !self.right {
            self.horizontal = -1.0;
        }
        if self.right && !self.left {
            self.horizontal = 1.0;
        }
        if self.up && !self.down {
            self.vertical = -1.0;
        }
        if self.down && !self.up {
            self.vertical = 1.0;
        }

        Ok(())
    }
    fn orientation(&mut self, input: KeyInput, value: bool) -> GameResult {
        match input.keycode {
            Some(KeyCode::E) => self.up = value,
            Some(KeyCode::D) => self.down = value,
            Some(KeyCode::S) => self.left = value,
            Some(KeyCode::F) => self.right = value,
            _ => (),
        }

        self.direction()
    }
}

impl EventHandler<GameError> for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.update(ctx)
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        self.draw(ctx)
    }
    fn key_down_event(&mut self, _ctx: &mut Context, input: KeyInput, _repeat: bool) -> GameResult {
        self.orientation(input, true)
    }
    fn key_up_event(&mut self, _ctx: &mut Context, input: KeyInput) -> GameResult {
        self.orientation(input, false)
    }
}
