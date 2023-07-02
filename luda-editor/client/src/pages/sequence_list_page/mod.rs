mod context_menu;
mod rename_modal;

use super::router::Router;
use namui::prelude::*;
use namui_prebuilt::*;
use rpc::list_project_sequences::SequenceNameAndId;

#[derive(Debug, Clone)]
pub struct SequenceListPage {
    project_id: namui::Uuid,
    list_view: list_view::ListView,
    context_menu: Option<context_menu::ContextMenu>,
    rename_modal: Option<rename_modal::RenameModal>,
    is_loading: bool,
    sequence_list: Vec<SequenceNameAndId>,
    error_message: Option<String>,
}

pub struct Props {
    pub wh: Wh<Px>,
}

enum Event {
    AddButtonClicked,
    CellRightClick {
        click_global_xy: Xy<Px>,
        sequence_id: namui::Uuid,
    },
    ContextMenuOutsideClicked,
    SequenceListLoaded(Vec<SequenceNameAndId>),
    Error(String),
}

impl SequenceListPage {
    pub fn new(project_id: namui::Uuid) -> Self {
        start_fetch_list(project_id);
        Self {
            project_id,
            list_view: list_view::ListView::new(),
            context_menu: None,
            rename_modal: None,
            is_loading: true,
            sequence_list: vec![],
            error_message: None,
        }
    }
    pub fn update(&mut self, event: &namui::Event) {
        event
            .is::<Event>(|event| match event {
                Event::AddButtonClicked => {
                    let project_id = self.project_id;
                    spawn_local(async move {
                        let result = crate::RPC
                            .create_sequence(rpc::create_sequence::Request {
                                name: "new sequence".to_string(),
                                project_id,
                            })
                            .await;
                        match result {
                            Ok(_) => {
                                start_fetch_list(project_id);
                            }
                            Err(error) => {
                                namui::event::send(Event::Error(error.to_string()));
                            }
                        }
                    })
                }
                &Event::CellRightClick {
                    click_global_xy,
                    sequence_id,
                } => {
                    self.context_menu =
                        Some(context_menu::ContextMenu::new(click_global_xy, sequence_id));
                }
                Event::ContextMenuOutsideClicked => {
                    self.context_menu = None;
                }
                Event::SequenceListLoaded(sequence_list) => {
                    self.sequence_list = sequence_list.clone();
                    self.is_loading = false;
                }
                Event::Error(error_message) => {
                    self.error_message = Some(error_message.clone());
                }
            })
            .is::<context_menu::Event>(|event| match event {
                &context_menu::Event::DeleteButtonClicked { sequence_id } => {
                    self.context_menu = None;

                    let project_id = self.project_id;

                    spawn_local(async move {
                        match crate::RPC
                            .delete_sequence(rpc::delete_sequence::Request { sequence_id })
                            .await
                        {
                            Ok(_) => {
                                start_fetch_list(project_id);
                            }
                            Err(error) => {
                                namui::event::send(Event::Error(error.to_string()));
                            }
                        }
                    });
                }
                &context_menu::Event::RenameButtonClicked { sequence_id } => {
                    self.context_menu = None;

                    let sequence_name =
                        self.sequence_list.iter().find_map(|sequence_name_and_id| {
                            if sequence_name_and_id.id == sequence_id {
                                Some(sequence_name_and_id.name.clone())
                            } else {
                                None
                            }
                        });

                    match sequence_name {
                        Some(sequence_name) => {
                            self.rename_modal =
                                Some(rename_modal::RenameModal::new(sequence_id, sequence_name))
                        }
                        None => {
                            namui::log!("sequence not found for id {sequence_id}");
                        }
                    }
                }
            })
            .is::<rename_modal::Event>(|event| match event {
                &rename_modal::Event::RenameDone {
                    sequence_id,
                    ref sequence_name,
                } => {
                    self.rename_modal = None;

                    let project_id = self.project_id;
                    let new_name = sequence_name.clone();

                    spawn_local(async move {
                        match crate::RPC
                            .rename_sequence(rpc::rename_sequence::Request {
                                sequence_id,
                                new_name,
                            })
                            .await
                        {
                            Ok(_) => {
                                start_fetch_list(project_id);
                            }
                            Err(error) => {
                                namui::event::send(Event::Error(error.to_string()));
                            }
                        }
                    });
                }
            });

        self.context_menu
            .as_mut()
            .map(|context_menu| context_menu.update(event));
        self.rename_modal
            .as_mut()
            .map(|rename_modal| rename_modal.update(event));
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
                                "[+] Add Sequence",
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
                                items: self.sequence_list.iter(),
                                item_render: |wh, sequence| self.render_sequence_cell(wh, sequence),
                            })
                        }),
                    ]),
                ),
                table::ratio(1.0, |_wh| RenderingTree::Empty),
            ])(props.wh),
            self.context_menu
                .as_ref()
                .map_or(RenderingTree::Empty, |context_menu| {
                    context_menu.render().attach_event(|builder| {
                        builder
                            .on_mouse_down_in(|event: MouseEvent| event.stop_propagation())
                            .on_mouse_down_out(|event: MouseEvent| {
                                namui::event::send(Event::ContextMenuOutsideClicked);
                                event.stop_propagation()
                            })
                            .on_mouse_up_in(|event: MouseEvent| event.stop_propagation())
                            .on_mouse_up_out(|event: MouseEvent| event.stop_propagation());
                    })
                }),
            self.rename_modal
                .as_ref()
                .map_or(RenderingTree::Empty, |rename_modal| {
                    rename_modal.render().attach_event(|builder| {
                        builder
                            .on_mouse_down_in(|event: MouseEvent| event.stop_propagation())
                            .on_mouse_down_out(|event: MouseEvent| {
                                namui::event::send(Event::ContextMenuOutsideClicked);
                                event.stop_propagation()
                            })
                            .on_mouse_up_in(|event: MouseEvent| event.stop_propagation())
                            .on_mouse_up_out(|event: MouseEvent| event.stop_propagation());
                    })
                }),
        ])
    }

    fn render_sequence_cell(
        &self,
        wh: Wh<Px>,
        sequence: &SequenceNameAndId,
    ) -> namui::RenderingTree {
        let sequence_id = sequence.id;
        namui_prebuilt::button::text_button(
            Rect::from_xy_wh(Xy::single(0.px()), wh),
            sequence.name.as_str(),
            Color::WHITE,
            Color::grayscale_f01(0.3),
            1.px(),
            Color::BLACK,
            [MouseButton::Left, MouseButton::Right],
            move |event: MouseEvent| {
                if event.button == Some(MouseButton::Left) {
                    Router::move_to(super::router::RoutePath::SequenceEdit(sequence_id));
                } else if event.button == Some(MouseButton::Right) {
                    namui::event::send(Event::CellRightClick {
                        click_global_xy: event.global_xy,
                        sequence_id,
                    });
                }
            },
        )
    }
}

fn start_fetch_list(project_id: Uuid) {
    spawn_local(async move {
        match crate::RPC
            .list_project_sequences(rpc::list_project_sequences::Request { project_id })
            .await
        {
            Ok(response) => {
                namui::event::send(Event::SequenceListLoaded(response.sequence_name_and_ids));
            }
            Err(error) => {
                namui::event::send(Event::Error(error.to_string()));
            }
        }
    })
}
