use super::*;

crate::vector_types!(Wh, { width, height });

impl<T: Clone> Wh<T> {
    pub fn as_xy(&self) -> Xy<T> {
        Xy {
            x: self.width.clone(),
            y: self.height.clone(),
        }
    }
    pub fn to_rect(self) -> crate::Rect<T>
    where
        T: From<f32>,
    {
        crate::Rect::Xywh {
            x: 0.0.into(),
            y: 0.0.into(),
            width: self.width,
            height: self.height,
        }
    }
}
