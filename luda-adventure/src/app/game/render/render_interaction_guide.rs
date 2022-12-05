use crate::app::game::{Game, GameState, RenderingContext, MAX_INTERACTION_DISTANCE};
use namui::prelude::*;

const ICON_SIZE: Px = px(36.0);
const OFFSET_Y: Px = px(-4.0);

impl Game {
    pub fn render_interaction_guide(
        &self,
        game_state: &GameState,
        rendering_context: &RenderingContext,
    ) -> RenderingTree {
        let interactive_object_list = self.get_interactive_object_with_distance(game_state);

        render(interactive_object_list.into_iter().map(
            |((_entity, (positioner, renderer)), distance)| {
                let xy = positioner.xy_with_interpolation(rendering_context.interpolation_progress);
                let visual_rect = renderer.visual_rect() + xy;
                let icon_xy = Xy {
                    x: rendering_context.px_per_tile
                        * (visual_rect.left() + visual_rect.right())
                        * 0.5,
                    y: rendering_context.px_per_tile * visual_rect.y() + OFFSET_Y
                        - (ICON_SIZE * 0.5),
                };
                let scale = 1. - (distance / MAX_INTERACTION_DISTANCE).max(0.).min(1.);
                interaction_icon(icon_xy, scale)
            },
        ))
    }
}

fn interaction_icon(xy: Xy<Px>, scale: f32) -> RenderingTree {
    text(TextParam {
        text: "press Z".to_string(),
        x: xy.x,
        y: xy.y,
        align: TextAlign::Center,
        baseline: TextBaseline::Middle,
        font_type: FontType {
            serif: false,
            size: (ICON_SIZE * scale).into(),
            language: Language::Ko,
            font_weight: FontWeight::BOLD,
        },
        style: TextStyle {
            border: Some(TextStyleBorder {
                width: 4.px(),
                color: Color::BLACK,
            }),
            drop_shadow: Some(TextStyleDropShadow {
                x: 0.0.px(),
                y: 0.0.px(),
                color: Some(Color::GREEN),
            }),
            color: Color::WHITE,
            background: None,
            ..Default::default()
        },
        max_width: None,
    })
}
