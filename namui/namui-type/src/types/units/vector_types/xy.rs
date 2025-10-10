use crate::Angle;

crate::vector_types!(Xy, { x, y });

impl<T> Xy<T>
where
    T: Into<f32> + From<f32> + Copy,
    T: std::fmt::Debug,
{
    pub fn angle_to(&self, rhs: Xy<T>) -> Angle {
        let x: f32 = self.x.into();
        let y: f32 = self.y.into();
        let rhs_x: f32 = rhs.x.into();
        let rhs_y: f32 = rhs.y.into();
        (x * rhs_y - y * rhs_x).atan2(x * rhs_x + y * rhs_y).rad()
    }
    pub fn atan2(&self) -> Angle {
        self.y.into().atan2(self.x.into()).rad()
    }
}

impl<T> Xy<T>
where
    T: std::fmt::Debug,
{
    pub fn to_wh(&self) -> Wh<T>
    where
        T: Clone,
    {
        Wh {
            width: self.x.clone(),
            height: self.y.clone(),
        }
    }
}

// TODO: Implement this on vector_types! macro.
impl<T, T2> From<Xy<T>> for (T2, T2)
where
    T: Into<T2>,
    T: std::fmt::Debug,
{
    fn from(val: Xy<T>) -> Self {
        (val.x.into(), val.y.into())
    }
}
// TODO: Implement this on vector_types! macro.
impl<T> From<(T, T)> for Xy<T>
where
    T: std::fmt::Debug,
{
    fn from(val: (T, T)) -> Self {
        Xy { x: val.0, y: val.1 }
    }
}

impl From<Xy<Px>> for skia_safe::Point {
    fn from(val: Xy<Px>) -> Self {
        skia_safe::Point::new(val.x.into(), val.y.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn xy_add_xy() {
        let xy = Xy::new(1.0, 2.0);
        let result = xy + Xy::new(2.0, 3.0);
        assert_eq!(result, Xy::new(3.0, 5.0));
    }

    #[test]
    fn xy_sub_xy() {
        let xy = Xy::new(1.px(), 2.px());
        let result = xy - Xy::new(2.px(), 3.px());
        assert_eq!(result, Xy::new(-(1.px()), -(1.px())));
    }

    #[test]
    fn xy_div_f32() {
        let xy = Xy::new(1.px(), 2.px());
        let result = xy / 2.0;
        assert_eq!(result, Xy::new(0.5.px(), 1.0.px()));
    }

    #[test]
    fn xy_div_xy() {
        let xy = Xy::new(1.px(), 2.px());
        let result = xy / Xy::new(2.0f32, 4.0f32);
        assert_eq!(result, Xy::new(0.5.px(), 0.5.px()));
    }

    #[test]
    fn xy_mul_f32() {
        let xy = Xy::new(1.0, 2.0);
        let result = xy * 2.0;
        assert_eq!(result, Xy::new(2.0, 4.0));
    }

    #[test]
    fn xy_mul_xy() {
        let xy = Xy::new(1.px(), 2.px());
        let result = xy * Xy::new(2, 3);
        assert_eq!(result, Xy::new(2.px(), 6.px()));
    }

    #[test]
    fn xy_velocity_multiply_time_vector() {
        let xy = Xy::new(Per::new(1.px(), 4.ms()), Per::new(2.px(), 8.ms()));
        let result = xy * Xy::single(4.ms());
        assert_eq!(result, Xy::new(1.px(), 1.px()));
    }

    // NOTE: This is not compiled because it needs specialization. https://github.com/rust-lang/rust/issues/31844
    // #[test]
    // fn xy_velocity_multiply_time() {
    //     let xy = Xy::new(Per::new(1.px(), 4.ms()), Per::new(2.px(), 8.ms()));
    //     let result = xy * 4.ms();
    //     assert_eq!(result, Xy::new(1.px(), 1.px()));
    // }
}
