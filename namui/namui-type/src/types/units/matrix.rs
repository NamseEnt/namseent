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

    pub fn transform_xy<T: Into<f32> + From<f32>>(&self, xy: crate::Xy<T>) -> crate::Xy<T> {
        let x: f32 = xy.x.into();
        let y: f32 = xy.y.into();

        let new_x = x * self.values[0][0] + y * self.values[0][1] + self.values[0][2];
        let new_y = x * self.values[1][0] + y * self.values[1][1] + self.values[1][2];

        crate::Xy {
            x: new_x.into(),
            y: new_y.into(),
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

        let m00 = self.values[0][0];
        let m01 = self.values[0][1];
        let m10 = self.values[1][0];
        let m11 = self.values[1][1];

        self.values[0][0] = m00 * cos + m01 * sin;
        self.values[0][1] = m00 * -sin + m01 * cos;
        self.values[1][0] = m10 * cos + m11 * sin;
        self.values[1][1] = m10 * -sin + m11 * cos;
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

        assert_approx_eq!(f32, inverse.values[0][0], -2.0, ulps = 2);
        assert_approx_eq!(f32, inverse.values[0][1], 1.0, ulps = 2);
        assert_approx_eq!(f32, inverse.values[0][2], 0.571_428_6, ulps = 2);

        assert_approx_eq!(f32, inverse.values[1][0], 1.5, ulps = 2);
        assert_approx_eq!(f32, inverse.values[1][1], -0.5, ulps = 2);
        assert_approx_eq!(f32, inverse.values[1][2], -0.642_857_13, ulps = 2);

        assert_approx_eq!(f32, inverse.values[2][0], 0.0, ulps = 2);
        assert_approx_eq!(f32, inverse.values[2][1], 0.0, ulps = 2);
        assert_approx_eq!(f32, inverse.values[2][2], 0.142_857_15, ulps = 2);
    }

    #[test]
    #[wasm_bindgen_test]
    fn multiply_should_work() {
        let a = Matrix3x3::from_slice([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]]);
        let b = Matrix3x3::from_slice([[9.0, 8.0, 7.0], [6.0, 5.0, 4.0], [3.0, 2.0, 1.0]]);

        let result = a * b;

        assert_approx_eq!(f32, result.values[0][0], 30.0, ulps = 2);
        assert_approx_eq!(f32, result.values[0][1], 24.0, ulps = 2);
        assert_approx_eq!(f32, result.values[0][2], 18.0, ulps = 2);

        assert_approx_eq!(f32, result.values[1][0], 84.0, ulps = 2);
        assert_approx_eq!(f32, result.values[1][1], 69.0, ulps = 2);
        assert_approx_eq!(f32, result.values[1][2], 54.0, ulps = 2);

        assert_approx_eq!(f32, result.values[2][0], 138.0, ulps = 2);
        assert_approx_eq!(f32, result.values[2][1], 114.0, ulps = 2);
        assert_approx_eq!(f32, result.values[2][2], 90.0, ulps = 2);
    }

    #[test]
    #[wasm_bindgen_test]
    fn translate_should_work() {
        let mut matrix = Matrix3x3::from_translate(10.0, 20.0);
        assert_eq!(matrix.x(), 10.0);
        assert_eq!(matrix.y(), 20.0);

        matrix.translate(10.0, 20.0);
        assert_eq!(matrix.x(), 20.0);
        assert_eq!(matrix.y(), 40.0);
    }

    #[test]
    #[wasm_bindgen_test]
    fn scale_should_work() {
        let mut matrix = Matrix3x3::from_scale(2.0, 3.0);
        assert_eq!(matrix.sx(), 2.0);
        assert_eq!(matrix.sy(), 3.0);

        matrix.scale(2.0, 3.0);
        assert_eq!(matrix.sx(), 4.0);
        assert_eq!(matrix.sy(), 9.0);
    }

    #[test]
    #[wasm_bindgen_test]
    fn rotate_should_work() {
        let degree = 90.0_f32;
        let mut matrix = Matrix3x3::from_rotate(degree.deg());
        let cos = degree.to_radians().cos(); // 0.0
        let sin = degree.to_radians().sin(); // 1.0

        assert_approx_eq!(f32, matrix.values[0][0], cos, ulps = 2);
        assert_approx_eq!(f32, matrix.values[0][1], -sin, ulps = 2);
        assert_approx_eq!(f32, matrix.values[1][0], sin, ulps = 2);
        assert_approx_eq!(f32, matrix.values[1][1], cos, ulps = 2);

        matrix.rotate(90.0.deg());
        let degree = 180.0_f32;
        let cos = degree.to_radians().cos(); // -1.0
        let sin = degree.to_radians().sin(); // 0.0

        assert_approx_eq!(f32, matrix.values[0][0], cos, ulps = 2);
        assert_approx_eq!(f32, matrix.values[0][1], -sin, ulps = 2);
        assert_approx_eq!(f32, matrix.values[1][0], sin, ulps = 2);
        assert_approx_eq!(f32, matrix.values[1][1], cos, ulps = 2);

        let xy = Xy::new(1.0, 2.0);
        let matrix = Matrix3x3::from_rotate(90.0.deg());
        let rotated = matrix.transform_xy(xy);
        assert_approx_eq!(f32, rotated.x, -2.0, ulps = 2);
        assert_approx_eq!(f32, rotated.y, 1.0, ulps = 2);
    }
}
