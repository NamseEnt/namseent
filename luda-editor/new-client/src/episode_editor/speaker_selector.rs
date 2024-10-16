use crate::*;
use luda_rpc::Scene;
use namui::*;
use namui_prebuilt::*;
use std::collections::BTreeMap;

pub struct SpeakerSelector<'a> {
    pub wh: Wh<Px>,
    pub scene: &'a Scene,
    pub project_id: &'a String,
    pub episode_id: &'a String,
    pub select_speaker: &'a dyn Fn(&String),
}

impl Component for SpeakerSelector<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            wh,
            scene,
            project_id,
            episode_id,
            select_speaker,
        } = self;
        let (speaker_ids, set_speaker_ids) = ctx.state::<Option<Vec<String>>>(|| None);
        let (speakers, set_speakers) =
            ctx.state::<Option<BTreeMap<String, Option<String>>>>(|| None);
        let (error_msg, set_error_msg) = ctx.state::<Option<String>>(|| None);
        let (show_edit_speaker_modal, set_show_edit_speaker_modal) = ctx.state(|| false);

        ctx.async_effect(
            "load speaker ids",
            episode_id,
            move |episode_id| async move {
                use crate::rpc::episode_editor::load_speaker_slots::*;
                match server_connection()
                    .load_speaker_slots(RefRequest {
                        episode_id: &episode_id,
                    })
                    .await
                {
                    Ok(response) => {
                        set_speaker_ids.set(Some(response.speaker_ids));
                    }
                    Err(err) => {
                        set_error_msg.set(Some(err.to_string()));
                    }
                }
            },
        );

        ctx.async_effect(
            "load speaker names",
            (speaker_ids, project_id),
            move |(speaker_ids, project_id)| async move {
                set_speakers.set(None);
                let Some(speaker_ids) = speaker_ids else {
                    return;
                };
                use crate::rpc::episode_editor::get_speaker_names::*;
                match server_connection()
                    .get_speaker_names(RefRequest {
                        language_code: "kor",
                        project_id: &project_id,
                        speaker_ids: &speaker_ids,
                    })
                    .await
                {
                    Ok(response) => {
                        set_speakers.set(Some(
                            speaker_ids
                                .into_iter()
                                .zip(response.speaker_names)
                                .collect(),
                        ));
                    }
                    Err(err) => {
                        set_error_msg.set(Some(err.to_string()));
                    }
                }
            },
        );

        let add_speaker = |_name| {
            todo!();
        };
        let close_modal = move || {
            set_show_edit_speaker_modal.set(false);
        };

        ctx.compose(|ctx| {
            if !*show_edit_speaker_modal {
                return;
            }
            ctx.add(EditSpeakerModal {
                speakers: speakers.as_ref(),
                add_speaker: &add_speaker,
                close: &close_modal,
            });
        });

        ctx.compose(|ctx| {
            let items = {
                let mut items = vec![];

                if let Some(speakers) = speakers.as_ref() {
                    for (speaker_id, name) in speakers {
                        items.push(table::fixed(16.px(), |_wh, _ctx| {}));
                        items.push(table::ratio(1, |wh, ctx| {
                            ctx.add(button::TextButton {
                                rect: wh.to_rect(),
                                text: name.as_ref().unwrap_or(&"N/A".to_string()),
                                text_color: Color::WHITE,
                                stroke_color: Color::WHITE,
                                stroke_width: 1.px(),
                                fill_color: Color::BLACK,
                                mouse_buttons: vec![MouseButton::Left],
                                on_mouse_up_in: |_event| {
                                    select_speaker(speaker_id);
                                },
                            });
                        }));
                    }
                }

                items.push(table::ratio(1, |_wh, _ctx| {}));
                // TODO: Fix table::fit
                items.push(table::fixed(96.px(), |wh, ctx| {
                    ctx.add(button::TextButton {
                        rect: wh.to_rect(),
                        text: "설정",
                        text_color: Color::WHITE,
                        stroke_color: Color::WHITE,
                        stroke_width: 1.px(),
                        fill_color: Color::BLACK,
                        mouse_buttons: vec![MouseButton::Left],
                        on_mouse_up_in: |_event| {
                            set_show_edit_speaker_modal.set(true);
                        },
                    });
                }));

                items
            };
            table::horizontal(items)(wh, ctx)
        });
    }
}

struct EditSpeakerModal<'a> {
    speakers: &'a Option<BTreeMap<String, Option<String>>>,
    add_speaker: &'a dyn Fn(&'a str),
    close: &'a dyn Fn(),
}
impl Component for EditSpeakerModal<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            close,
            speakers,
            add_speaker,
        } = self;
        const MODAL_WH: Wh<Px> = Wh::new(px(400.0), px(300.0));

        let screen_wh = screen::size().map(|x| x.into_px());

        ctx.compose(|ctx| {
            let ctx = ctx.on_top().absolute((0.px(), 0.px()));

            let left_top = (screen_wh / 2.0) - MODAL_WH / 2.0;

            ctx.translate(left_top.as_xy()).compose(|ctx| {
                ctx.compose(|ctx| {
                    table::padding(
                        8.px(),
                        vertical([
                            table::ratio(
                                1,
                                horizontal([
                                    ratio(1, |wh, ctx| {}),
                                    fixed(96.px(), padding(8.px(), |wh, ctx| {})),
                                    ratio(1, |wh, ctx| {}),
                                ]),
                            ),
                            table::fixed(
                                96.px(),
                                horizontal([
                                    ratio(1, |wh, ctx| {}),
                                    fixed(8.px(), |_wh, _ctx| {}),
                                    fixed(96.px(), |wh, ctx| {
                                        ctx.add(button::TextButton {
                                            rect: wh.to_rect(),
                                            text: "추가",
                                            text_color: Color::WHITE,
                                            stroke_color: Color::BLACK,
                                            stroke_width: 1.px(),
                                            fill_color: Color::BLUE,
                                            mouse_buttons: vec![MouseButton::Left],
                                            on_mouse_up_in: |_event| {
                                                close();
                                            },
                                        });
                                    }),
                                ]),
                            ),
                        ]),
                    )(MODAL_WH, ctx);
                });

                ctx.add(
                    simple_rect(
                        MODAL_WH,
                        Color::TRANSPARENT,
                        0.px(),
                        Color::from_u8(0xDD, 0xDD, 0xDD, 0xFF),
                    )
                    .attach_event(|event| {
                        let Event::MouseDown { event } = event else {
                            return;
                        };
                        if event.is_local_xy_in() {
                            event.stop_propagation();
                        }
                    }),
                );
            });

            ctx.add(
                simple_rect(
                    screen_wh,
                    Color::TRANSPARENT,
                    0.px(),
                    Color::grayscale_alpha_f01(0.0, 0.5),
                )
                .attach_event(|event| {
                    let Event::MouseDown { event } = event else {
                        return;
                    };
                    if !event.is_local_xy_in() {
                        return;
                    }
                    event.stop_propagation();
                    close();
                }),
            );
        });
    }
}
