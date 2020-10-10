use ggez::{
    self,
    event::{EventHandler, KeyCode, KeyMods},
    graphics, Context, GameResult,
};

use crate::utils::*;

pub const TITLE: &str = "Nano Platformer";
pub const WIDTH: f32 = 768.0;
pub const HEIGHT: f32 = 576.0;
pub const UNIT: f32 = 16.0;

const GRAVITY: f32 = 0.5;
const SPEED: f32 = 5.0;
const JUMP: f32 = 10.0;
const COLOR: [f32; 4] = [1.0, 1.0, 0.0, 1.0];
const BACKGROUND: [f32; 4] = [0.0, 0.2, 0.2, 1.0];
const FOREGROUND: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl Rectangle {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self { x, y, w, h }
    }
}

pub struct State {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub vx: f32,
    pub vy: f32,
    pub up: bool,
    pub right: bool,
    pub down: bool,
    pub left: bool,
    pub field: Vec<Rectangle>,
}

impl State {
    pub fn new(_ctx: &mut Context) -> GameResult<State> {
        let state = State {
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
            field: get_field(),
        };

        Ok(state)
    }
    fn update(&mut self) -> GameResult {
        let (direction, _) = get_direction(self);

        self.vx = SPEED * direction;
        self.vy += GRAVITY;

        let (dx, dy, cx, cy) = get_translation(self);

        self.x += dx;
        self.y += dy;
        self.vx = if cx == 0.0 { dx } else { 0.0 };
        self.vy = if cy == 0.0 { dy } else { 0.0 };

        if self.up && cy > 0.0 {
            self.vy -= JUMP;
        }

        Ok(())
    }
    fn draw_boundaries(&mut self, ctx: &mut Context) -> GameResult {
        for rect in get_boundaries().iter() {
            draw_rectangle(ctx, rect, BACKGROUND.into())?;
        }

        Ok(())
    }
    fn draw_field(&mut self, ctx: &mut Context) -> GameResult {
        for rect in self.field.iter() {
            draw_rectangle(ctx, rect, FOREGROUND.into())?;
        }

        Ok(())
    }
    fn draw_player(&mut self, ctx: &mut Context) -> GameResult {
        draw_rectangle(
            ctx,
            &Rectangle::new(self.x, self.y, self.w, self.h),
            COLOR.into(),
        )?;

        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, BACKGROUND.into());

        self.draw_boundaries(ctx)?;
        self.draw_field(ctx)?;
        self.draw_player(ctx)?;

        graphics::present(ctx)?;

        Ok(())
    }
}

impl EventHandler for State {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.update()
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        self.draw(ctx)
    }

    fn key_down_event(&mut self, _: &mut Context, keycode: KeyCode, _: KeyMods, _: bool) {
        match keycode {
            KeyCode::Up | KeyCode::W => self.up = true,
            KeyCode::Right | KeyCode::D => self.right = true,
            KeyCode::Down | KeyCode::S => self.down = true,
            KeyCode::Left | KeyCode::A => self.left = true,
            _ => (),
        }
    }

    fn key_up_event(&mut self, _: &mut Context, keycode: KeyCode, _: KeyMods) {
        match keycode {
            KeyCode::Up | KeyCode::W => self.up = false,
            KeyCode::Right | KeyCode::D => self.right = false,
            KeyCode::Down | KeyCode::S => self.down = false,
            KeyCode::Left | KeyCode::A => self.left = false,
            _ => (),
        }
    }
}
