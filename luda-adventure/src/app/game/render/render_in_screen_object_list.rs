use crate::app::game::*;
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
                .map(|(renderer, positioner)| {
                    renderer.render(
                        &self.state,
                        rendering_context,
                        positioner.xy_with_interpolation(rendering_context.interpolation_progress),
                    )
                }),
        )
    }

    fn get_in_screen_object_list(
        &self,
        rendering_context: &RenderingContext,
    ) -> Vec<(&Renderer, &Positioner)> {
        self.ecs_app
            .query_component::<(Renderer, Positioner)>()
            .into_iter()
            .filter(|(renderer, positioner)| {
                let visual_area = renderer.visual_rect + positioner.xy;
                visual_area
                    .intersect(rendering_context.screen_rect)
                    .is_some()
            })
            .collect::<Vec<_>>()
    }
}

fn sort_in_screen_object_list_with_z_index_then_sort_with_y_coordinate(
    in_screen_object_list: &mut Vec<(&Renderer, &Positioner)>,
) {
    in_screen_object_list.sort_by(|(a_renderer, a_positioner), (b_renderer, b_positioner)| {
        let z_index_comparison = a_renderer.z_index.cmp(&b_renderer.z_index);
        if z_index_comparison == std::cmp::Ordering::Equal {
            a_positioner
                .xy
                .y
                .partial_cmp(&b_positioner.xy.y)
                .unwrap_or(Ordering::Equal)
        } else {
            z_index_comparison
        }
    });
}
