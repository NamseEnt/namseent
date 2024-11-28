use crate::*;
use list_view::AutoListView;
use luda_rpc::Scene;
use namui::*;
use namui_prebuilt::*;
use std::collections::BTreeMap;

pub struct SpeakerSelector<'a> {
    pub wh: Wh<Px>,
    pub scene: &'a Scene,
    pub project_id: u128,
    pub episode_id: u128,
    pub select_speaker: &'a dyn Fn(u128),
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
        let (speaker_ids, set_speaker_ids) = ctx.state::<Option<Vec<u128>>>(|| None);
        let (speakers, set_speakers) = ctx.state::<Option<BTreeMap<u128, Option<String>>>>(|| None);
        let (error_msg, set_error_msg) = ctx.state::<Option<String>>(|| None);
        let (show_edit_speaker_modal, set_show_edit_speaker_modal) = ctx.state(|| false);

        ctx.async_effect(
            "load speaker ids",
            &episode_id,
            move |episode_id| async move {
                use crate::rpc::episode_editor::load_speaker_slots::*;
                match server_connection()
                    .load_speaker_slots(RefRequest { episode_id })
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
            (speaker_ids, &project_id),
            move |(speaker_ids, project_id)| async move {
                set_speakers.set(None);
                let Some(speaker_ids) = speaker_ids else {
                    return;
                };
                use crate::rpc::episode_editor::get_speaker_names::*;
                match server_connection()
                    .get_speaker_names(RefRequest {
                        language_code: "kor",
                        project_id,
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

        let add_speaker = |name: &str| {
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
                                    select_speaker(*speaker_id);
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
    speakers: &'a Option<BTreeMap<u128, Option<String>>>,
    add_speaker: &'a dyn Fn(&str),
    close: &'a dyn Fn(),
}
impl Component for EditSpeakerModal<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            speakers,
            add_speaker,
            close,
        } = self;
        const MODAL_WH: Wh<Px> = Wh::new(px(512.0), px(480.0));

        let screen_wh = screen::size().map(|x| x.into_px());
        let (new_speaker_name, set_new_speaker_name) = ctx.state(|| "".to_string());
        let (left_selected_speaker_id, set_left_selected_speaker_id) = ctx.state(|| None);
        let (right_selected_speaker_id, set_right_selected_speaker_id) = ctx.state(|| None);

        let select_left_speaker = |speaker_id: u128| {
            set_left_selected_speaker_id.set(Some(speaker_id));
        };

        let select_right_speaker = |speaker_id: u128| {
            set_right_selected_speaker_id.set(Some(speaker_id));
        };

        ctx.compose(move |ctx| {
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
                                    ratio(1, |wh, ctx| {
                                        ctx.add(SpeakerList {
                                            wh,
                                            speakers,
                                            select_speaker: &select_left_speaker,
                                            selected_speaker_id: left_selected_speaker_id.as_ref(),
                                        });
                                    }),
                                    fixed(
                                        96.px(),
                                        padding(
                                            8.px(),
                                            vertical([
                                                ratio(1, |_wh, _ctx| {}),
                                                fixed(36.px(), |wh, ctx| {
                                                    ctx.add(button::TextButton {
                                                        rect: wh.to_rect(),
                                                        text: ">>>>>",
                                                        text_color: Color::WHITE,
                                                        stroke_color: Color::BLACK,
                                                        stroke_width: 1.px(),
                                                        fill_color: Color::BLUE,
                                                        mouse_buttons: vec![MouseButton::Left],
                                                        on_mouse_up_in: |_event| {
                                                            todo!("");
                                                        },
                                                    });
                                                }),
                                                table::fixed(8.px(), |_wh, _ctx| {}),
                                                fixed(36.px(), |wh, ctx| {
                                                    ctx.add(button::TextButton {
                                                        rect: wh.to_rect(),
                                                        text: "<<<<<",
                                                        text_color: Color::WHITE,
                                                        stroke_color: Color::BLACK,
                                                        stroke_width: 1.px(),
                                                        fill_color: Color::BLUE,
                                                        mouse_buttons: vec![MouseButton::Left],
                                                        on_mouse_up_in: |_event| {
                                                            todo!("");
                                                        },
                                                    });
                                                }),
                                                ratio(1, |_wh, _ctx| {}),
                                            ]),
                                        ),
                                    ),
                                    ratio(1, |wh, ctx| {
                                        ctx.add(SpeakerList {
                                            wh,
                                            speakers,
                                            select_speaker: &select_right_speaker,
                                            selected_speaker_id: right_selected_speaker_id.as_ref(),
                                        });
                                    }),
                                ]),
                            ),
                            table::fixed(8.px(), |_wh, _ctx| {}),
                            table::fixed(
                                36.px(),
                                horizontal([
                                    ratio(1, |wh, ctx| {
                                        ctx.add(TextInput {
                                            rect: Rect::zero_wh(wh),
                                            start_text: new_speaker_name.as_ref(),
                                            text_align: TextAlign::Center,
                                            text_baseline: TextBaseline::Middle,
                                            font: Font {
                                                size: 16.int_px(),
                                                name: "NotoSansKR-Regular".to_string(),
                                            },
                                            style: Style {
                                                rect: RectStyle {
                                                    stroke: Some(RectStroke {
                                                        color: Color::WHITE,
                                                        width: 1.px(),
                                                        border_position: BorderPosition::Middle,
                                                    }),
                                                    fill: Some(RectFill {
                                                        color: Color::grayscale_f01(0.3),
                                                    }),
                                                    round: Some(RectRound { radius: 4.px() }),
                                                },
                                                text: TextStyle {
                                                    color: Color::WHITE,
                                                    ..Default::default()
                                                },
                                                padding: Ltrb::all(8.px()),
                                            },
                                            prevent_default_codes: &[Code::Enter],
                                            focus: None,
                                            on_edit_done: &|text| {
                                                set_new_speaker_name.set(text);
                                            },
                                        });
                                    }),
                                    fixed(8.px(), |_wh, _ctx| {}),
                                    fixed(192.px(), |wh, ctx| {
                                        ctx.add(button::TextButton {
                                            rect: wh.to_rect(),
                                            text: "새 캐릭터 추가하기",
                                            text_color: Color::WHITE,
                                            stroke_color: Color::BLACK,
                                            stroke_width: 1.px(),
                                            fill_color: Color::BLUE,
                                            mouse_buttons: vec![MouseButton::Left],
                                            on_mouse_up_in: |_event| {
                                                if new_speaker_name.is_empty() {
                                                    toast::negative("이름을 입력해주세요.");
                                                    return;
                                                }
                                                add_speaker(new_speaker_name.as_ref());
                                            },
                                        });
                                    }),
                                ]),
                            ),
                        ]),
                    )(MODAL_WH, ctx);
                });

                ctx.add(simple_rect(
                    MODAL_WH,
                    Color::from_u8(0xEE, 0xEE, 0xEE, 0xFF),
                    1.px(),
                    Color::from_u8(0x55, 0x55, 0x55, 0xFF),
                ));

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

struct SpeakerList<'a> {
    wh: Wh<Px>,
    speakers: &'a Option<BTreeMap<u128, Option<String>>>,
    select_speaker: &'a dyn Fn(u128),
    selected_speaker_id: &'a Option<u128>,
}
impl Component for SpeakerList<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            wh,
            speakers,
            select_speaker,
            selected_speaker_id: selected_speaker,
        } = self;

        const SCROLL_BAR_WIDTH: Px = px(4.0);

        let item_wh = Wh::new(wh.width - SCROLL_BAR_WIDTH, 36.px());

        ctx.compose(|ctx| {
            let Some(speakers) = speakers else {
                return;
            };
            ctx.add(AutoListView {
                height: wh.height,
                scroll_bar_width: 4.px(),
                item_wh,
                items: speakers.iter().map(|(speaker_id, name)| {
                    let is_on = selected_speaker
                        .as_ref()
                        .map(|x| x == speaker_id)
                        .unwrap_or_default();
                    (
                        speaker_id.to_string(),
                        simple_toggle_button(
                            item_wh,
                            name.clone().unwrap_or_default(),
                            is_on,
                            |_event| {
                                select_speaker(*speaker_id);
                            },
                        ),
                    )
                }),
            });
        });

        ctx.add(simple_rect(
            wh,
            Color::from_u8(0xEE, 0xEE, 0xEE, 0xFF),
            1.px(),
            Color::from_u8(0x44, 0x44, 0x44, 0xFF),
        ));
    }
}
