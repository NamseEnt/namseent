use super::spring::with_spring;
use namui::*;
use std::ops::{Add, Mul, Sub};

/// Xy<T> 타입에 특화된 spring 애니메이션
pub fn xy_with_spring<T>(ctx: &RenderCtx, target_xy: Xy<T>, initial_xy: Xy<T>) -> Xy<T>
where
    T: Copy
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<f32, Output = T>
        + PartialEq
        + std::fmt::Debug
        + Send
        + Sync
        + 'static
        + State,
    T: From<f32> + Into<f32>,
{
    with_spring(
        ctx,
        target_xy,
        initial_xy,
        |xy| {
            let x: f32 = xy.x.into();
            let y: f32 = xy.y.into();
            x * x + y * y // 제곱 거리
        },
        || Xy::new(T::from(0.0), T::from(0.0)),
    )
}
