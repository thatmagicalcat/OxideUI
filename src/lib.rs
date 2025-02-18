pub mod layout;

pub struct Rect {
    pub pos: glam::Vec2,
    pub size: glam::Vec2,
}

impl From<(f32, f32, f32, f32)> for Rect {
    fn from((x, y, w, h): (f32, f32, f32, f32)) -> Self {
        Self {
            pos: (x, y).into(),
            size: (w, h).into(),
        }
    }
}

pub trait Drawable {
    fn draw(&self);
    fn get_position(&self) -> (f32, f32);
    fn get_size(&self) -> (f32, f32);
}
