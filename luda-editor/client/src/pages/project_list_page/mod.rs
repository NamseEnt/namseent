use crate::{
    app::notification::{self, remove_notification},
    RPC,
};
use futures::FutureExt;
use namui::*;
use namui_prebuilt::*;
use rpc::list_editable_projects::EditableProject;

#[namui::component]
pub struct ProjectListPage {
    pub wh: Wh<Px>,
}

impl Component for ProjectListPage {
    fn render(self, ctx: &RenderCtx)  {
        let Self { wh } = self;
        const ITEM_HEIGHT: Px = px(40.0);
        let (error_message, set_error_message) = ctx.state::<Option<String>>(|| None);
        let (is_loading, set_is_loading) = ctx.state(|| true);
        let (project_list, set_project_list) =
            ctx.state::<Vec<EditableProject>>(std::vec::Vec::new);

        let start_fetch_list = move || {
            set_is_loading.set(true);
            spawn_local(async move {
                let response = crate::RPC
                    .list_editable_projects(rpc::list_editable_projects::Request {
                        start_after: None,
                    })
                    .await;

                set_is_loading.set(false);

                match response {
                    Ok(response) => {
                        set_project_list.set(response.projects);
                    }
                    Err(error) => {
                        set_error_message.set(Some(error.to_string()));
                    }
                }
            })
        };

        let on_add_button_clicked = move || {
            set_is_loading.set(true);
            spawn_local(async move {
                let response = crate::RPC
                    .create_project(rpc::create_project::Request {
                        name: "new project".to_string(),
                    })
                    .await;
                set_is_loading.set(false);
                match response {
                    Ok(_) => {
                        start_fetch_list();
                    }
                    Err(error) => {
                        set_error_message.set(Some(error.to_string()));
                    }
                }
            })
        };

        let on_copy_id_button_clicked = || {
            let loading_notification_id = notification::info!("Getting user id...")
                .set_loading(true)
                .push();
            spawn_local(
                async move {
                    let Ok(rpc::get_user_id::Response { user_id }) =
                        RPC.get_user_id(rpc::get_user_id::Request {}).await
                    else {
                        notification::error!("Failed to get user id").push();
                        return;
                    };

                    if let Err(error) = clipboard::write_text(user_id.to_string()).await {
                        notification::error!("Failed to copy: {error}");
                        notification::info!("user id: {user_id}").push();
                        return;
                    };

                    notification::info!("User id copied to clipboard").push();
                }
                .then(move |()| async move { remove_notification(loading_notification_id) }),
            );
        };

        ctx.effect("Fetch project list on mount", || {
            start_fetch_list();
        });

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
                            ctx.add(namui_prebuilt::button::TextButton {
                                rect: Rect::from_xy_wh(Xy::single(0.px()), wh),
                                text: "Copy User ID",
                                text_color: Color::WHITE,
                                stroke_color: Color::grayscale_f01(0.5),
                                stroke_width: 1.px(),
                                fill_color: Color::BLACK,
                                mouse_buttons: vec![MouseButton::Left],
                                on_mouse_up_in: &|_| on_copy_id_button_clicked(),
                            });
                        }),
                        table::fixed(ITEM_HEIGHT, |_, _| {}),
                        table::fixed(ITEM_HEIGHT, |wh, ctx| {
                            ctx.add(namui_prebuilt::button::TextButton {
                                rect: Rect::from_xy_wh(Xy::single(0.px()), wh),
                                text: "[+] Add Project",
                                text_color: Color::WHITE,
                                stroke_color: Color::grayscale_f01(0.5),
                                stroke_width: 1.px(),
                                fill_color: Color::BLACK,
                                mouse_buttons: vec![MouseButton::Left],
                                on_mouse_up_in: &|_| on_add_button_clicked(),
                            });
                        }),
                        table::ratio(1.0, |wh, ctx| {
                            let item_wh = Wh::new(wh.width, ITEM_HEIGHT);
                            ctx.add(list_view::AutoListView {
                                height: wh.height,
                                scroll_bar_width: 10.px(),
                                item_wh,
                                items: project_list
                                    .iter()
                                    .map(|project| {
                                        (
                                            project.id.to_string(),
                                            ProjectCell {
                                                wh: item_wh,
                                                project: project.clone(),
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

        // TODO: Clear name quick slot cache on project delete
        // TODO
        // self.context_menu
        //     .as_ref()
        //     .map_or(RenderingTree::Empty, |context_menu| {
        //         context_menu.render().attach_event(|builder| {
        //             builder
        //                 .on_mouse_down_in(|event: MouseEvent| event.stop_propagation())
        //                 .on_mouse_down_out(|event: MouseEvent| {
        //                     namui::event::send(Event::ContextMenuOutsideClicked);
        //                     event.stop_propagation()
        //                 })
        //                 .on_mouse_up_in(|event: MouseEvent| event.stop_propagation())
        //                 .on_mouse_up_out(|event: MouseEvent| event.stop_propagation());
        //         })
        //     }),
        // self.rename_modal
        //     .as_ref()
        //     .map_or(RenderingTree::Empty, |rename_modal| {
        //         rename_modal.render().attach_event(|builder| {
        //             builder
        //                 .on_mouse_down_in(|event: MouseEvent| event.stop_propagation())
        //                 .on_mouse_down_out(|event: MouseEvent| {
        //                     namui::event::send(Event::ContextMenuOutsideClicked);
        //                     event.stop_propagation()
        //                 })
        //                 .on_mouse_up_in(|event: MouseEvent| event.stop_propagation())
        //                 .on_mouse_up_out(|event: MouseEvent| event.stop_propagation());
        //         })
        //     }),

        
    }
}

#[namui::component]
pub struct ProjectCell {
    wh: Wh<Px>,
    project: EditableProject,
}

impl Component for ProjectCell {
    fn render(self, ctx: &RenderCtx)  {
        let project_id = self.project.id;

        ctx.component(namui_prebuilt::button::TextButton {
            rect: Rect::from_xy_wh(Xy::single(0.px()), self.wh),
            text: self.project.name.as_str(),
            text_color: Color::WHITE,
            stroke_color: Color::grayscale_f01(0.3),
            stroke_width: 1.px(),
            fill_color: Color::BLACK,
            mouse_buttons: vec![MouseButton::Left],
            on_mouse_up_in: &|event: MouseEvent| {
                if event.button == Some(MouseButton::Left) {
                    super::router::move_to(super::router::Route::SequenceList { project_id });
                } else if event.button == Some(MouseButton::Right) {
                    // TODO
                    // namui::event::send(Event::CellRightClick {
                    //     click_global_xy: event.global_xy,
                    //     project_id,
                    // });
                }
            },
        })
        .done()
    }
}
