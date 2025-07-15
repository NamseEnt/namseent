use std::collections::HashMap;

use namui::*;
use namui_prebuilt::rich_text::*;

pub fn main() {
    namui::start(|ctx: &RenderCtx| {
        let Some(Ok(ref apple_image)) = *ctx.image(ResourceLocation::bundle("resources/apple.png"))
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

        // Create regex handlers for pattern matching
        let regex_handlers = vec![
            // Handle icon tags like icon<gold:24:32:32:1>
            RegexHandler::new(
                r"icon<[^>]+>",
                Box::new(|matched_text| {
                    // Create a simple colored rectangle as icon placeholder
                    let parts: Vec<&str> = matched_text.split(':').collect();
                    let icon_name = if parts.len() > 1 { parts[1] } else { "unknown" };

                    let color = match icon_name {
                        "gold" => Color::from_u8(255, 215, 0, 255), // Gold color
                        "attack_damage" => Color::RED,
                        "shield" => Color::BLUE,
                        "health" => Color::GREEN,
                        _ => Color::from_u8(128, 128, 128, 255), // Gray color
                    };

                    namui::rect(RectParam {
                        rect: Rect::Xywh {
                            x: 0.px(),
                            y: 0.px(),
                            width: 16.px(),
                            height: 16.px(),
                        },
                        style: RectStyle {
                            fill: Some(RectFill { color }),
                            stroke: Some(RectStroke {
                                color: Color::BLACK,
                                width: 1.px(),
                                border_position: BorderPosition::Inside,
                            }),
                            ..Default::default()
                        },
                    })
                }),
            )
            .unwrap(),
            // Handle @mentions
            RegexHandler::new(
                r"@\w+",
                Box::new(|matched_text| {
                    namui::text(TextParam {
                        text: matched_text.to_string(),
                        x: 0.px(),
                        y: 0.px(),
                        align: TextAlign::Left,
                        baseline: TextBaseline::Top,
                        font: Font {
                            size: 16.int_px(),
                            name: "NotoSansKR-Bold".to_string(),
                        },
                        style: TextStyle {
                            color: Color::BLUE,
                            ..Default::default()
                        },
                        max_width: None,
                    })
                }),
            )
            .unwrap(),
            // Handle URLs
            RegexHandler::new(
                r"https?://[^\s]+",
                Box::new(|matched_text| {
                    namui::text(TextParam {
                        text: matched_text.to_string(),
                        x: 0.px(),
                        y: 0.px(),
                        align: TextAlign::Left,
                        baseline: TextBaseline::Top,
                        font: Font {
                            size: 16.int_px(),
                            name: "NotoSansKR-Regular".to_string(),
                        },
                        style: TextStyle {
                            color: Color::from_u8(0, 0, 255, 255),
                            underline: Some(Paint::new(Color::from_u8(0, 0, 255, 255))),
                            ..Default::default()
                        },
                        max_width: None,
                    })
                }),
            )
            .unwrap(),
        ];

        ctx.add(RichText::with_regex_handlers(
            include_str!("./text.txt").to_string(),
            Some(max_width),
            Font {
                size: 16.int_px(),
                name: "NotoSansKR-Regular".to_string(),
            },
            TextStyle {
                color: Color::grayscale_f01(0.5),
                ..Default::default()
            },
            &[
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
            &regex_handlers,
        ));

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
