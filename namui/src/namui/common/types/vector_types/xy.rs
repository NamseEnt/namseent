use crate::types::Angle;

crate::vector_types!(Xy, { x, y });

impl<T> Xy<T>
where
    T: Into<f32> + From<f32> + Copy,
{
    pub fn angle_to(&self, rhs: Xy<T>) -> Angle {
        let x: f32 = self.x.into();
        let y: f32 = self.y.into();
        let rhs_x: f32 = rhs.x.into();
        let rhs_y: f32 = rhs.y.into();
        Angle::Radian((x * rhs_y - y * rhs_x).atan2(x * rhs_x + y * rhs_y))
    }
    pub fn atan2(&self) -> Angle {
        Angle::Radian(self.y.into().atan2(self.x.into()))
    }
}
