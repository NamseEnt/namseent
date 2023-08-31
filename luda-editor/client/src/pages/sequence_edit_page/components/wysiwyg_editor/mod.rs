mod graphic_clip;
mod grid_guide;
mod mover;
mod resizer;
mod wysiwyg_tool;

use std::ops::Deref;

use self::grid_guide::render_grid_guide;
use super::character_editor;
use crate::{
    components::{
        cg_render,
        context_menu::*,
        sequence_player::{calculate_graphic_rect_on_screen, calculate_graphic_wh_on_screen},
    },
    pages::sequence_edit_page::atom::SEQUENCE_ATOM,
    storage::{get_project_cg_thumbnail_image_url, get_project_image_url},
};
use mover::Mover;
use namui::prelude::*;
use namui_prebuilt::*;
use resizer::Resizer;
use rpc::data::{CgFile, CutUpdateAction, ScreenGraphic};

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
    fn render<'a>(self, ctx: &'a RenderCtx) -> RenderDone {
        let Self {
            wh,
            cut_id,
            ref screen_graphics,
            project_id,
            ref cg_files,
            ref on_click_character_edit,
        } = self;
        let (dragging, set_dragging) = ctx.state(|| None);
        let (editing_image_index, set_editing_image_index) = ctx.state(|| None);

        let background = simple_rect(wh, Color::WHITE, 1.px(), Color::TRANSPARENT).attach_event(
            |event: Event<'_>| {
                let dragging = dragging.clone();
                let screen_graphics = screen_graphics.clone();
                let editing_image_index = *editing_image_index;
                match event {
                    Event::MouseDown { event } => {
                        if event.is_local_xy_in() {
                            if event.button == Some(MouseButton::Left) {
                                set_editing_image_index.set(None);
                            }
                        }
                    }
                    Event::MouseMove { event } => {
                        if event.is_local_xy_in() {
                            set_dragging.mutate(move |dragging| {
                                if let Some(Dragging::Mover { context }) = dragging {
                                    context.end_global_xy = event.global_xy;
                                }
                            });
                        }
                    }
                    Event::MouseUp { event } => {
                        if let Some(Dragging::Mover { mut context }) = dragging.deref() {
                            if let Some(graphic_index) = editing_image_index {
                                context.end_global_xy = event.global_xy;

                                let (_, graphic) = screen_graphics
                                    .iter()
                                    .find(|(index, _)| index == &graphic_index)
                                    .unwrap();
                                let circumscribed =
                                    context.move_circumscribed(graphic.circumscribed());

                                SEQUENCE_ATOM.mutate(move |sequence| {
                                    sequence.update_cut(
                                        cut_id,
                                        CutUpdateAction::ChangeGraphicCircumscribed {
                                            graphic_index,
                                            circumscribed,
                                        },
                                    )
                                });
                            }
                        }
                        set_dragging.set(None);
                    }
                    _ => {}
                }
            },
        );

        let graphic_clip_on_event = Box::new(move |e: graphic_clip::Event| match e {
            graphic_clip::Event::WysiwygTool(e) => match e {
                wysiwyg_tool::Event::Mover { event } => match event {
                    mover::Event::MoveStart {
                        start_global_xy,
                        end_global_xy,
                        container_wh,
                    } => {
                        set_dragging.set(Some(Dragging::Mover {
                            context: mover::MoverDraggingContext {
                                start_global_xy,
                                end_global_xy,
                                container_wh,
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
                        set_dragging.set(context.map(|context| Dragging::Resizer { context }));
                    }
                },
            },
            graphic_clip::Event::SelectImage { graphic_index } => {
                set_editing_image_index.set(Some(graphic_index))
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
        });

        ctx.compose(|ctx| {
            let mut ctx = ctx.clip(
                Path::new().add_rect(Rect::from_xy_wh(Xy::zero(), wh)),
                ClipOp::Intersect,
            );

            for (graphic_index, screen_graphic) in screen_graphics.into_iter() {
                let graphic_clip_on_event = graphic_clip_on_event.clone();
                ctx.add(graphic_clip::GraphicClip {
                    cut_id,
                    graphic_index: *graphic_index,
                    graphic: screen_graphic.clone(),
                    is_editing_graphic: editing_image_index.as_ref() == &Some(*graphic_index),
                    project_id,
                    wh,
                    dragging: dragging.deref().clone(),
                    cg_files: cg_files.clone(),
                    on_event: graphic_clip_on_event,
                });
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
