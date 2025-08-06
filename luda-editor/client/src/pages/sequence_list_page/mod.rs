mod rename_modal;

use self::rename_modal::RenameModal;
use crate::components::context_menu::{if_context_menu_for, open_context_menu};
use namui::*;
use namui_prebuilt::*;
use rpc::list_project_sequences::SequenceNameAndId;

pub struct SequenceListPage {
    pub wh: Wh<Px>,
    pub project_id: namui::Uuid,
}

#[derive(Debug)]
enum ContextMenu {
    SequenceCell { sequence_id: Uuid },
}

impl Component for SequenceListPage {
    fn render(self, ctx: &RenderCtx) {
        let Self { wh, project_id } = self;

        const ITEM_HEIGHT: Px = px(40.0);

        let (error_message, set_error_message) = ctx.state::<Option<String>>(|| None);
        let (is_loading, set_is_loading) = ctx.state(|| true);
        let (sequence_list, set_sequence_list) =
            ctx.state::<Vec<SequenceNameAndId>>(std::vec::Vec::new);
        let (rename_modal, set_rename_modal) = ctx.state(|| None);

        let start_fetch_list = move || {
            set_is_loading.set(true);
            spawn_local(async move {
                match crate::RPC
                    .list_project_sequences(rpc::list_project_sequences::Request { project_id })
                    .await
                {
                    Ok(response) => {
                        set_sequence_list.set(response.sequence_name_and_ids);
                    }
                    Err(error) => {
                        set_error_message.set(Some(error.to_string()));
                    }
                }
                set_is_loading.set(false);
            })
        };

        let on_add_button_click = move || {
            set_is_loading.set(true);
            spawn_local(async move {
                let result = crate::RPC
                    .create_sequence(rpc::create_sequence::Request {
                        name: "new sequence".to_string(),
                        project_id,
                    })
                    .await;
                match result {
                    Ok(_) => {
                        start_fetch_list();
                    }
                    Err(error) => {
                        set_error_message.set(Some(error.to_string()));
                    }
                }
                set_is_loading.set(false);
            })
        };

        ctx.effect("Fetch list on mount", || {
            start_fetch_list();
        });

        enum InternalEvent {
            OnRenameDone { sequence_id: Uuid, new_name: String },
            CloseRenameModal,
        }

        let on_internal_event = move |event: InternalEvent| match event {
            InternalEvent::OnRenameDone {
                sequence_id,
                new_name,
            } => {
                set_rename_modal.set(None);

                set_is_loading.set(true);
                spawn_local(async move {
                    match crate::RPC
                        .update_sequence(rpc::update_sequence::Request {
                            sequence_id,
                            action: rpc::data::SequenceUpdateAction::RenameSequence {
                                name: new_name.clone(),
                            },
                        })
                        .await
                    {
                        Ok(_) => {
                            start_fetch_list();
                        }
                        Err(error) => {
                            set_error_message.set(Some(error.to_string()));
                        }
                    }
                    set_is_loading.set(false);
                });
            }
            InternalEvent::CloseRenameModal => {
                set_rename_modal.set(None);
            }
        };

        if let Some(error_message) = &*error_message {
            return ctx
                .component(typography::body::center(wh, error_message, Color::RED))
                .done();
        }

        if *is_loading {
            return ctx
                .component(typography::body::center(wh, "loading...123", Color::WHITE))
                .done();
        }

        ctx.compose(|ctx| {
            table::horizontal([
                table::ratio(1.0, |_wh, _ctx| {}),
                table::ratio(
                    2.0,
                    table::vertical([
                        table::fixed(ITEM_HEIGHT, |wh, ctx| {
                            ctx.add(button::TextButton {
                                rect: Rect::from_xy_wh(Xy::single(0.px()), wh),
                                text: "Manage Graphic Assets",
                                text_color: Color::WHITE,
                                stroke_color: Color::grayscale_f01(0.5),
                                stroke_width: 1.px(),
                                fill_color: Color::BLACK,
                                mouse_buttons: vec![MouseButton::Left],
                                on_mouse_up_in: &|_| {
                                    super::router::move_to(
                                        super::router::Route::GraphicAssetManage { project_id },
                                    );
                                },
                            });
                        }),
                        table::fixed(ITEM_HEIGHT, |wh, ctx| {
                            ctx.add(button::TextButton {
                                rect: Rect::from_xy_wh(Xy::single(0.px()), wh),
                                text: "Manage Project ACL",
                                text_color: Color::WHITE,
                                stroke_color: Color::grayscale_f01(0.5),
                                stroke_width: 1.px(),
                                fill_color: Color::BLACK,
                                mouse_buttons: vec![MouseButton::Left],
                                on_mouse_up_in: &|_| {
                                    super::router::move_to(super::router::Route::ProjectAclManage {
                                        project_id,
                                    })
                                },
                            });
                        }),
                        table::fixed(ITEM_HEIGHT, |_wh, _ctx| {}),
                        table::fixed(ITEM_HEIGHT, |wh, ctx| {
                            ctx.add(button::TextButton {
                                rect: Rect::from_xy_wh(Xy::single(0.px()), wh),
                                text: "[+] Add Sequence",
                                text_color: Color::WHITE,
                                stroke_color: Color::grayscale_f01(0.5),
                                stroke_width: 1.px(),
                                fill_color: Color::BLACK,
                                mouse_buttons: vec![MouseButton::Left],
                                on_mouse_up_in: &|_| on_add_button_click(),
                            });
                        }),
                        table::ratio(1.0, |wh, ctx| {
                            let item_wh = Wh::new(wh.width, ITEM_HEIGHT);

                            ctx.add(list_view::AutoListView {
                                height: wh.height,
                                scroll_bar_width: 10.px(),
                                item_wh,
                                items: sequence_list
                                    .iter()
                                    .map(|sequence| {
                                        (
                                            sequence.name.clone(),
                                            SequenceCell {
                                                wh: item_wh,
                                                project_id,
                                                sequence: sequence.clone(),
                                                on_right_click: Box::new({
                                                    let sequence_id = sequence.id;
                                                    move |mouse_event| {
                                                        open_context_menu(
                                                            mouse_event.global_xy,
                                                            ContextMenu::SequenceCell {
                                                                sequence_id,
                                                            },
                                                        )
                                                    }
                                                }),
                                            },
                                        )
                                    })
                                    .collect(),
                            });
                        }),
                    ]),
                ),
                table::ratio(1.0, |_wh, _ctx| {}),
            ])(wh, ctx)
        });

        if_context_menu_for::<ContextMenu>(|context_menu, builder| match context_menu {
            &ContextMenu::SequenceCell { sequence_id } => builder
                .add_button("Delete", move || {
                    spawn_local(async move {
                        match crate::RPC
                            .delete_sequence(rpc::delete_sequence::Request { sequence_id })
                            .await
                        {
                            Ok(_) => {
                                start_fetch_list();
                            }
                            Err(error) => {
                                set_error_message.set(Some(error.to_string()));
                            }
                        }
                    });
                })
                .add_button("Rename", move || {
                    let sequence_name = sequence_list
                        .iter()
                        .find(|x| x.id == sequence_id)
                        .map(|x| x.name.clone());

                    if let Some(sequence_name) = sequence_name {
                        set_rename_modal.set(Some((sequence_id, sequence_name)));
                    }
                }),
        });

        ctx.compose(|ctx| {
            if let Some((sequence_id, ref initial_sequence_name)) = *rename_modal {
                namui::log!("render rename modal");
                ctx.add(RenameModal {
                    init_sequence_name: initial_sequence_name.clone(),
                    on_rename_done: &|new_name| {
                        on_internal_event(InternalEvent::OnRenameDone {
                            sequence_id,
                            new_name,
                        })
                    },
                    close_modal: &|| on_internal_event(InternalEvent::CloseRenameModal),
                });
            }
        });
    }
}

pub struct SequenceCell<'a> {
    wh: Wh<Px>,
    project_id: Uuid,
    sequence: SequenceNameAndId,
    // on_right_click: callback!('a, MouseEvent),
    on_right_click: Box<dyn Fn(MouseEvent) + 'a>,
}

impl Component for SequenceCell<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            wh,
            project_id,
            sequence,
            on_right_click,
        } = self;
        let sequence_id = sequence.id;

        ctx.component(button::TextButton {
            rect: Rect::from_xy_wh(Xy::single(0.px()), wh),
            text: sequence.name.as_str(),
            text_color: Color::WHITE,
            stroke_color: Color::grayscale_f01(0.3),
            stroke_width: 1.px(),
            fill_color: Color::BLACK,
            mouse_buttons: vec![MouseButton::Left, MouseButton::Right],
            on_mouse_up_in: &|event: MouseEvent| {
                if event.button == Some(MouseButton::Left) {
                    super::router::move_to(super::router::Route::SequenceEdit {
                        project_id,
                        sequence_id,
                    });
                } else if event.button == Some(MouseButton::Right) {
                    on_right_click(event);
                }
            },
        })
        .done()
    }
}
