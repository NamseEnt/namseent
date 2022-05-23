use crate::types::*;

pub struct KeyframePoint<T> {
    pub time: Time,
    pub value: T,
}

pub enum KeyframeLine {
    Linear,
}

pub trait KeyframeValue {
    fn interpolate(&self, next: &Self, ratio: f32) -> Self;
    fn unit() -> &'static str;
}

pub struct KeyframeGraph<TValue: KeyframeValue> {
    start_point: KeyframePoint<TValue>,
    next_points_with_lines: Vec<(KeyframePoint<TValue>, KeyframeLine)>,
}

impl<'a, TValue: KeyframeValue> KeyframeGraph<TValue> {
    pub fn new(start_point: KeyframePoint<TValue>) -> Self {
        Self {
            start_point,
            next_points_with_lines: Vec::new(),
        }
    }
    pub fn push(&mut self, point: KeyframePoint<TValue>, line: KeyframeLine) {
        self.next_points_with_lines.push((point, line));
        self.next_points_with_lines
            .sort_by_key(|(point, _)| point.time);
    }
    pub fn get_value(&'a self, time: &Time) -> Option<TValue> {
        let mut current_point = &self.start_point;
        for (next_point, line) in &self.next_points_with_lines {
            if current_point.time <= time && time < next_point.time {
                match line {
                    KeyframeLine::Linear => {
                        let relative_time_ratio =
                            (time - current_point.time) / (next_point.time - current_point.time);
                        return Some(
                            current_point
                                .value
                                .interpolate(&next_point.value, relative_time_ratio),
                        );
                    }
                }
            }
            current_point = next_point;
        }
        None
    }
    pub(crate) fn get_last_point(&self) -> &KeyframePoint<TValue> {
        &self
            .next_points_with_lines
            .last()
            .map(|(point, _)| point)
            .unwrap_or(&self.start_point)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::wasm_bindgen_test;

    impl KeyframeValue for f32 {
        fn interpolate(&self, next: &Self, ratio: f32) -> Self {
            self * (1.0 - ratio) + next * ratio
        }

        fn unit() -> &'static str {
            todo!()
        }
    }

    #[test]
    #[wasm_bindgen_test]
    fn should_none_if_time_is_before_than_start_point() {
        let graph = KeyframeGraph::new(KeyframePoint {
            time: Time::from_ms(5.0),
            value: 0.0,
        });
        let value = graph.get_value(&Time::from_ms(1.0));
        assert_eq!(value, None);
    }

    #[test]
    #[wasm_bindgen_test]
    fn should_linear_interpolated_value_if_time_is_between_start_and_end_point_in_linear_line() {
        let mut graph = KeyframeGraph::new(KeyframePoint {
            time: Time::from_ms(0.0),
            value: 0.0,
        });
        graph.push(
            KeyframePoint {
                time: Time::from_ms(10.0),
                value: 10.0,
            },
            KeyframeLine::Linear,
        );
        for time in 0..10 {
            let value = graph.get_value(&Time::from_ms(time as f32));
            assert_eq!(value, Some(time as f32));
        }
    }

    #[test]
    #[wasm_bindgen_test]
    fn should_last_value_if_time_is_after_last_point() {
        let graph = KeyframeGraph::new(KeyframePoint {
            time: Time::from_ms(5.0),
            value: 0.0,
        });

        let last_point = graph.get_last_point();
        let time_after_last_point = last_point.time + Time::from_ms(1.0);
        let value = graph.get_value(&time_after_last_point);
        assert_eq!(value, None);
    }

    #[test]
    #[wasm_bindgen_test]
    fn get_last_point_should_return_start_point_if_no_next_points() {
        let graph = KeyframeGraph::new(KeyframePoint {
            time: Time::from_ms(5.0),
            value: 0.0,
        });
        let last_point = graph.get_last_point();
        assert_eq!(last_point.time, Time::from_ms(5.0));
        assert_eq!(last_point.value, 0.0);
    }

    #[test]
    #[wasm_bindgen_test]
    fn get_last_point_should_return_last_point_if_next_points_exist() {
        let mut graph = KeyframeGraph::new(KeyframePoint {
            time: Time::from_ms(5.0),
            value: 0.0,
        });
        graph.push(
            KeyframePoint {
                time: Time::from_ms(10.0),
                value: 1.0,
            },
            KeyframeLine::Linear,
        );

        let last_point = graph.get_last_point();
        assert_eq!(last_point.time, Time::from_ms(10.0));
        assert_eq!(last_point.value, 1.0);
    }

    #[test]
    #[wasm_bindgen_test]
    fn get_last_point_should_order_by_time() {
        let mut graph = KeyframeGraph::new(KeyframePoint {
            time: Time::from_ms(5.0),
            value: 0.0,
        });
        graph.push(
            KeyframePoint {
                time: Time::from_ms(10.0),
                value: 1.0,
            },
            KeyframeLine::Linear,
        );
        graph.push(
            KeyframePoint {
                time: Time::from_ms(1.0),
                value: 2.0,
            },
            KeyframeLine::Linear,
        );

        let last_point = graph.get_last_point();
        assert_eq!(last_point.time, Time::from_ms(10.0));
        assert_eq!(last_point.value, 1.0);
    }
}
