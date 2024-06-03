#[derive(Debug, Copy, Clone)]
pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Rectangle {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
    pub fn from_array(r: &[i32; 4]) -> Rectangle {
        Rectangle::new(r[0] as f32, r[1] as f32, r[2] as f32, r[3] as f32)
    }
}
