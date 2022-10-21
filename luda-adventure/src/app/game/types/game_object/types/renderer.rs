use crate::app::game::*;
use namui::prelude::*;

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

pub trait RenderGameObjectList {
    fn render(
        &self,
        game_state: &GameState,
        rendering_context: &RenderingContext,
    ) -> namui::RenderingTree;
}
