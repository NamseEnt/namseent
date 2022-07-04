use crate::{event::EventReceiver, *};
use std::time::Duration;
mod main_loop;

pub struct NamuiContext {
    fps_info: FpsInfo,
    pub(crate) rendering_tree: RenderingTree,
    pub(crate) event_receiver: EventReceiver,
    event_count: u32,
    is_surface_resize_requested: Option<Wh<i16>>,
}

struct FpsInfo {
    pub fps: u16,
    pub frame_count: u16,
    pub last_60_frame_time: Duration,
}

impl NamuiContext {
    pub(crate) fn new(event_receiver: EventReceiver) -> Self {
        Self {
            fps_info: FpsInfo {
                fps: 0,
                frame_count: 0,
                last_60_frame_time: crate::now(),
            },
            rendering_tree: RenderingTree::Empty,
            event_receiver,
            event_count: 0,
            is_surface_resize_requested: None,
        }
    }
    pub async fn start<TProps>(mut self, state: &mut dyn Entity<Props = TProps>, props: &TProps) {
        self.rendering_tree = state.render(props);

        fn on_frame() {
            crate::event::send(crate::NamuiEvent::AnimationFrame);

            crate::graphics::request_animation_frame(on_frame);
        }

        crate::graphics::request_animation_frame(on_frame);
        self.run_main_loop(state, props).await;
    }
    pub fn get_rendering_tree_xy_by_id(&self, id: &str) -> Option<Xy<Px>> {
        self.rendering_tree.get_xy_by_id(id)
    }
    pub fn get_rendering_tree_xy(&self, rendering_tree: &RenderingTree) -> Option<Xy<Px>> {
        self.rendering_tree.get_xy_of_child(rendering_tree)
    }
}
