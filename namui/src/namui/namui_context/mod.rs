use super::{on_frame, skia::Surface};
use crate::{event::EventReceiver, Entity, NamuiImpl, RenderingTree, Typeface, Wh, Xy};
use std::{sync::Arc, time::Duration};
mod main_loop;

pub struct NamuiContext {
    pub(crate) surface: Surface,
    fps_info: FpsInfo,
    pub(crate) rendering_tree: RenderingTree,
    pub(crate) event_receiver: EventReceiver,
    pub(crate) fallback_font_typefaces: Vec<Arc<Typeface>>,
    event_count: u32,
    canvas_should_be_resized_to: Option<Wh<i16>>,
}

struct FpsInfo {
    pub fps: u16,
    pub frame_count: u16,
    pub last_60_frame_time: Duration,
}

impl NamuiContext {
    pub(crate) fn new(surface: Surface) -> Self {
        Self {
            surface,
            fps_info: FpsInfo {
                fps: 0,
                frame_count: 0,
                last_60_frame_time: crate::now(),
            },
            rendering_tree: RenderingTree::Empty,
            event_receiver: crate::event::init(),
            fallback_font_typefaces: Vec::new(),
            event_count: 0,
            canvas_should_be_resized_to: None,
        }
    }
    pub async fn start<TProps>(mut self, state: &mut dyn Entity<Props = TProps>, props: &TProps) {
        self.rendering_tree = state.render(props);

        crate::Namui::request_animation_frame(Box::new(move || {
            on_frame();
        }));
        self.run_main_loop(state, props).await;
    }
    pub fn get_rendering_tree_xy_by_id(&self, id: &str) -> Option<Xy<f32>> {
        self.rendering_tree.get_xy_by_id(id)
    }
    pub fn get_rendering_tree_xy(&self, rendering_tree: &RenderingTree) -> Option<Xy<f32>> {
        self.rendering_tree.get_xy_of_child(rendering_tree)
    }
}
