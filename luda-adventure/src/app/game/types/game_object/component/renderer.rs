use crate::app::game::*;
use namui::prelude::*;
use std::fmt::Debug;

#[derive(ecs_macro::Component)]
pub struct Renderer {
    pub z_index: i32,
    pub visual_rect: Rect<Tile>,
    render:
        Box<dyn Fn(&crate::ecs::Entity, &GameState, &RenderingContext) -> RenderingTree + 'static>,
}

impl Renderer {
    pub fn new(
        z_index: i32,
        visual_rect: Rect<Tile>,
        render: impl Fn(&crate::ecs::Entity, &GameState, &RenderingContext) -> RenderingTree + 'static,
    ) -> Self {
        Self {
            z_index,
            visual_rect,
            render: Box::new(render),
        }
    }
    pub fn render(
        &self,
        entity: &crate::ecs::Entity,
        game_state: &GameState,
        rendering_context: &RenderingContext,
    ) -> RenderingTree {
        (self.render)(entity, game_state, rendering_context)
    }
}

impl Debug for Renderer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Renderer")
            .field("z_index", &self.z_index)
            .field("visual_rect", &self.visual_rect)
            .finish()
    }
}
