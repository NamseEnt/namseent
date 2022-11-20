use crate::{app::game::*, component::*};
use namui::prelude::*;

#[ecs_macro::component]
pub struct Renderer {
    pub z_index: i32,
    pub render_type: RenderType,
}

impl Renderer {
    pub fn new(z_index: i32, render_type: RenderType) -> Self {
        Self {
            z_index,
            render_type,
        }
    }
    pub fn visual_rect(&self) -> Rect<Tile> {
        self.render_type.visual_rect()
    }
    pub fn render(
        &self,
        entity: &crate::ecs::Entity,
        game_state: &GameState,
        rendering_context: &RenderingContext,
    ) -> RenderingTree {
        let xy = entity
            .get_component::<&Positioner>()
            .map(|positioner| {
                Xy::single(rendering_context.px_per_tile)
                    * positioner.xy_with_interpolation(rendering_context.interpolation_progress)
            })
            .unwrap_or(Xy::zero());

        translate(
            xy.x,
            xy.y,
            self.render_type.render(rendering_context, game_state),
        )
    }
}
