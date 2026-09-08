#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct CubicBezier {
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
}

impl CubicBezier {
    pub(crate) const fn new(x1: f32, y1: f32, x2: f32, y2: f32) -> Self {
        Self { x1, y1, x2, y2 }
    }

    pub(crate) fn sample(self, progress: f32) -> f32 {
        let progress = progress.clamp(0.0, 1.0);
        if progress <= 0.0 || progress >= 1.0 {
            return progress;
        }

        let mut lower = 0.0;
        let mut upper = 1.0;
        for _ in 0..12 {
            let parameter = (lower + upper) * 0.5;
            if Self::axis(parameter, self.x1, self.x2) < progress {
                lower = parameter;
            } else {
                upper = parameter;
            }
        }
        Self::axis((lower + upper) * 0.5, self.y1, self.y2)
    }

    fn axis(parameter: f32, first_control: f32, second_control: f32) -> f32 {
        let inverse = 1.0 - parameter;
        3.0 * inverse * inverse * parameter * first_control
            + 3.0 * inverse * parameter * parameter * second_control
            + parameter * parameter * parameter
    }
}

#[cfg(test)]
mod tests {
    use super::CubicBezier;

    #[test]
    fn samples_endpoints_exactly() {
        let curve = CubicBezier::new(0.0, 0.6, 1.0, 0.12);
        assert_eq!(curve.sample(0.0), 0.0);
        assert_eq!(curve.sample(1.0), 1.0);
    }

    #[test]
    fn samples_the_requested_curve() {
        let curve = CubicBezier::new(0.0, 0.6, 1.0, 0.12);
        assert!((curve.sample(0.5) - 0.395).abs() < 0.001);
        assert!(curve.sample(0.2) > 0.2);
        assert!(curve.sample(0.8) < 0.8);
    }
}
