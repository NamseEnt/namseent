use super::*;
use std::fmt::Display;

define_singular_floating_tuple!(Percent, f32, |f32_value| 100.0 * f32_value, |tuple_value| {
    tuple_value / 100.0
});

impl Display for Percent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let in_f32: f32 = self.into();
        write!(f, "{:.*?}%", f.precision().unwrap_or(1), in_f32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::wasm_bindgen_test;

    #[test]
    #[wasm_bindgen_test]
    fn percent_multiply_should_work() {
        let a = 4.0_f32;
        let b = Percent::new(150.0);
        let c = 6.0_f32;
        let a_b: i32 = (a * b).into();

        assert_eq!(c as i32, a_b);
    }
}
