use super::*;
use crate::storage::{Cut, Sequence};
use namui_prebuilt::{button::text_button, *};

impl LoadedSequenceEditorPage {
    pub fn render_line_list(&self, wh: Wh<Px>, sequence: &Sequence) -> namui::RenderingTree {
        let line_list_label = table::fixed(20.px(), |wh| {
            typography::body::center(wh, format!("Line List"), Color::WHITE)
        });
        let line_list = table::ratio(1.0, |wh| {
            let item_wh = Wh::new(wh.width, 40.px());
            enum Item<'a> {
                Cut(&'a Cut),
                AddButton,
            }
            let items: Vec<Item> = sequence
                .cuts
                .iter()
                .map(|cut| Item::Cut(cut))
                .chain(std::iter::once(Item::AddButton))
                .collect();
            self.cut_list_view.render(list_view::Props {
                xy: Xy::zero(),
                height: wh.height,
                scroll_bar_width: 10.px(),
                item_wh,
                items,
                item_render: |wh, item| match item {
                    Item::Cut(cut) => {
                        let line_text_input = self.line_text_inputs.get(cut.id()).unwrap();

                        let is_selected = line_text_input.is_focused();

                        line_text_input.render(text_input::Props {
                            rect_param: RectParam {
                                rect: Rect::from_xy_wh(Xy::zero(), wh),
                                style: RectStyle {
                                    stroke: Some(match is_selected {
                                        true => RectStroke {
                                            color: Color::BLUE,
                                            width: 2.px(),
                                            border_position: BorderPosition::Inside,
                                        },
                                        false => RectStroke {
                                            color: Color::WHITE,
                                            width: 1.px(),
                                            border_position: BorderPosition::Inside,
                                        },
                                    }),
                                    fill: Some(RectFill {
                                        color: match is_selected {
                                            true => Color::WHITE,
                                            false => Color::BLACK,
                                        },
                                    }),
                                    round: None,
                                },
                            },
                            text_param: TextParam {
                                text: cut.line.clone(),
                                x: 10.px(),
                                y: wh.height / 2.0,
                                align: TextAlign::Left,
                                baseline: TextBaseline::Middle,
                                font_type: FontType {
                                    serif: false,
                                    size: 14.int_px(),
                                    language: Language::Ko,
                                    font_weight: FontWeight::REGULAR,
                                },
                                style: TextStyle {
                                    border: None,
                                    drop_shadow: None,
                                    color: match is_selected {
                                        true => Color::BLUE,
                                        false => Color::WHITE,
                                    },
                                    background: None,
                                },
                                max_width: None,
                            },
                        })
                    }
                    Item::AddButton => text_button(
                        Rect::from_xy_wh(Xy::zero(), wh),
                        "+ Add new cut",
                        Color::WHITE,
                        Color::WHITE,
                        1.px(),
                        Color::BLACK,
                        || namui::event::send(Event::AddCutClicked),
                    ),
                },
            })
        });

        render([
            simple_rect(wh, Color::WHITE, 1.px(), Color::TRANSPARENT),
            table::vertical([line_list_label, line_list])(wh),
        ])
    }
}
