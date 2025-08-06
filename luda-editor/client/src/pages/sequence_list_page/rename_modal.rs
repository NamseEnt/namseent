use namui::*;
use namui_prebuilt::simple_rect;

pub struct RenameModal<'a> {
    pub init_sequence_name: String,
    pub on_rename_done: &'a (dyn 'a + Fn(String)),
    pub close_modal: &'a (dyn 'a + Fn()),
}

impl Component for RenameModal<'_> {
    fn render(self, ctx: &RenderCtx)  {
        let Self {
            init_sequence_name,
            on_rename_done,
            close_modal,
        } = self;
        let (sequence_name, set_sequence_name) = ctx.state(|| init_sequence_name.clone());
        let text_input_instance = namui::text_input::TextInputInstance::new(ctx);

        let screen_wh = namui::screen::size();
        let screen_wh = Wh::new(screen_wh.width.into_px(), screen_wh.height.into_px());
        let modal_wh = screen_wh * 0.5;
        let modal_xy = ((screen_wh - modal_wh) * 0.5).to_xy();
        let text_input_rect_in_modal = Rect::Xywh {
            x: modal_wh.width / 4.0,
            y: modal_wh.height / 4.0,
            width: modal_wh.width / 2.0,
            height: 20.px(),
        };
        let enter_button_rect_in_modal = Rect::Xywh {
            x: text_input_rect_in_modal.x() + text_input_rect_in_modal.width() + 10.px(),
            y: text_input_rect_in_modal.y(),
            width: 40.px(),
            height: 20.px(),
        };

        let sequence_name = sequence_name.to_string();
        ctx.compose(|ctx| {
            let mut ctx = ctx.absolute((0.px(), 0.px()));
            ctx.translate((modal_xy.x, modal_xy.y))
                .add(namui_prebuilt::button::TextButton {
                    rect: enter_button_rect_in_modal,
                    text: "Save",
                    text_color: Color::WHITE,
                    stroke_color: Color::WHITE,
                    stroke_width: 1.px(),
                    fill_color: Color::BLACK,
                    mouse_buttons: vec![MouseButton::Left],
                    on_mouse_up_in: &{
                        let sequence_name = sequence_name.clone();
                        move |_| {
                            let sequence_name = sequence_name.clone();
                            on_rename_done(sequence_name);
                        }
                    },
                })
                .add(TextInput {
                    instance: text_input_instance,
                    rect: text_input_rect_in_modal,
                    text: sequence_name.to_string(),
                    text_align: TextAlign::Left,
                    text_baseline: TextBaseline::Top,
                    font: Font {
                        size: 12.int_px(),
                        name: "NotoSansKR-Regular".to_string(),
                    },
                    style: text_input::Style {
                        rect: RectStyle {
                            stroke: Some(RectStroke {
                                color: Color::BLACK,
                                width: 1.px(),
                                border_position: BorderPosition::Outside,
                            }),
                            fill: Some(RectFill {
                                color: Color::WHITE,
                            }),
                            ..Default::default()
                        },
                        text: TextStyle {
                            color: Color::BLACK,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    prevent_default_codes: vec![],
                    on_event: &|event| match event {
                        text_input::Event::TextUpdated { text } => {
                            set_sequence_name.set(text.to_string());
                        }
                        text_input::Event::SelectionUpdated { selection: _ } => {}
                        text_input::Event::KeyDown { event } => {
                            let sequence_name = sequence_name.clone();
                            if event.code == Code::Enter {
                                on_rename_done(sequence_name);
                            }
                        }
                    },
                })
                .add(namui_prebuilt::typography::body::center(
                    Wh::new(modal_wh.width, modal_wh.height / 3.0),
                    "Rename Sequence",
                    Color::WHITE,
                ))
                .add(simple_rect(
                    modal_wh,
                    Color::WHITE,
                    1.px(),
                    Color::grayscale_f01(0.5),
                ));

            ctx.add(
                simple_rect(
                    screen_wh,
                    Color::TRANSPARENT,
                    0.px(),
                    Color::from_f01(0.8, 0.8, 0.8, 0.8),
                )
                .attach_event(|event| match event {
                    Event::MouseDown { event } => {
                        if !event.is_local_xy_in() {
                            close_modal();
                        }
                        event.stop_propagation();
                    }
                    Event::MouseUp { event } => event.stop_propagation(),
                    _ => {}
                }),
            );
        });
        
    }
}
