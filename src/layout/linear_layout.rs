use super::*;

#[derive(Default, Clone)]
pub struct VerticalLayout {
    v_align: Option<VerticalAlignment>,
    h_align: Option<HorizontalAlignment>,
}

impl VerticalLayout {
    pub fn new(v_align: Option<VerticalAlignment>, h_align: Option<HorizontalAlignment>) -> Self {
        Self { v_align, h_align }
    }
}

impl Layout for VerticalLayout {
    fn arrange(&self, children: &[&dyn Drawable], layout_rect: Rect) -> Vec<(f32, f32, f32, f32)> {
        let mut result = Vec::new();
        let mut y = 0.0;

        for child in children {
            let (width, height) = child.get_size();
            let child_x = match self.h_align {
                Some(HorizontalAlignment::Left) => layout_rect.pos.x,
                Some(HorizontalAlignment::Right) => layout_rect.pos.x + layout_rect.size.x - width,
                Some(HorizontalAlignment::Center) => {
                    layout_rect.pos.x + (layout_rect.size.x - width) / 2.0
                }

                None => layout_rect.pos.x,
            };

            let child_y = match self.v_align {
                Some(VerticalAlignment::Top | VerticalAlignment::Bottom) => y,
                Some(VerticalAlignment::Center) => {
                    layout_rect.pos.y + (layout_rect.size.y - height) / 2.0
                }

                None => y,
            };

            result.push((child_x, child_y + layout_rect.pos.y, width, height));
            y += height;
        }

        if matches!(self.v_align, Some(VerticalAlignment::Bottom)) && !children.is_empty() {
            let last = result.last().unwrap();
            let height = last.1 + last.3 - layout_rect.pos.y;
            let shift = layout_rect.pos.y + layout_rect.size.y - height - layout_rect.pos.y;

            result.iter_mut().for_each(|(_, y, _, _)| *y += shift);
        }

        result
    }
}

#[derive(Default, Clone, Copy)]
pub struct HorizontalLayout {
    h_align: Option<HorizontalAlignment>,
    v_align: Option<VerticalAlignment>,
}

impl HorizontalLayout {
    pub fn new(h_align: Option<HorizontalAlignment>, v_align: Option<VerticalAlignment>) -> Self {
        Self { h_align, v_align }
    }
}

impl Layout for HorizontalLayout {
    fn arrange(&self, children: &[&dyn Drawable], layout_rect: Rect) -> Vec<(f32, f32, f32, f32)> {
        let mut result = Vec::new();
        let mut x = 0.0;

        for child in children {
            let (width, height) = child.get_size();
            let child_y = match self.v_align {
                Some(VerticalAlignment::Top) => layout_rect.pos.y,
                Some(VerticalAlignment::Bottom) => layout_rect.pos.y + layout_rect.size.y - height,
                Some(VerticalAlignment::Center) => {
                    layout_rect.pos.y + (layout_rect.size.y - height) / 2.0
                }

                None => layout_rect.pos.y,
            };

            let child_x = match self.h_align {
                Some(HorizontalAlignment::Left | HorizontalAlignment::Right) => x,
                Some(HorizontalAlignment::Center) => {
                    layout_rect.pos.x + (layout_rect.size.x - width) / 2.0
                }

                None => x,
            };

            result.push((child_x + layout_rect.pos.x, child_y, width, height));
            x += width;
        }

        if matches!(self.h_align, Some(HorizontalAlignment::Right)) && !children.is_empty() {
            let last = result.last().unwrap();
            let width = last.0 + last.2 - layout_rect.pos.x;
            let shift = layout_rect.pos.x + layout_rect.size.x - width - layout_rect.pos.x;

            result.iter_mut().for_each(|(x, _, _, _)| *x += shift);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    use super::super::tests::MockDrawable;

    mod vertical {
        use super::*;

        #[test]
        fn layout() {
            let layout = VerticalLayout::default();
            let children: Vec<&dyn Drawable> = vec![
                &MockDrawable { size: (10.0, 10.0) },
                &MockDrawable { size: (10.0, 20.0) },
            ];

            let result = layout.arrange(&children, (100.0, 200.0, 400.0, 400.0).into());
            assert_eq!(
                result.as_slice(),
                &[(100.0, 200.0, 10.0, 10.0), (100.0, 210.0, 10.0, 20.0)]
            );
        }

        #[test]
        fn layout_with_top_valign() {
            let layout = VerticalLayout::new(Some(VerticalAlignment::Top), None);
            let children: Vec<&dyn Drawable> = vec![
                &MockDrawable { size: (20.0, 10.0) },
                &MockDrawable { size: (10.0, 20.0) },
            ];

            let result = layout.arrange(&children, (100.0, 100.0, 800.0, 800.0).into());
            assert_eq!(
                result.as_slice(),
                &[(100.0, 100.0, 20.0, 10.0), (100.0, 110.0, 10.0, 20.0)]
            );
        }

        #[test]
        fn layout_with_bot_valign() {
            let layout = VerticalLayout::new(Some(VerticalAlignment::Bottom), None);
            let children: Vec<&dyn Drawable> = vec![
                &MockDrawable { size: (20.0, 10.0) },
                &MockDrawable { size: (10.0, 20.0) },
            ];

            let result = layout.arrange(&children, (100.0, 100.0, 800.0, 800.0).into());
            assert_eq!(
                result.as_slice(),
                &[(100.0, 870.0, 20.0, 10.0), (100.0, 880.0, 10.0, 20.0)]
            );
        }

        #[test]
        fn layout_with_left_halign() {
            let layout = VerticalLayout::new(None, Some(HorizontalAlignment::Left));
            let children: Vec<&dyn Drawable> = vec![
                &MockDrawable { size: (20.0, 10.0) },
                &MockDrawable { size: (10.0, 20.0) },
            ];

            let result = layout.arrange(&children, (100.0, 100.0, 800.0, 800.0).into());
            assert_eq!(
                result.as_slice(),
                &[(100.0, 100.0, 20.0, 10.0), (100.0, 110.0, 10.0, 20.0)]
            );
        }

        #[test]
        fn layout_with_right_halign() {
            let layout = VerticalLayout::new(None, Some(HorizontalAlignment::Right));
            let children: Vec<&dyn Drawable> = vec![
                &MockDrawable { size: (20.0, 10.0) },
                &MockDrawable { size: (10.0, 20.0) },
            ];

            let result = layout.arrange(&children, (100.0, 100.0, 800.0, 800.0).into());
            assert_eq!(
                result.as_slice(),
                &[(880.0, 100.0, 20.0, 10.0), (890.0, 110.0, 10.0, 20.0)]
            );
        }
    }

    mod horizontal {
        use super::*;

        #[test]
        fn layout() {
            let layout = HorizontalLayout::default();
            let children: Vec<&dyn Drawable> = vec![
                &MockDrawable { size: (10.0, 10.0) },
                &MockDrawable { size: (20.0, 10.0) },
            ];

            let result = layout.arrange(&children, (100.0, 100.0, 800.0, 800.0).into());
            assert_eq!(
                result.as_slice(),
                &[(100.0, 100.0, 10.0, 10.0), (110.0, 100.0, 20.0, 10.0)]
            );
        }

        #[test]
        fn layout_with_left_halign() {
            let layout = HorizontalLayout::new(Some(HorizontalAlignment::Left), None);
            let children: Vec<&dyn Drawable> = vec![
                &MockDrawable { size: (10.0, 10.0) },
                &MockDrawable { size: (20.0, 10.0) },
            ];

            let result = layout.arrange(&children, (100.0, 100.0, 800.0, 800.0).into());
            assert_eq!(
                result.as_slice(),
                &[(100.0, 100.0, 10.0, 10.0), (110.0, 100.0, 20.0, 10.0)]
            );
        }

        #[test]
        fn layout_with_right_halign() {
            let layout = HorizontalLayout::new(Some(HorizontalAlignment::Right), None);
            let children: Vec<&dyn Drawable> = vec![
                &MockDrawable { size: (20.0, 30.0) },
                &MockDrawable { size: (20.0, 10.0) },
            ];

            let result = layout.arrange(&children, (100.0, 100.0, 800.0, 800.0).into());
            assert_eq!(
                result.as_slice(),
                &[(860.0, 100.0, 20.0, 30.0), (880.0, 100.0, 20.0, 10.0)]
            );
        }

        #[test]
        fn layout_with_top_valign() {
            let layout = HorizontalLayout::new(None, Some(VerticalAlignment::Top));
            let children: Vec<&dyn Drawable> = vec![
                &MockDrawable { size: (10.0, 20.0) },
                &MockDrawable { size: (20.0, 10.0) },
            ];

            let result = layout.arrange(&children, (100.0, 100.0, 800.0, 800.0).into());
            assert_eq!(
                result.as_slice(),
                &[(100.0, 100.0, 10.0, 20.0), (110.0, 100.0, 20.0, 10.0)]
            );
        }

        #[test]
        fn layout_with_bot_valign() {
            let layout = HorizontalLayout::new(None, Some(VerticalAlignment::Bottom));
            let children: Vec<&dyn Drawable> = vec![
                &MockDrawable { size: (10.0, 30.0) },
                &MockDrawable { size: (20.0, 10.0) },
            ];

            let result = layout.arrange(&children, (100.0, 100.0, 800.0, 800.0).into());
            assert_eq!(
                result.as_slice(),
                &[(100.0, 870.0, 10.0, 30.0), (110.0, 890.0, 20.0, 10.0)]
            );
        }
    }
}
