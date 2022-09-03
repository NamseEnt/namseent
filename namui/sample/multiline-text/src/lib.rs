use namui::prelude::*;

pub async fn main() {
    let namui_context = namui::init().await;

    namui::start(namui_context, &mut MultilineTextExample::new(), &()).await
}

struct MultilineTextExample {}

impl MultilineTextExample {
    fn new() -> Self {
        Self {}
    }
}

impl Entity for MultilineTextExample {
    type Props = ();

    fn render(&self, _props: &Self::Props) -> RenderingTree {
        let wh = namui::screen::size();
        let mut trees = vec![];
        for horizontal in 0..3 {
            for vertical in 0..3 {
                let x = wh.width / 2.0 - 400.px() + 400.px() * horizontal;
                let y = wh.height / 2.0 - 400.px() + 400.px() * vertical;
                let text_rendering_tree = namui::text(TextParam {
                    text: "Hello\nWorld!\nMyFriend~".to_string(),
                    x,
                    y,
                    align: match horizontal {
                        0 => TextAlign::Left,
                        1 => TextAlign::Center,
                        2 => TextAlign::Right,
                        _ => unreachable!(),
                    },
                    baseline: match vertical {
                        0 => TextBaseline::Top,
                        1 => TextBaseline::Middle,
                        2 => TextBaseline::Bottom,
                        _ => unreachable!(),
                    },
                    font_type: FontType {
                        size: 12.int_px(),
                        serif: false,
                        language: Language::Ko,
                        font_weight: FontWeight::REGULAR,
                    },
                    style: TextStyle {
                        border: None,
                        drop_shadow: None,
                        color: Color::BLACK,
                        background: None,
                    },
                });
                let paint_builder = PaintBuilder::new()
                    .set_color(Color::RED)
                    .set_style(PaintStyle::Stroke);
                trees.push(namui::path(
                    PathBuilder::new()
                        .move_to(x - 100.px(), y)
                        .line_to(x + 100.px(), y),
                    paint_builder.clone(),
                ));
                trees.push(namui::path(
                    PathBuilder::new()
                        .move_to(x, y - 100.px())
                        .line_to(x, y + 100.px()),
                    paint_builder.clone(),
                ));
                trees.push(namui::path(
                    PathBuilder::new().add_rect(text_rendering_tree.get_bounding_box().unwrap()),
                    paint_builder.clone(),
                ));
                trees.push(text_rendering_tree);
            }
        }
        for vertical in 0..3 {
            let x = wh.width / 2.0 - 800.px();
            let y = wh.height / 2.0 - 400.px() + 400.px() * vertical;
            let text_rendering_tree = namui::text(TextParam {
                text: "Ahoy".to_string(),
                x,
                y,
                align: TextAlign::Left,
                baseline: match vertical {
                    0 => TextBaseline::Top,
                    1 => TextBaseline::Middle,
                    2 => TextBaseline::Bottom,
                    _ => unreachable!(),
                },
                font_type: FontType {
                    size: 12.int_px(),
                    serif: false,
                    language: Language::Ko,
                    font_weight: FontWeight::REGULAR,
                },
                style: TextStyle {
                    border: None,
                    drop_shadow: None,
                    color: Color::BLACK,
                    background: None,
                },
            });
            let paint_builder = PaintBuilder::new()
                .set_color(Color::RED)
                .set_style(PaintStyle::Stroke);
            trees.push(namui::path(
                PathBuilder::new()
                    .move_to(x - 100.px(), y)
                    .line_to(x + 100.px(), y),
                paint_builder.clone(),
            ));
            trees.push(namui::path(
                PathBuilder::new()
                    .move_to(x, y - 100.px())
                    .line_to(x, y + 100.px()),
                paint_builder.clone(),
            ));
            trees.push(namui::path(
                PathBuilder::new().add_rect(text_rendering_tree.get_bounding_box().unwrap()),
                paint_builder.clone(),
            ));
            trees.push(text_rendering_tree);
        }
        render(trees)
    }

    fn update(&mut self, _event: &dyn std::any::Any) {}
}
