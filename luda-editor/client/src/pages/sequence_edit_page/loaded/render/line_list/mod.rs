mod character_cell;

use super::*;
use namui_prebuilt::{button::text_button, *};
use rpc::data::{Character, Cut, Sequence};

impl LoadedSequenceEditorPage {
    pub fn render_line_list(
        &self,
        wh: Wh<Px>,
        sequence: &Sequence,
        characters: &Vec<Character>,
    ) -> namui::RenderingTree {
        let line_list_label = table::fixed(20.px(), |wh| {
            typography::body::center(wh, format!("Line List"), Color::WHITE)
        });
        let line_list = table::ratio(1.0, |wh| {
            let item_wh = Wh::new(wh.width, 80.px());
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
                        let line_text_input = self.line_text_inputs.get(&cut.id()).unwrap();

                        let is_selected = line_text_input.is_focused();

                        table::horizontal([
                            table::calculative(
                                |wh| wh.height * 2.0 / 3.0,
                                |wh| self.character_cell(wh, cut, characters),
                            ),
                            table::ratio(1.0, |wh| {
                                line_text_input.render(text_input::Props {
                                    rect: Rect::from_xy_wh(Xy::zero(), wh),
                                    rect_style: RectStyle {
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
                                    text: cut.line.clone(),
                                    text_align: TextAlign::Left,
                                    text_baseline: TextBaseline::Middle,
                                    font_type: FontType {
                                        serif: false,
                                        size: 14.int_px(),
                                        language: Language::Ko,
                                        font_weight: FontWeight::REGULAR,
                                    },
                                    text_style: TextStyle {
                                        border: None,
                                        drop_shadow: None,
                                        color: match is_selected {
                                            true => Color::BLUE,
                                            false => Color::WHITE,
                                        },
                                        background: None,
                                    },
                                    event_handler: None,
                                })
                            }),
                        ])(wh)
                    }
                    Item::AddButton => text_button(
                        Rect::from_xy_wh(Xy::zero(), wh),
                        "+ Add New Cut",
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
