use super::*;

pub struct Props {
    pub wh: Wh<Px>,
}

impl ImageEditModal {
    pub fn render_label_input(&self, props: Props) -> namui::RenderingTree {
        self.label_text_input.render(text_input::Props {
            rect: Rect::zero_wh(props.wh),
            rect_style: RectStyle {
                stroke: Some(RectStroke {
                    color: Color::WHITE,
                    width: 1.px(),
                    border_position: BorderPosition::Inside,
                }),
                fill: Some(RectFill {
                    color: Color::BLACK,
                }),
                round: None,
            },
            text: self.label_text.clone(),
            text_align: TextAlign::Left,
            text_baseline: TextBaseline::Middle,
            font_type: FontType {
                serif: false,
                size: 12.int_px(),
                language: Language::Ko,
                font_weight: FontWeight::REGULAR,
            },
            text_style: TextStyle {
                border: None,
                drop_shadow: None,
                color: Color::WHITE,
                background: None,
            },
            event_handler: Some(text_input::EventHandler::new().on_key_down(|event| {
                if event.code == Code::Enter && !event.is_composing {
                    namui::event::send(InternalEvent::LabelInputEnterPressed);
                    event.prevent_default();
                }
            })),
        })
    }
}
