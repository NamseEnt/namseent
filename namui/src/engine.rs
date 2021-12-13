pub(crate) mod draw;
mod engine_common;
mod font;
mod manager;
use std::any::Any;
use std::{sync::Arc, time::Duration};
mod engine_state;
mod skia;
pub use draw::{DrawCall, DrawCommand, PathDrawCommand, TextAlign, TextBaseline, TextDrawCommand};
pub use engine_common::*;
pub use render::{
    rect::*, text::*, text_input_event, translate, types::*, RenderingData, RenderingTree,
    TextInput,
};
use skia::*;
pub use skia::{types::*, Paint, Path};
pub mod event;
pub use event::EngineEvent;
mod render;
use self::manager::{Code, Managers};
use self::{
    engine_state::{get_engine_state, EngineState},
    font::*,
};
mod random;
pub use self::random::*;
pub mod screen;

#[cfg(target_family = "wasm")]
mod engine_web;
#[cfg(target_family = "wasm")]
pub use self::engine_web::*;

pub trait Entity {
    type RenderingContext;
    fn update(&mut self, event: &dyn Any);
    fn render(&self, context: &Self::RenderingContext) -> RenderingTree;
}

pub fn init() -> EngineContext {
    Engine::init()
}

pub async fn start<TRenderingContext>(
    mut engine_context: EngineContext,
    state: &mut dyn Entity<RenderingContext = TRenderingContext>,
    context: &TRenderingContext,
) {
    let mut event_receiver = event::init();

    init_font().await;

    let mut rendering_tree = state.render(context);

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
            _ => {
                state.update(event.as_ref());
                rendering_tree = state.render(context);
            }
        }
    }
}

async fn init_font() {
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
