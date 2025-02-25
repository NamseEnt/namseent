use crate::{
    card::{Rank, Suit},
    game_state::{
        cursor_preview::PreviewKind,
        mutate_game_state,
        tower::{AnimationKind, TowerKind, TowerTemplate, tower_image_resource_location},
        use_game_state,
    },
    palette,
};
use namui::*;
use namui_prebuilt::{table, typography};
use std::iter::once;

const HAND_HEIGHT: Px = px(160.);
const CARD_WIDTH: Px = px(120.);
const PADDING: Px = px(4.);

#[derive(Clone, Default)]
pub enum PlacingTowerSlot {
    #[default]
    Empty,
    Tower {
        tower_template: TowerTemplate,
    },
}
impl PlacingTowerSlot {
    pub fn barricade() -> Self {
        Self::Tower {
            tower_template: TowerTemplate::new(TowerKind::Barricade, Suit::Spades, Rank::Ace),
        }
    }
}

pub struct TowerPlacingHand<'a> {
    pub screen_wh: Wh<Px>,
    pub placing_tower_slots: &'a [PlacingTowerSlot; 5],
}
impl Component for TowerPlacingHand<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            screen_wh,
            placing_tower_slots,
        } = self;

        let game_state = use_game_state(ctx);

        let selected_placing_tower_slot_index = match game_state.cursor_preview.kind {
            PreviewKind::PlacingTower {
                placing_tower_slot_index,
                ..
            } => Some(placing_tower_slot_index),
            _ => None,
        };

        let select_tower = |placing_tower_slot_index: usize| {
            if selected_placing_tower_slot_index.is_some() {
                return;
            };

            let PlacingTowerSlot::Tower { tower_template } =
                &placing_tower_slots[placing_tower_slot_index]
            else {
                return;
            };

            let tower_template = tower_template.clone();
            mutate_game_state(move |game_state| {
                game_state.cursor_preview.kind = PreviewKind::PlacingTower {
                    tower_template,
                    placing_tower_slot_index,
                };
            });
        };

        ctx.compose(|ctx| {
            table::vertical([
                table::ratio_no_clip(1, |_, _| {}),
                table::fixed_no_clip(
                    HAND_HEIGHT,
                    table::horizontal(
                        once(table::ratio_no_clip(1, |_, _| {}))
                            .chain(placing_tower_slots.iter().enumerate().map(
                                |(index, placing_tower_slot)| {
                                    table::fixed(
                                        CARD_WIDTH,
                                        table::padding(PADDING, move |wh, ctx| {
                                            let selected = selected_placing_tower_slot_index
                                                .is_some_and(|i| i == index);
                                            ctx.add(RenderPlacingTowerSlot {
                                                placing_tower_slot,
                                                wh,
                                                selected,
                                                on_click: &|| {
                                                    select_tower(index);
                                                },
                                            });
                                        }),
                                    )
                                },
                            ))
                            .chain(once(table::ratio(1, |_, _| {}))),
                    ),
                ),
            ])(screen_wh, ctx);
        });
    }
}

struct RenderPlacingTowerSlot<'a> {
    wh: Wh<Px>,
    placing_tower_slot: &'a PlacingTowerSlot,
    selected: bool,
    on_click: &'a dyn Fn(),
}
impl Component for RenderPlacingTowerSlot<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            wh,
            placing_tower_slot,
            selected,
            on_click,
        } = self;

        if let PlacingTowerSlot::Tower { tower_template } = placing_tower_slot {
            ctx.compose(|ctx| {
                table::padding(PADDING * 3.0, |wh, ctx| {
                    ctx.add(RenderPlacingTowerSlotContent { wh, tower_template });
                })(wh, ctx);
            });
        }

        ctx.add(rect(RectParam {
            rect: Rect::from_xy_wh(Xy::single(PADDING * 2.), wh - Wh::single(PADDING * 4.0)),
            style: RectStyle {
                stroke: None,
                fill: Some(RectFill {
                    color: palette::SURFACE_CONTAINER,
                }),
                round: Some(RectRound {
                    radius: palette::ROUND,
                }),
            },
        }));

        ctx.add(
            rect(RectParam {
                rect: wh.to_rect(),
                style: RectStyle {
                    stroke: Some(RectStroke {
                        color: palette::OUTLINE,
                        width: 1.px(),
                        border_position: BorderPosition::Inside,
                    }),
                    fill: Some(RectFill {
                        color: match selected {
                            true => palette::PRIMARY,
                            false => palette::SURFACE_CONTAINER_HIGH,
                        },
                    }),
                    round: Some(RectRound {
                        radius: palette::ROUND,
                    }),
                },
            })
            .attach_event(|event| {
                let Event::MouseDown { event } = event else {
                    return;
                };
                if !event.is_local_xy_in() {
                    return;
                }
                on_click();
            }),
        );
    }
}

struct RenderPlacingTowerSlotContent<'a> {
    wh: Wh<Px>,
    tower_template: &'a TowerTemplate,
}
impl Component for RenderPlacingTowerSlotContent<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self { tower_template, wh } = self;

        let tower_image = ctx.image(tower_image_resource_location(
            tower_template.kind,
            AnimationKind::Idle1,
        ));

        ctx.add(typography::body::left_top(
            format!("{}", tower_template.kind),
            palette::ON_SURFACE,
        ));

        ctx.compose(|ctx| {
            let Some(Ok(tower_image)) = tower_image.as_ref() else {
                return;
            };

            ctx.add(image(ImageParam {
                rect: wh.to_rect(),
                image: tower_image.clone(),
                style: ImageStyle {
                    fit: ImageFit::Contain,
                    paint: None,
                },
            }));
        });
    }
}
