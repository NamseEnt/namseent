use super::*;

impl ImageBrowser {
    pub fn render_empty_button(
        &self,
        browser_id: &str,
        item_size: Wh<f32>,
        thumbnail_rect: XywhRect<f32>,
    ) -> RenderingTree {
        let x_path = namui::PathBuilder::new()
            .move_to(0.0, 0.0)
            .line_to(1.0, 1.0)
            .move_to(1.0, 0.0)
            .line_to(0.0, 1.0)
            .scale(thumbnail_rect.width, thumbnail_rect.height)
            .translate(thumbnail_rect.x, thumbnail_rect.y);

        let paint = namui::PaintBuilder::new()
            .set_color(namui::Color::BLACK)
            .set_style(namui::PaintStyle::Stroke)
            .set_stroke_width(2.0);

        let is_selected = self.selected_item == Some(ImageBrowserItem::Empty);

        render![
            rect(RectParam {
                x: 0.0,
                y: 0.0,
                width: item_size.width,
                height: item_size.height,
                style: RectStyle {
                    stroke: Some(RectStroke {
                        width: if is_selected { 3.0 } else { 1.0 },
                        border_position: BorderPosition::Inside,
                        color: if is_selected {
                            namui::Color::RED
                        } else {
                            namui::Color::BLACK
                        },
                    }),
                    round: Some(RectRound { radius: 5.0 }),
                    fill: Some(RectFill {
                        color: namui::Color::WHITE,
                    }),
                    ..Default::default()
                },
                ..Default::default()
            })
            .attach_event(|builder| {
                let browser_id = browser_id.to_string();
                builder.on_mouse_down(move |_| {
                    namui::event::send(ImageBrowserEvent::Select {
                        browser_id: browser_id.clone(),
                        item: ImageBrowserItem::Empty,
                    });
                });
            }),
            text(TextParam {
                x: item_size.width / 2.0,
                y: item_size.height - 20.0,
                text: "Empty".to_string(),
                align: TextAlign::Center,
                baseline: TextBaseline::Top,
                font_type: FontType {
                    font_weight: FontWeight::REGULAR,
                    language: Language::Ko,
                    serif: false,
                    size: 16,
                },
                style: TextStyle {
                    color: namui::Color::BLACK,
                    ..Default::default()
                },
            }),
            path(x_path, paint),
        ]
    }
}
