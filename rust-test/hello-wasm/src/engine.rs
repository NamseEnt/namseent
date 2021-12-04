pub(crate) mod draw;
mod engine_common;
mod font;
mod manager;
use std::{borrow::Borrow, time::Duration};

pub use engine_common::*;

#[cfg(target_family = "wasm")]
mod engine_web;

#[cfg(target_family = "wasm")]
pub use self::engine_web::*;
use self::font::*;

pub async fn start_engine<TState: 'static + std::marker::Send>(
    state: TState,
    render: Render<TState>,
) {
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
    let typeface_manager = &mut *engine_context.typeface_manager;
    load_sans_typeface_of_all_languages(typeface_manager).await;
}

fn on_frame<TState: 'static + std::marker::Send>(
    mut boxed_engine_context: Box<EngineContext<TState>>,
) {
    let engine_context = &mut *boxed_engine_context;

    update_fps_info(&mut engine_context.fps_info);

    let engine_state = get_engine_state(engine_context);

    let rendering_tree = (engine_context.render)(&engine_state, &mut engine_context.state);
    match rendering_tree {
        Some(rendering_tree) => rendering_tree.draw(),
        None => (),
    }

    engine_context.surface.flush();

    Engine::request_animation_frame(Box::new(move || {
        on_frame(boxed_engine_context);
    }));
}

fn get_engine_state<TState>(engine_context: &EngineContext<TState>) -> EngineState {
    EngineState {
        mouse_position: engine_context.mouse_manager.mouse_position(),
    }
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
