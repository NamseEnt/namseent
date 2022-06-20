use super::*;
use std::fmt::Display;

define_singular_floating_tuple!(Percent, f32, |f32_value| 100.0 * f32_value, |tuple_value| {
    tuple_value / 100.0
});

impl Display for Percent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.*?}%", f.precision().unwrap_or(1), self.0)
    }
}

impl Percent {
    pub fn new(percent: f32) -> Percent {
        Percent(percent)
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

    #[test]
    #[wasm_bindgen_test]
    fn percent_display_should_work() {
        let b = Percent::new(150.0);

        assert_eq!(format!("{}", b), "150.0%");
    }
}
