#[derive(Clone, Copy, Debug)]
pub(crate) struct Matrix3x3 {
    pub values: [[f32; 3]; 3],
}

impl Matrix3x3 {
    pub(crate) fn new(
        a: f32,
        b: f32,
        c: f32,
        d: f32,
        e: f32,
        f: f32,
        g: f32,
        h: f32,
        i: f32,
    ) -> Self {
        Matrix3x3 {
            values: [[a, b, c], [d, e, f], [g, h, i]],
        }
    }
    pub(crate) fn from_slice(values: &[[f32; 3]; 3]) -> Self {
        Matrix3x3 {
            values: [
                [values[0][0], values[0][1], values[0][2]],
                [values[1][0], values[1][1], values[1][2]],
                [values[2][0], values[2][1], values[2][2]],
            ],
        }
    }
    pub(crate) fn identity() -> Self {
        Self::from_slice(&[[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]])
    }
    pub(crate) fn multiply(&self, other: &Matrix3x3) -> Matrix3x3 {
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

    pub(crate) fn transform_xy(&self, xy: &crate::Xy<f32>) -> crate::Xy<f32> {
        crate::Xy {
            x: self.values[0][0] * xy.x + self.values[0][1] * xy.y + self.values[0][2],
            y: self.values[1][0] * xy.x + self.values[1][1] * xy.y + self.values[1][2],
        }
    }

    pub(crate) fn transform_rect(&self, rect: &crate::LtrbRect) -> crate::LtrbRect {
        crate::LtrbRect {
            left: self.values[0][0] * rect.left + self.values[0][1] * rect.top + self.values[0][2],
            top: self.values[1][0] * rect.left + self.values[1][1] * rect.top + self.values[1][2],
            right: self.values[0][0] * rect.right
                + self.values[0][1] * rect.bottom
                + self.values[0][2],
            bottom: self.values[1][0] * rect.right
                + self.values[1][1] * rect.bottom
                + self.values[1][2],
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
