use namui::prelude::*;
use namui_prebuilt::*;
use rpc::list_editable_projects::EditableProject;

#[namui::component]
pub struct ProjectListPage2 {
    pub wh: Wh<Px>,
}

impl Component for ProjectListPage2 {
    fn render<'a>(&'a self, ctx: &'a RenderCtx) -> RenderDone {
        let &Self { wh } = self;
        let (error_message, set_error_message) = ctx.use_state::<Option<String>>(|| None);
        let (is_loading, set_is_loading) = ctx.use_state(|| true);
        let (project_list, set_project_list) = ctx.use_state::<Vec<EditableProject>>(|| vec![]);

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

        ctx.use_effect("Fetch project list on mount", || {
            start_fetch_list();
        });

        if let Some(error_message) = &*error_message {
            ctx.add(typography::body::center(wh, error_message, Color::RED));
            return ctx.done();
        }

        if *is_loading {
            ctx.add(typography::body::center(wh, "loading...123", Color::WHITE));
            return ctx.done();
        }

        ctx.add(table::hooks::horizontal([
            table::hooks::ratio(1.0, |_wh| RenderingTree::Empty),
            table::hooks::ratio(
                2.0,
                table::hooks::vertical([
                    table::hooks::fixed(40.px(), |wh| {
                        namui_prebuilt::button::text_button(
                            Rect::from_xy_wh(Xy::single(0.px()), wh),
                            "[+] Add Project",
                            Color::WHITE,
                            Color::grayscale_f01(0.5),
                            1.px(),
                            Color::BLACK,
                            [MouseButton::Left],
                            move |_| on_add_button_clicked(),
                        )
                    }),
                    table::hooks::ratio(1.0, |wh| {
                        let item_wh = Wh::new(wh.width, 40.px());
                        list_view::ListView {
                            xy: Xy::single(0.px()),
                            height: wh.height,
                            scroll_bar_width: 10.px(),
                            item_wh,
                            items: project_list
                                .iter()
                                .map(|project| ProjectCell {
                                    wh: item_wh,
                                    project: project.clone(),
                                })
                                .collect(),
                        }
                    }),
                ]),
            ),
            table::hooks::ratio(1.0, |_wh| RenderingTree::Empty),
        ])(wh));

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
    fn render<'a>(&'a self, ctx: &'a RenderCtx) -> RenderDone {
        let project_id = self.project.id;

        ctx.add(namui_prebuilt::button::text_button(
            Rect::from_xy_wh(Xy::single(0.px()), self.wh),
            self.project.name.as_str(),
            Color::WHITE,
            Color::grayscale_f01(0.3),
            1.px(),
            Color::BLACK,
            [MouseButton::Left],
            move |event: MouseEvent| {
                if event.button == Some(MouseButton::Left) {
                    super::router::move_to(super::router::RoutePath::SequenceList { project_id });
                } else if event.button == Some(MouseButton::Right) {
                    // TODO
                    // namui::event::send(Event::CellRightClick {
                    //     click_global_xy: event.global_xy,
                    //     project_id,
                    // });
                }
            },
        ));

        ctx.done()
    }
}
