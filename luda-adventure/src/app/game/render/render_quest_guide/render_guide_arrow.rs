use crate::app::game::*;
use crate::component::*;
use crate::ecs::Entity;
use namui::prelude::*;

const ARROW_SIZE: f32 = 16.0;

pub fn render_guide_arrow(
    character_visual_rect: Option<Rect<Tile>>,
    quest_entity_list: &Vec<&Entity>,
    rendering_context: &RenderingContext,
) -> RenderingTree {
    match character_visual_rect {
        Some(character_visual_rect) => {
            let character_visual_center =
                Xy::single(rendering_context.px_per_tile) * character_visual_rect.center();
            let character_visual_radius =
                character_visual_radius(character_visual_rect, rendering_context);
            translate(
                character_visual_center.x,
                character_visual_center.y,
                render(quest_entity_list.iter().filter_map(|entity| {
                    entity
                        .get_component::<&Positioner>()
                        .map(|quest_entity_positioner| {
                            let quest_entity_xy = Xy::single(rendering_context.px_per_tile)
                                * quest_entity_positioner.xy_with_interpolation(
                                    rendering_context.interpolation_progress,
                                );
                            let angle = Xy::new(1.px(), 0.px())
                                .angle_to(quest_entity_xy - character_visual_center);
                            rotate(angle, arrow(character_visual_radius))
                        })
                })),
            )
        }
        None => RenderingTree::Empty,
    }
}

fn arrow(character_visual_radius: Px) -> RenderingTree {
    let path_builder = PathBuilder::new()
        .line_to(-0.2.px(), -0.8.px())
        .line_to(1.px(), 0.px())
        .line_to(-0.2.px(), 0.8.px())
        .scale(ARROW_SIZE, ARROW_SIZE);
    let paint_builder = PaintBuilder::new()
        .set_color(Color::from_u8(255, 252, 127, 255))
        .set_style(PaintStyle::Fill);
    translate(
        character_visual_radius,
        0.px(),
        path(path_builder, paint_builder),
    )
}

fn character_visual_radius(
    character_visual_rect: Rect<Tile>,
    rendering_context: &RenderingContext,
) -> Px {
    let width_px = (rendering_context.px_per_tile * character_visual_rect.width()).as_f32();
    let height_px = (rendering_context.px_per_tile * character_visual_rect.height()).as_f32();
    let radius_px = (width_px * width_px + height_px * height_px).sqrt() / 2.;
    radius_px.into()
}
