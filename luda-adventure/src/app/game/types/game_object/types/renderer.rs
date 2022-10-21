use crate::app::game::*;
use namui::prelude::*;
use std::cmp::Ordering;

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

impl RenderGameObjectList for Vec<(&crate::ecs::Entity, (&Renderer, &Positioner))> {
    fn render(
        &self,
        game_state: &GameState,
        rendering_context: &RenderingContext,
    ) -> namui::RenderingTree {
        let mut sorted_object_list = Vec::from_iter(self);
        sorted_object_list.sort_by(
            |(_, (a_renderer, a_positioner)), (_, (b_renderer, b_positioner))| {
                let z_index_comparison = a_renderer.z_index.cmp(&b_renderer.z_index);
                if z_index_comparison == std::cmp::Ordering::Equal {
                    a_positioner
                        .xy()
                        .y
                        .partial_cmp(&b_positioner.xy().y)
                        .unwrap_or(Ordering::Equal)
                } else {
                    z_index_comparison
                }
            },
        );
        render(
            sorted_object_list
                .into_iter()
                .map(|(entity, (renderer, _))| {
                    renderer.render(entity, game_state, rendering_context)
                }),
        )
    }
}
