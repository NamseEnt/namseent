use crate::*;
use list_view::AutoListView;
use luda_rpc::Speaker;
use namui::*;
use namui_prebuilt::*;

pub struct SpeakerSelector<'a> {
    pub wh: Wh<Px>,
    pub select_speaker: &'a dyn Fn(u128),
    pub speakers_in_slot: Sig<'a, Vec<Speaker>>,
    pub open_edit_speaker_modal: &'a dyn Fn(),
}

impl Component for SpeakerSelector<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            wh,
            select_speaker,
            speakers_in_slot,
            open_edit_speaker_modal,
        } = self;

        ctx.compose(|ctx| {
            let items = {
                let mut items = vec![];

                for speaker in speakers_in_slot.iter() {
                    items.push(table::ratio(1, |wh, ctx| {
                        ctx.add(button::TextButton {
                            rect: wh.to_rect(),
                            text: speaker.name_l10n.get("kor").unwrap_or(&"N/A".to_string()),
                            text_color: Color::WHITE,
                            stroke_color: Color::WHITE,
                            stroke_width: 1.px(),
                            fill_color: Color::BLACK,
                            mouse_buttons: vec![MouseButton::Left],
                            on_mouse_up_in: |_event| {
                                select_speaker(speaker.id);
                            },
                        });
                    }));
                    items.push(table::fixed(16.px(), |_wh, _ctx| {}));
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
                            open_edit_speaker_modal();
                        },
                    });
                }));

                items
            };
            table::horizontal(items)(wh, ctx)
        });
    }
}

pub struct EditSpeakerModal<'a> {
    pub speakers: Sig<'a, Vec<Speaker>>,
    pub speaker_slots: Sig<'a, Vec<u128>>,
    pub add_speaker: &'a dyn Fn(String),
    pub save_speaker_slots: &'a dyn Fn(Vec<u128>),
    pub close: &'a dyn Fn(),
}
impl Component for EditSpeakerModal<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            speakers,
            speaker_slots,
            add_speaker,
            save_speaker_slots,
            close,
        } = self;
        const MODAL_WH: Wh<Px> = Wh::new(px(512.0), px(480.0));

        let splitted_speakers = ctx.memo(|| {
            let mut left = vec![];
            let mut right = vec![];
            for speaker in speakers.iter().cloned() {
                if speaker_slots.contains(&speaker.id) {
                    right.push(speaker);
                } else {
                    left.push(speaker);
                }
            }
            (left, right)
        });
        let (left_speakers, right_speakers) = splitted_speakers.as_ref();

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
            let ctx = ctx.absolute((0.px(), 0.px()));

            let left_top = (screen_wh / 2.0) - MODAL_WH / 2.0;
            ctx.translate(left_top.to_xy()).compose(|ctx| {
                ctx.compose(|ctx| {
                    table::padding(
                        8.px(),
                        vertical([
                            table::fixed(
                                36.px(),
                                table::horizontal([
                                    ratio(1, |_, _| {}),
                                    table::fixed(36.px(), |wh, ctx| {
                                        ctx.add(button::TextButton {
                                            rect: wh.to_rect(),
                                            text: "X",
                                            text_color: Color::WHITE,
                                            stroke_color: Color::BLACK,
                                            stroke_width: 1.px(),
                                            fill_color: Color::RED,
                                            mouse_buttons: vec![MouseButton::Left],
                                            on_mouse_up_in: |_event| {
                                                close();
                                            },
                                        });
                                    }),
                                ]),
                            ),
                            table::fixed(8.px(), |_wh, _ctx| {}),
                            table::ratio(
                                1,
                                horizontal([
                                    ratio(1, |wh, ctx| {
                                        ctx.add(SpeakerList {
                                            wh,
                                            speakers: left_speakers,
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
                                                            let Some(speaker_id) =
                                                                left_selected_speaker_id.as_ref()
                                                            else {
                                                                return;
                                                            };
                                                            if speaker_slots.contains(speaker_id) {
                                                                return;
                                                            }
                                                            let mut speaker_slots =
                                                                speaker_slots.clone_inner();
                                                            speaker_slots.push(*speaker_id);
                                                            save_speaker_slots(speaker_slots);
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
                                                            let Some(speaker_id) =
                                                                right_selected_speaker_id.as_ref()
                                                            else {
                                                                return;
                                                            };
                                                            if !speaker_slots.contains(speaker_id) {
                                                                return;
                                                            }
                                                            let speaker_slots = speaker_slots
                                                                .iter()
                                                                .filter(|x| x != &speaker_id)
                                                                .cloned()
                                                                .collect();
                                                            save_speaker_slots(speaker_slots);
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
                                            speakers: right_speakers,
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
                                                add_speaker(new_speaker_name.clone_inner());
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
    speakers: &'a Vec<Speaker>,
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
            ctx.add(AutoListView {
                height: wh.height,
                scroll_bar_width: 4.px(),
                item_wh,
                items: speakers.iter().map(|speaker| {
                    let is_on = selected_speaker
                        .as_ref()
                        .map(|x| x == &speaker.id)
                        .unwrap_or_default();
                    (
                        speaker.id.to_string(),
                        simple_toggle_button(
                            item_wh,
                            speaker.name_l10n.get("kor").cloned().unwrap_or_default(),
                            is_on,
                            |_event| {
                                select_speaker(speaker.id);
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
