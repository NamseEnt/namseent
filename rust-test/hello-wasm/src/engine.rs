pub(crate) mod draw;
mod engine_common;
mod font;
mod manager;
use std::borrow::{Borrow, BorrowMut};
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

#[cfg(target_family = "wasm")]
mod engine_web;

#[cfg(target_family = "wasm")]
pub use self::engine_web::*;
use self::manager::Managers;
use self::{
    engine_state::{get_engine_state, EngineState},
    font::*,
};

pub async fn start<TState: 'static + std::marker::Send>(state: TState, render: Render<TState>) {
    let mut engine_context = Engine::init(state, render).await;

    init_font(&mut engine_context).await;

    let boxed_engine_context = Box::new(engine_context);

    Engine::request_animation_frame(Box::new(move || {
        on_frame(boxed_engine_context);
    }));
}

async fn init_font<TState: 'static + std::marker::Send>(
    engine_context: &mut EngineContext<TState>,
) {
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

fn on_frame<TState: 'static + std::marker::Send>(
    mut boxed_engine_context: Box<EngineContext<TState>>,
) {
    let engine_context = &mut *boxed_engine_context;

    update_fps_info(&mut engine_context.fps_info);

    let rendering_tree = (engine_context.render)(&mut engine_context.state);
    match serde_json::to_string(&rendering_tree) {
        Ok(s) => {
            log(s);
        }
        Err(e) => {
            log(format!("Failed to serialize rendering tree: {}", e));
        }
    };
    rendering_tree.draw(&engine_context);

    engine_context.surface.flush();

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

pub fn state() -> Arc<EngineState> {
    get_engine_state()
}

pub fn managers() -> std::sync::MutexGuard<'static, Managers> {
    get_managers()
}

pub fn log(format: String) {
    Engine::log(format);
}
