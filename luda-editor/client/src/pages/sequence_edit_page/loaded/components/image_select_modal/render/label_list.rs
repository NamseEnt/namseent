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

        let labels = self.images.iter().map(|image| &image.labels).flatten();

        let images_contains_selected_labels = self
            .images
            .iter()
            .filter(|image| {
                self.selected_labels
                    .iter()
                    .any(|label| image.labels.contains(label))
            })
            .collect::<Vec<_>>();

        struct Item<'a> {
            label: &'a Label,
            is_in_selectable_images: bool,
        }

        let label_key_items = {
            let mut map = BTreeMap::new();
            for label in labels {
                map.entry(&label.key).or_insert_with(|| vec![]).push(Item {
                    label,
                    is_in_selectable_images: images_contains_selected_labels.is_empty()
                        || images_contains_selected_labels
                            .iter()
                            .any(|image| image.labels.contains(label)),
                });
            }
            map
        };

        let mut label_key_top = 0.px();

        let scroll_content = label_key_items.into_iter().map(|(key, mut items)| {
            let key_title = typography::body::left_top_bold(key, Color::WHITE);
            let mut value_horizontal_list = vec![];

            items.sort_by_key(|item| {
                !(images_contains_selected_labels.is_empty()
                    || images_contains_selected_labels
                        .iter()
                        .any(|image| image.labels.contains(item.label)))
            });

            let value_buttons_with_bounding_box = items.into_iter().map(|item| {
                let is_selected = self.selected_labels.contains(item.label);

                let stroke_color = if is_selected {
                    Color::BLACK
                } else if item.is_in_selectable_images {
                    Color::WHITE
                } else {
                    Color::grayscale_f01(0.5)
                };

                let fill_color = if is_selected {
                    Color::WHITE
                } else {
                    Color::BLACK
                };

                let text = namui::text(TextParam {
                    text: item.label.value.to_string(),
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
                        color: stroke_color,
                        background: None,
                    },
                    max_width: None,
                });
                let bounding_box = text.get_bounding_box().unwrap_or_default();
                (
                    render([
                        simple_rect(
                            bounding_box.wh() + Wh::single(8.px()),
                            stroke_color,
                            1.px(),
                            fill_color,
                        ),
                        translate(4.px(), 4.px(), text),
                    ])
                    .attach_event(move |builder| {
                        let label = item.label.clone();
                        if is_selected || item.is_in_selectable_images {
                            builder.on_mouse_down_in(move |_| {
                                namui::event::send(InternalEvent::ToggleLabel(label.clone()));
                            });
                        }
                    }),
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
