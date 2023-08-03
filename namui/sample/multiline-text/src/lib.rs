use namui::prelude::*;

pub fn main() {
    let namui_context = namui::init();

    namui::start(namui_context, &mut MultilineTextExample {})
}

#[namui::component]
struct MultilineTextExample {}

impl Component for MultilineTextExample {
    fn render<'a>(&'a self, ctx: &'a RenderCtx) -> RenderDone {
        let wh = namui::screen::size();
        let mut trees = vec![];

        for vertical in 0..3 {
            let x = 100.px();
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
                    size: 20.int_px(),
                    serif: false,
                    language: Language::Ko,
                    font_weight: FontWeight::REGULAR,
                },
                style: TextStyle {
                    color: Color::BLACK,
                    ..Default::default()
                },
                max_width: None,
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

        for horizontal in 0..3 {
            for vertical in 0..3 {
                let x = 200.px() + 200.px() * horizontal;
                let y = wh.height / 2.0 - 400.px() + 400.px() * vertical;
                let text_rendering_tree = namui::text(TextParam {
                    /// y and g is for descend test
                    text: "Helloy\nWorlg!\nMyFriend~".to_string(),
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
                        size: 20.int_px(),
                        serif: false,
                        language: Language::Ko,
                        font_weight: FontWeight::REGULAR,
                    },
                    style: TextStyle {
                        color: Color::BLACK,
                        ..Default::default()
                    },
                    max_width: None,
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

        for horizontal in 0..3 {
            for vertical in 0..3 {
                let x = 700.px() + 450.px() * horizontal;
                let y = wh.height / 2.0 - 400.px() + 400.px() * vertical;
                let text_rendering_tree = namui::text(TextParam {
                    text: "Welcome to the interactive WebAssembly demo!\n안녕하세요. 여기는 한글입니다. 반갑습니다!You can test the word-wrapping behavior by editing the text 😍✨".to_string(),
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
                        size: 20.int_px(),
                        serif: false,
                        language: Language::Ko,
                        font_weight: FontWeight::REGULAR,
                    },
                    style: TextStyle {
                        color: Color::BLACK,
                        ..Default::default()
                    },
                    max_width: Some(278.px()),
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

        ctx.add(render(trees));

        ctx.done()
    }
}
