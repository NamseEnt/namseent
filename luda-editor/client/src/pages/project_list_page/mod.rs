use super::router::Router;
use namui::prelude::*;
use namui_prebuilt::*;
use rpc::list_editable_projects::EditableProject;

#[derive(Debug, Clone)]
pub struct ProjectListPage {
    list_view: list_view::ListView,
    project_list: Vec<EditableProject>,
    is_loading: bool,
    error_message: Option<String>,
}

pub struct Props {
    pub wh: Wh<Px>,
}

enum Event {
    AddButtonClicked,
    ProjectListLoaded(Vec<EditableProject>),
    Error(String),
}

impl ProjectListPage {
    pub fn new() -> Self {
        start_fetch_list();
        Self {
            list_view: list_view::ListView::new(),
            project_list: vec![],
            is_loading: true,
            error_message: None,
        }
    }
    pub fn update(&mut self, event: &namui::Event) {
        event.is::<Event>(|event| match event {
            Event::AddButtonClicked => spawn_local(async move {
                match crate::RPC
                    .create_project(rpc::create_project::Request {
                        name: "new project".to_string(),
                    })
                    .await
                {
                    Ok(_) => {
                        start_fetch_list();
                    }
                    Err(error) => {
                        namui::event::send(Event::Error(error.to_string()));
                    }
                }
            }),
            Event::ProjectListLoaded(projects) => {
                self.project_list = projects.to_vec();
                self.is_loading = false;
            }
            Event::Error(message) => {
                namui::log!("error: {}", message);
                self.error_message = Some(message.to_string());
            }
        });
    }
    pub fn render(&self, props: Props) -> namui::RenderingTree {
        if let Some(error_message) = &self.error_message {
            return typography::body::center(props.wh, error_message, Color::RED);
        }
        if self.is_loading {
            return typography::body::center(props.wh, "loading...", Color::WHITE);
        }
        render([
            table::horizontal([
                table::ratio(1.0, |_wh| RenderingTree::Empty),
                table::ratio(
                    2.0,
                    table::vertical([
                        table::fixed(40.px(), |wh| {
                            namui_prebuilt::button::text_button(
                                Rect::from_xy_wh(Xy::single(0.px()), wh),
                                "[+] Add Project",
                                Color::WHITE,
                                Color::grayscale_f01(0.5),
                                1.px(),
                                Color::BLACK,
                                [MouseButton::Left],
                                |_| namui::event::send(Event::AddButtonClicked),
                            )
                        }),
                        table::ratio(1.0, |wh| {
                            self.list_view.render(list_view::Props {
                                xy: Xy::single(0.px()),
                                height: wh.height,
                                scroll_bar_width: 10.px(),
                                item_wh: Wh::new(wh.width, 40.px()),
                                items: self.project_list.iter(),
                                item_render: |wh, project| self.render_project_cell(wh, project),
                            })
                        }),
                    ]),
                ),
                table::ratio(1.0, |_wh| RenderingTree::Empty),
            ])(props.wh),
            // self.context_menu
            //     .as_ref()
            //     .map_or(RenderingTree::Empty, |context_menu| {
            //         context_menu.render().attach_event(|builder| {
            //             builder
            //                 .on_mouse_down_in(|event| event.stop_propagation())
            //                 .on_mouse_down_out(|event| {
            //                     namui::event::send(Event::ContextMenuOutsideClicked);
            //                     event.stop_propagation()
            //                 })
            //                 .on_mouse_up_in(|event| event.stop_propagation())
            //                 .on_mouse_up_out(|event| event.stop_propagation());
            //         })
            //     }),
            // self.rename_modal
            //     .as_ref()
            //     .map_or(RenderingTree::Empty, |rename_modal| {
            //         rename_modal.render().attach_event(|builder| {
            //             builder
            //                 .on_mouse_down_in(|event| event.stop_propagation())
            //                 .on_mouse_down_out(|event| {
            //                     namui::event::send(Event::ContextMenuOutsideClicked);
            //                     event.stop_propagation()
            //                 })
            //                 .on_mouse_up_in(|event| event.stop_propagation())
            //                 .on_mouse_up_out(|event| event.stop_propagation());
            //         })
            //     }),
        ])
    }

    fn render_project_cell(&self, wh: Wh<Px>, project: &EditableProject) -> namui::RenderingTree {
        let project_id = project.id;
        namui_prebuilt::button::text_button(
            Rect::from_xy_wh(Xy::single(0.px()), wh),
            project.name.as_str(),
            Color::WHITE,
            Color::grayscale_f01(0.3),
            1.px(),
            Color::BLACK,
            [MouseButton::Left],
            move |event| {
                if event.button == Some(MouseButton::Left) {
                    Router::move_to(super::router::RoutePath::SequenceList { project_id });
                } else if event.button == Some(MouseButton::Right) {
                    // TODO
                    // namui::event::send(Event::CellRightClick {
                    //     click_global_xy: event.global_xy,
                    //     project_id,
                    // });
                }
            },
        )
    }
}

fn start_fetch_list() {
    spawn_local(async move {
        match crate::RPC
            .list_editable_projects(rpc::list_editable_projects::Request { start_after: None })
            .await
        {
            Ok(response) => {
                namui::event::send(Event::ProjectListLoaded(response.projects));
            }
            Err(error) => {
                namui::event::send(Event::Error(error.to_string()));
            }
        }
    })
}
