use crate::app::game::*;
use namui::prelude::*;

impl Game {
    pub fn render_guide_icon(&self, rendering_context: &RenderingContext) -> RenderingTree {
        const OFFSET_Y: Px = px(-4.0);
        render(
            self.state
                .quest
                .get_quest_object_list(&self.ecs_app)
                .iter()
                .map(|entity| {
                    let (renderer, positioner) =
                        entity.get_component::<(&Renderer, &Positioner)>().unwrap();
                    let visual_area = renderer.visual_rect + positioner.xy();
                    render([
                        translate(
                            rendering_context.px_per_tile
                                * (visual_area.left() + visual_area.right())
                                * 0.5
                                - 4.px(),
                            rendering_context.px_per_tile * visual_area.y() + OFFSET_Y - 32.px(),
                            namui_prebuilt::simple_rect(
                                Wh {
                                    width: 8.px(),
                                    height: 16.px(),
                                },
                                Color::BLACK,
                                2.px(),
                                Color::WHITE,
                            ),
                        ),
                        translate(
                            rendering_context.px_per_tile
                                * (visual_area.left() + visual_area.right())
                                * 0.5
                                - 4.px(),
                            rendering_context.px_per_tile * visual_area.y() + OFFSET_Y - 16.px(),
                            namui_prebuilt::simple_rect(
                                Wh {
                                    width: 8.px(),
                                    height: 8.px(),
                                },
                                Color::BLACK,
                                2.px(),
                                Color::WHITE,
                            ),
                        ),
                    ])
                }),
        )
    }
}
