use super::*;
use crate::app::game::*;
use namui::prelude::*;
use namui_prebuilt::*;

#[ecs_macro::component]
pub struct Renderer {
    pub z_index: i32,
    pub visual_rect: Rect<Tile>,
    pub render_type: RenderType,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub enum RenderType {
    Wall {
        positions: Vec<Xy<Tile>>,
        visual_offset_rect: Rect<Tile>,
    },
    Floor {
        positions: Vec<Xy<Tile>>,
        visual_offset_rect: Rect<Tile>,
    },
    PlayerCharacter {
        visual_offset_rect: Rect<Tile>,
    },
}

impl Renderer {
    pub fn new(z_index: i32, visual_rect: Rect<Tile>, render_type: RenderType) -> Self {
        Self {
            z_index,
            visual_rect,
            render_type,
        }
    }
    pub fn render(
        &self,
        entity: &crate::ecs::Entity,
        game_state: &GameState,
        rendering_context: &RenderingContext,
    ) -> RenderingTree {
        self.render_type
            .render(entity, game_state, rendering_context)
    }
}

impl RenderType {
    fn render(
        &self,
        entity: &crate::ecs::Entity,
        _game_state: &GameState,
        rendering_context: &RenderingContext,
    ) -> RenderingTree {
        match self {
            RenderType::Wall {
                positions,
                visual_offset_rect,
            } => render(
                positions
                    .iter()
                    .filter(|position| {
                        rendering_context
                            .screen_rect
                            .intersect(Rect::from_xy_wh(
                                *position + Xy::new(visual_offset_rect.x(), visual_offset_rect.y()),
                                Wh::new(visual_offset_rect.width(), visual_offset_rect.height()),
                            ))
                            .is_some()
                    })
                    .map(|position| {
                        translate(
                            rendering_context.px_per_tile * (position.x + visual_offset_rect.x()),
                            rendering_context.px_per_tile * (position.y + visual_offset_rect.y()),
                            simple_rect(
                                Wh {
                                    width: rendering_context.px_per_tile
                                        * visual_offset_rect.width(),
                                    height: rendering_context.px_per_tile
                                        * visual_offset_rect.height(),
                                },
                                Color::TRANSPARENT,
                                0.px(),
                                Color::from_f01(0.9, 0.3, 0.3, 1.0),
                            ),
                        )
                    }),
            ),
            RenderType::Floor {
                positions,
                visual_offset_rect,
            } => {
                let positioner = entity.get_component::<&Positioner>().unwrap();
                let main_position = positioner.xy;

                render(
                    positions
                        .iter()
                        .map(|position| position + main_position)
                        .filter(|position| {
                            rendering_context
                                .screen_rect
                                .intersect(Rect::from_xy_wh(
                                    *position
                                        + Xy::new(visual_offset_rect.x(), visual_offset_rect.y()),
                                    Wh::new(
                                        visual_offset_rect.width(),
                                        visual_offset_rect.height(),
                                    ),
                                ))
                                .is_some()
                        })
                        .map(|position| {
                            translate(
                                rendering_context.px_per_tile
                                    * (position.x + visual_offset_rect.x()),
                                rendering_context.px_per_tile
                                    * (position.y + visual_offset_rect.y()),
                                simple_rect(
                                    Wh {
                                        width: rendering_context.px_per_tile
                                            * visual_offset_rect.width(),
                                        height: rendering_context.px_per_tile
                                            * visual_offset_rect.height(),
                                    },
                                    Color::TRANSPARENT,
                                    0.px(),
                                    Color::from_f01(0.3, 0.9, 0.3, 1.0),
                                ),
                            )
                        }),
                )
            }
            RenderType::PlayerCharacter { visual_offset_rect } => {
                let positioner = entity.get_component::<&Positioner>().unwrap();
                let character = entity.get_component::<&PlayerCharacter>().unwrap();
                let position =
                    positioner.xy_with_interpolation(rendering_context.interpolation_progress);
                translate(
                    (rendering_context.px_per_tile * (position.x + visual_offset_rect.x())).floor(),
                    (rendering_context.px_per_tile * (position.y + visual_offset_rect.y())).floor(),
                    render([
                        simple_rect(
                            Wh {
                                width: rendering_context.px_per_tile * visual_offset_rect.width(),
                                height: rendering_context.px_per_tile * visual_offset_rect.height(),
                            },
                            Color::TRANSPARENT,
                            0.px(),
                            Color::from_f01(0.5, 0.5, 1.0, 0.5),
                        ),
                        namui_prebuilt::typography::center_text_full_height(
                            Wh {
                                width: rendering_context.px_per_tile * visual_offset_rect.width(),
                                height: rendering_context.px_per_tile * visual_offset_rect.height(),
                            },
                            match character.heading {
                                Heading::Left => "L",
                                Heading::Right => "R",
                            },
                            Color::WHITE,
                        ),
                    ]),
                )
            }
        }
    }
}
