use crate::app::game::*;
use namui::prelude::*;
use std::fmt::Debug;

#[derive(ecs_macro::Component)]
pub struct Renderer {
    pub z_index: i32,
    pub visual_rect: Rect<Tile>,
    render: Box<dyn Fn(&GameState, &RenderingContext, Xy<Tile>) -> RenderingTree + 'static>,
}

impl Renderer {
    pub fn new(
        z_index: i32,
        visual_rect: Rect<Tile>,
        render: impl Fn(&GameState, &RenderingContext, Xy<Tile>) -> RenderingTree + 'static,
    ) -> Self {
        Self {
            z_index,
            visual_rect,
            render: Box::new(render),
        }
    }
    pub fn render(
        &self,
        game_state: &GameState,
        rendering_context: &RenderingContext,
        xy: Xy<Tile>,
    ) -> RenderingTree {
        (self.render)(game_state, rendering_context, xy)
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
