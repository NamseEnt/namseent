use crate::{app::game::*, ecs::Entity};
use namui::prelude::*;
use std::cmp::Ordering;

impl Game {
    pub fn render_in_screen_object_list(
        &self,
        rendering_context: &RenderingContext,
    ) -> RenderingTree {
        let mut in_screen_object_list = self.get_in_screen_object_list(rendering_context);
        sort_in_screen_object_list_with_z_index_then_sort_with_y_coordinate(
            &mut in_screen_object_list,
        );

        render(
            in_screen_object_list
                .into_iter()
                .map(|(entity, (renderer, _))| {
                    renderer.render(entity, &self.state, rendering_context)
                }),
        )
    }

    fn get_in_screen_object_list(
        &self,
        rendering_context: &RenderingContext,
    ) -> Vec<(&crate::ecs::Entity, (&Renderer, &Positioner))> {
        self.ecs_app
            .query_entities::<(&Renderer, &Positioner)>()
            .into_iter()
            .filter(|(_, (renderer, positioner))| {
                let visual_area = renderer.visual_rect + positioner.xy();
                visual_area
                    .intersect(rendering_context.screen_rect)
                    .is_some()
            })
            .collect::<Vec<_>>()
    }
}

fn sort_in_screen_object_list_with_z_index_then_sort_with_y_coordinate(
    in_screen_object_list: &mut Vec<(&Entity, (&Renderer, &Positioner))>,
) {
    in_screen_object_list.sort_by(
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
}
