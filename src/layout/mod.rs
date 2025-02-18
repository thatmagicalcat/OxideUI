//! The layout manager.

mod alignment;
mod linear_layout;

pub use alignment::*;
pub use linear_layout::*;

use crate::Drawable;
use crate::Rect;

pub trait Layout {
    /// Arrange the children in the container.
    /// Returns a vector of tuples, each tuple contains the position and size of the child.
    fn arrange(&self, children: &[&dyn Drawable], layout_rect: Rect) -> Vec<(f32, f32, f32, f32)>;
}

#[cfg(test)]
mod tests {
    use crate::Drawable;

    pub struct MockDrawable {
        pub size: (f32, f32),
    }

    impl Drawable for MockDrawable {
        fn draw(&self) {
        }

        fn get_position(&self) -> (f32, f32) {
            (0.0, 0.0)
        }

        fn get_size(&self) -> (f32, f32) {
            self.size
        }
    }
}
