use namui::prelude::*;
use namui_prebuilt::*;

#[derive(Debug, Clone)]
pub struct RenameModal {
    sequence_id: String,
    sequence_name: String,
    text_input: TextInput,
}
pub enum Event {
    RenameDone {
        sequence_id: String,
        sequence_name: String,
    },
}
impl RenameModal {
    pub fn new(sequence_id: String, sequence_name: String) -> Self {
        Self {
            sequence_id,
            text_input: TextInput::new(),
            sequence_name,
        }
    }
    pub fn update(&mut self, event: &dyn std::any::Any) {
        if let Some(text_input::Event::TextUpdated(event)) = event.downcast_ref() {
            self.sequence_name = event.text.clone();
        }
        if let Some(namui::event::NamuiEvent::KeyUp(event)) = event.downcast_ref() {
            if event.code == Code::Enter {
                namui::event::send(Event::RenameDone {
                    sequence_id: self.sequence_id.clone(),
                    sequence_name: self.sequence_name.clone(),
                });
            }
        }
        self.text_input.update(event);
    }
    pub fn render(&self) -> namui::RenderingTree {
        let screen_wh = namui::screen::size();
        let modal_wh = screen_wh * 0.5;
        let modal_xy = ((screen_wh - modal_wh) * 0.5).as_xy();
        let text_input_rect_in_modal = Rect::Xywh {
            x: modal_wh.width / 4.0,
            y: modal_wh.height / 4.0,
            width: modal_wh.width / 2.0,
            height: 20.px(),
        };
        let enter_button_rect_in_modal = Rect::Xywh {
            x: text_input_rect_in_modal.x() + text_input_rect_in_modal.width() + 10.px(),
            y: text_input_rect_in_modal.y(),
            width: 40.px(),
            height: 20.px(),
        };

        absolute(
            0.px(),
            0.px(),
            render([
                simple_rect(
                    screen_wh,
                    Color::TRANSPARENT,
                    0.px(),
                    Color::from_f01(0.8, 0.8, 0.8, 0.8),
                ),
                translate(
                    modal_xy.x,
                    modal_xy.y,
                    render([
                        simple_rect(modal_wh, Color::WHITE, 1.px(), Color::grayscale_f01(0.5)),
                        namui_prebuilt::typography::body::center(
                            Wh::new(modal_wh.width, modal_wh.height / 3.0),
                            "Rename Sequence",
                            Color::WHITE,
                        ),
                        self.text_input.render(text_input::Props {
                            rect_param: RectParam {
                                rect: text_input_rect_in_modal,
                                style: RectStyle {
                                    stroke: Some(RectStroke {
                                        color: Color::BLACK,
                                        width: 1.px(),
                                        border_position: BorderPosition::Outside,
                                    }),
                                    fill: Some(RectFill {
                                        color: Color::WHITE,
                                    }),
                                    round: None,
                                },
                            },
                            text_param: TextParam {
                                text: self.sequence_name.clone(),
                                x: text_input_rect_in_modal.x() + 10.px(),
                                y: text_input_rect_in_modal.y(),
                                align: TextAlign::Left,
                                baseline: TextBaseline::Top,
                                font_type: FontType {
                                    serif: false,
                                    size: 12.int_px(),
                                    language: Language::Ko,
                                    font_weight: FontWeight::REGULAR,
                                },
                                style: TextStyle {
                                    border: None,
                                    drop_shadow: None,
                                    color: Color::BLACK,
                                    background: None,
                                },
                            },
                        }),
                        namui_prebuilt::button::text_button(
                            enter_button_rect_in_modal,
                            "Save",
                            Color::WHITE,
                            Color::WHITE,
                            1.px(),
                            Color::BLACK,
                            {
                                let sequence_id = self.sequence_id.clone();
                                let sequence_name = self.sequence_name.clone();
                                move || {
                                    namui::event::send(Event::RenameDone {
                                        sequence_id: sequence_id.clone(),
                                        sequence_name: sequence_name.clone(),
                                    });
                                }
                            },
                        ),
                    ]),
                ),
            ]),
        )
    }
}
