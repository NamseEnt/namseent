#[macro_use]
mod define_singular_floating_tuple;
mod time;
pub use self::time::*;
mod time_per_pixel;
pub use self::time_per_pixel::*;
mod pixel_size;
pub use pixel_size::*;
mod degree;
pub use degree::*;
mod one_zero;
pub use one_zero::*;
mod radian;
pub use radian::*;
mod percent;
pub use percent::*;

// NOTE: Please move type into new file when it has impl.

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::wasm_bindgen_test;

    #[test]
    #[wasm_bindgen_test]
    fn sub_singular_floating_tuple() {
        define_singular_floating_tuple!(A, f32);
        assert_eq!(A::from(80.0), A::from(100.0) - A::from(20.0));
    }
}
