use crate::*;
use std::fmt::Debug;

#[type_derives(Copy, Eq, Hash)]
pub struct TransformMatrix {
    values: [[OrderedFloat; 3]; 2],
}

impl Default for TransformMatrix {
    fn default() -> Self {
        TransformMatrix {
            values: [
                [
                    OrderedFloat::new(1.0),
                    OrderedFloat::new(0.0),
                    OrderedFloat::new(0.0),
                ],
                [
                    OrderedFloat::new(0.0),
                    OrderedFloat::new(1.0),
                    OrderedFloat::new(0.0),
                ],
            ],
        }
    }
}

impl TransformMatrix {
    pub fn from_slice(values: [[f32; 3]; 2]) -> Self {
        TransformMatrix {
            values: [
                [
                    OrderedFloat::new(values[0][0]),
                    OrderedFloat::new(values[0][1]),
                    OrderedFloat::new(values[0][2]),
                ],
                [
                    OrderedFloat::new(values[1][0]),
                    OrderedFloat::new(values[1][1]),
                    OrderedFloat::new(values[1][2]),
                ],
            ],
        }
    }
    pub fn from_translate(x: f32, y: f32) -> Self {
        Self::from_slice([[1.0, 0.0, x], [0.0, 1.0, y]])
    }
    pub fn from_scale(sx: f32, sy: f32) -> Self {
        Self::from_slice([[sx, 0.0, 0.0], [0.0, sy, 0.0]])
    }
    pub fn from_rotate(angle: Angle) -> Self {
        let s = angle.sin();
        let c = angle.cos();
        Self::from_slice([[c, -s, 0.0], [s, c, 0.0]])
    }
    pub fn identity() -> Self {
        Self::from_slice([[1.0, 0.0, 0.0], [0.0, 1.0, 0.0]])
    }

    pub fn into_slice(self) -> [[f32; 3]; 2] {
        [
            [*self.values[0][0], *self.values[0][1], *self.values[0][2]],
            [*self.values[1][0], *self.values[1][1], *self.values[1][2]],
        ]
    }
    pub fn into_linear_slice(self) -> [f32; 6] {
        [
            *self.values[0][0],
            *self.values[0][1],
            *self.values[0][2],
            *self.values[1][0],
            *self.values[1][1],
            *self.values[1][2],
        ]
    }

    pub fn transform_xy<T>(&self, xy: crate::Xy<T>) -> crate::Xy<T>
    where
        T: Into<f32> + From<f32> + std::fmt::Debug + State,
    {
        let x: f32 = xy.x.into();
        let y: f32 = xy.y.into();

        let new_x = self.values[0][0] * x + self.values[0][1] * y + self.values[0][2];
        let new_y = self.values[1][0] * x + self.values[1][1] * y + self.values[1][2];

        crate::Xy {
            x: (*new_x).into(),
            y: (*new_y).into(),
        }
    }

    pub fn transform_rect<T>(&self, rect: Rect<T>) -> Rect<T>
    where
        T: std::ops::Add<Output = T>
            + Copy
            + std::ops::Mul<f32, Output = T>
            + From<f32>
            + PartialOrd
            + Debug
            + State,
        f32: From<T>,
    {
        let Ltrb {
            left,
            top,
            right,
            bottom,
        } = rect.as_ltrb();
        let a = Xy::new(left, top);
        let b = Xy::new(right, bottom);
        let c = Xy::new(left, bottom);
        let d = Xy::new(right, top);

        let a_transformed = self.transform_xy(a);
        let b_transformed = self.transform_xy(b);
        let c_transformed = self.transform_xy(c);
        let d_transformed = self.transform_xy(d);
        let xs = [
            a_transformed.x,
            b_transformed.x,
            c_transformed.x,
            d_transformed.x,
        ];
        let ys = [
            a_transformed.y,
            b_transformed.y,
            c_transformed.y,
            d_transformed.y,
        ];

        fn min<T: PartialOrd + Copy>(xs: [T; 4]) -> T {
            xs.into_iter()
                .reduce(|a, b| if a < b { a } else { b })
                .unwrap()
        }
        fn max<T: PartialOrd + Copy>(xs: [T; 4]) -> T {
            xs.into_iter()
                .reduce(|a, b| if a > b { a } else { b })
                .unwrap()
        }
        Rect::Ltrb {
            left: min(xs),
            top: min(ys),
            right: max(xs),
            bottom: max(ys),
        }
    }
    pub fn x(&self) -> f32 {
        *self.values[0][2]
    }
    pub fn y(&self) -> f32 {
        *self.values[1][2]
    }
    pub fn sx(&self) -> f32 {
        *self.values[0][0]
    }
    pub fn sy(&self) -> f32 {
        *self.values[1][1]
    }
    pub fn inverse(&self) -> Option<Self> {
        let det = self.values[0][0] * self.values[1][1] - self.values[0][1] * self.values[1][0];

        if *det == 0.0 {
            return None;
        }

        let inv_det = 1.0 / *det;

        Some(Self::from_slice([
            [
                *self.values[1][1] * inv_det,
                -*self.values[0][1] * inv_det,
                (*self.values[0][1] * *self.values[1][2] - *self.values[1][1] * *self.values[0][2])
                    * inv_det,
            ],
            [
                -*self.values[1][0] * inv_det,
                *self.values[0][0] * inv_det,
                (*self.values[1][0] * *self.values[0][2] - *self.values[0][0] * *self.values[1][2])
                    * inv_det,
            ],
        ]))
    }
    pub fn translate(&mut self, x: f32, y: f32) {
        *self.values[0][2] += x;
        *self.values[1][2] += y;
    }
    pub fn set_translate(&mut self, x: f32, y: f32) {
        *self.values[0][2] = x;
        *self.values[1][2] = y;
    }
    pub fn scale(&mut self, x: f32, y: f32) {
        *self.values[0][0] *= x;
        *self.values[1][1] *= y;
    }
    pub fn rotate(&mut self, angle: Angle) {
        let sin = angle.sin();
        let cos = angle.cos();

        let m00 = *self.values[0][0];
        let m01 = *self.values[0][1];
        let m10 = *self.values[1][0];
        let m11 = *self.values[1][1];

        *self.values[0][0] = m00 * cos + m01 * sin;
        *self.values[0][1] = m00 * -sin + m01 * cos;
        *self.values[1][0] = m10 * cos + m11 * sin;
        *self.values[1][1] = m10 * -sin + m11 * cos;
    }
}

impl std::ops::Index<usize> for TransformMatrix {
    type Output = [OrderedFloat; 3];

    fn index(&self, index: usize) -> &Self::Output {
        &self.values[index]
    }
}

crate::impl_op_forward_ref!(
    *|a: TransformMatrix, b: TransformMatrix| -> TransformMatrix {
        TransformMatrix {
            values: [
                [
                    a.values[0][0] * b.values[0][0] + a.values[0][1] * b.values[1][0],
                    a.values[0][0] * b.values[0][1] + a.values[0][1] * b.values[1][1],
                    a.values[0][0] * b.values[0][2]
                        + a.values[0][1] * b.values[1][2]
                        + a.values[0][2],
                ],
                [
                    a.values[1][0] * b.values[0][0] + a.values[1][1] * b.values[1][0],
                    a.values[1][0] * b.values[0][1] + a.values[1][1] * b.values[1][1],
                    a.values[1][0] * b.values[0][2]
                        + a.values[1][1] * b.values[1][2]
                        + a.values[1][2],
                ],
            ],
        }
    }
);
crate::impl_op_forward_ref!(+|a: TransformMatrix, b: TransformMatrix| -> TransformMatrix {
    TransformMatrix {
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
        ],
    }
});

crate::impl_op_forward_ref_reversed!(*|a: TransformMatrix, b: f32| -> TransformMatrix {
    TransformMatrix {
        values: [
            [a.values[0][0] * b, a.values[0][1] * b, a.values[0][2] * b],
            [a.values[1][0] * b, a.values[1][1] * b, a.values[1][2] * b],
        ],
    }
});

impl From<skia_safe::Matrix> for TransformMatrix {
    fn from(matrix: skia_safe::Matrix) -> Self {
        Self::from_slice([
            [matrix[0], matrix[1], matrix[2]],
            [matrix[3], matrix[4], matrix[5]],
        ])
    }
}

impl From<TransformMatrix> for skia_safe::Matrix {
    fn from(val: TransformMatrix) -> Self {
        skia_safe::Matrix::new_all(
            val.values[0][0].as_f32(),
            val.values[0][1].as_f32(),
            val.values[0][2].as_f32(),
            val.values[1][0].as_f32(),
            val.values[1][1].as_f32(),
            val.values[1][2].as_f32(),
            0.0,
            0.0,
            1.0,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use float_cmp::assert_approx_eq;

    #[test]
    fn inverse_should_work() {
        let matrix = TransformMatrix::from_slice([[1.0, 2.0, 5.0], [3.0, 4.0, 6.0]]);

        let inverse = matrix.inverse().unwrap();

        assert_approx_eq!(f32, *inverse.values[0][0], -2.0, ulps = 2);
        assert_approx_eq!(f32, *inverse.values[0][1], 1.0, ulps = 2);
        assert_approx_eq!(f32, *inverse.values[0][2], 4.0, ulps = 2);

        assert_approx_eq!(f32, *inverse.values[1][0], 1.5, ulps = 2);
        assert_approx_eq!(f32, *inverse.values[1][1], -0.5, ulps = 2);
        assert_approx_eq!(f32, *inverse.values[1][2], -4.5, ulps = 2);
    }

    #[test]
    fn multiply_should_work() {
        let a = TransformMatrix::from_slice([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]]);
        let b = TransformMatrix::from_slice([[9.0, 8.0, 7.0], [6.0, 5.0, 4.0]]);

        let result = a * b;

        assert_approx_eq!(f32, *result.values[0][0], 21.0, ulps = 2);
        assert_approx_eq!(f32, *result.values[0][1], 18.0, ulps = 2);
        assert_approx_eq!(f32, *result.values[0][2], 18.0, ulps = 2);

        assert_approx_eq!(f32, *result.values[1][0], 66.0, ulps = 2);
        assert_approx_eq!(f32, *result.values[1][1], 57.0, ulps = 2);
        assert_approx_eq!(f32, *result.values[1][2], 54.0, ulps = 2);
    }

    #[test]
    fn translate_should_work() {
        let mut matrix = TransformMatrix::from_translate(10.0, 20.0);
        assert_eq!(matrix.x(), 10.0);
        assert_eq!(matrix.y(), 20.0);

        matrix.translate(10.0, 20.0);
        assert_eq!(matrix.x(), 20.0);
        assert_eq!(matrix.y(), 40.0);
    }

    #[test]
    fn scale_should_work() {
        let mut matrix = TransformMatrix::from_scale(2.0, 3.0);
        assert_eq!(matrix.sx(), 2.0);
        assert_eq!(matrix.sy(), 3.0);

        matrix.scale(2.0, 3.0);
        assert_eq!(matrix.sx(), 4.0);
        assert_eq!(matrix.sy(), 9.0);
    }

    #[test]
    fn rotate_should_work() {
        let degree = 90.0_f32;
        let mut matrix = TransformMatrix::from_rotate(degree.deg());
        let cos = degree.to_radians().cos(); // 0.0
        let sin = degree.to_radians().sin(); // 1.0

        assert_approx_eq!(f32, *matrix.values[0][0], cos, ulps = 2);
        assert_approx_eq!(f32, *matrix.values[0][1], -sin, ulps = 2);
        assert_approx_eq!(f32, *matrix.values[1][0], sin, ulps = 2);
        assert_approx_eq!(f32, *matrix.values[1][1], cos, ulps = 2);

        matrix.rotate(90.0.deg());
        let degree = 180.0_f32;
        let cos = degree.to_radians().cos(); // -1.0
        let sin = degree.to_radians().sin(); // 0.0

        assert_approx_eq!(f32, *matrix.values[0][0], cos, ulps = 2);
        assert_approx_eq!(f32, *matrix.values[0][1], -sin, ulps = 2);
        assert_approx_eq!(f32, *matrix.values[1][0], sin, ulps = 2);
        assert_approx_eq!(f32, *matrix.values[1][1], cos, ulps = 2);

        let xy = Xy::new(1.0, 2.0);
        let matrix = TransformMatrix::from_rotate(90.0.deg());
        let rotated = matrix.transform_xy(xy);
        assert_approx_eq!(f32, rotated.x, -2.0, ulps = 2);
        assert_approx_eq!(f32, rotated.y, 1.0, ulps = 2);
    }
}
