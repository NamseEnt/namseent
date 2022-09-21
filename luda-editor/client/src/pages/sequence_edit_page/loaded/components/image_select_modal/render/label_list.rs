use super::*;
use std::collections::BTreeMap;

pub struct Props {
    pub wh: Wh<Px>,
}

impl ImageSelectModal {
    pub fn render_label_list(&self, props: Props) -> namui::RenderingTree {
        let title = translate(
            12.px(),
            12.px(),
            typography::title::left_top("Select Labels", Color::WHITE),
        );

        let labels: Vec<Label> = vec![
            // TODO
            Label {
                key: "캐릭터".to_string(),
                value: "오하연".to_string(),
            },
            Label {
                key: "캐릭터".to_string(),
                value: "김혜찐".to_string(),
            },
            Label {
                key: "캐릭터".to_string(),
                value: "선임피디".to_string(),
            },
            Label {
                key: "의상".to_string(),
                value: "트레이닝복".to_string(),
            },
            Label {
                key: "의상".to_string(),
                value: "사복".to_string(),
            },
            Label {
                key: "의상".to_string(),
                value: "무대의상".to_string(),
            },
        ];

        let label_key_and_values: BTreeMap<String, Vec<String>> = {
            let mut map = BTreeMap::new();
            for label in labels {
                map.entry(label.key)
                    .or_insert_with(|| vec![])
                    .push(label.value);
            }
            map
        };

        let mut label_key_top = 0.px();

        let scroll_content = label_key_and_values.iter().map(|(key, values)| {
            let key_title = typography::body::left_top_bold(key, Color::WHITE);
            let mut value_horizontal_list = vec![];

            let value_buttons_with_bounding_box = values.iter().map(|value| {
                let text = namui::text(TextParam {
                    text: value.clone(),
                    x: 0.px(),
                    y: 0.px(),
                    align: TextAlign::Left,
                    baseline: TextBaseline::Top,
                    font_type: FontType {
                        serif: false,
                        size: typography::body::FONT_SIZE,
                        language: Language::Ko,
                        font_weight: FontWeight::BOLD,
                    },
                    style: TextStyle {
                        border: None,
                        drop_shadow: None,
                        color: Color::WHITE,
                        background: None,
                    },
                    max_width: None,
                });
                let bounding_box = text.get_bounding_box().unwrap_or_default();
                (
                    render([
                        simple_rect(
                            bounding_box.wh() + Wh::single(8.px()),
                            Color::WHITE,
                            1.px(),
                            Color::BLACK,
                        ),
                        translate(4.px(), 4.px(), text),
                    ]),
                    bounding_box,
                )
            });

            let mut last_right = 0.px();
            let title_button_spacing = 4.px();
            let mut top = typography::body::FONT_SIZE + title_button_spacing;
            let padding = 12.px();
            let button_height_with_padding = padding + typography::body::FONT_SIZE;

            for (value_button, bounding_box) in value_buttons_with_bounding_box {
                let next_left = {
                    let next_left = last_right + padding;
                    if next_left + bounding_box.width() > props.wh.width {
                        top += button_height_with_padding;
                        0.px()
                    } else {
                        next_left
                    }
                };

                value_horizontal_list.push(translate(next_left, top, value_button));

                last_right = next_left + bounding_box.width();
            }

            let cell = translate(
                12.px(),
                label_key_top,
                render([key_title, render(value_horizontal_list)]),
            );

            let bottom = top + button_height_with_padding;

            label_key_top += bottom + padding;

            cell
        });

        let title_height_with_padding = 24.px() + typography::title::FONT_SIZE;

        render([
            simple_rect(props.wh, Color::WHITE, 1.px(), Color::BLACK),
            title,
            self.label_scroll_view.render(&scroll_view::Props {
                xy: Xy::new(0.px(), title_height_with_padding),
                scroll_bar_width: 4.px(),
                height: props.wh.height - title_height_with_padding,
                content: render(scroll_content),
            }),
        ])
    }
}
