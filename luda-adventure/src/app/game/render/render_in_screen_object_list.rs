use crate::app::game::*;
use namui::prelude::*;

impl Game {
    pub fn render_in_screen_object_list(
        &self,
        rendering_context: &RenderingContext,
    ) -> RenderingTree {
        self.camera
            .get_in_screen_object_list(&self.ecs_app, &rendering_context)
            .render(&self.state, &rendering_context)
    }
}
