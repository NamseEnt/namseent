use crate::app::game::{Game, RenderingContext};
use namui::*;

impl Game {
    pub fn translate_to_camera_screen(
        &self,
        rendering_context: &RenderingContext,
        rendering_tree: RenderingTree,
    ) -> namui::RenderingTree {
        translate(
            -(rendering_context.px_per_tile * rendering_context.screen_rect.x()).floor(),
            -(rendering_context.px_per_tile * rendering_context.screen_rect.y()).floor(),
            rendering_tree,
        )
    }
}
