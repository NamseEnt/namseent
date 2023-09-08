use namui::prelude::*;
use namui_prebuilt::*;
use rpc::list_editable_projects::EditableProject;

#[namui::component]
pub struct ProjectListPage {
    pub wh: Wh<Px>,
}

impl Component for ProjectListPage {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        let Self { wh } = self;
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
            table::hooks::horizontal([
                table::hooks::ratio(1.0, |_wh, _ctx| {}),
                table::hooks::ratio(
                    2.0,
                    table::hooks::vertical([
                        table::hooks::fixed(40.px(), |wh, ctx| {
                            ctx.add(namui_prebuilt::button::TextButton {
                                rect: Rect::from_xy_wh(Xy::single(0.px()), wh),
                                text: "[+] Add Project",
                                text_color: Color::WHITE,
                                stroke_color: Color::grayscale_f01(0.5),
                                stroke_width: 1.px(),
                                fill_color: Color::BLACK,
                                mouse_buttons: vec![MouseButton::Left],
                                on_mouse_up_in: Box::new(|_| on_add_button_clicked()),
                            });
                        }),
                        table::hooks::ratio(1.0, |wh, ctx| {
                            let item_wh = Wh::new(wh.width, 40.px());
                            ctx.add(list_view::ListView {
                                xy: Xy::single(0.px()),
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
                table::hooks::ratio(1.0, |_wh, _ctx| {}),
            ])(wh, ctx)
        });

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

        ctx.done()
    }
}

#[namui::component]
pub struct ProjectCell {
    wh: Wh<Px>,
    project: EditableProject,
}

impl Component for ProjectCell {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        let project_id = self.project.id;

        ctx.component(namui_prebuilt::button::TextButton {
            rect: Rect::from_xy_wh(Xy::single(0.px()), self.wh),
            text: self.project.name.as_str(),
            text_color: Color::WHITE,
            stroke_color: Color::grayscale_f01(0.3),
            stroke_width: 1.px(),
            fill_color: Color::BLACK,
            mouse_buttons: vec![MouseButton::Left],
            on_mouse_up_in: Box::new(|event: MouseEvent| {
                if event.button == Some(MouseButton::Left) {
                    super::router::move_to(super::router::Route::SequenceList { project_id });
                } else if event.button == Some(MouseButton::Right) {
                    // TODO
                    // namui::event::send(Event::CellRightClick {
                    //     click_global_xy: event.global_xy,
                    //     project_id,
                    // });
                }
            }),
        })
        .done()
    }
}
