use crate::*;

#[derive(Clone, Copy, Debug)]
pub struct Matrix3x3 {
    pub values: [[f32; 3]; 3],
}

impl Matrix3x3 {
    pub fn new(a: f32, b: f32, c: f32, d: f32, e: f32, f: f32, g: f32, h: f32, i: f32) -> Self {
        Matrix3x3 {
            values: [[a, b, c], [d, e, f], [g, h, i]],
        }
    }
    pub fn from_slice(values: &[[f32; 3]; 3]) -> Self {
        Matrix3x3 {
            values: [
                [values[0][0], values[0][1], values[0][2]],
                [values[1][0], values[1][1], values[1][2]],
                [values[2][0], values[2][1], values[2][2]],
            ],
        }
    }
    pub fn identity() -> Self {
        Self::from_slice(&[[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]])
    }
    pub fn multiply(&self, other: &Matrix3x3) -> Matrix3x3 {
        Matrix3x3 {
            values: [
                [
                    self.values[0][0] * other.values[0][0]
                        + self.values[0][1] * other.values[1][0]
                        + self.values[0][2] * other.values[2][0],
                    self.values[0][0] * other.values[0][1]
                        + self.values[0][1] * other.values[1][1]
                        + self.values[0][2] * other.values[2][1],
                    self.values[0][0] * other.values[0][2]
                        + self.values[0][1] * other.values[1][2]
                        + self.values[0][2] * other.values[2][2],
                ],
                [
                    self.values[1][0] * other.values[0][0]
                        + self.values[1][1] * other.values[1][0]
                        + self.values[1][2] * other.values[2][0],
                    self.values[1][0] * other.values[0][1]
                        + self.values[1][1] * other.values[1][1]
                        + self.values[1][2] * other.values[2][1],
                    self.values[1][0] * other.values[0][2]
                        + self.values[1][1] * other.values[1][2]
                        + self.values[1][2] * other.values[2][2],
                ],
                [
                    self.values[2][0] * other.values[0][0]
                        + self.values[2][1] * other.values[1][0]
                        + self.values[2][2] * other.values[2][0],
                    self.values[2][0] * other.values[0][1]
                        + self.values[2][1] * other.values[1][1]
                        + self.values[2][2] * other.values[2][1],
                    self.values[2][0] * other.values[0][2]
                        + self.values[2][1] * other.values[1][2]
                        + self.values[2][2] * other.values[2][2],
                ],
            ],
        }
    }

    pub fn transform_xy(&self, xy: crate::Xy<Px>) -> crate::Xy<Px> {
        crate::Xy {
            x: self.values[0][0] * xy.x + self.values[0][1] * xy.y + px(self.values[0][2]),
            y: self.values[1][0] * xy.x + self.values[1][1] * xy.y + px(self.values[1][2]),
        }
    }

    pub fn transform_rect<T>(&self, rect: Rect<T>) -> Rect<T>
    where
        f32: std::ops::Mul<T, Output = T> + Into<T>,
        T: std::ops::Add<Output = T> + Copy,
    {
        let Ltrb {
            left,
            top,
            right,
            bottom,
        } = rect.as_ltrb();
        Rect::Ltrb {
            left: self.values[0][0] * left + self.values[0][1] * top + self.values[0][2].into(),
            top: self.values[1][0] * left + self.values[1][1] * top + self.values[1][2].into(),
            right: self.values[0][0] * right
                + self.values[0][1] * bottom
                + self.values[0][2].into(),
            bottom: self.values[1][0] * right
                + self.values[1][1] * bottom
                + self.values[1][2].into(),
        }
    }
}

impl std::ops::Mul for Matrix3x3 {
    type Output = Matrix3x3;

    fn mul(self, other: Matrix3x3) -> Matrix3x3 {
        self.multiply(&other)
    }
}

impl<'a> std::ops::Mul<&'a Matrix3x3> for Matrix3x3 {
    type Output = Matrix3x3;

    fn mul(self, other: &Matrix3x3) -> Matrix3x3 {
        self.multiply(&other)
    }
}

impl<'b> std::ops::Mul<Matrix3x3> for &'b Matrix3x3 {
    type Output = Matrix3x3;

    fn mul(self, other: Matrix3x3) -> Matrix3x3 {
        self.multiply(&other)
    }
}

impl<'a, 'b> std::ops::Mul<&'a Matrix3x3> for &'b Matrix3x3 {
    type Output = Matrix3x3;

    fn mul(self, other: &Matrix3x3) -> Matrix3x3 {
        self.multiply(&other)
    }
}
