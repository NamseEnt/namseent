use crate::*;

#[type_derives(Copy)]
pub struct Matrix3x3 {
    values: [[f32; 3]; 3],
}

impl Default for Matrix3x3 {
    fn default() -> Self {
        Matrix3x3 {
            values: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
        }
    }
}

impl Matrix3x3 {
    pub fn from_slice(values: [[f32; 3]; 3]) -> Self {
        Matrix3x3 { values }
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
        self.values
    }
    pub fn into_linear_slice(self) -> [f32; 9] {
        [
            self.values[0][0],
            self.values[0][1],
            self.values[0][2],
            self.values[1][0],
            self.values[1][1],
            self.values[1][2],
            self.values[2][0],
            self.values[2][1],
            self.values[2][2],
        ]
    }

    pub fn transform_xy(&self, xy: crate::Xy<Px>) -> crate::Xy<Px> {
        let x = xy.x.as_f32();
        let y = xy.y.as_f32();

        let x = x * self.values[0][0] + y * self.values[0][1] + self.values[0][2];
        let y = x * self.values[1][3] + y * self.values[1][4] + self.values[1][5];

        crate::Xy {
            x: x.px(),
            y: y.px(),
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
            left: left * self.values[0][0] + top * self.values[0][1] + self.values[0][2].into(),
            top: left * self.values[1][0] + top * self.values[1][1] + self.values[1][2].into(),
            right: right * self.values[0][0]
                + bottom * self.values[0][1]
                + self.values[0][2].into(),
            bottom: right * self.values[1][0]
                + bottom * self.values[1][1]
                + self.values[1][2].into(),
        }
    }
    pub fn x(&self) -> f32 {
        self.values[0][2]
    }
    pub fn y(&self) -> f32 {
        self.values[1][2]
    }
    pub fn sx(&self) -> f32 {
        self.values[0][0]
    }
    pub fn sy(&self) -> f32 {
        self.values[1][1]
    }
    pub fn inverse(&self) -> Option<Self> {
        let det = self.values[0][0]
            * (self.values[1][1] * self.values[2][2] - self.values[1][2] * self.values[2][1])
            - self.values[0][1]
                * (self.values[1][0] * self.values[2][2] - self.values[1][2] * self.values[2][0])
            + self.values[0][2]
                * (self.values[1][0] * self.values[2][1] - self.values[1][1] * self.values[2][0]);

        if det == 0.0 {
            return None;
        }

        let inv_det = 1.0 / det;

        Some(Self::from_slice([
            [
                (self.values[1][1] * self.values[2][2] - self.values[1][2] * self.values[2][1])
                    * inv_det,
                (self.values[0][2] * self.values[2][1] - self.values[0][1] * self.values[2][2])
                    * inv_det,
                (self.values[0][1] * self.values[1][2] - self.values[0][2] * self.values[1][1])
                    * inv_det,
            ],
            [
                (self.values[1][2] * self.values[2][0] - self.values[1][0] * self.values[2][2])
                    * inv_det,
                (self.values[0][0] * self.values[2][2] - self.values[0][2] * self.values[2][0])
                    * inv_det,
                (self.values[1][0] * self.values[0][2] - self.values[0][0] * self.values[1][2])
                    * inv_det,
            ],
            [
                (self.values[1][0] * self.values[2][1] - self.values[1][1] * self.values[2][0])
                    * inv_det,
                (self.values[0][1] * self.values[2][0] - self.values[0][0] * self.values[2][1])
                    * inv_det,
                (self.values[0][0] * self.values[1][1] - self.values[0][1] * self.values[1][0])
                    * inv_det,
            ],
        ]))
    }
    pub fn translate(&mut self, x: f32, y: f32) {
        self.values[0][2] += x;
        self.values[1][2] += y;
    }
    pub fn scale(&mut self, x: f32, y: f32) {
        self.values[0][0] *= x;
        self.values[1][1] *= y;
    }
    pub fn rotate(&mut self, angle: Angle) {
        let sin = angle.sin();
        let cos = angle.cos();

        let value00 = self.values[0][0];
        let value01 = self.values[0][1];
        let value10 = self.values[1][0];
        let value11 = self.values[1][1];

        self.values[0][0] = value00 * cos - value01 * sin;
        self.values[0][1] = value00 * sin + value01 * cos;
        self.values[1][0] = value10 * cos - value11 * sin;
        self.values[1][1] = value10 * sin + value11 * cos;
    }
}

impl std::ops::Index<usize> for Matrix3x3 {
    type Output = [f32; 3];

    fn index(&self, index: usize) -> &Self::Output {
        &self.values[index]
    }
}

crate::impl_op_forward_ref!(*|a: Matrix3x3, b: Matrix3x3| -> Matrix3x3 {
    Matrix3x3 {
        values: [
            [
                a.values[0][0] * b.values[0][0]
                    + a.values[0][1] * b.values[1][0]
                    + a.values[0][2] * b.values[2][0],
                a.values[0][0] * b.values[0][1]
                    + a.values[0][1] * b.values[1][1]
                    + a.values[0][2] * b.values[2][1],
                a.values[0][0] * b.values[0][2]
                    + a.values[0][1] * b.values[1][2]
                    + a.values[0][2] * b.values[2][2],
            ],
            [
                a.values[1][0] * b.values[0][0]
                    + a.values[1][1] * b.values[1][0]
                    + a.values[1][2] * b.values[2][0],
                a.values[1][0] * b.values[0][1]
                    + a.values[1][1] * b.values[1][1]
                    + a.values[1][2] * b.values[2][1],
                a.values[1][0] * b.values[0][2]
                    + a.values[1][1] * b.values[1][2]
                    + a.values[1][2] * b.values[2][2],
            ],
            [
                a.values[2][0] * b.values[0][0]
                    + a.values[2][1] * b.values[1][0]
                    + a.values[2][2] * b.values[2][0],
                a.values[2][0] * b.values[0][1]
                    + a.values[2][1] * b.values[1][1]
                    + a.values[2][2] * b.values[2][1],
                a.values[2][0] * b.values[0][2]
                    + a.values[2][1] * b.values[1][2]
                    + a.values[2][2] * b.values[2][2],
            ],
        ],
    }
});
crate::impl_op_forward_ref!(+|a: Matrix3x3, b: Matrix3x3| -> Matrix3x3 {
    Matrix3x3 {
        values: [
            [
                a.values[0][0] + b.values[0][0],
                a.values[0][1] + b.values[0][1],
                a.values[0][2] + b.values[0][2],
            ],
            [
                a.values[1][0] + b.values[1][0],
                a.values[1][1] + b.values[1][1],
                a.values[1][2] + b.values[1][2],
            ],
            [
                a.values[2][0] + b.values[2][0],
                a.values[2][1] + b.values[2][1],
                a.values[2][2] + b.values[2][2],
            ],
        ],
    }
});

crate::impl_op_forward_ref_reversed!(*|a: Matrix3x3, b: f32| -> Matrix3x3 {
    Matrix3x3 {
        values: [
            [a.values[0][0] * b, a.values[0][1] * b, a.values[0][2] * b],
            [a.values[1][0] * b, a.values[1][1] * b, a.values[1][2] * b],
            [a.values[2][0] * b, a.values[2][1] * b, a.values[2][2] * b],
        ],
    }
});

#[cfg(feature = "skia")]
impl From<skia_safe::Matrix> for Matrix3x3 {
    fn from(matrix: skia_safe::Matrix) -> Self {
        Self::from_slice([
            [matrix[0], matrix[1], matrix[2]],
            [matrix[3], matrix[4], matrix[5]],
            [matrix[6], matrix[7], matrix[8]],
        ])
    }
}

#[cfg(feature = "skia")]
impl Into<skia_safe::Matrix> for Matrix3x3 {
    fn into(self) -> skia_safe::Matrix {
        skia_safe::Matrix::new_all(
            self.values[0][0],
            self.values[0][1],
            self.values[0][2],
            self.values[1][0],
            self.values[1][1],
            self.values[1][2],
            self.values[2][0],
            self.values[2][1],
            self.values[2][2],
        )
    }
}

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
