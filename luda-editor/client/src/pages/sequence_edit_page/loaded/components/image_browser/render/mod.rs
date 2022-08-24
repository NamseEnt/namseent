use super::*;
use namui_prebuilt::{button::text_button, *};

impl ImageBrowser {
    pub fn render(&self, props: Props) -> namui::RenderingTree {
        render([
            simple_rect(props.wh, Color::WHITE, 1.px(), Color::TRANSPARENT),
            table::vertical([self.render_header(), self.render_body(&props)])(props.wh),
        ])
    }

    fn render_header(&self) -> table::TableCell {
        // TODO: Print upload/list request status
        table::fixed(
            20.px(),
            table::horizontal([
                table::ratio(1.0, |wh| typography::body::left(wh, "Title", Color::WHITE)),
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
                            || namui::event::send(Event::PlusButtonClicked),
                        )
                    },
                ),
            ]),
        )
    }

    fn render_body(&self, props: &Props) -> table::TableCell {
        let selected_resource = props.selected_resource.clone();
        let on_item_click = self.on_item_click.clone();
        table::ratio(1.0, move |wh| {
            self.list_view.render(list_view::Props {
                xy: Xy::zero(),
                height: wh.height,
                scroll_bar_width: 10.px(),
                item_wh: Wh::new(wh.width, wh.width),
                items: self.resources.iter(),
                item_render: |wh, resource| {
                    let is_selected = selected_resource.as_ref() == Some(resource);
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
                        typography::body::left(wh, resource, stroke_color),
                    ])
                    .attach_event(|builder| {
                        let resource = resource.clone();
                        let on_item_click = on_item_click.clone();
                        builder.on_mouse_down_in(move |_event| {
                            (on_item_click)(resource.as_str());
                        });
                    })
                },
            })
        })
    }
}
