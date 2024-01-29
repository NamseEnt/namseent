use crate::app::{
    components::{Backdrop, ButtonHoverEffect, DarkFrame, FilledButton, LightFrame},
    music::{MusicSpeedMap, Speed, SPEEDS},
    theme::THEME,
    MUSIC_SPEED_MAP_ATOM,
};
use namui::prelude::*;
use namui_prebuilt::{table::hooks::*, typography::adjust_font_size};

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

        let (mouse_hover, set_mouse_hover) = ctx.state(|| false);

        ctx.component(ButtonHoverEffect {
            wh,
            focused: *mouse_hover,
        });

        ctx.compose(|ctx| {
            horizontal([
                ratio(1, |wh, ctx| {
                    ctx.add(text(TextParam {
                        text: speed
                            .map(|speed| speed.to_string())
                            .unwrap_or("?".to_string()),
                        x: wh.width / 2,
                        y: wh.height / 2,
                        align: TextAlign::Center,
                        baseline: TextBaseline::Middle,
                        font: Font {
                            size: adjust_font_size(wh.height),
                            name: THEME.font_name.to_string(),
                        },
                        style: TextStyle {
                            color: THEME.text.with_alpha(216),
                            ..Default::default()
                        },
                        max_width: None,
                    }));
                }),
                fixed(56.px(), |wh, ctx| {
                    ctx.add(text(TextParam {
                        // https://fontawesome.com/v5/icons/angle-down?f=classic&s=solid
                        text: "".to_string(),
                        x: wh.width / 2,
                        y: wh.height / 2,
                        align: TextAlign::Center,
                        baseline: TextBaseline::Middle,
                        font: Font {
                            size: adjust_font_size(wh.height),
                            name: THEME.icon_font_name.to_string(),
                        },
                        style: TextStyle {
                            color: THEME.text.with_alpha(216),
                            ..Default::default()
                        },
                        max_width: None,
                    }));
                    ctx.add(LightFrame { wh });
                }),
            ])(wh, ctx);
        });

        ctx.component(LightFrame { wh }.attach_event(|event| {
            let Event::MouseMove { event } = event else {
                return;
            };
            let hovering = event.is_local_xy_in();
            if *mouse_hover == hovering {
                return;
            }
            set_mouse_hover.set(hovering);
        }));

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

        let (music_speed_map, set_music_speed_map) = ctx.atom(&MUSIC_SPEED_MAP_ATOM);
        let selected_speed = (*music_speed_map)
            .as_ref()
            .map(|music_speed_map| music_speed_map.get(music_id.unwrap_or_default()))
            .unwrap_or_default();

        ctx.compose(|ctx| {
            let content = ctx.ghost_compose(None, |ctx| {
                vertical(SPEEDS.map(|speed| {
                    fixed_no_clip(
                        item_wh.height,
                        padding_no_clip(4.px(), move |wh, ctx| {
                            ctx.add(FilledButton {
                                wh,
                                text: speed.to_string(),
                                on_click: &|| {
                                    let Some(music_id) = music_id else {
                                        return;
                                    };

                                    let music_id = music_id.to_string();
                                    set_music_speed_map.mutate(move |music_speed_map| {
                                        let Some(music_speed_map) = music_speed_map else {
                                            return;
                                        };
                                        music_speed_map.set(music_id, speed);
                                        let music_speed_map = music_speed_map.clone();
                                        namui::spawn(async move {
                                            music_speed_map.save().await;
                                        });
                                    });
                                },
                                focused: selected_speed == speed,
                            });
                        }),
                    )
                }))(item_wh, ctx);
            });

            let content_height = content.bounding_box().unwrap_or_default().height();
            ctx.add(content);

            ctx.add(DarkFrame {
                wh: Wh::new(item_wh.width, content_height),
            });

            ctx.add(Backdrop {
                wh: Wh::new(item_wh.width, content_height),
            });
        });

        ctx.done()
    }
}
