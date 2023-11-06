use crate::*;

#[type_derives(Copy)]
pub struct Matrix3x3 {
    values: nalgebra::Matrix3<f32>,
}

impl Default for Matrix3x3 {
    fn default() -> Self {
        Matrix3x3 {
            values: nalgebra::Matrix3::identity(),
        }
    }
}

impl Matrix3x3 {
    pub fn from_slice(values: [[f32; 3]; 3]) -> Self {
        Matrix3x3 {
            values: nalgebra::Matrix3::new(
                values[0][0],
                values[0][1],
                values[0][2],
                values[1][0],
                values[1][1],
                values[1][2],
                values[2][0],
                values[2][1],
                values[2][2],
            ),
        }
    }
    pub fn from_translate(x: f32, y: f32) -> Self {
        Self::from_slice([[1.0, 0.0, x], [0.0, 1.0, y], [0.0, 0.0, 1.0]])
    }
    pub fn from_scale(sx: f32, sy: f32) -> Self {
        Self::from_slice([[sx, 0.0, 0.0], [0.0, sy, 0.0], [0.0, 0.0, 1.0]])
    }
    pub fn from_rotate(angle: Angle) -> Self {
        let s = angle.sin();
        let c = angle.cos();
        Self::from_slice([[c, -s, 0.0], [s, c, 0.0], [0.0, 0.0, 1.0]])
    }
    pub fn identity() -> Self {
        Self::from_slice([[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]])
    }

    pub fn into_slice(self) -> [[f32; 3]; 3] {
        [
            [
                *self.values.index((0, 0)),
                *self.values.index((0, 1)),
                *self.values.index((0, 2)),
            ],
            [
                *self.values.index((1, 0)),
                *self.values.index((1, 1)),
                *self.values.index((1, 2)),
            ],
            [
                *self.values.index((2, 0)),
                *self.values.index((2, 1)),
                *self.values.index((2, 2)),
            ],
        ]
    }
    pub fn into_linear_slice(self) -> [f32; 9] {
        [
            *self.values.index((0, 0)),
            *self.values.index((0, 1)),
            *self.values.index((0, 2)),
            *self.values.index((1, 0)),
            *self.values.index((1, 1)),
            *self.values.index((1, 2)),
            *self.values.index((2, 0)),
            *self.values.index((2, 1)),
            *self.values.index((2, 2)),
        ]
    }

    pub fn transform_xy(&self, xy: crate::Xy<Px>) -> crate::Xy<Px> {
        let transformed = self
            .values
            .transform_point(&nalgebra::point![xy.x.as_f32(), xy.y.as_f32()]);
        crate::Xy {
            x: transformed.x.px(),
            y: transformed.y.px(),
        }
    }

    pub fn transform_rect<T>(&self, rect: Rect<T>) -> Rect<T>
    where
        T: std::ops::Add<Output = T> + Copy + std::ops::Mul<f32, Output = T> + From<f32>,
    {
        let Ltrb {
            left,
            top,
            right,
            bottom,
        } = rect.as_ltrb();
        Rect::Ltrb {
            left: left * *self.values.index((0, 0))
                + top * *self.values.index((0, 1))
                + (*self.values.index((0, 2))).into(),
            top: left * *self.values.index((1, 0))
                + top * *self.values.index((1, 1))
                + (*self.values.index((1, 2))).into(),
            right: right * *self.values.index((0, 0))
                + bottom * *self.values.index((0, 1))
                + (*self.values.index((0, 2))).into(),
            bottom: right * *self.values.index((1, 0))
                + bottom * *self.values.index((1, 1))
                + (*self.values.index((1, 2))).into(),
        }
    }
    pub fn x(&self) -> f32 {
        *self.values.index((0, 2))
    }
    pub fn y(&self) -> f32 {
        *self.values.index((1, 2))
    }
    pub fn sx(&self) -> f32 {
        *self.values.index((0, 0))
    }
    pub fn sy(&self) -> f32 {
        *self.values.index((1, 1))
    }
    pub fn inverse(&self) -> Option<Self> {
        Some(Matrix3x3 {
            values: self.values.try_inverse()?,
        })
    }
    pub fn index_0_0(&self) -> f32 {
        *self.values.index((0, 0))
    }
    pub fn index_0_1(&self) -> f32 {
        *self.values.index((0, 1))
    }
    pub fn index_0_2(&self) -> f32 {
        *self.values.index((0, 2))
    }
    pub fn index_1_0(&self) -> f32 {
        *self.values.index((1, 0))
    }
    pub fn index_1_1(&self) -> f32 {
        *self.values.index((1, 1))
    }
    pub fn index_1_2(&self) -> f32 {
        *self.values.index((1, 2))
    }
    pub fn index_2_0(&self) -> f32 {
        *self.values.index((2, 0))
    }
    pub fn index_2_1(&self) -> f32 {
        *self.values.index((2, 1))
    }
    pub fn index_2_2(&self) -> f32 {
        *self.values.index((2, 2))
    }
    pub fn set_index_0_0(&mut self, value: f32) {
        *self.values.index_mut((0, 0)) = value
    }
    pub fn set_index_0_1(&mut self, value: f32) {
        *self.values.index_mut((0, 1)) = value
    }
    pub fn set_index_0_2(&mut self, value: f32) {
        *self.values.index_mut((0, 2)) = value
    }
    pub fn set_index_1_0(&mut self, value: f32) {
        *self.values.index_mut((1, 0)) = value
    }
    pub fn set_index_1_1(&mut self, value: f32) {
        *self.values.index_mut((1, 1)) = value
    }
    pub fn set_index_1_2(&mut self, value: f32) {
        *self.values.index_mut((1, 2)) = value
    }
    pub fn set_index_2_0(&mut self, value: f32) {
        *self.values.index_mut((2, 0)) = value
    }
    pub fn set_index_2_1(&mut self, value: f32) {
        *self.values.index_mut((2, 1)) = value
    }
    pub fn set_index_2_2(&mut self, value: f32) {
        *self.values.index_mut((2, 2)) = value
    }
    pub fn translate(&mut self, x: f32, y: f32) {
        let matrix = Self::from_translate(x, y);
        self.values = matrix.values * self.values;
    }
    pub fn scale(&mut self, x: f32, y: f32) {
        let matrix = Self::from_scale(x, y);
        self.values = matrix.values * self.values;
    }
    pub fn rotate(&mut self, angle: Angle) {
        let matrix = Self::from_rotate(angle);
        self.values = matrix.values * self.values;
    }
}

crate::impl_op_forward_ref!(*|a: Matrix3x3, b: Matrix3x3| -> Matrix3x3 {
    Matrix3x3 {
        values: a.values * b.values,
    }
});
crate::impl_op_forward_ref!(+|a: Matrix3x3, b: Matrix3x3| -> Matrix3x3 {
    Matrix3x3 {
        values: a.values + b.values,
    }
});

crate::impl_op_forward_ref_reversed!(*|a: Matrix3x3, b: f32| -> Matrix3x3 {
    Matrix3x3 {
        values: a.values * b,
    }
});

#[cfg(test)]
mod tests {
    use super::*;
    use float_cmp::assert_approx_eq;
    use wasm_bindgen_test::wasm_bindgen_test;

    #[test]
    #[wasm_bindgen_test]
    fn inverse_should_work() {
        let matrix = Matrix3x3::from_slice([[1.0, 2.0, 5.0], [3.0, 4.0, 6.0], [0.0, 0.0, 7.0]]);

        let inverse = matrix.inverse().unwrap();

        assert_approx_eq!(f32, *inverse.values.index((0, 0)), -2.0, ulps = 2);
        assert_approx_eq!(f32, *inverse.values.index((0, 1)), 1.0, ulps = 2);
        assert_approx_eq!(f32, *inverse.values.index((0, 2)), 0.571_428_6, ulps = 2);

        assert_approx_eq!(f32, *inverse.values.index((1, 0)), 1.5, ulps = 2);
        assert_approx_eq!(f32, *inverse.values.index((1, 1)), -0.5, ulps = 2);
        assert_approx_eq!(f32, *inverse.values.index((1, 2)), -0.642_857_13, ulps = 2);

        assert_approx_eq!(f32, *inverse.values.index((2, 0)), 0.0, ulps = 2);
        assert_approx_eq!(f32, *inverse.values.index((2, 1)), 0.0, ulps = 2);
        assert_approx_eq!(f32, *inverse.values.index((2, 2)), 0.142_857_15, ulps = 2);
    }
}
