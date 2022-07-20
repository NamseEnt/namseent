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

pub trait KeyframeValue<TKeyframeLine> {
    fn interpolate(&self, next: &Self, context: &InterpolationContext<TKeyframeLine>) -> Self;
}

pub struct InterpolationContext<'a, TKeyframeLine> {
    pub line: &'a TKeyframeLine,
    pub time_ratio: f32,
    pub duration: Time,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyframeGraph<TValue: KeyframeValue<TKeyframeLine> + Clone, TKeyframeLine> {
    point_line_tuples: Vec<(KeyframePoint<TValue>, TKeyframeLine)>,
}

impl<'a, TValue: KeyframeValue<TKeyframeLine> + Clone, TKeyframeLine>
    KeyframeGraph<TValue, TKeyframeLine>
{
    pub fn new() -> Self {
        Self {
            point_line_tuples: Vec::new(),
        }
    }
    pub fn update_point(
        &mut self,
        point_id: &str,
        update: impl FnOnce(&mut KeyframePoint<TValue>),
    ) -> Result<(), Box<dyn std::error::Error>> {
        let point = self
            .point_line_tuples
            .iter_mut()
            .find(|(point, _)| point.id == point_id)
            .map(|(point, _)| point)
            .ok_or_else(|| format!("Point with id {} not found", point_id))?;
        update(point);

        Ok(())
    }
    pub fn put(&mut self, point: KeyframePoint<TValue>, line: TKeyframeLine) {
        let same_id_point = self
            .point_line_tuples
            .iter_mut()
            .find(|(p, _)| p.id.eq(&point.id));
        match same_id_point {
            Some((p, l)) => {
                *p = point;
                *l = line;
            }
            None => {
                let same_time_point = self
                    .point_line_tuples
                    .iter_mut()
                    .find(|(p, _)| p.time.eq(&point.time));

                match same_time_point {
                    Some((p, l)) => {
                        *p = point;
                        *l = line;
                    }
                    None => {
                        self.point_line_tuples.push((point, line));
                    }
                }
            }
        }

        self.point_line_tuples.sort_by_key(|(point, _)| point.time);
    }
    pub fn get_value(&'a self, time: Time) -> Option<TValue> {
        let mut iter = self.point_line_tuples.iter().peekable();

        loop {
            let (current_point, line) = iter.next()?;
            if current_point.time == time {
                return Some(current_point.value.clone());
            }

            let (next_point, _) = iter.peek()?;
            if current_point.time <= time && time < next_point.time {
                let time_ratio =
                    (time - current_point.time) / (next_point.time - current_point.time);
                return Some(current_point.value.interpolate(
                    &next_point.value,
                    &InterpolationContext {
                        line,
                        time_ratio,
                        duration: next_point.time - current_point.time,
                    },
                ));
            }
        }
    }
    pub fn delete(&mut self, id: impl AsRef<str>) {
        self.point_line_tuples
            .retain(|(point, _)| point.id.ne(id.as_ref()));
    }
    pub fn delete_by_time(&mut self, time: Time) {
        self.point_line_tuples
            .retain(|(point, _)| point.time != time);
    }
    pub fn get_first_point(&self) -> Option<&KeyframePoint<TValue>> {
        self.point_line_tuples.first().map(|(point, _)| point)
    }
    pub fn get_last_point(&self) -> Option<&KeyframePoint<TValue>> {
        self.point_line_tuples.last().map(|(point, _)| point)
    }
    pub fn get_point_line_tuples(
        &self,
    ) -> impl Iterator<Item = &(KeyframePoint<TValue>, TKeyframeLine)> {
        self.point_line_tuples.iter()
    }
    pub fn get_point(&self, id: &str) -> Option<&KeyframePoint<TValue>> {
        self.point_line_tuples
            .iter()
            .find(|(point, _)| point.id.eq(id))
            .map(|(point, _)| point)
    }
    pub fn get_point_by_time(&self, time: Time) -> Option<&KeyframePoint<TValue>> {
        self.point_line_tuples
            .iter()
            .find(|(point, _)| point.time.eq(&time))
            .map(|(point, _)| point)
    }
    pub fn get_point_mut(&mut self, id: &str) -> Option<&mut KeyframePoint<TValue>> {
        self.point_line_tuples
            .iter_mut()
            .find(|(point, _)| point.id.eq(id))
            .map(|(point, _)| point)
    }
    pub fn get_point_mut_by_time(&mut self, time: Time) -> Option<&mut KeyframePoint<TValue>> {
        self.point_line_tuples
            .iter_mut()
            .find(|(point, _)| point.time.eq(&time))
            .map(|(point, _)| point)
    }
    pub fn get_point_and_line(&self, id: &str) -> Option<&(KeyframePoint<TValue>, TKeyframeLine)> {
        self.point_line_tuples
            .iter()
            .find(|(point, _)| point.id.eq(id))
    }
    pub fn get_point_and_line_mut(
        &mut self,
        id: &str,
    ) -> Option<&mut (KeyframePoint<TValue>, TKeyframeLine)> {
        self.point_line_tuples
            .iter_mut()
            .find(|(point, _)| point.id.eq(id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::wasm_bindgen_test;

    struct LinearKeyframeLine {}

    impl KeyframeValue<LinearKeyframeLine> for f32 {
        fn interpolate(
            &self,
            next: &Self,
            context: &InterpolationContext<LinearKeyframeLine>,
        ) -> Self {
            self + (next - self) * context.time_ratio
        }
    }

    #[test]
    #[wasm_bindgen_test]
    fn should_none_if_time_is_before_than_start_point() {
        let mut graph = KeyframeGraph::new();
        graph.put(
            KeyframePoint::new(Time::Ms(5.0), 0.0),
            LinearKeyframeLine {},
        );
        let value = graph.get_value(Time::Ms(1.0));
        assert_eq!(value, None);
    }

    #[test]
    #[wasm_bindgen_test]
    fn should_linear_interpolated_value_if_time_is_between_start_and_end_point_in_linear_line() {
        let mut graph = KeyframeGraph::new();
        graph.put(
            KeyframePoint::new(Time::Ms(0.0), 0.0),
            LinearKeyframeLine {},
        );
        graph.put(
            KeyframePoint::new(Time::Ms(10.0), 10.0),
            LinearKeyframeLine {},
        );
        for time in 0..10 {
            let value = graph.get_value(Time::Ms(time as f32));
            assert_eq!(value, Some(time as f32));
        }
    }

    #[test]
    #[wasm_bindgen_test]
    fn should_get_none_if_time_is_after_last_point() {
        let mut graph = KeyframeGraph::new();
        graph.put(
            KeyframePoint::new(Time::Ms(5.0), 0.0),
            LinearKeyframeLine {},
        );

        let last_point = graph.get_last_point().unwrap();
        let time_after_last_point = last_point.time + Time::Ms(1.0);
        let value = graph.get_value(time_after_last_point);
        assert_eq!(value, None);
    }

    #[test]
    #[wasm_bindgen_test]
    fn should_get_value_if_time_is_on_single_point() {
        let time = Time::Ms(5.0);
        let value = 3.0;

        let mut graph = KeyframeGraph::new();
        graph.put(
            KeyframePoint::new(time.clone(), value.clone()),
            LinearKeyframeLine {},
        );

        let result = graph.get_value(time);
        assert_eq!(result, Some(value));
    }

    #[test]
    #[wasm_bindgen_test]
    fn get_last_point_should_return_start_point_if_no_next_points() {
        let mut graph = KeyframeGraph::new();
        graph.put(
            KeyframePoint::new(Time::Ms(5.0), 0.0),
            LinearKeyframeLine {},
        );
        let last_point = graph.get_last_point().unwrap();
        assert_eq!(last_point.time, Time::Ms(5.0));
        assert_eq!(last_point.value, 0.0);
    }

    #[test]
    #[wasm_bindgen_test]
    fn get_last_point_should_return_last_point_if_next_points_exist() {
        let mut graph = KeyframeGraph::new();
        graph.put(
            KeyframePoint::new(Time::Ms(5.0), 0.0),
            LinearKeyframeLine {},
        );
        graph.put(
            KeyframePoint::new(Time::Ms(10.0), 1.0),
            LinearKeyframeLine {},
        );

        let last_point = graph.get_last_point().unwrap();
        assert_eq!(last_point.time, Time::Ms(10.0));
        assert_eq!(last_point.value, 1.0);
    }

    #[test]
    #[wasm_bindgen_test]
    fn get_last_point_should_order_by_time() {
        let mut graph = KeyframeGraph::new();
        graph.put(
            KeyframePoint::new(Time::Ms(5.0), 0.0),
            LinearKeyframeLine {},
        );
        graph.put(
            KeyframePoint::new(Time::Ms(10.0), 1.0),
            LinearKeyframeLine {},
        );
        graph.put(
            KeyframePoint::new(Time::Ms(1.0), 2.0),
            LinearKeyframeLine {},
        );

        let last_point = graph.get_last_point().unwrap();
        assert_eq!(last_point.time, Time::Ms(10.0));
        assert_eq!(last_point.value, 1.0);
    }

    #[test]
    #[wasm_bindgen_test]
    fn get_point_with_id_should_work() {
        let mut graph = KeyframeGraph::new();
        let point = KeyframePoint::new(Time::Ms(5.0), 0.0);
        graph.put(point, LinearKeyframeLine {});

        let last_point = graph.get_last_point();
        assert_eq!(last_point.is_some(), true);
        let last_point = last_point.unwrap();
        assert_eq!(last_point.time, Time::Ms(5.0));
        assert_eq!(last_point.value, 0.0);
    }

    #[test]
    #[wasm_bindgen_test]
    fn should_delete_by_id() {
        let mut graph = KeyframeGraph::new();
        let first_point = KeyframePoint::new(Time::Ms(0.0), 0.0);
        let second_point = KeyframePoint::new(Time::Ms(10.0), 10.0);
        graph.put(first_point.clone(), LinearKeyframeLine {});
        graph.put(second_point.clone(), LinearKeyframeLine {});

        graph.delete(second_point.id);

        assert_eq!(graph.get_last_point(), Some(&first_point.clone()));
        assert_eq!(graph.get_point_line_tuples().count(), 1);
    }

    #[test]
    #[wasm_bindgen_test]
    fn should_delete_by_time() {
        let mut graph = KeyframeGraph::new();
        let first_time = Time::Ms(1.0);
        let first_point = KeyframePoint::new(first_time, 10.0);
        let second_time = Time::Ms(2.0);
        let second_point = KeyframePoint::new(second_time, 10.0);
        let third_time = Time::Ms(3.0);
        let third_point = KeyframePoint::new(third_time, 10.0);
        graph.put(first_point.clone(), LinearKeyframeLine {});
        graph.put(second_point.clone(), LinearKeyframeLine {});
        graph.put(third_point.clone(), LinearKeyframeLine {});

        graph.delete_by_time(second_time);

        assert_eq!(graph.get_last_point(), Some(&third_point));
        assert_eq!(graph.get_point_line_tuples().count(), 2);

        graph.delete_by_time(third_time);

        assert_eq!(graph.get_last_point(), Some(&first_point));
        assert_eq!(graph.get_point_line_tuples().count(), 1);

        graph.delete_by_time(first_time);

        assert_eq!(graph.get_last_point(), None);
        assert_eq!(graph.get_point_line_tuples().count(), 0);
    }
}
