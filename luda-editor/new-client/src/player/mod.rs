use super::*;
use luda_rpc::*;
use psd_sprite_util::render_psd_sprite;
use router::Route;
use std::collections::HashMap;

pub struct Player<'a> {
    pub scenes: &'a [Scene],
    pub texts: &'a HashMap<String, HashMap<String, String>>,
    pub language_code: &'a str,
}

impl Component for Player<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            scenes,
            texts,
            language_code,
        } = self;

        let wh = namui::screen::size().map(|x| x.into_px());
        let (scene_index, set_scene_index) = ctx.state(|| 0);
        let scene = scenes.get(*scene_index);

        let exit = || {
            router::route(Route::Home {
                initial_selection: home::Selection::Nothing,
            });
        };
        let next = || {
            let next_scene_index = *scene_index + 1;
            if next_scene_index >= scenes.len() {
                exit();
                return;
            }
            set_scene_index.set(next_scene_index);
        };

        let text = scene
            .and_then(|scene| texts.get(&scene.id))
            .and_then(|texts| texts.get(language_code))
            .map(|text| text.as_str())
            .unwrap_or_default();
        ctx.add(SceneScreen {
            scene,
            text,
            screen_wh: wh,
        });

        ctx.on_raw_event(|event| match event {
            RawEvent::KeyDown { event } => match event.code {
                Code::ArrowRight => {
                    next();
                }
                Code::Escape => {
                    exit();
                }
                _ => {}
            },
            RawEvent::MouseDown { .. } => {
                next();
            }
            _ => {}
        });
    }
}

struct SceneScreen<'a> {
    scene: Option<&'a Scene>,
    text: &'a str,
    screen_wh: Wh<Px>,
}
impl Component for SceneScreen<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            scene,
            text,
            screen_wh,
        } = self;

        ctx.add(TextBox {
            wh: screen_wh,
            text,
        });

        if let Some(scene) = scene {
            for scene_sprite in scene.scene_sprites.iter() {
                render_psd_sprite(ctx, scene_sprite, screen_wh);
            }
        }

        ctx.add(simple_rect(
            screen_wh,
            Color::TRANSPARENT,
            1.px(),
            Color::BLACK,
        ));
    }
}

pub struct TextBox<'a> {
    pub wh: Wh<Px>,
    pub text: &'a str,
}
impl Component for TextBox<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self { wh, text } = self;

        ctx.compose(|ctx| {
            vertical([
                ratio(1, |_, _| {}),
                fixed(256.px(), |wh, ctx| {
                    let text_box_width = wh.width.min(720.px());
                    horizontal([
                        ratio(1, |_, _| {}),
                        fixed(text_box_width, |wh, ctx| {
                            ctx.compose(|ctx| {
                                vertical([
                                    fixed(32.px(), |wh, ctx| {
                                        ctx.add(typography::title::left(
                                            wh.height,
                                            "name",
                                            Color::WHITE,
                                        ));
                                    }),
                                    ratio(1, |wh, ctx| {
                                        ctx.add(TextDrawCommand {
                                            text: text.to_string(),
                                            font: Font {
                                                name: "NotoSansKR-Regular".to_string(),
                                                size: 24.int_px(),
                                            },
                                            x: 0.px(),
                                            y: 0.px(),
                                            paint: Paint::new(Color::WHITE),
                                            align: TextAlign::Left,
                                            baseline: TextBaseline::Top,
                                            max_width: Some(wh.width),
                                            line_height_percent: 120.percent(),
                                            underline: None,
                                        });
                                    }),
                                ])(wh, ctx);
                            });
                            ctx.add(simple_rect(
                                wh,
                                Color::TRANSPARENT,
                                0.px(),
                                Color::from_u8(0, 0, 0, 128),
                            ));
                        }),
                        ratio(1, |_, _| {}),
                    ])(wh, ctx);
                }),
            ])(wh, ctx);
        });
    }
}
