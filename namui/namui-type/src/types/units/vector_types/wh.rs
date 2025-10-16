use super::*;

crate::vector_types!(Wh, { width, height });

impl<T> Wh<T>
where
    T: Clone + std::fmt::Debug + State,
{
    pub fn to_xy(&self) -> Xy<T> {
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
