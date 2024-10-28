use namui::*;

pub struct TextEditor<'a> {
    pub wh: Wh<Px>,
    pub text: &'a String,
    pub scene_id: &'a String,
    pub on_edit_done: &'a dyn Fn(String, String),
}

impl Component for TextEditor<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            wh,
            text,
            scene_id,
            on_edit_done,
        } = self;

        let scene_id = ctx.track_eq(scene_id);

        let (last_updated, set_last_updated) = ctx.state(|| text.clone());
        let (delayed_update, set_delayed_update) = ctx.state::<Option<DelayedUpdate>>(|| None);

        let delayed_update = delayed_update.as_ref();
        let last_updated = last_updated.as_ref();
        ctx.effect("update previous scene text if needed", {
            || {
                scene_id.record_as_used();
                let Some(DelayedUpdate { scene_id, text, .. }) = delayed_update else {
                    return;
                };
                if last_updated == text {
                    return;
                }
                on_edit_done(scene_id.to_string(), text.to_string());
                set_delayed_update.set(None);
            }
        });

        ctx.effect("init last updated", {
            || {
                scene_id.record_as_used();
                set_last_updated.set(text.to_string());
            }
        });

        ctx.interval("text edit observer", 250.ms(), |_| {
            let Some(DelayedUpdate {
                updated_at,
                scene_id,
                text,
            }) = delayed_update
            else {
                return;
            };
            if time::now() - updated_at < 1.sec() {
                return;
            }
            on_edit_done(scene_id.to_string(), text.to_string());
            set_last_updated.set(text.to_string());
            set_delayed_update.set(None);
        });

        ctx.compose(|ctx| {
            ctx.add_with_key(
                scene_id.to_string(),
                text_input::TextInput {
                    rect: Rect::zero_wh(wh),
                    start_text: text.as_ref(),
                    text_align: TextAlign::Left,
                    text_baseline: TextBaseline::Top,
                    font: Font {
                        size: 16.int_px(),
                        name: "NotoSansKR-Regular".to_string(),
                    },
                    style: text_input::Style {
                        rect: RectStyle {
                            stroke: Some(RectStroke {
                                border_position: BorderPosition::Inside,
                                color: Color::from_u8(0xEE, 0xEE, 0xEE, 0xFF),
                                width: px(1.0),
                            }),
                            fill: Some(RectFill {
                                color: Color::from_u8(0x44, 0x44, 0x44, 0xFF),
                            }),
                            ..Default::default()
                        },
                        text: namui::TextStyle {
                            color: namui::Color::WHITE,
                            ..Default::default()
                        },
                        padding: Ltrb::all(8.px()),
                    },
                    prevent_default_codes: &[Code::Enter],
                    focus: None,
                    on_edit_done: &|text| {
                        set_delayed_update.set(Some(DelayedUpdate {
                            updated_at: time::now(),
                            scene_id: scene_id.to_string(),
                            text,
                        }));
                    },
                },
            );
        });
    }
}

struct DelayedUpdate {
    updated_at: Instant,
    scene_id: String,
    text: String,
}
