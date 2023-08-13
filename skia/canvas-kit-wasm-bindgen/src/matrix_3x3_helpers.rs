use super::*;

unsafe impl Sync for Matrix3x3Helpers {}
unsafe impl Send for Matrix3x3Helpers {}

#[wasm_bindgen]
extern "C" {
    pub type Matrix3x3Helpers;

    // ///
    // ///Returns a new identity 3x3 matrix.
    // #[wasm_bindgen(method)]
    // pub fn identity(this: &Matrix3x3Helpers) -> Box<[f32]>;

    ///
    ///Returns the inverse of the given 3x3 matrix or null if it is not invertible.
    ///@param m
    #[wasm_bindgen(method)]
    pub fn invert(this: &Matrix3x3Helpers, matrix: &[f32]) -> Box<[f32]>;

    // ///
    // ///Maps the given 2d points according to the given 3x3 matrix.
    // ///@param m
    // ///@param points - the flattened points to map; the results are computed in place on this array.
    // #[wasm_bindgen(method)]
    // pub fn mapPoints(this: &Matrix3x3Helpers,m: Matrix3x3 | [f32], points: [f32]) -> Box<[f32]>;

    // ///
    // ///Multiplies the provided 3x3 matrices together from left to right.
    // ///@param matrices
    // #[wasm_bindgen(method)]
    // pub fn multiply(this: &Matrix3x3Helpers,...matrices: Array<(Matrix3x3 | [f32])>) -> Box<[f32]>;

    // ///
    // ///Returns a new 3x3 matrix representing a rotation by n radians.
    // ///@param radians
    // ///@param px - the X value to rotate around, defaults to 0.
    // ///@param py - the Y value to rotate around, defaults to 0.
    // #[wasm_bindgen(method)]
    // pub fn rotated(this: &Matrix3x3Helpers,radians: AngleInRadians, px?: number, py?: number) -> Box<[f32]>;

    // ///
    // ///Returns a new 3x3 matrix representing a scale in the x and y directions.
    // ///@param sx - the scale in the X direction.
    // ///@param sy - the scale in the Y direction.
    // ///@param px - the X value to scale from, defaults to 0.
    // ///@param py - the Y value to scale from, defaults to 0.
    // #[wasm_bindgen(method)]
    // pub fn scaled(this: &Matrix3x3Helpers,sx: number, sy: number, px?: number, py?: number) -> Box<[f32]>;

    // ///
    // ///Returns a new 3x3 matrix representing a scale in the x and y directions.
    // ///@param kx - the kurtosis in the X direction.
    // ///@param ky - the kurtosis in the Y direction.
    // ///@param px - the X value to skew from, defaults to 0.
    // ///@param py - the Y value to skew from, defaults to 0.
    // #[wasm_bindgen(method)]
    // pub fn skewed(this: &Matrix3x3Helpers,kx: number, ky: number, px?: number, py?: number) -> Box<[f32]>;

    // ///
    // ///Returns a new 3x3 matrix representing a translation in the x and y directions.
    // ///@param dx
    // ///@param dy
    // #[wasm_bindgen(method)]
    // pub fn translated(this: &Matrix3x3Helpers,dx: number, dy: number) -> Box<[f32]>;

}
