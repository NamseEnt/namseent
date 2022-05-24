#[macro_use]
mod define_singular_floating_tuple;
mod time;
pub use self::time::*;
mod time_per_pixel;
pub use self::time_per_pixel::*;

// NOTE: Please move type into new file when it has impl.
define_singular_floating_tuple!(PixelSize, f32); // NOTE: `PixelSize` naming is for distinguishing from `PixelColor`.
define_singular_floating_tuple!(Angle, f32);
define_singular_floating_tuple!(OneZero, f32, |value| num::clamp(value, 0.0, 1.0));

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::wasm_bindgen_test;

    #[test]
    #[wasm_bindgen_test]
    fn sub_singular_floating_tuple() {
        define_singular_floating_tuple!(A, f32);
        assert_eq!(A::new(80.0), A::new(100.0) - A::new(20.0));
    }
}
