pub(crate) mod draw;
mod engine_common;
use std::time::Duration;

pub use engine_common::*;

#[cfg(target_family = "wasm")]
mod engine_web;

use self::draw::{RenderingData, RenderingTree};
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

    let rendering_tree = (engine_context.render)(&mut engine_context.state);
    match rendering_tree {
        Some(rendering_tree) => rendering_tree.draw(),
        None => (),
    }

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

#[macro_export]
macro_rules! render_func(
    ($_func_name:ident, $_state_type:ty, $_state_identity:ident, $body:expr) => (
        paste::item! {
            fn [<render_ $ _func_name>] ($_state_identity: &mut $_state_type) -> Option<RenderingTree> { $body }
        }
    )
);

pub trait ToTree {
    fn process(self) -> Option<RenderingTree>;
}

impl ToTree for Option<RenderingTree> {
    fn process(self) -> Option<RenderingTree> {
        self
    }
}

impl ToTree for RenderingData {
    fn process(self) -> Option<RenderingTree> {
        Some(RenderingTree::Node(self))
    }
}

#[macro_export]
macro_rules! render {
    ( $( $x:expr ),* ) => {
        {

            let mut temp_vec: Vec<Option<RenderingTree>> = Vec::new();
            $(
                let option_rendering_tree = ToTree::process($x);
                temp_vec.push(option_rendering_tree);
            )*
            Some(RenderingTree::Children(temp_vec))
        }
    };
}
