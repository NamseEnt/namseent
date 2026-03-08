use crate::game_state::{mutate_game_state, use_game_state};
use crate::icon::IconKind;
use crate::shop_panel::constants::*;
use crate::sound::{self, EmitSoundParams, SoundGroup, SpatialMode, VolumePreset};
use crate::theme::button::{Button, ButtonColor, ButtonVariant};
use crate::theme::palette;
use crate::theme::paper_container::{PaperContainerBackground, PaperTexture, PaperVariant};
use crate::theme::typography::memoized_text;
use namui::*;
use namui_prebuilt::table;

pub(super) struct ShopActionArea {
    pub wh: Wh<Px>,
}

impl Component for ShopActionArea {
    fn render(self, ctx: &RenderCtx) {
        let Self { wh } = self;
        let game_state = use_game_state(ctx);

        ctx.compose(|ctx| {
            table::padding_no_clip(
                INNER_PADDING + ACTION_MARGIN_Y * 0.5,
                table::horizontal([
                    table::ratio_no_clip(1, |wh, ctx| {
                        let level = game_state.level.get();
                        let level_up_cost = game_state.level_up_cost();
                        let can_upgrade = level < 10 && game_state.gold >= level_up_cost;

                        ctx.add(
                            Button::new(
                                wh,
                                &|| {
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
                                &|wh, color, ctx| {
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
        });
    }
}
