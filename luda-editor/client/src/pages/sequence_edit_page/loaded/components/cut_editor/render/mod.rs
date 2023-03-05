use super::*;
use crate::{
    components::sequence_player::{self, render_image},
    *,
};
use namui_prebuilt::*;
use std::collections::BTreeSet;

impl CutEditor {
    pub fn render(&self, props: Props) -> namui::RenderingTree {
        let Some(cut) = props.cut else {
            return RenderingTree::Empty;
        };

        let content_rect = sequence_player::get_inner_content_rect(props.wh);

        let character_name = cut.character_name.clone();
        let line_text = cut.line.clone();
        let cut_id = cut.id();
        let prev_cut_id = props.cuts.iter().enumerate().find_map(|(i, cut)| {
            if cut.id() == cut_id {
                if i == 0 {
                    None
                } else {
                    Some(props.cuts[i - 1].id())
                }
            } else {
                None
            }
        });
        let next_cut_id = props.cuts.iter().enumerate().find_map(|(i, cut)| {
            if cut.id() == cut_id {
                if i == props.cuts.len() - 1 {
                    None
                } else {
                    Some(props.cuts[i + 1].id())
                }
            } else {
                None
            }
        });

        let character_name_candidates = get_character_name_candidates(&props.cuts, &cut);

        render([
            simple_rect(props.wh, color::STROKE_NORMAL, 1.px(), color::BACKGROUND).attach_event(
                |build| {
                    build
                        .on_file_drop(move |event| {
                            let file = event.files[0].clone();
                            spawn_local(async move {
                                let content = file.content().await;
                                namui::event::send(Event::AddNewImage {
                                    png_bytes: content.into(),
                                    cut_id,
                                })
                            });
                        })
                        .on_key_down(move |event| {
                            if event.code == Code::KeyV && namui::keyboard::ctrl_press() {
                                spawn_local(async move {
                                    let Ok(buffers) = clipboard::read_image_buffers().await else {
                                    return
                                };
                                    for png_bytes in buffers {
                                        namui::event::send(Event::AddNewImage { png_bytes, cut_id })
                                    }
                                });
                            }
                        });
                },
            ),
            translate(
                content_rect.x(),
                content_rect.y(),
                render([
                    simple_rect(
                        content_rect.wh(),
                        color::STROKE_NORMAL,
                        1.px(),
                        color::BACKGROUND,
                    ),
                    props.cut.map_or(RenderingTree::Empty, |cut| {
                        self.image_wysiwyg_editor.render(wysiwyg_editor::Props {
                            wh: content_rect.wh(),
                            screen_images: &cut.screen_images,
                            project_id: props.project_id,
                            cut_id: cut.id(),
                        })
                    }),
                    sequence_player::render_text_box(content_rect.wh()),
                    sequence_player::render_over_text(
                        content_rect.wh(),
                        |wh| {
                            render([
                                transparent_rect(wh),
                                if props.is_focused
                                    && self.selected_target == Some(ClickTarget::CharacterName)
                                {
                                    self.character_name_input.render(
                                        auto_complete_text_input::Props {
                                            text: character_name,
                                            wh,
                                            candidates: character_name_candidates,
                                            on_text_change: move |text| {
                                                namui::event::send(Event::ChangeCharacterName {
                                                    name: text,
                                                    cut_id,
                                                })
                                            },
                                            on_edit_done: move || {
                                                namui::event::send(Event::Click {
                                                    target: ClickTarget::CutText,
                                                })
                                            },
                                            on_key_down: move |event| {
                                                if event.code == Code::Tab && !event.is_composing {
                                                    event.prevent_default();
                                                    if namui::keyboard::any_code_press([
                                                        Code::ShiftLeft,
                                                        Code::ShiftRight,
                                                    ]) {
                                                        let Some(prev_cut_id) = prev_cut_id else {
                                                            return
                                                        };
                                                        namui::event::send(Event::MoveCutByTab {
                                                            cut_id: prev_cut_id,
                                                            to_prev: true,
                                                        })
                                                    } else {
                                                        namui::event::send(Event::Click {
                                                            target: ClickTarget::CutText,
                                                        })
                                                    }
                                                }
                                            },
                                        },
                                    )
                                } else {
                                    text(TextParam {
                                        text: character_name,
                                        x: 0.px(),
                                        y: wh.height / 2.0,
                                        align: TextAlign::Left,
                                        baseline: TextBaseline::Middle,
                                        font_type: sequence_player::CHARACTER_NAME_FONT,
                                        style: sequence_player::character_name_text_style(
                                            1.one_zero(),
                                        ),
                                        max_width: Some(wh.width),
                                    })
                                },
                            ])
                            .attach_event(|builder| {
                                builder.on_mouse_down_in(|event| {
                                    if event.button == Some(MouseButton::Left) {
                                        namui::event::send(Event::Click {
                                            target: ClickTarget::CharacterName,
                                        })
                                    }
                                });
                            })
                        },
                        |wh| {
                            render([
                                transparent_rect(wh),
                                if props.is_focused
                                    && self.selected_target == Some(ClickTarget::CutText)
                                {
                                    self.text_input.render(text_input::Props {
                                        rect: wh.to_rect(),
                                        text: line_text,
                                        text_align: TextAlign::Left,
                                        text_baseline: TextBaseline::Top,
                                        font_type: sequence_player::CUT_TEXT_FONT,
                                        style: text_input::Style {
                                            text: sequence_player::cut_text_style(1.one_zero()),
                                            rect: RectStyle {
                                                stroke: Some(RectStroke {
                                                    color: color::STROKE_FOCUS,
                                                    width: 2.px(),
                                                    border_position: BorderPosition::Middle,
                                                }),
                                                fill: Some(RectFill {
                                                    color: color::BACKGROUND,
                                                }),
                                                ..Default::default()
                                            },
                                            ..Default::default()
                                        },
                                        event_handler: Some(
                                            text_input::EventHandler::new()
                                                .on_text_updated(move |text| {
                                                    namui::event::send(Event::ChangeCutLine {
                                                        text: text.to_string(),
                                                        cut_id,
                                                    })
                                                })
                                                .on_key_down(move |event| {
                                                    if event.code == Code::Tab {
                                                        event.prevent_default();
                                                        if namui::keyboard::shift_press() {
                                                            namui::event::send(Event::Click {
                                                                target: ClickTarget::CharacterName,
                                                            })
                                                        } else {
                                                            let Some(next_cut_id) = next_cut_id else {
                                                                return
                                                            };
                                                            namui::event::send(
                                                                Event::MoveCutByTab {
                                                                    cut_id: next_cut_id,
                                                                    to_prev: false,
                                                                },
                                                            )
                                                        }
                                                    }
                                                }),
                                        ),
                                    })
                                } else {
                                    text(TextParam {
                                        text: line_text,
                                        x: 0.px(),
                                        y: 0.px(),
                                        align: TextAlign::Left,
                                        baseline: TextBaseline::Top,
                                        font_type: sequence_player::CUT_TEXT_FONT,
                                        style: sequence_player::cut_text_style(1.one_zero()),
                                        max_width: Some(wh.width),
                                    })
                                },
                            ])
                            .attach_event(|builder| {
                                builder
                                    .on_mouse_down_in(|event| {
                                        if event.button == Some(MouseButton::Left) {
                                            namui::event::send(Event::Click {
                                                target: ClickTarget::CutText,
                                            })
                                        }
                                    });
                            })
                        },
                    ),
                ]),
            ),
        ])
    }
}

fn get_character_name_candidates(cuts: &[Cut], current_cut: &Cut) -> Vec<String> {
    let character_name = &current_cut.character_name;

    let mut candidates = cuts
        .iter()
        .enumerate()
        .map(|(index, cut)| (index, cut.character_name.clone()))
        .filter(|(_, name)| name != character_name)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();

    let cut_index = cuts
        .iter()
        .position(|cut| cut.id() == current_cut.id())
        .unwrap();

    candidates.sort_by_key(|(index, _)| (cut_index as isize - *index as isize).abs());

    candidates
        .into_iter()
        .map(|(_, name)| name)
        .collect::<Vec<_>>()
}
