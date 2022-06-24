mod common;
pub(crate) mod draw;
mod font;
mod manager;
use std::{any::Any, sync::Arc, time::Duration};
mod namui_state;
mod skia;
pub use common::*;
pub use draw::{DrawCall, DrawCommand, PathDrawCommand, TextAlign, TextBaseline, TextDrawCommand};
use futures::future::join;
pub use render::{
    absolute, clip, image::*, path::*, rect::*, rotate, scale, text::*, text_input, translate,
    types::*, ImageSource, MouseCursor, MouseEvent, MouseEventCallback, MouseEventType,
    RenderingData, RenderingTree, TextInput, WheelEventCallback,
};
pub use skia::{
    types::{ClipOp, Color, PaintStyle, StrokeJoin},
    BlendMode, Font, Image, LtrbRect, PaintBuilder, PathBuilder, StrokeCap, Typeface,
};
pub(crate) use skia::{ColorFilter, Paint, Path};
pub mod event;
pub use event::NamuiEvent;
mod render;
pub use self::manager::{managers, Code};
use self::render::WheelEvent;
use self::{
    font::*,
    namui_state::{get_namui_state, NamuiState},
};
mod random;
pub use self::random::*;
pub mod screen;
pub use namui_cfg::*;
pub mod fs;
pub mod math;
pub use url::Url;

#[cfg(not(test))]
#[cfg(target_family = "wasm")]
mod namui_web;
#[cfg(not(test))]
#[cfg(target_family = "wasm")]
pub use self::namui_web::*;

#[cfg(test)]
mod namui_mock;
#[cfg(test)]
pub use self::namui_mock::*;

pub trait Entity {
    type Props;
    fn update(&mut self, event: &dyn Any);
    fn render(&self, props: &Self::Props) -> RenderingTree;
}

pub async fn init() -> NamuiContext {
    let mut namui_context = Namui::init();

    join(init_font(&mut namui_context), init_filesystem()).await;

    namui_context
}

pub async fn start<TProps>(
    mut namui_context: NamuiContext,
    state: &mut dyn Entity<Props = TProps>,
    props: &TProps,
) {
    namui_context.rendering_tree = state.render(props);

    Namui::request_animation_frame(Box::new(move || {
        on_frame();
    }));

    let mut event_count = 0;

    loop {
        let event = namui_context.event_receiver.recv().await.unwrap();
        event_count += 1;

        match event.downcast_ref::<NamuiEvent>() {
            Some(NamuiEvent::AnimationFrame) => {
                invoke_and_flush_all_animation_frame_callbacks();
                state.update(event.as_ref());
                namui_context.rendering_tree = state.render(props);

                update_fps_info(&mut namui_context.fps_info);

                namui_context.rendering_tree.draw(&namui_context);

                set_mouse_cursor(&namui_context.rendering_tree);

                namui_context.surface.flush();

                if namui_context.fps_info.frame_count == 0 {
                    log(format!("event_count: {}", event_count));
                    event_count = 0;
                }
            }
            Some(NamuiEvent::MouseDown(raw_mouse_event)) => {
                {
                    let managers = managers();
                    managers
                        .text_input_manager
                        .on_mouse_down(&namui_context, &raw_mouse_event);
                }
                namui_context.rendering_tree.call_mouse_event(
                    MouseEventType::Down,
                    raw_mouse_event,
                    &namui_context,
                );
                state.update(event.as_ref());
                namui_context.rendering_tree = state.render(props);
            }
            Some(NamuiEvent::MouseUp(raw_mouse_event)) => {
                {
                    let managers = managers();
                    managers
                        .text_input_manager
                        .on_mouse_up(&namui_context, &raw_mouse_event);
                }
                namui_context.rendering_tree.call_mouse_event(
                    MouseEventType::Up,
                    raw_mouse_event,
                    &namui_context,
                );
                state.update(event.as_ref());
                namui_context.rendering_tree = state.render(props);
            }
            Some(NamuiEvent::MouseMove(raw_mouse_event)) => {
                {
                    let managers = managers();
                    managers
                        .text_input_manager
                        .on_mouse_move(&namui_context, &raw_mouse_event);
                }
                namui_context.rendering_tree.call_mouse_event(
                    MouseEventType::Move,
                    raw_mouse_event,
                    &namui_context,
                );
                state.update(event.as_ref());
                namui_context.rendering_tree = state.render(props);
            }
            Some(NamuiEvent::Wheel(xy)) => {
                namui_context.rendering_tree.call_wheel_event(
                    format!("wheel-{:?}-{}", now(), nanoid()),
                    xy,
                    &namui_context,
                );
                state.update(event.as_ref());
                namui_context.rendering_tree = state.render(props);
            }
            _ => {
                state.update(event.as_ref());
                namui_context.rendering_tree = state.render(props);
            }
        }

        let now = crate::now();
        while let Some(timeout) = pull_timeout(now) {
            timeout();
        }
    }
}

fn set_mouse_cursor(rendering_tree: &RenderingTree) {
    let managers = managers();
    let mouse_manager = &managers.mouse_manager;
    let mouse_xy = mouse_manager.mouse_position();

    let cursor = rendering_tree
        .get_mouse_cursor(Xy {
            x: mouse_xy.x as f32,
            y: mouse_xy.y as f32,
        })
        .unwrap_or(MouseCursor::Default);

    mouse_manager.set_mouse_cursor(cursor);
}

async fn init_font(namui_context: &mut NamuiContext) {
    match load_all_fonts(namui_context, &managers().font_manager.typeface_manager).await {
        Ok(()) => {
            log("Font loaded".to_string());
        }
        Err(e) => {
            log(format!("Font loading failed: {}", e));
        }
    };
}

async fn init_filesystem() {
    match fs::init().await {
        Ok(()) => {
            log("Filesystem initialized".to_string());
        }
        Err(e) => {
            log(format!("Filesystem initialize failed: {:?}", e));
        }
    };
}

fn on_frame() {
    event::send(NamuiEvent::AnimationFrame);

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

pub fn log(format: String) {
    Namui::log(format);
}

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {{
        $crate::log(format!($($arg)*));
    }}
}

/// `now()` is not ISO 8601. It's time since the program started.
pub fn now() -> Duration {
    Namui::now()
}
