pub(crate) mod draw;
mod engine_common;
use std::time::Duration;

pub use engine_common::*;

#[cfg(target_family = "wasm")]
mod engine_web;

#[cfg(target_family = "wasm")]
pub use self::engine_web::*;

pub fn start_engine<TState: 'static>(state: TState, render: Render<TState>) {
    let engine_context = Engine::init(state, render);
    let boxed_engine_context = Box::new(engine_context);

    Engine::request_animation_frame(Box::new(move || {
        on_frame(boxed_engine_context);
    }));
}

fn on_frame<TState: 'static>(mut boxed_engine_context: Box<EngineContext<TState>>) {
    let engine_context = &mut *boxed_engine_context;

    update_fps_info(&mut engine_context.fps_info);

    (engine_context.render)(&mut engine_context.state);

    Engine::request_animation_frame(Box::new(move || {
        on_frame(boxed_engine_context);
    }));
}

fn update_fps_info(fps_info: &mut FpsInfo) {
    let now = Engine::now();
    let duration = now - fps_info.last_60_frame_time;

    if duration > Duration::from_secs(1) {
        fps_info.last_60_frame_time = Engine::now();
        fps_info.fps = (fps_info.frame_count as f32 / duration.as_secs_f32()) as u16;
        fps_info.frame_count = 0;

        Engine::log(format!("FPS: {}", fps_info.fps));
    } else {
        fps_info.frame_count += 1;
    }
}
