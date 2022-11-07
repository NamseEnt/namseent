use super::*;

crate::vector_types!(Wh, { width, height });

impl<T: Clone> Wh<T> {
    pub fn as_xy(&self) -> Xy<T> {
        Xy {
            x: self.width.clone(),
            y: self.height.clone(),
        }
    }
}
