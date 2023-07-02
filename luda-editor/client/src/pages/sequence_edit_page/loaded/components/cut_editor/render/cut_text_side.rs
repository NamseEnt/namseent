use super::*;
use crate::pages::sequence_edit_page::sequence_atom::SEQUENCE_ATOM;
use rpc::data::CutUpdateAction;

impl CutEditor {
    pub fn render_cut_text_side(
        &self,
        wh: Wh<Px>,
        props: &Props,
        cut: &Cut,
    ) -> namui::RenderingTree {
        let line_text = cut.line.clone();
        let cut_id = cut.id;

        render([
            transparent_rect(wh),
            if props.is_focused && self.selected_target == Some(ClickTarget::CutText) {
                let next_cut_id = next_cut_id(&props, cut_id);

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
                            .on_text_updated(move |text: String| {
                                SEQUENCE_ATOM.update(move |sequence| {
                                    sequence.update_cut(
                                        cut_id,
                                        CutUpdateAction::ChangeCutLine {
                                            line: text,
                                        },
                                    )
                                });
                            })
                            .on_key_down(move |event: KeyDownEvent| {
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
                                        namui::event::send(Event::MoveCutRequest {
                                            cut_id: next_cut_id,
                                            to_prev: false,
                                            focused: true,
                                        })
                                    }
                                } else if event.code == Code::Escape {
                                    namui::event::send(InternalEvent::EscapeKeyDown)
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
            builder.on_mouse_down_in(|event: MouseEvent| {
                if event.button == Some(MouseButton::Left) {
                    namui::event::send(Event::Click {
                        target: ClickTarget::CutText,
                    })
                }
            });
        })
    }
}
