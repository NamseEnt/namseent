mod rename_modal;

use self::rename_modal::RenameModal;
use crate::components::context_menu::{use_context_menu, Item};
use namui::prelude::*;
use namui_prebuilt::*;
use rpc::list_project_sequences::SequenceNameAndId;
use std::sync::Arc;

#[namui::component]
pub struct SequenceListPage {
    pub wh: Wh<Px>,
    pub project_id: namui::Uuid,
}

impl Component for SequenceListPage {
    fn render<'a>(&'a self, ctx: &'a RenderCtx) -> RenderDone {
        let &Self { wh, project_id } = self;
        let (error_message, set_error_message) = ctx.use_state::<Option<String>>(|| None);
        let (is_loading, set_is_loading) = ctx.use_state(|| true);
        let (sequence_list, set_sequence_list) = ctx.use_state::<Vec<SequenceNameAndId>>(|| vec![]);
        let (context_menu, open_context_menu) = use_context_menu();
        let (rename_modal, set_rename_modal) = ctx.use_state(|| None);

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

        ctx.use_effect("Fetch list on mount", || {
            start_fetch_list();
        });

        enum Event {
            OnRightClickSequenceCell {
                mouse_event: MouseEvent,
                sequence_id: Uuid,
            },
            OnRenameDone {
                sequence_id: Uuid,
                new_name: String,
            },
            CloseRenameModal,
        }

        ctx.use_children(
            |event| match event {
                Event::OnRightClickSequenceCell {
                    ref mouse_event,
                    sequence_id,
                } => {
                    open_context_menu.set(Some((
                        mouse_event.global_xy,
                        [
                            Item::new_button(
                                "Delete",
                                Callback::once(move || {
                                    spawn_local(async move {
                                        match crate::RPC
                                            .delete_sequence(rpc::delete_sequence::Request {
                                                sequence_id,
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
                                    });
                                }),
                            ),
                            Item::new_button(
                                "Rename",
                                Callback::once(move || {
                                    let sequence_name = sequence_list
                                        .iter()
                                        .find(|x| x.id == sequence_id)
                                        .map(|x| x.name.clone());

                                    if let Some(sequence_name) = sequence_name {
                                        set_rename_modal.set(Some((sequence_id, sequence_name)));
                                    }
                                }),
                            ),
                        ]
                        .to_vec(),
                    )));
                }
                Event::OnRenameDone {
                    sequence_id,
                    new_name,
                } => {
                    namui::log!("new_name: {:?}", new_name);
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
                Event::CloseRenameModal => {
                    set_rename_modal.set(None);
                }
            },
            |ctx| {
                if let Some(error_message) = &*error_message {
                    return ctx.add(typography::body::center(wh, error_message, Color::RED));
                }

                if *is_loading {
                    return ctx.add(typography::body::center(wh, "loading...123", Color::WHITE));
                }

                ctx.add(table::hooks::horizontal([
                    table::hooks::ratio(1.0, |_wh| RenderingTree::Empty),
                    table::hooks::ratio(
                        2.0,
                        table::hooks::vertical([
                            table::hooks::fixed(40.px(), |wh| {
                                namui_prebuilt::button::text_button(
                                    Rect::from_xy_wh(Xy::single(0.px()), wh),
                                    "[+] Add Sequence",
                                    Color::WHITE,
                                    Color::grayscale_f01(0.5),
                                    1.px(),
                                    Color::BLACK,
                                    [MouseButton::Left],
                                    move |_| on_add_button_click(),
                                )
                            }),
                            table::hooks::ratio(1.0, |wh| {
                                let item_wh = Wh::new(wh.width, 40.px());

                                list_view::ListView2 {
                                    xy: Xy::single(0.px()),
                                    height: wh.height,
                                    scroll_bar_width: 10.px(),
                                    item_wh,
                                    items: sequence_list
                                        .iter()
                                        .map(|sequence| {
                                            Arc::new(SequenceCell {
                                                wh: item_wh,
                                                project_id,
                                                sequence: sequence.clone(),
                                                on_right_click: ctx.event_with_param({
                                                    let sequence_id = sequence.id;
                                                    move |mouse_event| {
                                                        Some(Event::OnRightClickSequenceCell {
                                                            mouse_event,
                                                            sequence_id,
                                                        })
                                                    }
                                                }),
                                            })
                                                as Arc<dyn Component>
                                        })
                                        .collect(),
                                }
                            }),
                        ]),
                    ),
                    table::hooks::ratio(1.0, |_wh| RenderingTree::Empty),
                ])(wh));

                if let Some(context_menu) = context_menu {
                    ctx.add(context_menu);
                }

                namui::log!("rename_modal: {:?}", rename_modal);
                if let Some((sequence_id, ref initial_sequence_name)) = *rename_modal {
                    namui::log!("render rename modal");
                    ctx.add(RenameModal {
                        init_sequence_name: initial_sequence_name.clone(),
                        on_rename_done: ctx.event_with_param(move |new_name| {
                            Some(Event::OnRenameDone {
                                sequence_id,
                                new_name,
                            })
                        }),
                        close_modal: ctx.event(Event::CloseRenameModal),
                    });
                }
            },
        )
    }
}

#[namui::component]
pub struct SequenceCell {
    wh: Wh<Px>,
    project_id: Uuid,
    sequence: SequenceNameAndId,
    on_right_click: CallbackWithParam<MouseEvent>,
}

impl Component for SequenceCell {
    fn render<'a>(&'a self, ctx: &'a RenderCtx) -> RenderDone {
        let &Self {
            wh,
            project_id,
            ref sequence,
            ref on_right_click,
        } = self;
        let sequence_id = sequence.id;
        let on_right_click = on_right_click.clone();

        ctx.use_children(|ctx| {
            ctx.add(namui_prebuilt::button::text_button(
                Rect::from_xy_wh(Xy::single(0.px()), wh),
                sequence.name.as_str(),
                Color::WHITE,
                Color::grayscale_f01(0.3),
                1.px(),
                Color::BLACK,
                [MouseButton::Left, MouseButton::Right],
                move |event: MouseEvent| {
                    if event.button == Some(MouseButton::Left) {
                        super::router::move_to(super::router::RoutePath::SequenceEdit {
                            project_id,
                            sequence_id,
                        });
                    } else if event.button == Some(MouseButton::Right) {
                        on_right_click.call(event);
                    }
                },
            ));
        })
    }
}
