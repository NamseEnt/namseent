use super::{render::resizer, *};
use crate::pages::sequence_edit_page::{
    loaded::components::character_editor, sequence_atom::SEQUENCE_ATOM,
};

impl WysiwygEditor {
    pub fn update(&mut self, event: &namui::Event) {
        event
            .is::<resizer::Event>(|event| match event {
                resizer::Event::UpdateDraggingContext(context) => {
                    if matches!(self.dragging, Some(Dragging::Resizer { .. })) {
                        self.dragging =
                            context.and_then(|context| Some(Dragging::Resizer { context }));
                    }
                }
                &resizer::Event::StartDraggingContext(context) => {
                    if self.dragging.is_none() {
                        self.dragging = Some(Dragging::Resizer { context });
                    }
                }
            })
            .is::<InternalEvent>(|event| match event {
                &InternalEvent::SelectImage { index } => {
                    self.editing_image_index = Some(index);
                }
                &InternalEvent::ImageMoveStart {
                    start_global_xy,
                    end_global_xy,
                    container_wh,
                } => {
                    if self.dragging.is_none() {
                        self.dragging = Some(Dragging::Mover {
                            context: render::mover::MoverDraggingContext {
                                start_global_xy,
                                end_global_xy,
                                container_wh,
                            },
                        });
                    }
                }
                &InternalEvent::MouseMoveContainer { global_xy } => {
                    if let Some(Dragging::Mover { context }) = self.dragging.as_mut() {
                        context.end_global_xy = global_xy;
                    }
                }
                InternalEvent::MouseDownContainer => {
                    self.editing_image_index = None;
                }
                &InternalEvent::MouseUp => {
                    self.dragging = None;
                }
                InternalEvent::OpenContextMenu {
                    global_xy,
                    cut_id,
                    graphic_index,
                    graphic_wh,
                    graphic,
                } => {
                    let global_xy = global_xy.clone();
                    let cut_id = cut_id.clone();
                    let graphic_index = graphic_index.clone();
                    let graphic_wh = graphic_wh.clone();

                    let image_width_per_height_ratio = graphic_wh.width / graphic_wh.height;

                    let fit_items = [
                        context_menu::Item::new_button("Fit - contain", move || {
                            SEQUENCE_ATOM.update(|sequence| sequence.update_cut(cut_id,  CutUpdateAction::GraphicFitContain {
                                    graphic_index,
                                    image_width_per_height_ratio,
                                },
                            ));
                        }),
                        context_menu::Item::new_button("Fit - cover", move || {
                            SEQUENCE_ATOM.update(|sequence| sequence.update_cut(cut_id,  CutUpdateAction::GraphicFitCover {
                                    graphic_index,
                                    image_width_per_height_ratio,
                                },
                            ));
                        }),
                    ];

                    // TODO: This is not re-implemented yet. Should think about strategy about editing multiple cuts.
                    // let spread_as_background = if graphic_index != 0 {
                    //     [].to_vec()
                    // } else {
                    //     [context_menu::Item::new_button("Spread as background", {
                    //         let graphic = graphic.clone();
                    //         move || {
                    //             let graphic = graphic.clone();
                    //             namui::event::send(Event::UpdateSequenceGraphics {
                    //                 callback: Box::new({
                    //                     move |graphics| {
                    //                         let graphic = graphic.clone();
                    //                         if graphics.len() == 0 {
                    //                             graphics.push(graphic);
                    //                             return;
                    //                         }

                    //                         let first = graphics.first_mut().unwrap();
                    //                         match (first, &graphic) {
                    //                             (
                    //                                 ScreenGraphic::Image(first),
                    //                                 ScreenGraphic::Image(graphic),
                    //                             ) => {
                    //                                 if first.id == graphic.id {
                    //                                     first.circumscribed = graphic.circumscribed;
                    //                                     return;
                    //                                 }
                    //                             }
                    //                             (
                    //                                 ScreenGraphic::Cg(first),
                    //                                 ScreenGraphic::Cg(graphic),
                    //                             ) => {
                    //                                 if first.cg_file_checksum == graphic.cg_file_checksum {
                    //                                     first.circumscribed = graphic.circumscribed;
                    //                                     return;
                    //                                 }
                    //                             }
                    //                             _ => (),
                    //                         }

                    //                         graphics.insert(0, graphic);
                    //                     }
                    //                 }),
                    //             });
                    //         }
                    //     })]
                    //     .to_vec()
                    // };
                    let edit_character_button_group = match graphic {
                        ScreenGraphic::Cg(cg) => {
                            let cg_id = cg.id;
                            Some(context_menu::Item::new_button(
                                "Edit character",
                                move |_| {
                                    namui::event::send(character_editor::Event::OpenCharacterEditor {
                                        target: character_editor::EditTarget::ExistingCharacterPart {
                                            cut_id,
                                            cg_id,
                                            graphic_index,
                                        },
                                    });
                                },
                            ))
                        },
                        ScreenGraphic::Image(_) => None,
                    }
                    .into_iter();

                    self.context_menu = Some(context_menu::ContextMenu::new(
                        global_xy,
                        fit_items
                            .into_iter()
                            // .chain(spread_as_background)
                            .chain(edit_character_button_group),
                    ));
                }
            })
            .is::<context_menu::Event>(|event| match event {
                context_menu::Event::Close => {
                    self.context_menu = None;
                }
            });

        self.context_menu.as_mut().map(|context_menu| {
            context_menu.update(event);
        });
    }
}
