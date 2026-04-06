use super::{Tower, mutate_game_state};
use crate::flow_ui::TowerPreviewContent;
use crate::theme::{
    button::{Button, ButtonColor, ButtonVariant},
    paper_container::{
        ArrowSide, PaperArrow, PaperContainerBackground, PaperTexture, PaperVariant,
    },
    typography::{FontSize, memoized_text},
};
use crate::{sound, theme};
use namui::*;
use namui_prebuilt::table;

const BUBBLE_PADDING: Px = px(12.);
const BUBBLE_WIDTH: Px = px(280.);
const BUBBLE_HEIGHT: Px = px(200.);

pub struct TowerInfoPopup<'a> {
    pub tower: &'a Tower,
}

impl Component for TowerInfoPopup<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self { tower } = self;

        ctx.translate((-BUBBLE_WIDTH * 0.5, -BUBBLE_HEIGHT))
            .compose(|ctx| {
                ctx.compose(|ctx| {
                    table::padding_no_clip(BUBBLE_PADDING, |wh, ctx| {
                        table::vertical([
                            table::ratio_no_clip(1.0, |wh, ctx| {
                                ctx.add(TowerPreviewContent {
                                    wh,
                                    tower_template: tower,
                                });
                            }),
                            table::fixed_no_clip(36.px(), |wh, ctx| {
                                let tower_id = tower.id();
                                ctx.add(
                                    Button::new(
                                        wh,
                                        &move || {
                                            mutate_game_state(move |game_state| {
                                                let tower_removed =
                                                    game_state.remove_tower(tower_id);
                                                if tower_removed {
                                                    sound::emit_sound(
                                                        sound::EmitSoundParams::one_shot(
                                                            sound::random_paper_crumpling(),
                                                            sound::SoundGroup::Sfx,
                                                            sound::VolumePreset::High,
                                                            sound::SpatialMode::NonSpatial,
                                                        ),
                                                    );
                                                }
                                            });
                                        },
                                        &|wh, text_color, ctx| {
                                            ctx.add(memoized_text(
                                                (&text_color, &wh),
                                                |mut builder| {
                                                    builder
                                                        .size(FontSize::Medium)
                                                        .color(text_color)
                                                        .max_width(wh.width)
                                                        .text("철거")
                                                        .render_center(wh)
                                                },
                                            ));
                                        },
                                    )
                                    .variant(ButtonVariant::Contained)
                                    .color(ButtonColor::Error),
                                );
                            }),
                        ])(wh, ctx);
                    })(Wh::new(BUBBLE_WIDTH, BUBBLE_HEIGHT), ctx);
                });

                ctx.add(PaperContainerBackground {
                    width: BUBBLE_WIDTH,
                    height: BUBBLE_HEIGHT,
                    texture: PaperTexture::Rough,
                    variant: PaperVariant::Sticky,
                    color: theme::palette::SURFACE_CONTAINER_HIGHEST,
                    shadow: true,
                    arrow: Some(PaperArrow {
                        side: ArrowSide::Bottom,
                        width: px(16.0),
                        height: px(16.0),
                        offset: BUBBLE_WIDTH * 0.5,
                    }),
                });
            })
            .attach_event(|event| {
                if let Event::MouseDown { event } = event
                    && let Some(MouseButton::Left) = event.button
                    && event.is_local_xy_in()
                {
                    event.stop_propagation();
                }
            });
    }
}
