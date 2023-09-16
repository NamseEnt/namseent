mod graphic_clip;
mod grid_guide;
mod mover;
mod resizer;
mod wysiwyg_tool;

use self::{
    grid_guide::render_grid_guide,
    mover::{MoverDraggingContext, MovingWith},
};
use super::character_editor;
use crate::{
    components::{
        cg_render,
        context_menu::*,
        sequence_player::{calculate_graphic_rect_on_screen, calculate_graphic_wh_on_screen},
    },
    pages::sequence_edit_page::atom::{
        FocusableComponent, EDITING_GRAPHIC_INDEX_ATOM, FOCUSED_COMPONENT, SEQUENCE_ATOM,
    },
    storage::{get_project_cg_thumbnail_image_url, get_project_image_url},
};
use mover::Mover;
use namui::prelude::*;
use namui_prebuilt::*;
use resizer::Resizer;
use rpc::data::{CgFile, CutUpdateAction, ScreenGraphic};
use std::ops::Deref;

#[namui::component]
pub struct WysiwygEditor<'a> {
    pub wh: Wh<Px>,
    pub cut_id: Uuid,
    pub screen_graphics: Vec<(Uuid, ScreenGraphic)>,
    pub project_id: Uuid,
    pub cg_files: Vec<CgFile>,
    pub on_click_character_edit: Box<dyn 'a + Fn(character_editor::EditTarget)>,
}

#[derive(Debug)]
enum ContextMenu {
    WysiwygEditor {
        cut_id: Uuid,
        graphic_index: Uuid,
        graphic_wh: Wh<Px>,
        graphic: ScreenGraphic,
    },
}

impl Component for WysiwygEditor<'_> {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        let Self {
            wh,
            cut_id,
            ref screen_graphics,
            project_id,
            ref cg_files,
            ref on_click_character_edit,
        } = self;
        let (dragging, set_dragging) = ctx.state(|| None);
        let (editing_graphic_index, set_editing_graphic_index) =
            ctx.atom_init(&EDITING_GRAPHIC_INDEX_ATOM, || None);
        let (focused_component, _set_focused_component) = ctx.atom(&FOCUSED_COMPONENT);

        let cut_editor_focused = matches!(*focused_component, Some(FocusableComponent::CutEditor));

        let update_sequence_with_mover = |context: MoverDraggingContext, graphic_index: Uuid| {
            let (_, graphic) = screen_graphics
                .iter()
                .find(|(index, _)| *index == graphic_index)
                .unwrap();
            let circumscribed = context.move_circumscribed(graphic.circumscribed());

            SEQUENCE_ATOM.mutate(move |sequence| {
                sequence.update_cut(
                    cut_id,
                    CutUpdateAction::ChangeGraphicCircumscribed {
                        graphic_index,
                        circumscribed,
                    },
                )
            });
        };

        let background = simple_rect(wh, Color::WHITE, 1.px(), Color::TRANSPARENT).attach_event(
            |event: Event<'_>| {
                let editing_image_index = *editing_graphic_index;
                match event {
                    Event::MouseDown { event } => {
                        if event.is_local_xy_in() && event.button == Some(MouseButton::Left) {
                            set_editing_graphic_index.set(None);
                        }
                    }
                    Event::MouseMove { event } => {
                        if event.is_local_xy_in() {
                            set_dragging.mutate(move |dragging| {
                                if let Some(Dragging::Mover { context }) = dragging {
                                    if context.moving_with == MovingWith::Mouse {
                                        context.end_global_xy = event.global_xy;
                                    }
                                }
                            });
                        }
                    }
                    Event::MouseUp { event } => {
                        if let Some(Dragging::Mover { mut context }) = dragging.deref() {
                            if context.moving_with == MovingWith::Mouse {
                                if let Some(graphic_index) = editing_image_index {
                                    context.end_global_xy = event.global_xy;
                                    update_sequence_with_mover(context, graphic_index);
                                }
                            }
                        }
                        set_dragging.set(None);
                    }
                    Event::KeyDown { event } => {
                        let moving_with = MovingWith::try_from(event.code);
                        if let (Some(graphic_index), Ok(moving_with)) =
                            (*editing_graphic_index, moving_with)
                        {
                            if cut_editor_focused {
                                let set_new_dragging = || {
                                    set_dragging.set(Some(Dragging::Mover {
                                        context: MoverDraggingContext {
                                            start_global_xy: Xy::zero(),
                                            end_global_xy: Xy::zero(),
                                            container_wh: wh,
                                            moving_with,
                                        },
                                    }));
                                };
                                let mutate_dragging = || {
                                    let mut delta_xy = moving_with.delta_xy();
                                    if !namui::keyboard::shift_press() {
                                        delta_xy *= 10.0;
                                    }
                                    set_dragging.mutate(move |dragging| {
                                        if let Some(Dragging::Mover { context }) = dragging {
                                            context.end_global_xy =
                                                context.end_global_xy + delta_xy;
                                        }
                                    });
                                };
                                match *dragging {
                                    Some(Dragging::Mover { context }) => {
                                        if context.moving_with.key_changed(event.code) {
                                            update_sequence_with_mover(context, graphic_index);
                                            set_new_dragging();
                                        }
                                        mutate_dragging();
                                        event.stop_propagation();
                                    }
                                    None => {
                                        set_new_dragging();
                                        mutate_dragging();
                                        event.stop_propagation();
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                    Event::KeyUp { event } => {
                        if let Some(Dragging::Mover { context }) = *dragging {
                            if let Some(graphic_index) = editing_image_index {
                                let moving_key_up = !context.moving_with.key_changed(event.code);
                                if cut_editor_focused && moving_key_up {
                                    update_sequence_with_mover(context, graphic_index);
                                    set_dragging.set(None);
                                }
                            }
                        }
                    }
                    _ => {}
                }
            },
        );

        ctx.compose(|ctx| {
            let mut ctx = ctx.clip(
                Path::new().add_rect(Rect::from_xy_wh(Xy::zero(), wh)),
                ClipOp::Intersect,
            );

            for (graphic_index, screen_graphic) in screen_graphics.iter() {
                ctx.add_with_key(
                    graphic_index,
                    graphic_clip::GraphicClip {
                        cut_id,
                        graphic_index: *graphic_index,
                        graphic: screen_graphic,
                        is_editing_graphic: editing_graphic_index.as_ref() == &Some(*graphic_index),
                        project_id,
                        wh,
                        dragging: dragging.as_ref(),
                        cg_files,
                        on_event: &move |e: graphic_clip::Event| match e {
                            graphic_clip::Event::WysiwygTool(e) => match e {
                                wysiwyg_tool::Event::Mover { event } => match event {
                                    mover::Event::MoveStart {
                                        start_global_xy,
                                        end_global_xy,
                                        container_wh,
                                        moving_with,
                                    } => {
                                        set_dragging.set(Some(Dragging::Mover {
                                            context: mover::MoverDraggingContext {
                                                start_global_xy,
                                                end_global_xy,
                                                container_wh,
                                                moving_with,
                                            },
                                        }));
                                    }
                                },
                                wysiwyg_tool::Event::Resizer { event } => match event {
                                    resizer::Event::OnResize {
                                        circumscribed,
                                        graphic_index,
                                    } => {
                                        SEQUENCE_ATOM.mutate(move |sequence| {
                                            sequence.update_cut(
                                                cut_id,
                                                CutUpdateAction::UpdateCircumscribed {
                                                    graphic_index,
                                                    circumscribed,
                                                },
                                            )
                                        });
                                    }
                                    resizer::Event::OnUpdateDraggingContext { context } => {
                                        set_dragging.set(
                                            context.map(|context| Dragging::Resizer { context }),
                                        );
                                    }
                                },
                            },
                            graphic_clip::Event::SelectImage { graphic_index } => {
                                set_editing_graphic_index.set(Some(graphic_index))
                            }
                            graphic_clip::Event::GraphicRightClick {
                                global_xy,
                                cut_id,
                                graphic_index,
                                graphic_wh,
                                graphic,
                            } => {
                                open_context_menu(
                                    global_xy,
                                    ContextMenu::WysiwygEditor {
                                        cut_id,
                                        graphic_index,
                                        graphic_wh,
                                        graphic: graphic.clone(),
                                    },
                                );
                            }
                            graphic_clip::Event::DeleteGraphic { graphic_index } => {
                                SEQUENCE_ATOM.mutate(move |sequence| {
                                    sequence.update_cut(
                                        cut_id,
                                        CutUpdateAction::DeleteGraphic { graphic_index },
                                    )
                                });
                            }
                        },
                    },
                );
            }
        });

        ctx.component(render_grid_guide(wh));
        if_context_menu_for::<ContextMenu>(|context_menu, builder| {
            match context_menu {
                &ContextMenu::WysiwygEditor {
                    cut_id,
                    graphic_index,
                    graphic_wh,
                    ref graphic,
                } => {
                    let image_width_per_height_ratio = graphic_wh.width / graphic_wh.height;
                    builder
                        .add_button("Send To Back", || {
                            if let Some(last_graphic_index) =
                                screen_graphics.last().map(|(index, _)| *index)
                            {
                                SEQUENCE_ATOM.mutate(move |sequence| {
                                    sequence.update_cut(
                                        cut_id,
                                        CutUpdateAction::ChangeGraphicOrder {
                                            graphic_index,
                                            after_graphic_index: Some(last_graphic_index),
                                        },
                                    )
                                });
                            }
                        })
                        .add_button("Send Backward", || {
                            let Some(next_or_last_graphic_index) = ({
                                screen_graphics
                                    .iter()
                                    .position(|(index, _)| *index == graphic_index)
                                    .and_then(|position| {
                                        let next_position =
                                            (position + 1).min(screen_graphics.len() - 1);
                                        screen_graphics.get(next_position).map(|(index, _)| *index)
                                    })
                            }) else {
                                return;
                            };

                            SEQUENCE_ATOM.mutate(move |sequence| {
                                sequence.update_cut(
                                    cut_id,
                                    CutUpdateAction::ChangeGraphicOrder {
                                        graphic_index,
                                        after_graphic_index: Some(next_or_last_graphic_index),
                                    },
                                )
                            });
                        })
                        .add_button("Bring Forward", || {
                            let previous_graphic_index = {
                                screen_graphics
                                    .iter()
                                    .position(|(index, _)| *index == graphic_index)
                                    .and_then(|position| match position.checked_sub(2) {
                                        Some(position) => {
                                            screen_graphics.get(position).map(|(index, _)| *index)
                                        }
                                        None => None,
                                    })
                            };

                            SEQUENCE_ATOM.mutate(move |sequence| {
                                sequence.update_cut(
                                    cut_id,
                                    CutUpdateAction::ChangeGraphicOrder {
                                        graphic_index,
                                        after_graphic_index: previous_graphic_index,
                                    },
                                )
                            });
                        })
                        .add_button("Bring To Front", || {
                            SEQUENCE_ATOM.mutate(move |sequence| {
                                sequence.update_cut(
                                    cut_id,
                                    CutUpdateAction::ChangeGraphicOrder {
                                        graphic_index,
                                        after_graphic_index: None,
                                    },
                                )
                            });
                        })
                        .add_divider()
                        .add_button("Fit - contain", || {
                            SEQUENCE_ATOM.mutate(move |sequence| {
                                sequence.update_cut(
                                    cut_id,
                                    CutUpdateAction::GraphicFitContain {
                                        graphic_index,
                                        image_width_per_height_ratio,
                                    },
                                )
                            });
                        })
                        .add_button("Fit - cover", || {
                            SEQUENCE_ATOM.mutate(move |sequence| {
                                sequence.update_cut(
                                    cut_id,
                                    CutUpdateAction::GraphicFitCover {
                                        graphic_index,
                                        image_width_per_height_ratio,
                                    },
                                )
                            });
                        })
                        .and(|builder| {
                            let ScreenGraphic::Cg(cg) = graphic else {
                                return builder;
                            };
                            let cg_id = cg.id;

                            builder.add_button("Edit character", move || {
                                on_click_character_edit(
                                    character_editor::EditTarget::ExistingCharacterPart {
                                        cut_id,
                                        cg_id,
                                        graphic_index,
                                    },
                                );
                            })
                        })
                        .add_button("Delete", || {
                            SEQUENCE_ATOM.mutate(move |sequence| {
                                sequence.update_cut(
                                    cut_id,
                                    CutUpdateAction::DeleteGraphic { graphic_index },
                                )
                            });
                        })
                    // TODO: This is not re-implemented yet. Should think about strategy about editing multiple cuts.
                    // .and(|builder| {
                    //     if graphic_index != 0 {
                    //         return builder;
                    //     }
                    //     builder.add_button("Spread as background", {
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
                    //                                     first.circumscribed =
                    //                                         graphic.circumscribed;
                    //                                     return;
                    //                                 }
                    //                             }
                    //                             (
                    //                                 ScreenGraphic::Cg(first),
                    //                                 ScreenGraphic::Cg(graphic),
                    //                             ) => {
                    //                                 if first.cg_file_checksum
                    //                                     == graphic.cg_file_checksum
                    //                                 {
                    //                                     first.circumscribed =
                    //                                         graphic.circumscribed;
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
                    //     })
                    // })
                }
            }
        });
        ctx.component(background);

        ctx.done()
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Dragging {
    Resizer {
        context: resizer::ResizerDraggingContext,
    },
    Mover {
        context: mover::MoverDraggingContext,
    },
}
