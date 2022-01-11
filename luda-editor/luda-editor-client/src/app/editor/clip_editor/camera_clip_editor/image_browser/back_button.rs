use super::ImageBrowser;
use crate::app::editor::{
    clip_editor::camera_clip_editor::image_browser::ImageBrowserItem, events::EditorEvent,
};
use namui::prelude::*;

impl ImageBrowser {
    pub fn render_back_button(
        &self,
        item_size: Wh<f32>,
        thumbnail_rect: XywhRect<f32>,
    ) -> RenderingTree {
        let arrow_path = namui::PathBuilder::new()
            .move_to(0.0, 0.5)
            .line_to(0.5, 0.0)
            .line_to(0.5, 0.25)
            .line_to(1.0, 0.25)
            .line_to(1.0, 0.75)
            .line_to(0.5, 0.75)
            .line_to(0.5, 1.0)
            .line_to(0.0, 0.5)
            .scale(thumbnail_rect.width, thumbnail_rect.height)
            .translate(thumbnail_rect.x, thumbnail_rect.y);

        let arrow_paint = namui::PaintBuilder::new()
            .set_color(namui::Color::BLACK)
            .set_style(namui::PaintStyle::Stroke)
            .set_stroke_width(2.0);

        let is_selected = self.selected_item == Some(ImageBrowserItem::Back);

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
                builder.on_mouse_down(Box::new(move |_| {
                    namui::log(format!("select browser item {}", "back"));
                    namui::event::send(Box::new(EditorEvent::ImageBrowserSelectEvent {
                        selected_item: ImageBrowserItem::Back,
                    }));
                }))
            }),
            text(TextParam {
                x: item_size.width / 2.0,
                y: item_size.height - 20.0,
                text: "Back".to_string(),
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
            path(arrow_path, arrow_paint),
        ]
    }
}
