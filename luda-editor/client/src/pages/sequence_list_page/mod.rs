mod context_menu;
mod rename_modal;

use super::{router, sequence_edit_page::SequenceEditPage};
use crate::storage::{EditorHistorySystem, Sequence, Storage};
use namui::prelude::*;
use namui_prebuilt::*;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct SequenceListPage {
    list_view: list_view::ListView,
    editor_history_system: EditorHistorySystem,
    context_menu: Option<context_menu::ContextMenu>,
    rename_modal: Option<rename_modal::RenameModal>,
    storage: Storage,
}

pub struct Props {
    pub wh: Wh<Px>,
}

enum Event {
    AddButtonClicked,
    CellRightClick {
        click_global_xy: Xy<Px>,
        sequence_id: String,
    },
    ContextMenuOutsideClicked,
}

impl SequenceListPage {
    pub fn new(editor_history_system: EditorHistorySystem, storage: Storage) -> Self {
        Self {
            list_view: list_view::ListView::new(),
            editor_history_system,
            context_menu: None,
            rename_modal: None,
            storage,
        }
    }
    pub fn update(&mut self, event: &dyn std::any::Any) {
        if let Some(event) = event.downcast_ref::<Event>() {
            match event {
                Event::AddButtonClicked => self.editor_history_system.mutate(|system_tree| {
                    system_tree
                        .sequence_list
                        .push(Sequence::new("new sequence".to_string()));
                }),
                Event::CellRightClick {
                    click_global_xy,
                    sequence_id,
                } => {
                    self.context_menu = Some(context_menu::ContextMenu::new(
                        *click_global_xy,
                        sequence_id.clone(),
                    ));
                }
                Event::ContextMenuOutsideClicked => {
                    self.context_menu = None;
                }
            }
        } else if let Some(event) = event.downcast_ref::<context_menu::Event>() {
            match event {
                context_menu::Event::DeleteButtonClicked { sequence_id } => {
                    self.editor_history_system.mutate(|system_tree| {
                        let index = system_tree
                            .sequence_list
                            .iter()
                            .position(|sequence| sequence.id() == *sequence_id)
                            .unwrap();
                        system_tree.sequence_list.remove(index);
                    });
                    self.context_menu = None;
                }
                context_menu::Event::RenameButtonClicked { sequence_id } => {
                    self.context_menu = None;
                    let sequence_name = self
                        .editor_history_system
                        .get_state()
                        .sequence_list
                        .iter()
                        .find(|sequence| sequence.id() == *sequence_id)
                        .map(|sequence| sequence.name.clone());

                    match sequence_name {
                        Some(sequence_name) => {
                            self.rename_modal = Some(rename_modal::RenameModal::new(
                                sequence_id.clone(),
                                sequence_name,
                            ))
                        }
                        None => {
                            namui::log!("sequence not found for id {sequence_id}");
                        }
                    }
                }
            }
        } else if let Some(event) = event.downcast_ref::<rename_modal::Event>() {
            match event {
                rename_modal::Event::RenameDone {
                    sequence_id,
                    sequence_name,
                } => {
                    self.rename_modal = None;
                    self.editor_history_system.mutate(|system_tree| {
                        let index = system_tree
                            .sequence_list
                            .iter()
                            .position(|sequence| sequence.id() == *sequence_id)
                            .unwrap();
                        system_tree.sequence_list.update(index, |sequence| {
                            sequence.name = sequence_name.clone();
                        });
                    });
                }
            }
        }

        self.context_menu
            .as_mut()
            .map(|context_menu| context_menu.update(event));
        self.rename_modal
            .as_mut()
            .map(|rename_modal| rename_modal.update(event));
    }
    pub fn render(&self, props: Props) -> namui::RenderingTree {
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
                                || namui::event::send(Event::AddButtonClicked),
                            )
                        }),
                        table::ratio(1.0, |wh| {
                            self.list_view.render(list_view::Props {
                                xy: Xy::single(0.px()),
                                height: wh.height,
                                scroll_bar_width: 10.px(),
                                item_wh: Wh::new(wh.width, 40.px()),
                                items: self.editor_history_system.get_state().sequence_list.iter(),
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
                            .on_mouse_down_in(|event| event.stop_propagation())
                            .on_mouse_down_out(|event| {
                                namui::event::send(Event::ContextMenuOutsideClicked);
                                event.stop_propagation()
                            })
                            .on_mouse_up_in(|event| event.stop_propagation())
                            .on_mouse_up_out(|event| event.stop_propagation());
                    })
                }),
            self.rename_modal
                .as_ref()
                .map_or(RenderingTree::Empty, |rename_modal| {
                    rename_modal.render().attach_event(|builder| {
                        builder
                            .on_mouse_down_in(|event| event.stop_propagation())
                            .on_mouse_down_out(|event| {
                                namui::event::send(Event::ContextMenuOutsideClicked);
                                event.stop_propagation()
                            })
                            .on_mouse_up_in(|event| event.stop_propagation())
                            .on_mouse_up_out(|event| event.stop_propagation());
                    })
                }),
        ])
    }

    fn render_sequence_cell(&self, wh: Wh<Px>, sequence: &Sequence) -> namui::RenderingTree {
        namui_prebuilt::button::text_button(
            Rect::from_xy_wh(Xy::single(0.px()), wh),
            sequence.name.as_str(),
            Color::WHITE,
            Color::grayscale_f01(0.3),
            1.px(),
            Color::BLACK,
            || {},
        )
        .attach_event(|builder| {
            let sequence_id = sequence.id().to_string();
            let editor_history_system = self.editor_history_system.clone();
            let storage = self.storage.clone();
            builder.on_mouse_up_in(move |event| {
                if event.button == Some(MouseButton::Left) {
                    let sequence_id = sequence_id.clone();
                    let editor_history_system = editor_history_system.clone();
                    let storage = storage.clone();
                    namui::event::send(router::Event::Route(Arc::new(move || {
                        router::Route::SequenceEditPage(SequenceEditPage::new(
                            editor_history_system.clone(),
                            sequence_id.clone(),
                            storage.clone(),
                        ))
                    })));
                } else if event.button == Some(MouseButton::Right) {
                    namui::event::send(Event::CellRightClick {
                        click_global_xy: event.global_xy,
                        sequence_id: sequence_id.clone(),
                    });
                }
            });
        })
    }
}
