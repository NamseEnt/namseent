use namui::*;

pub fn main() {
    namui::start(|ctx| {
        ctx.add(TypographyEffectExample);
    })
}

struct TypographyEffectExample;

impl Component for TypographyEffectExample {
    fn render(self, ctx: &RenderCtx) {
        let screen_wh = namui::screen::size();
        let (tile_mode, set_tile_mode) = ctx.state(|| TileMode::Clamp);

        ctx.on_raw_event(|event| {
            if let RawEvent::KeyUp { event } = event {
                if event.code == Code::ArrowRight {
                    set_tile_mode.set(match *tile_mode {
                        TileMode::Clamp => TileMode::Repeat,
                        TileMode::Decal => TileMode::Clamp,
                        TileMode::Mirror => TileMode::Decal,
                        TileMode::Repeat => TileMode::Mirror,
                    })
                } else if event.code == Code::ArrowLeft {
                    set_tile_mode.set(match *tile_mode {
                        TileMode::Clamp => TileMode::Decal,
                        TileMode::Decal => TileMode::Mirror,
                        TileMode::Mirror => TileMode::Repeat,
                        TileMode::Repeat => TileMode::Clamp,
                    })
                }
            }
        });

        ctx.add(namui_prebuilt::typography::effect::glow(
            "Hello world!",
            Font {
                size: 64.int_px(),
                name: "NotoSansKR-Regular".to_string(),
            },
            screen_wh.as_xy().into_type::<Px>() / 4.0,
            Paint::new(Color::WHITE),
            TextAlign::Center,
            TextBaseline::Middle,
            BlurStyle::Normal,
            blur_sigma::from_radius(24.0),
            24.px(),
            Color::from_u8(255, 128, 0, 255),
        ));

        ctx.add({
            let xy = screen_wh.as_xy().into_type::<Px>() * 3.0 / 4.0;
            let font = Font {
                size: 64.int_px(),
                name: "NotoSansKR-Regular".to_string(),
            };
            let font_metrics = namui::font::font_metrics(&font).unwrap();
            let height = font_metrics.height();

            let text = format!("Gradient_p - {:?}", *tile_mode);

            TextDrawCommand {
                text,
                font,
                x: xy.x,
                y: xy.y,
                paint: Paint::new(Color::WHITE).set_shader(Shader::LinearGradient {
                    // Up/down 20.px margin
                    start_xy: Xy::new(xy.x, xy.y - height / 2.0 + 20.px()),
                    end_xy: Xy::new(xy.x, xy.y + height / 2.0 - 20.px()),
                    colors: vec![Color::RED, Color::BLUE],
                    tile_mode: *tile_mode,
                }),
                align: TextAlign::Center,
                baseline: TextBaseline::Middle,
                max_width: None,
                line_height_percent: 100.percent(),
                underline: None,
            }
        });

        ctx.add({
            let x = screen_wh.width.into_px() / 8.0;
            let y = screen_wh.height.into_px() / 8.0;
            rect(RectParam {
                rect: Rect::Xywh {
                    x,
                    y,
                    width: screen_wh.width.into_px() - x * 2.0,
                    height: screen_wh.height.into_px() - y * 2.0,
                },
                style: RectStyle {
                    stroke: Some(RectStroke {
                        color: Color::BLACK,
                        width: 1.px(),
                        border_position: BorderPosition::Inside,
                    }),
                    fill: Some(RectFill {
                        color: Color::BLACK,
                    }),
                    round: None,
                },
            })
        });
    }
}
