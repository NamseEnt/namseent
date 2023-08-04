use namui::prelude::*;


#[namui::component]
pub struct RenameModal<'a> {
    pub init_sequence_name: String,
    pub on_rename_done: &'a (dyn 'a + Fn(String)),
    pub close_modal: &'a (dyn 'a + Fn()),
}

impl Component for RenameModal<'_> {
    fn render<'a>(&'a self, ctx: &'a RenderCtx) -> RenderDone {
        let Self {
            init_sequence_name,
            on_rename_done,
            close_modal,
        } = self;
        todo!()
        // let (text_input, _) = ctx.state(|| TextInput::new());
        // let (sequence_name, set_sequence_name) = ctx.state(|| init_sequence_name.clone());

        // let close_modal = close_modal.clone();
        // let on_rename_done = on_rename_done.clone();

        // let screen_wh = namui::screen::size();
        // let modal_wh = screen_wh * 0.5;
        // let modal_xy = ((screen_wh - modal_wh) * 0.5).as_xy();
        // let text_input_rect_in_modal = Rect::Xywh {
        //     x: modal_wh.width / 4.0,
        //     y: modal_wh.height / 4.0,
        //     width: modal_wh.width / 2.0,
        //     height: 20.px(),
        // };
        // let enter_button_rect_in_modal = Rect::Xywh {
        //     x: text_input_rect_in_modal.x() + text_input_rect_in_modal.width() + 10.px(),
        //     y: text_input_rect_in_modal.y(),
        //     width: 40.px(),
        //     height: 20.px(),
        // };

        //
        //     let sequence_name = sequence_name.to_string();
        //     ctx.add(
        //         absolute(
        //             0.px(),
        //             0.px(),
        //             render([
        //                 simple_rect(
        //                     screen_wh,
        //                     Color::TRANSPARENT,
        //                     0.px(),
        //                     Color::from_f01(0.8, 0.8, 0.8, 0.8),
        //                 ),
        //                 translate(
        //                     modal_xy.x,
        //                     modal_xy.y,
        //                     render([
        //                         simple_rect(
        //                             modal_wh,
        //                             Color::WHITE,
        //                             1.px(),
        //                             Color::grayscale_f01(0.5),
        //                         ),
        //                         namui_prebuilt::typography::body::center(
        //                             Wh::new(modal_wh.width, modal_wh.height / 3.0),
        //                             "Rename Sequence",
        //                             Color::WHITE,
        //                         ),
        //                         text_input.render(text_input::Props {
        //                             rect: text_input_rect_in_modal,
        //                             text: sequence_name.to_string(),
        //                             text_align: TextAlign::Left,
        //                             text_baseline: TextBaseline::Top,
        //                             font_type: FontType {
        //                                 serif: false,
        //                                 size: 12.int_px(),
        //                                 language: Language::Ko,
        //                                 font_weight: FontWeight::REGULAR,
        //                             },
        //                             style: text_input::Style {
        //                                 rect: RectStyle {
        //                                     stroke: Some(RectStroke {
        //                                         color: Color::BLACK,
        //                                         width: 1.px(),
        //                                         border_position: BorderPosition::Outside,
        //                                     }),
        //                                     fill: Some(RectFill {
        //                                         color: Color::WHITE,
        //                                     }),
        //                                     ..Default::default()
        //                                 },
        //                                 text: TextStyle {
        //                                     color: Color::BLACK,
        //                                     ..Default::default()
        //                                 },
        //                                 ..Default::default()
        //                             },
        //                             event_handler: Some(
        //                                 text_input::EventHandler::new()
        //                                     .on_key_down({
        //                                         let on_rename_done = on_rename_done.clone();
        //                                         let sequence_name = sequence_name.clone();

        //                                         move |event: KeyDownEvent| {
        //                                             if event.code == Code::Enter {
        //                                                 on_rename_done
        //                                                     .call(sequence_name.to_string());
        //                                             }
        //                                         }
        //                                     })
        //                                     .on_text_updated(move |text| {
        //                                         set_sequence_name.set(text.clone());
        //                                     }),
        //                             ),
        //                         }),
        //                         namui_prebuilt::button::text_button(
        //                             enter_button_rect_in_modal,
        //                             "Save",
        //                             Color::WHITE,
        //                             Color::WHITE,
        //                             1.px(),
        //                             Color::BLACK,
        //                             [MouseButton::Left],
        //                             {
        //                                 let on_rename_done = on_rename_done.clone();
        //                                 let sequence_name = sequence_name.clone();

        //                                 move |_| {
        //                                     on_rename_done.call(sequence_name.to_string());
        //                                 }
        //                             },
        //                         ),
        //                     ]),
        //                 ),
        //             ]),
        //         )
        //         .attach_event(|builder| {
        //             builder
        //                 .on_mouse_down_in(|event: MouseEvent| event.stop_propagation())
        //                 .on_mouse_down_out(move |event: MouseEvent| {
        //                     close_modal.call();
        //                     event.stop_propagation();
        //                 })
        //                 .on_mouse_up_in(|event: MouseEvent| event.stop_propagation())
        //                 .on_mouse_up_out(|event: MouseEvent| event.stop_propagation());
        //         }),
        //     )
        // })
    }
}
