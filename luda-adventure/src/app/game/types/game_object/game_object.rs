
use crate::app::game::{GameState, Position, RenderingContext, Tile};
use namui::prelude::*;
use std::cmp::Ordering;

pub trait GameObject {
    fn get_id(&self) -> Uuid;
    fn render(
        &self,
        game_state: &GameState,
        rendering_context: &RenderingContext,
    ) -> namui::RenderingTree;
    fn get_position(&self, current_time: Time) -> Position;
    fn get_z_index(&self) -> i32;
    fn get_visual_area(&self, current_time: Time) -> VisualArea;
}

pub trait RenderGameObjectList {
    fn render(
        &self,
        game_state: &GameState,
        rendering_context: &RenderingContext,
    ) -> namui::RenderingTree;
}

impl RenderGameObjectList for Vec<&Box<dyn GameObject>> {
    fn render(
        &self,
        game_state: &GameState,
        rendering_context: &RenderingContext,
    ) -> namui::RenderingTree {
        let mut sorted_object_list = Vec::from_iter(self);
        sorted_object_list.sort_by(|a, b| {
            let z_index_comparison = a.get_z_index().cmp(&b.get_z_index());
            if z_index_comparison == std::cmp::Ordering::Equal {
                a.get_position(rendering_context.current_time)
                    .y
                    .partial_cmp(&b.get_position(rendering_context.current_time).y)
                    .unwrap_or(Ordering::Equal)
            } else {
                z_index_comparison
            }
        });
        render(
            sorted_object_list
                .into_iter()
                .map(|object| object.render(game_state, rendering_context)),
        )
    }
}

pub type VisualArea = Rect<Tile>;
