use crate::app::{
    music::{MusicSpeedMap, Speed, SPEEDS},
    MUSIC_SPEED_MAP_ATOM,
};
use namui::prelude::*;
use namui_prebuilt::{simple_rect, table::hooks::*, typography};

const ROUND: Px = px(8.0);

#[component]
pub struct SpeedDropdown<'a> {
    pub wh: Wh<Px>,
    pub music_id: Option<&'a str>,
    pub music_speed_map: Option<&'a MusicSpeedMap>,
}
impl Component for SpeedDropdown<'_> {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        let Self {
            wh,
            music_id,
            music_speed_map,
        } = self;

        let (selecting, set_selecting) = ctx.state(|| false);
        let speed = {
            if let (Some(music_id), Some(music_speed_map)) = (music_id, music_speed_map) {
                Some(music_speed_map.get(music_id))
            } else {
                None
            }
        };

        ctx.component(SpeedDropdownButton { wh, speed }.attach_event(|event| {
            let Event::MouseDown { event } = event else {
                return;
            };
            if !event.is_local_xy_in() {
                return;
            }
            set_selecting.set(true);
        }));

        ctx.compose(|ctx| {
            if !*selecting {
                return;
            }

            ctx.translate((0.px(), wh.height)).on_top().add(
                SpeedDropdownContent {
                    item_wh: wh,
                    music_id,
                }
                .attach_event(|event| {
                    let Event::MouseDown { event } = event else {
                        return;
                    };
                    set_selecting.set(false);
                    event.stop_propagation();
                }),
            );
        });

        ctx.done()
    }
}

#[component]
struct SpeedDropdownButton {
    wh: Wh<Px>,
    speed: Option<Speed>,
}
impl Component for SpeedDropdownButton {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        let Self { wh, speed } = self;
        ctx.compose(|ctx| {
            let mut ctx = ctx.clip(
                Path::new().add_rrect(Rect::zero_wh(wh), ROUND, ROUND),
                ClipOp::Intersect,
            );

            horizontal([
                ratio(1, |wh, ctx| {
                    ctx.add(typography::center_text(
                        wh,
                        speed
                            .map(|speed| speed.to_string())
                            .unwrap_or("?".to_string()),
                        Color::WHITE,
                        typography::adjust_font_size(wh.height),
                    ));
                }),
                fixed(36.px(), |wh, ctx| {
                    ctx.add(typography::center_text(wh, "▼", Color::WHITE, 36.int_px()));
                    ctx.add(simple_rect(
                        wh,
                        Color::TRANSPARENT,
                        0.px(),
                        Color::grayscale_u8(128),
                    ));
                }),
            ])(wh, &mut ctx);

            ctx.add(simple_rect(
                wh,
                Color::TRANSPARENT,
                0.px(),
                Color::from_u8(0, 0, 0, 128),
            ));
        });
        ctx.done()
    }
}

#[component]
struct SpeedDropdownContent<'a> {
    item_wh: Wh<Px>,
    music_id: Option<&'a str>,
}
impl Component for SpeedDropdownContent<'_> {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        let Self { item_wh, music_id } = self;

        ctx.compose(|ctx| {
            let content = ctx.ghost_compose(None, |ctx| {
                vertical(SPEEDS.map(|speed| {
                    fixed(item_wh.height, move |wh, ctx| {
                        ctx.add(typography::center_text(
                            wh,
                            speed.to_string(),
                            Color::WHITE,
                            typography::adjust_font_size(wh.height),
                        ));
                        ctx.add(
                            simple_rect(
                                wh,
                                Color::TRANSPARENT,
                                0.px(),
                                Color::from_u8(0, 0, 0, 128),
                            )
                            .attach_event(|event| {
                                let Event::MouseDown { event } = event else {
                                    return;
                                };
                                if !event.is_local_xy_in() {
                                    return;
                                }
                                let Some(music_id) = music_id else {
                                    return;
                                };

                                let music_id = music_id.to_string();
                                MUSIC_SPEED_MAP_ATOM.mutate(move |music_speed_map| {
                                    let Some(music_speed_map) = music_speed_map else {
                                        return;
                                    };
                                    music_speed_map.set(music_id, speed);
                                    let music_speed_map = music_speed_map.clone();
                                    namui::spawn(async move {
                                        music_speed_map.save().await;
                                    });
                                });
                            }),
                        );
                    })
                }))(item_wh, ctx);
            });

            let content_height = content.bounding_box().unwrap_or_default().height();
            ctx.clip(
                Path::new().add_rrect(
                    Rect::zero_wh(Wh::new(item_wh.width, content_height)),
                    ROUND,
                    ROUND,
                ),
                ClipOp::Intersect,
            )
            .add(content);
        });

        ctx.done()
    }
}
