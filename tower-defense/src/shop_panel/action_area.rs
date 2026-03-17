use crate::animation::with_spring;
use crate::game_state::{mutate_game_state, use_game_state};
use crate::icon::IconKind;
use crate::shop_panel::constants::*;
use crate::shop_panel::level_tooltip as tooltip;
use crate::sound::{self, EmitSoundParams, SoundGroup, SpatialMode, VolumePreset};
use crate::theme::button::{Button, ButtonColor, ButtonVariant};
use crate::theme::palette;
use crate::theme::paper_container::{PaperContainerBackground, PaperTexture, PaperVariant};
use crate::theme::typography::memoized_text;
use namui::*;
use namui_prebuilt::{simple_rect, table};

pub(super) struct ShopActionArea {
    pub wh: Wh<Px>,
}

impl Component for ShopActionArea {
    fn render(self, ctx: &RenderCtx) {
        let Self { wh } = self;
        let game_state = use_game_state(ctx);

        let level = game_state.level.get();
        let level_up_cost = game_state.level_up_cost();
        let can_upgrade = level < 10 && game_state.gold >= level_up_cost;

        let (hovering, set_hovering) = ctx.state(|| false);
        let tooltip_scale = with_spring(
            ctx,
            if level < 10 && *hovering { 1.0 } else { 0.0 },
            0.0,
            |v| v * v,
            || 0.0,
        );

        ctx.compose(|ctx| {
            table::padding_no_clip(
                INNER_PADDING + ACTION_MARGIN_Y * 0.5,
                table::horizontal([
                    table::ratio_no_clip(1, |wh, ctx| {
                        ctx.compose(|ctx| {
                            if tooltip_scale > 0.01 {
                                let next_level = (level + 1).min(10);
                                let tooltip_wh = tooltip::TOOLTIP_WH;
                                let pivot = Xy::new(tooltip_wh.width, tooltip_wh.height / 2.0);
                                let total_width = tooltip_wh.width
                                    + tooltip::BACKGROUND_ARROW_WIDTH
                                    + tooltip::BACKGROUND_ARROW_OFFSET_X;
                                let base =
                                    Xy::new(-total_width, (wh.height - tooltip_wh.height) / 2.0);
                                ctx.translate(base + pivot)
                                    .scale(Xy::new(tooltip_scale, tooltip_scale))
                                    .translate(Xy::new(-pivot.x, -pivot.y))
                                    .on_top()
                                    .add(tooltip::LevelUpTooltip {
                                        current_level: level,
                                        next_level,
                                    });
                            }
                        });

                        ctx.add(
                            simple_rect(wh, Color::TRANSPARENT, 0.px(), Color::TRANSPARENT)
                                .attach_event(move |event| {
                                    let Event::MouseMove { event } = event else {
                                        return;
                                    };
                                    set_hovering.set(event.is_local_xy_in());
                                }),
                        );

                        ctx.add(
                            Button::new(
                                wh,
                                &move || {
                                    if !can_upgrade {
                                        return;
                                    }
                                    mutate_game_state(move |gs| {
                                        gs.level = gs.level.checked_add(1).expect("Level overflow");
                                        gs.spend_gold(level_up_cost);
                                    });
                                    sound::emit_sound(EmitSoundParams::one_shot(
                                        sound::random_level_up(),
                                        SoundGroup::Sfx,
                                        VolumePreset::Medium,
                                        SpatialMode::NonSpatial,
                                    ));
                                },
                                &move |wh, color, ctx| {
                                    ctx.add(memoized_text(
                                        (&color, &level_up_cost),
                                        |mut builder| {
                                            builder
                                                .headline()
                                                .icon(IconKind::Level)
                                                .space()
                                                .icon(IconKind::Gold)
                                                .color(color)
                                                .text(format!("{level_up_cost}"))
                                                .render_center(wh)
                                        },
                                    ));
                                },
                            )
                            .variant(ButtonVariant::Contained)
                            .color(ButtonColor::Primary)
                            .disabled(!can_upgrade),
                        );
                    }),
                    table::fixed_no_clip(BUTTON_SPACING, |_, _| {}),
                    table::ratio_no_clip(1, |wh, ctx| {
                        ctx.add(super::refresh_button::RefreshButton::new(wh));
                    }),
                ]),
            )(wh, ctx);
        });

        ctx.add(PaperContainerBackground {
            width: wh.width,
            height: wh.height,
            texture: PaperTexture::Rough,
            variant: PaperVariant::Sticky,
            color: palette::SURFACE_CONTAINER_LOW,
            shadow: true,
            arrow: None,
        });
    }
}
