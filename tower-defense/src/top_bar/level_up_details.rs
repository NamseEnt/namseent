use crate::{
    game_state::level_rarity_weight,
    icon::{Icon, IconKind, IconSize},
    palette,
    rarity::Rarity,
    theme::typography::paragraph,
};
use namui::*;
use namui_prebuilt::{simple_rect, table};
use std::num::NonZero;

const LINE_HEIGHT: Px = px(32.);
const CONTAINER_HEIGHT: Px = px(128.);
const RARITY_LABEL_WIDTH: Px = px(64.);
const PADDING: Px = px(8.);

pub struct LevelUpDetails {
    pub width: Px,
    pub current_level: usize,
}
impl Component for LevelUpDetails {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            width,
            current_level,
        } = self;
        let current_level = ctx.track_eq(&current_level);
        let weights = ctx.memo(|| {
            let current_level = NonZero::new(*current_level).expect("Level must be non-zero");
            let next_level = current_level
                .checked_add(1)
                .unwrap()
                .min(NonZero::new(10).unwrap());
            let mut current_weights = level_rarity_weight(current_level);
            let current_total_weight: usize = current_weights.iter().sum();
            current_weights.iter_mut().for_each(|weight| {
                *weight = (*weight as f32 / current_total_weight as f32 * 100.0).round() as usize;
            });
            let mut next_weights = level_rarity_weight(next_level);
            let next_total_weight: usize = next_weights.iter().sum();
            next_weights.iter_mut().for_each(|weight| {
                *weight = (*weight as f32 / next_total_weight as f32 * 100.0).round() as usize;
            });
            [
                [current_weights[0], next_weights[0]],
                [current_weights[1], next_weights[1]],
                [current_weights[2], next_weights[2]],
                [current_weights[3], next_weights[3]],
            ]
        });
        let wh = Wh::new(width, CONTAINER_HEIGHT);

        ctx.compose(|ctx| {
            table::vertical([
                table::fixed(
                    LINE_HEIGHT,
                    table::horizontal([
                        table::fixed(PADDING, |_, _| {}),
                        table::fixed(RARITY_LABEL_WIDTH, |wh, ctx| {
                            ctx.add(
                                Icon::new(IconKind::Rarity {
                                    rarity: Rarity::Common,
                                })
                                .size(IconSize::Medium)
                                .wh(wh),
                            );
                        }),
                        table::ratio(1, |_, _| {}),
                        table::ratio(1, |wh, ctx| {
                            ctx.add(
                                paragraph(format!("{}%", weights[0][0]))
                                    .size(crate::theme::typography::FontSize::Medium)
                                    .align(crate::theme::typography::TextAlign::Center { wh })
                                    .build(),
                            );
                        }),
                        table::ratio(1, |wh, ctx| {
                            ctx.add(
                                paragraph(">>>".to_string())
                                    .size(crate::theme::typography::FontSize::Medium)
                                    .align(crate::theme::typography::TextAlign::Center { wh })
                                    .build(),
                            );
                        }),
                        table::ratio(1, |wh, ctx| {
                            ctx.add(
                                paragraph(format!("{}%", weights[0][1]))
                                    .size(crate::theme::typography::FontSize::Medium)
                                    .align(crate::theme::typography::TextAlign::Center { wh })
                                    .build(),
                            );
                        }),
                    ]),
                ),
                table::fixed(
                    LINE_HEIGHT,
                    table::horizontal([
                        table::fixed(PADDING, |_, _| {}),
                        table::fixed(RARITY_LABEL_WIDTH, |wh, ctx| {
                            ctx.add(
                                Icon::new(IconKind::Rarity {
                                    rarity: Rarity::Rare,
                                })
                                .size(IconSize::Medium)
                                .wh(wh),
                            );
                        }),
                        table::ratio(1, |_, _| {}),
                        table::ratio(1, |wh, ctx| {
                            ctx.add(
                                paragraph(format!("{}%", weights[1][0]))
                                    .size(crate::theme::typography::FontSize::Medium)
                                    .align(crate::theme::typography::TextAlign::Center { wh })
                                    .build(),
                            );
                        }),
                        table::ratio(1, |wh, ctx| {
                            ctx.add(
                                paragraph(">>>".to_string())
                                    .size(crate::theme::typography::FontSize::Medium)
                                    .align(crate::theme::typography::TextAlign::Center { wh })
                                    .build(),
                            );
                        }),
                        table::ratio(1, |wh, ctx| {
                            ctx.add(
                                paragraph(format!("{}%", weights[1][1]))
                                    .size(crate::theme::typography::FontSize::Medium)
                                    .align(crate::theme::typography::TextAlign::Center { wh })
                                    .build(),
                            );
                        }),
                    ]),
                ),
                table::fixed(
                    LINE_HEIGHT,
                    table::horizontal([
                        table::fixed(PADDING, |_, _| {}),
                        table::fixed(RARITY_LABEL_WIDTH, |wh, ctx| {
                            ctx.add(
                                Icon::new(IconKind::Rarity {
                                    rarity: Rarity::Epic,
                                })
                                .size(IconSize::Medium)
                                .wh(wh),
                            );
                        }),
                        table::ratio(1, |_, _| {}),
                        table::ratio(1, |wh, ctx| {
                            ctx.add(
                                paragraph(format!("{}%", weights[2][0]))
                                    .size(crate::theme::typography::FontSize::Medium)
                                    .align(crate::theme::typography::TextAlign::Center { wh })
                                    .build(),
                            );
                        }),
                        table::ratio(1, |wh, ctx| {
                            ctx.add(
                                paragraph(">>>".to_string())
                                    .size(crate::theme::typography::FontSize::Medium)
                                    .align(crate::theme::typography::TextAlign::Center { wh })
                                    .build(),
                            );
                        }),
                        table::ratio(1, |wh, ctx| {
                            ctx.add(
                                paragraph(format!("{}%", weights[2][1]))
                                    .size(crate::theme::typography::FontSize::Medium)
                                    .align(crate::theme::typography::TextAlign::Center { wh })
                                    .build(),
                            );
                        }),
                    ]),
                ),
                table::fixed(
                    LINE_HEIGHT,
                    table::horizontal([
                        table::fixed(PADDING, |_, _| {}),
                        table::fixed(RARITY_LABEL_WIDTH, |wh, ctx| {
                            ctx.add(
                                Icon::new(IconKind::Rarity {
                                    rarity: Rarity::Legendary,
                                })
                                .size(IconSize::Medium)
                                .wh(wh),
                            );
                        }),
                        table::ratio(1, |_, _| {}),
                        table::ratio(1, |wh, ctx| {
                            ctx.add(
                                paragraph(format!("{}%", weights[3][0]))
                                    .size(crate::theme::typography::FontSize::Medium)
                                    .align(crate::theme::typography::TextAlign::Center { wh })
                                    .build(),
                            );
                        }),
                        table::ratio(1, |wh, ctx| {
                            ctx.add(
                                paragraph(">>>".to_string())
                                    .size(crate::theme::typography::FontSize::Medium)
                                    .align(crate::theme::typography::TextAlign::Center { wh })
                                    .build(),
                            );
                        }),
                        table::ratio(1, |wh, ctx| {
                            ctx.add(
                                paragraph(format!("{}%", weights[3][1]))
                                    .size(crate::theme::typography::FontSize::Medium)
                                    .align(crate::theme::typography::TextAlign::Center { wh })
                                    .build(),
                            );
                        }),
                    ]),
                ),
            ])(wh, ctx);
        });

        ctx.add(simple_rect(
            wh,
            palette::OUTLINE,
            1.px(),
            palette::SURFACE_CONTAINER,
        ));
    }
}
