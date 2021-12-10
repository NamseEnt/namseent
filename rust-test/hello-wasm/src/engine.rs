pub(crate) mod draw;
mod engine_common;
mod font;
mod manager;
use std::any::Any;
use std::{sync::Arc, time::Duration};
mod engine_state;
mod skia;
pub use draw::{
    DrawCall, DrawCommand, PathDrawCommand, RenderingData, RenderingTree, TextAlign, TextBaseline,
    TextDrawCommand,
};
pub use engine_common::*;
pub use render::types::*;
use skia::*;
pub use skia::{types::*, Paint, Path};
mod render;
pub use render::rect::*;
pub use render::text::*;
pub mod event;

#[cfg(target_family = "wasm")]
mod engine_web;

#[cfg(target_family = "wasm")]
pub use self::engine_web::*;
use self::manager::Managers;
use self::{
    engine_state::{get_engine_state, EngineState},
    font::*,
};

pub trait Update {
    fn update(&self, event: &dyn Any) -> Self;
}

pub async fn start<TState>(mut state: TState, render: Render<TState>)
where
    TState: Update + 'static + std::marker::Send,
{
    let mut event_receiver = event::init();
    let mut engine_context = Engine::init();

    init_font(&mut engine_context).await;

    let mut rendering_tree = render(&state);

    Engine::request_animation_frame(Box::new(move || {
        on_frame();
    }));

    loop {
        let event = event_receiver.recv().await.unwrap();

        match event.downcast_ref::<EngineEvent>() {
            Some(EngineEvent::AnimationFrame) => {
                update_fps_info(&mut engine_context.fps_info);

                rendering_tree.draw(&engine_context);

                engine_context.surface.flush();
            }
            Some(EngineEvent::MoveClick(xy)) => {
                rendering_tree.call_on_click(xy);
            }
            None => {
                state = state.update(&event);
                rendering_tree = render(&state);
            }
        }
    }
}

async fn init_font(engine_context: &mut EngineContext) {
    let font_manager = &mut *managers().font_manager;
    let typeface_manager = &mut font_manager.typeface_manager;

    match load_sans_typeface_of_all_languages(typeface_manager).await {
        Ok(()) => {
            log("Font loaded".to_string());
        }
        Err(e) => {
            log(format!("Font loading failed: {}", e));
        }
    };
}

enum EngineEvent {
    AnimationFrame,
    MoveClick(Xy<f32>),
}

fn on_frame() {
    event::send(Box::new(EngineEvent::AnimationFrame));

    Engine::request_animation_frame(Box::new(move || {
        on_frame();
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

pub fn state() -> Arc<EngineState> {
    get_engine_state()
}

pub fn managers() -> std::sync::MutexGuard<'static, Managers> {
    get_managers()
}

pub fn log(format: String) {
    Engine::log(format);
}
