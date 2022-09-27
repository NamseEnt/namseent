use super::{Game, RenderingContext};
use namui::prelude::*;

impl Game<'_> {
    pub fn render_guide_icon(&self, rendering_context: &RenderingContext) -> RenderingTree {
        const OFFSET_Y: Px = px(-4.0);
        render(
            self.state
                .quest
                .get_quest_object_list(&self.object_list)
                .iter()
                .map(|object| {
                    let visual_area = object.get_visual_area(rendering_context.current_time);
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
