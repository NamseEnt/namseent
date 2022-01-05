mod common;
pub(crate) mod draw;
mod font;
mod manager;
use std::any::Any;
use std::{sync::Arc, time::Duration};
mod namui_state;
mod skia;
pub use common::*;
pub use draw::{DrawCall, DrawCommand, PathDrawCommand, TextAlign, TextBaseline, TextDrawCommand};
pub use render::{
    clip, image::*, path::*, rect::*, text::*, text_input_event, translate, types::*, ImageSource,
    MouseCursor, MouseEvent, MouseEventCallback, MouseEventType, RenderingData, RenderingTree,
    TextInput, WheelEventCallback,
};
pub(crate) use skia::Path;
pub use skia::{
    types::{ClipOp, Color, PaintStyle},
    BlendMode, ColorFilter, Font, Image, LtrbRect, Paint, PathBuilder, StrokeCap, Typeface,
};
pub mod event;
pub use event::NamuiEvent;
mod render;
use self::manager::{Code, Managers};
use self::{
    font::*,
    namui_state::{get_namui_state, NamuiState},
};
mod random;
pub use self::random::*;
pub mod screen;

#[cfg(target_family = "wasm")]
mod namui_web;
#[cfg(target_family = "wasm")]
pub use self::namui_web::*;

pub trait Entity {
    type Props;
    fn update(&mut self, event: &dyn Any);
    fn render(&self, props: &Self::Props) -> RenderingTree;
}

pub fn init() -> NamuiContext {
    Namui::init()
}

pub async fn start<TProps>(
    mut namui_context: NamuiContext,
    state: &mut dyn Entity<Props = TProps>,
    props: &TProps,
) {
    let mut event_receiver = event::init();

    init_font().await;

    let mut rendering_tree = state.render(props);

    Namui::request_animation_frame(Box::new(move || {
        on_frame();
    }));

    let mut event_count = 0;

    loop {
        let event = event_receiver.recv().await.unwrap();
        event_count += 1;

        match event.downcast_ref::<NamuiEvent>() {
            Some(NamuiEvent::AnimationFrame) => {
                update_fps_info(&mut namui_context.fps_info);

                rendering_tree.draw(&namui_context);

                set_mouse_cursor(&rendering_tree);

                namui_context.surface.flush();

                if namui_context.fps_info.frame_count == 0 {
                    log(format!("event_count: {}", event_count));
                    event_count = 0;
                }
            }
            Some(NamuiEvent::MouseDown(xy)) => {
                rendering_tree.call_mouse_event(MouseEventType::Down, xy);
                state.update(event.as_ref());
                rendering_tree = state.render(props);
            }
            Some(NamuiEvent::MouseUp(xy)) => {
                rendering_tree.call_mouse_event(MouseEventType::Up, xy);
                state.update(event.as_ref());
                rendering_tree = state.render(props);
            }
            Some(NamuiEvent::MouseMove(xy)) => {
                rendering_tree.call_mouse_event(MouseEventType::Move, xy);
                state.update(event.as_ref());
                rendering_tree = state.render(props);
            }
            Some(NamuiEvent::Wheel(xy)) => {
                rendering_tree.call_wheel_event(xy);
                state.update(event.as_ref());
                rendering_tree = state.render(props);
            }
            _ => {
                state.update(event.as_ref());
                rendering_tree = state.render(props);
            }
        }
    }
}

fn set_mouse_cursor(rendering_tree: &RenderingTree) {
    let mouse_manager = &managers().mouse_manager;
    let mouse_xy = mouse_manager.mouse_position();

    let cursor = rendering_tree
        .get_mouse_cursor(&Xy {
            x: mouse_xy.x as f32,
            y: mouse_xy.y as f32,
        })
        .unwrap_or(MouseCursor::Default);

    mouse_manager.set_mouse_cursor(cursor);
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
    event::send(Box::new(NamuiEvent::AnimationFrame));

    Namui::request_animation_frame(Box::new(move || {
        on_frame();
    }));
}

fn update_fps_info(fps_info: &mut FpsInfo) {
    let now = Namui::now();
    let duration = now - fps_info.last_60_frame_time;

    if duration > Duration::from_secs(1) {
        fps_info.last_60_frame_time = Namui::now();
        fps_info.fps = (fps_info.frame_count as f32 / duration.as_secs_f32()) as u16;
        fps_info.frame_count = 0;

        Namui::log(format!("FPS: {}", fps_info.fps));
    } else {
        fps_info.frame_count += 1;
    }
}

pub fn state() -> Arc<NamuiState> {
    get_namui_state()
}

pub fn managers() -> std::sync::MutexGuard<'static, Managers> {
    get_managers()
}

pub fn log(format: String) {
    Namui::log(format);
}

#[macro_export]
#[macro_use]
macro_rules! log {
    ($($arg:tt)*) => {{
        $crate::log(format!($($arg)*));
    }}
}
