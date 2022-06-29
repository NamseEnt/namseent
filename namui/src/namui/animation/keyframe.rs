use crate::types::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct KeyframePoint<T: Clone> {
    id: String,
    pub time: Time,
    pub value: T,
}

impl<T: Clone> KeyframePoint<T> {
    pub fn new(time: Time, value: T) -> Self {
        Self {
            id: crate::nanoid(),
            time,
            value,
        }
    }
    pub fn id(&self) -> &str {
        &self.id
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KeyframeLine {
    Linear,
}

pub trait KeyframeValue {
    fn interpolate(&self, next: &Self, ratio: f32) -> Self;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyframeGraph<TValue: KeyframeValue + Clone> {
    points_with_lines: Vec<(KeyframePoint<TValue>, KeyframeLine)>,
}

impl<'a, TValue: KeyframeValue + Clone> KeyframeGraph<TValue> {
    pub fn new() -> Self {
        Self {
            points_with_lines: Vec::new(),
        }
    }
    pub fn put(&mut self, point: KeyframePoint<TValue>, line: KeyframeLine) {
        let same_id_point = self
            .points_with_lines
            .iter_mut()
            .find(|(p, _)| p.id.eq(&point.id));
        match same_id_point {
            Some((p, l)) => {
                *p = point;
                *l = line;
            }
            None => {
                let same_time_point = self
                    .points_with_lines
                    .iter_mut()
                    .find(|(p, _)| p.time.eq(&point.time));

                match same_time_point {
                    Some((p, l)) => {
                        *p = point;
                        *l = line;
                    }
                    None => {
                        self.points_with_lines.push((point, line));
                    }
                }
            }
        }

        self.points_with_lines.sort_by_key(|(point, _)| point.time);
    }
    pub fn get_value(&'a self, time: Time) -> Option<TValue> {
        let mut iter = self.points_with_lines.iter().peekable();

        loop {
            let (current_point, line) = iter.next()?;
            if current_point.time == time {
                return Some(current_point.value.clone());
            }

            let (next_point, _) = iter.peek()?;
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
        }
    }
    pub fn delete(&mut self, id: impl AsRef<str>) {
        self.points_with_lines
            .retain(|(point, _)| point.id.ne(id.as_ref()));
    }
    pub fn delete_by_time(&mut self, time: Time) {
        self.points_with_lines
            .retain(|(point, _)| point.time != time);
    }
    pub fn get_first_point(&self) -> Option<&KeyframePoint<TValue>> {
        self.points_with_lines.first().map(|(point, _)| point)
    }
    pub fn get_last_point(&self) -> Option<&KeyframePoint<TValue>> {
        self.points_with_lines.last().map(|(point, _)| point)
    }
    pub fn get_points_with_lines(&self) -> &[(KeyframePoint<TValue>, KeyframeLine)] {
        &self.points_with_lines
    }
    pub fn get_point(&self, id: impl AsRef<str>) -> Option<&KeyframePoint<TValue>> {
        self.points_with_lines
            .iter()
            .find(|(point, _)| point.id.eq(id.as_ref()))
            .map(|(point, _)| point)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::wasm_bindgen_test;

    #[test]
    #[wasm_bindgen_test]
    fn should_none_if_time_is_before_than_start_point() {
        let mut graph = KeyframeGraph::new();
        graph.put(
            KeyframePoint::new(Time::from_ms(5.0), 0.0),
            KeyframeLine::Linear,
        );
        let value = graph.get_value(Time::from_ms(1.0));
        assert_eq!(value, None);
    }

    #[test]
    #[wasm_bindgen_test]
    fn should_linear_interpolated_value_if_time_is_between_start_and_end_point_in_linear_line() {
        let mut graph = KeyframeGraph::new();
        graph.put(
            KeyframePoint::new(Time::from_ms(0.0), 0.0),
            KeyframeLine::Linear,
        );
        graph.put(
            KeyframePoint::new(Time::from_ms(10.0), 10.0),
            KeyframeLine::Linear,
        );
        for time in 0..10 {
            let value = graph.get_value(Time::from_ms(time as f32));
            assert_eq!(value, Some(time as f32));
        }
    }

    #[test]
    #[wasm_bindgen_test]
    fn should_get_none_if_time_is_after_last_point() {
        let mut graph = KeyframeGraph::new();
        graph.put(
            KeyframePoint::new(Time::from_ms(5.0), 0.0),
            KeyframeLine::Linear,
        );

        let last_point = graph.get_last_point().unwrap();
        let time_after_last_point = last_point.time + Time::from_ms(1.0);
        let value = graph.get_value(time_after_last_point);
        assert_eq!(value, None);
    }

    #[test]
    #[wasm_bindgen_test]
    fn should_get_value_if_time_is_on_single_point() {
        let time = Time::from_ms(5.0);
        let value = 3.0;

        let mut graph = KeyframeGraph::new();
        graph.put(
            KeyframePoint::new(time.clone(), value.clone()),
            KeyframeLine::Linear,
        );

        let result = graph.get_value(time);
        assert_eq!(result, Some(value));
    }

    #[test]
    #[wasm_bindgen_test]
    fn get_last_point_should_return_start_point_if_no_next_points() {
        let mut graph = KeyframeGraph::new();
        graph.put(
            KeyframePoint::new(Time::from_ms(5.0), 0.0),
            KeyframeLine::Linear,
        );
        let last_point = graph.get_last_point().unwrap();
        assert_eq!(last_point.time, Time::from_ms(5.0));
        assert_eq!(last_point.value, 0.0);
    }

    #[test]
    #[wasm_bindgen_test]
    fn get_last_point_should_return_last_point_if_next_points_exist() {
        let mut graph = KeyframeGraph::new();
        graph.put(
            KeyframePoint::new(Time::from_ms(5.0), 0.0),
            KeyframeLine::Linear,
        );
        graph.put(
            KeyframePoint::new(Time::from_ms(10.0), 1.0),
            KeyframeLine::Linear,
        );

        let last_point = graph.get_last_point().unwrap();
        assert_eq!(last_point.time, Time::from_ms(10.0));
        assert_eq!(last_point.value, 1.0);
    }

    #[test]
    #[wasm_bindgen_test]
    fn get_last_point_should_order_by_time() {
        let mut graph = KeyframeGraph::new();
        graph.put(
            KeyframePoint::new(Time::from_ms(5.0), 0.0),
            KeyframeLine::Linear,
        );
        graph.put(
            KeyframePoint::new(Time::from_ms(10.0), 1.0),
            KeyframeLine::Linear,
        );
        graph.put(
            KeyframePoint::new(Time::from_ms(1.0), 2.0),
            KeyframeLine::Linear,
        );

        let last_point = graph.get_last_point().unwrap();
        assert_eq!(last_point.time, Time::from_ms(10.0));
        assert_eq!(last_point.value, 1.0);
    }

    #[test]
    #[wasm_bindgen_test]
    fn get_point_with_id_should_work() {
        let mut graph = KeyframeGraph::new();
        let point = KeyframePoint::new(Time::from_ms(5.0), 0.0);
        graph.put(point, KeyframeLine::Linear);

        let last_point = graph.get_last_point();
        assert_eq!(last_point.is_some(), true);
        let last_point = last_point.unwrap();
        assert_eq!(last_point.time, Time::from_ms(5.0));
        assert_eq!(last_point.value, 0.0);
    }

    #[test]
    #[wasm_bindgen_test]
    fn should_delete_by_id() {
        let mut graph = KeyframeGraph::new();
        let first_point = KeyframePoint::new(Time::from_ms(0.0), 0.0);
        let second_point = KeyframePoint::new(Time::from_ms(10.0), 10.0);
        graph.put(first_point.clone(), KeyframeLine::Linear);
        graph.put(second_point.clone(), KeyframeLine::Linear);

        graph.delete(second_point.id);

        assert_eq!(graph.get_last_point(), Some(&first_point.clone()));
        assert_eq!(graph.get_points_with_lines().len(), 1);
    }

    #[test]
    #[wasm_bindgen_test]
    fn should_delete_by_time() {
        let mut graph = KeyframeGraph::new();
        let first_time = Time::from_ms(1.0);
        let first_point = KeyframePoint::new(first_time, 10.0);
        let second_time = Time::from_ms(2.0);
        let second_point = KeyframePoint::new(second_time, 10.0);
        let third_time = Time::from_ms(3.0);
        let third_point = KeyframePoint::new(third_time, 10.0);
        graph.put(first_point.clone(), KeyframeLine::Linear);
        graph.put(second_point.clone(), KeyframeLine::Linear);
        graph.put(third_point.clone(), KeyframeLine::Linear);

        graph.delete_by_time(second_time);

        assert_eq!(graph.get_last_point(), Some(&third_point));
        assert_eq!(graph.get_points_with_lines().len(), 2);

        graph.delete_by_time(third_time);

        assert_eq!(graph.get_last_point(), Some(&first_point));
        assert_eq!(graph.get_points_with_lines().len(), 1);

        graph.delete_by_time(first_time);

        assert_eq!(graph.get_last_point(), None);
        assert_eq!(graph.get_points_with_lines().len(), 0);
    }
}
