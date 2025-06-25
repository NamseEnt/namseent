use std::collections::HashMap;

use namui::*;
use namui_prebuilt::rich_text::*;

pub fn main() {
    namui::start(|ctx: &RenderCtx| {
        let Some(Ok(ref apple_image)) =
            *ctx.image(ResourceLocation::bundle("resources/apple.png"))
        else {
            return;
        };

        let bold_font = Font {
            size: 16.int_px(),
            name: "NotoSansKR-Bold".to_string(),
        };
        let bold_text_style = TextStyle {
            color: Color::BLACK,
            ..Default::default()
        };

        let max_width = 300.px();

        ctx.add(RichText {
            text: include_str!("./text.txt").to_string(),
            max_width: Some(max_width),
            default_font: Font {
                size: 16.int_px(),
                name: "NotoSansKR-Regular".to_string(),
            },
            default_text_style: TextStyle {
                color: Color::grayscale_f01(0.5),
                ..Default::default()
            },
            tag_map: &[
                (
                    "B".to_string(),
                    Tag::StyledText {
                        font: bold_font,
                        style: bold_text_style,
                    },
                ),
                (
                    "Apple".to_string(),
                    Tag::Image {
                        param: ImageParam {
                            rect: Rect::Xywh {
                                x: 0.px(),
                                y: 0.px(),
                                width: 16.px(),
                                height: 16.px(),
                            },
                            image: apple_image.clone(),
                            style: ImageStyle {
                                fit: ImageFit::Contain,
                                paint: None,
                            },
                        },
                    },
                ),
            ]
            .into_iter()
            .collect::<HashMap<_, _>>(),
            on_parse_error: Some(&|err| {
                println!("Parse error: {:?}", err);
            }),
        });

        ctx.add(path(
            Path::new()
                .move_to(max_width, 0.px())
                .line_to(max_width, 500.px()),
            Paint::new(Color::RED)
                .set_style(PaintStyle::Stroke)
                .set_stroke_width(1.px()),
        ));
    })
}
