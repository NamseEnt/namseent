use std::collections::HashMap;

use namui::*;
use namui_prebuilt::rich_text::*;

pub fn main() {
    namui::start(|ctx: &RenderCtx| {
        let image_result = ctx.image(ResourceLocation::bundle("resources/apple.png"));
        let Some(Ok(ref apple_image)) = *image_result.lock().unwrap()
        else {
            return;
        };

        let demo_text = include_str!("text.txt");
        let demo_font = Font {
            size: 14.int_px(),
            name: "NotoSansKR-Regular".to_string(),
        };
        let demo_text_style = TextStyle {
            color: Color::BLACK,
            ..Default::default()
        };

        let cell_width = 320.px();
        let cell_height = 240.px();
        let grid_margin = 24.px();

        // Define the alignment combinations for 3x3 matrix
        let text_alignments = [TextAlign::Left, TextAlign::Center, TextAlign::Right];
        let vertical_alignments = [
            VerticalAlign::Top,
            VerticalAlign::Center,
            VerticalAlign::Bottom,
        ];

        // Create tag map for styled text
        let tag_map: HashMap<String, Tag> = [
            (
                "B".to_string(),
                Tag::StyledText {
                    font: Font {
                        size: 14.int_px(),
                        name: "NotoSansKR-Bold".to_string(),
                    },
                    style: TextStyle {
                        color: Color::from_u8(200, 0, 0, 255), // Dark red
                        ..Default::default()
                    },
                },
            ),
            (
                "Apple".to_string(),
                Tag::Image {
                    param: ImageParam {
                        rect: Rect::Xywh {
                            x: 0.px(),
                            y: 0.px(),
                            width: 12.px(),
                            height: 12.px(),
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
        .collect();

        // Create regex handlers for dynamic content
        let regex_handlers = [
            RegexHandler::new(
                r"icon<([^:>]+):(\d+):(\d+):(\d+):(\d+)>",
                Box::new(|_matched_text| {
                    // Render a 24px x 24px colored rectangle for icon
                    namui::rect(RectParam {
                        rect: Rect::Xywh {
                            x: 0.px(),
                            y: 0.px(),
                            width: 24.px(),
                            height: 24.px(),
                        },
                        style: RectStyle {
                            fill: Some(RectFill {
                                color: Color::from_u8(0, 150, 0, 255), // Green for icons
                            }),
                            stroke: Some(RectStroke {
                                color: Color::from_u8(0, 100, 0, 255), // Darker green border
                                width: 1.px(),
                                border_position: BorderPosition::Inside,
                            }),
                            ..Default::default()
                        },
                    })
                }),
            )
            .unwrap(),
            RegexHandler::new(
                r"@(\w+)",
                Box::new(|matched_text| {
                    namui::text(TextParam {
                        text: matched_text.to_string(),
                        x: 0.px(),
                        y: 0.px(),
                        align: TextAlign::Left,
                        baseline: TextBaseline::Top,
                        font: Font {
                            size: 14.int_px(),
                            name: "NotoSansKR-Bold".to_string(),
                        },
                        style: TextStyle {
                            color: Color::from_u8(0, 100, 200, 255), // Blue for mentions
                            ..Default::default()
                        },
                        max_width: None,
                    })
                }),
            )
            .unwrap(),
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
                            size: 12.int_px(),
                            name: "NotoSansKR-Regular".to_string(),
                        },
                        style: TextStyle {
                            color: Color::from_u8(100, 0, 200, 255), // Purple for URLs
                            ..Default::default()
                        },
                        max_width: None,
                    })
                }),
            )
            .unwrap(),
        ];

        // Draw 3x3 grid with different alignment combinations
        for (row, &vertical_align) in vertical_alignments.iter().enumerate() {
            for (col, &text_align) in text_alignments.iter().enumerate() {
                let x = grid_margin + (col as f32 * (cell_width + grid_margin).as_f32()).px();
                let y =
                    grid_margin * 4.0 + (row as f32 * (cell_height + grid_margin).as_f32()).px();

                ctx.compose(|ctx| {
                    ctx.translate((x + 10.px(), y + 10.px())).add(RichText {
                        text: demo_text.to_string(),
                        max_width: Some(cell_width - 20.px()), // Account for padding
                        default_font: demo_font.clone(),
                        default_text_style: demo_text_style.clone(),
                        default_text_align: text_align,
                        default_vertical_align: vertical_align,
                        tag_map: &tag_map,
                        regex_handlers: &regex_handlers,
                        on_parse_error: None,
                    });
                });

                // Add alignment labels
                ctx.add(text(TextParam {
                    text: format!("{text_align:?} / {vertical_align:?}"),
                    x: x + cell_width / 2.0,
                    y: y + cell_height - 15.px(),
                    align: TextAlign::Center,
                    baseline: TextBaseline::Middle,
                    font: Font {
                        size: 10.int_px(),
                        name: "NotoSansKR-Bold".to_string(),
                    },
                    style: TextStyle {
                        color: Color::from_u8(100, 100, 100, 255),
                        ..Default::default()
                    },
                    max_width: None,
                }));

                // Draw cell background
                ctx.add(rect(RectParam {
                    rect: Rect::Xywh {
                        x,
                        y,
                        width: cell_width,
                        height: cell_height,
                    },
                    style: RectStyle {
                        fill: Some(RectFill {
                            color: Color::from_u8(245, 245, 245, 255), // Light gray background
                        }),
                        stroke: Some(RectStroke {
                            color: Color::from_u8(200, 200, 200, 255),
                            width: 1.px(),
                            border_position: BorderPosition::Inside,
                        }),
                        ..Default::default()
                    },
                }));
            }
        }

        // Add column headers for text alignment
        for (col, &text_align) in text_alignments.iter().enumerate() {
            let x = grid_margin + (col as f32 * (cell_width + grid_margin).as_f32()).px();
            let y = grid_margin * 3.0;

            ctx.add(text(TextParam {
                text: format!("{text_align:?}"),
                x: x + cell_width / 2.0,
                y,
                align: TextAlign::Center,
                baseline: TextBaseline::Middle,
                font: Font {
                    size: 14.int_px(),
                    name: "NotoSansKR-Bold".to_string(),
                },
                style: TextStyle {
                    color: Color::from_u8(50, 50, 50, 255),
                    ..Default::default()
                },
                max_width: None,
            }));
        }

        // Add row headers for vertical alignment
        for (row, &vertical_align) in vertical_alignments.iter().enumerate() {
            let x = grid_margin / 2.0;
            let y = grid_margin * 4.0
                + (row as f32 * (cell_height + grid_margin).as_f32()).px()
                + cell_height / 2.0;

            ctx.add(text(TextParam {
                text: format!("{vertical_align:?}"),
                x,
                y,
                align: TextAlign::Center,
                baseline: TextBaseline::Middle,
                font: Font {
                    size: 14.int_px(),
                    name: "NotoSansKR-Bold".to_string(),
                },
                style: TextStyle {
                    color: Color::from_u8(50, 50, 50, 255),
                    ..Default::default()
                },
                max_width: None,
            }));
        }

        // Add title
        ctx.add(text(TextParam {
            text: "RichText TextAlign & VerticalAlign Demo".to_string(),
            x: (3.0 * (cell_width + grid_margin).as_f32() + grid_margin.as_f32()).px() / 2.0,
            y: grid_margin,
            align: TextAlign::Center,
            baseline: TextBaseline::Middle,
            font: Font {
                size: 18.int_px(),
                name: "NotoSansKR-Bold".to_string(),
            },
            style: TextStyle {
                color: Color::BLACK,
                ..Default::default()
            },
            max_width: None,
        }));
    })
}
