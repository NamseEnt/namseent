use crate::game_state::upgrade::Upgrade;
use crate::game_state::{flow::GameFlow, mutate_game_state, use_game_state};
use crate::theme::{halo::Halo, palette};
use namui::*;
use namui_prebuilt::{simple_rect, table};

use super::card::TreasureCard;

use super::{
    CARD_GAP, MODAL_HEIGHT, MODAL_WIDTH, PADDING, TREASURE_BG_PADDING, TREASURE_HALO_PADDING,
};

pub struct TreasureSelectionUi;

impl Component for TreasureSelectionUi {
    fn render(self, ctx: &RenderCtx) {
        let game_state = use_game_state(ctx);
        let locale = game_state.text().locale();

        let flow = if let GameFlow::TreasureSelection(flow) = &game_state.flow {
            flow
        } else {
            return;
        };

        let screen_wh = screen::size().into_type::<Px>();
        let modal_wh = Wh::new(MODAL_WIDTH, MODAL_HEIGHT);
        let modal_xy = ((screen_wh - modal_wh) * 0.5).to_xy();

        let is_closing = flow.pending_selection.is_some();
        let target_modal_scale = if is_closing { 0.0 } else { 1.0 };
        let modal_scale =
            crate::animation::with_spring(ctx, target_modal_scale, 0.0, |v| v * v, || 0.0);

        let target_backdrop_alpha = if is_closing { 0.0 } else { 1.0 };
        let backdrop_progress =
            crate::animation::with_spring(ctx, target_backdrop_alpha, 0.0, |v| v * v, || 0.0);
        let backdrop_alpha = (backdrop_progress * 180.0).clamp(0.0, 180.0) as u8;

        if let Some(selected_index) = flow.pending_selection
            && modal_scale < 0.02
        {
            mutate_game_state(move |gs| {
                if let GameFlow::TreasureSelection(_) = gs.flow {
                    gs.select_treasure(selected_index);
                }
            });
            return;
        }

        let modal_center = modal_wh.to_xy() * 0.5;

        Self::render_modal(
            ctx,
            modal_wh,
            modal_xy,
            modal_scale,
            &flow.options,
            locale,
            modal_center,
        );

        ctx.add(simple_rect(
            screen_wh,
            Color::TRANSPARENT,
            0.px(),
            Color::BLACK.with_alpha(backdrop_alpha),
        ))
        .attach_event(|event| match event {
            Event::MouseDown { event } | Event::MouseUp { event } | Event::MouseMove { event } => {
                event.stop_propagation();
            }
            _ => {}
        });
    }
}

impl TreasureSelectionUi {
    fn render_modal(
        ctx: &RenderCtx,
        modal_wh: Wh<Px>,
        modal_xy: Xy<Px>,
        modal_scale: f32,
        options: &[Upgrade],
        locale: crate::l10n::Locale,
        modal_center: Xy<Px>,
    ) {
        ctx.compose(|ctx| {
            let ctx = ctx
                .translate(modal_xy + modal_center)
                .scale(Xy::single(modal_scale))
                .translate(-modal_center);

            Self::render_treasure_layer(&ctx, modal_wh, options, locale);
        });
    }

    fn render_treasure_layer(
        ctx: &ComposeCtx,
        modal_wh: Wh<Px>,
        options: &[Upgrade],
        locale: crate::l10n::Locale,
    ) {
        let treasure_bg_wh = Wh::new(
            modal_wh.width + TREASURE_BG_PADDING * 2.0,
            modal_wh.height + TREASURE_BG_PADDING * 2.0,
        );
        let treasure_bg_xy = Xy::new(-TREASURE_BG_PADDING, -TREASURE_BG_PADDING);

        ctx.compose(|ctx| {
            table::padding_no_clip(
                PADDING,
                table::vertical([
                    table::ratio_no_clip(1, |_, _| {}),
                    table::fixed_no_clip(modal_wh.height - CARD_GAP * 2.0, |wh, ctx| {
                        ctx.compose(|ctx| {
                            let card_height = (wh.height - CARD_GAP * 2.0) / 3.0;
                            table::vertical([
                                table::fixed_no_clip(card_height, |card_wh, ctx| {
                                    ctx.add(TreasureCard {
                                        wh: card_wh,
                                        upgrade: options[0],
                                        locale,
                                        index: 0,
                                    });
                                }),
                                table::fixed_no_clip(CARD_GAP, |_, _| {}),
                                table::fixed_no_clip(card_height, |card_wh, ctx| {
                                    ctx.add(TreasureCard {
                                        wh: card_wh,
                                        upgrade: options[1],
                                        locale,
                                        index: 1,
                                    });
                                }),
                                table::fixed_no_clip(CARD_GAP, |_, _| {}),
                                table::fixed_no_clip(card_height, |card_wh, ctx| {
                                    ctx.add(TreasureCard {
                                        wh: card_wh,
                                        upgrade: options[2],
                                        locale,
                                        index: 2,
                                    });
                                }),
                            ])(wh, ctx);
                        });
                    }),
                    table::ratio(1, |_, _| {}),
                ]),
            )(modal_wh, ctx);
        });

        render_treasure_background(ctx, treasure_bg_wh, treasure_bg_xy);
    }
}

fn render_treasure_background(ctx: &ComposeCtx, treasure_bg_wh: Wh<Px>, treasure_bg_xy: Xy<Px>) {
    let treasure_halo_wh = Wh::new(
        treasure_bg_wh.width + TREASURE_HALO_PADDING * 2.0,
        treasure_bg_wh.height + TREASURE_HALO_PADDING * 2.0,
    );

    ctx.compose(|ctx| {
        let ctx = ctx.translate(treasure_bg_xy);

        ctx.add(Halo {
            wh: treasure_halo_wh,
            radius: TREASURE_HALO_PADDING,
            color: palette::YELLOW.with_alpha(200),
            strength: 0.125,
            rotation_deg_per_sec: 10.0,
        });

        ctx.add(namui::image(ImageParam {
            rect: Rect::from_xy_wh(Xy::zero(), treasure_bg_wh),
            image: crate::asset::image::ui::TREASURE,
            style: ImageStyle {
                fit: ImageFit::Contain,
                paint: None,
            },
        }));
    });
}
