use crate::{color, pages::sequence_edit_page::atom::NAME_QUICK_SLOT};
use namui::{prelude::*, text_input::Style};
use namui_prebuilt::{button::TextButton, simple_rect, table::*, typography};

const MODAL_MAX_WH: Wh<Px> = Wh {
    width: px(512.0),
    height: px(227.0),
};
const TITLE_HEIGHT: Px = px(32.0);
const ITEM_HEIGHT: Px = px(39.0);
const ITEM_PADDING: Px = px(8.0);
const NAME_LABEL_WIDTH: Px = px(64.0);
const TEXT_INPUT_PADDING: Ltrb<Px> = Ltrb {
    left: px(4.0),
    top: px(4.0),
    right: px(4.0),
    bottom: px(4.0),
};

pub enum Event {
    Close,
}

pub struct NameQuickSlotModal<'a> {
    pub wh: Wh<Px>,
    pub on_event: &'a dyn Fn(Event),
}
impl Component for NameQuickSlotModal<'_> {
    fn render(self, ctx: &RenderCtx)  {
        let Self { wh, on_event } = self;

        let modal_wh = Wh {
            width: wh.width.min(MODAL_MAX_WH.width),
            height: wh.height.min(MODAL_MAX_WH.height),
        };
        let modal_xy = ((wh - modal_wh) / 2.0).as_xy();
        let (name_quick_slot, set_name_quick_slot) = ctx.atom(&NAME_QUICK_SLOT);
        let text_input_instances = [
            namui::text_input::TextInputInstance::new(ctx),
            namui::text_input::TextInputInstance::new(ctx),
            namui::text_input::TextInputInstance::new(ctx),
            namui::text_input::TextInputInstance::new(ctx),
            namui::text_input::TextInputInstance::new(ctx),
        ];

        let title_cell = fixed(TITLE_HEIGHT, |wh, ctx| {
            horizontal([
                ratio(1, |_, _| {}),
                fixed(wh.height, |wh, ctx| {
                    ctx.add(TextButton {
                        rect: wh.to_rect(),
                        text: "X",
                        text_color: color::STROKE_NORMAL,
                        stroke_color: color::STROKE_NORMAL,
                        stroke_width: 1.px(),
                        fill_color: Color::TRANSPARENT,
                        mouse_buttons: vec![MouseButton::Left],
                        on_mouse_up_in: &|_| {
                            on_event(Event::Close);
                        },
                    });
                }),
            ])(wh, ctx);
            ctx.add(typography::title::center(
                wh,
                "Name Quick Slot",
                color::STROKE_NORMAL,
            ));
            ctx.add(simple_rect(
                wh,
                color::STROKE_NORMAL,
                1.px(),
                Color::TRANSPARENT,
            ));
        });

        let body_cell = ratio(
            1,
            vertical(
                text_input_instances
                    .into_iter()
                    .enumerate()
                    .map(|(index, instance)| {
                        let name = name_quick_slot.get_name(index).cloned().unwrap_or_default();
                        fixed(
                            ITEM_HEIGHT,
                            padding(
                                ITEM_PADDING,
                                horizontal([
                                    fixed(
                                        NAME_LABEL_WIDTH,
                                        horizontal_padding(ITEM_PADDING, move |wh, ctx| {
                                            ctx.add(typography::body::right(
                                                wh,
                                                format!("Ctrl+{}", index + 1),
                                                color::STROKE_NORMAL,
                                            ));
                                        }),
                                    ),
                                    ratio(1, move |wh, ctx| {
                                        ctx.add(
                                            TextInput {
                                                instance,
                                                rect: wh.to_rect(),
                                                text: name.clone(),
                                                text_align: TextAlign::Left,
                                                text_baseline: TextBaseline::Top,
                                                font: Font {
                                                    size: 12.int_px(),
                                                    name: "NotoSansKR-Regular".to_string(),
                                                },
                                                style: Style {
                                                    rect: RectStyle {
                                                        stroke: Some(RectStroke {
                                                            color: color::STROKE_NORMAL,
                                                            width: 1.px(),
                                                            border_position: BorderPosition::Inside,
                                                        }),
                                                        fill: None,
                                                        round: None,
                                                    },
                                                    text: TextStyle {
                                                        border: None,
                                                        drop_shadow: None,
                                                        color: color::STROKE_NORMAL,
                                                        background: None,
                                                        line_height_percent: 100.percent(),
                                                        underline: None,
                                                    },
                                                    padding: TEXT_INPUT_PADDING,
                                                },
                                                prevent_default_codes: Vec::new(),
                                                on_event: &|event| {
                                                    let text_input::Event::TextUpdated { text } =
                                                        event
                                                    else {
                                                        return;
                                                    };
                                                    let name = text.to_string();
                                                    set_name_quick_slot.mutate(
                                                        move |name_quick_slot| {
                                                            name_quick_slot.set_name(index, name);
                                                        },
                                                    );
                                                },
                                            }
                                            .attach_event(|event| {
                                                if !instance.focused() {
                                                    return;
                                                }
                                                let namui::Event::MouseDown { event } = event
                                                else {
                                                    return;
                                                };
                                                if event.is_local_xy_in() {
                                                    return;
                                                }
                                                instance.blur();
                                            }),
                                        );
                                    }),
                                ]),
                            ),
                        )
                    }),
            ),
        );

        ctx.compose(|ctx| {
            let mut ctx = ctx.translate(modal_xy);
            vertical([title_cell, body_cell])(modal_wh, &mut ctx);

            ctx.add(simple_rect(
                modal_wh,
                color::STROKE_NORMAL,
                1.px(),
                color::BACKGROUND,
            ))
            .attach_event(|event| {
                let namui::Event::MouseDown { event } = event else {
                    return;
                };
                if !event.is_local_xy_in() {
                    return;
                }
                event.stop_propagation();
            });
        });

        ctx.component(
            simple_rect(wh, Color::TRANSPARENT, 0.px(), Color::from_u8(0, 0, 0, 128)).attach_event(
                |event| {
                    let namui::Event::MouseDown { event } = event else {
                        return;
                    };
                    event.stop_propagation();
                    on_event(Event::Close)
                },
            ),
        );

        
    }
}
