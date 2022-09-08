use super::*;
use namui_prebuilt::{button::text_button, *};

impl PropertyEditor {
    pub fn render_layer_list(
        &self,
        props: &Props,
        wh: Wh<Px>,
        image_clip_id: &str,
        layer_list_view: &list_view::ListView,
    ) -> RenderingTree {
        let image_clip = props
            .cut
            .image_clips
            .iter()
            .find(|image_clip| image_clip.id() == image_clip_id)
            .unwrap();
        render([
            simple_rect(wh, Color::WHITE, 1.px(), Color::TRANSPARENT),
            table::vertical([
                table::fixed(
                    20.px(),
                    table::horizontal([
                        table::ratio(1.0, |wh| {
                            typography::body::left(wh.height, "Layers", Color::WHITE)
                        }),
                        table::calculative(
                            |wh| wh.height,
                            |wh| {
                                text_button(
                                    Rect::from_xy_wh(Xy::zero(), wh),
                                    "+",
                                    Color::WHITE,
                                    Color::WHITE,
                                    1.px(),
                                    Color::BLACK,
                                    {
                                        let selected_sequence_id =
                                            props.selected_sequence_id.to_string();
                                        let selected_cut_id = props.selected_cut_id.to_string();
                                        let selected_image_clip_id = image_clip_id.to_string();
                                        move || {
                                            namui::event::send(Event::LayerListPlusButtonClicked {
                                                image_clip_address: ImageClipAddress {
                                                    sequence_id: selected_sequence_id.clone(),
                                                    cut_id: selected_cut_id.clone(),
                                                    image_clip_id: selected_image_clip_id.clone(),
                                                },
                                            })
                                        }
                                    },
                                )
                            },
                        ),
                    ]),
                ),
                table::ratio(1.0, |wh| {
                    render([
                        simple_rect(wh, Color::WHITE, 1.px(), Color::TRANSPARENT),
                        layer_list_view.render(list_view::Props {
                            xy: Xy::zero(),
                            height: wh.height,
                            scroll_bar_width: 10.px(),
                            item_wh: Wh::new(wh.width, 30.px()),
                            items: image_clip.images.iter().enumerate(),
                            item_render: |wh, (index, image)| {
                                let is_selected = props.selected_layer_index == Some(index);
                                let stroke_color = if is_selected {
                                    Color::BLACK
                                } else {
                                    Color::WHITE
                                };
                                let fill_color = if is_selected {
                                    Color::WHITE
                                } else {
                                    Color::BLACK
                                };
                                render([
                                    simple_rect(wh, stroke_color, 1.px(), fill_color),
                                    typography::body::left(
                                        wh.height,
                                        format!("layer {index}"),
                                        stroke_color,
                                    ),
                                ])
                                .attach_event(move |builder| {
                                    builder.on_mouse_down_in(move |_event| {
                                        namui::event::send(
                                            crate::pages::sequence_edit_page::Event::LayerClicked {
                                                layer_index: index,
                                            },
                                        )
                                    });
                                })
                            },
                        }),
                    ])
                }),
            ])(wh),
        ])
    }
}
