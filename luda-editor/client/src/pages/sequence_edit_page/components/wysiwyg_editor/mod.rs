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
        context_menu::use_context_menu,
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
    pub on_click_character_edit: callback!('a, character_editor::EditTarget),
}

impl Component for WysiwygEditor<'_> {
    fn render<'a>(&'a self, ctx: &'a RenderCtx) -> RenderDone {
        let &Self {
            wh,
            cut_id,
            ref screen_graphics,
            project_id,
            ref cg_files,
            ref on_click_character_edit,
        } = self;
        let on_click_character_edit = on_click_character_edit.clone();

        let (dragging, set_dragging) = ctx.state(|| None);
        let (editing_image_index, set_editing_image_index) = ctx.state(|| None);
        let (context_menu, set_context_menu) = ctx.state(|| None);

        let background =
            simple_rect(wh, Color::WHITE, 1.px(), Color::TRANSPARENT).attach_event(|builder| {
                let dragging = dragging.clone();
                let screen_graphics = screen_graphics.clone();
                let editing_image_index = *editing_image_index;
                builder
                    .on_mouse_move_in(move |event: MouseEvent| {
                        set_dragging.mutate(|dragging| {
                            if let Some(Dragging::Mover { context }) = dragging {
                                context.end_global_xy = event.global_xy;
                            }
                        });
                    })
                    .on_mouse_down_in(move |event: MouseEvent| {
                        if event.button == Some(MouseButton::Left) {
                            set_editing_image_index.set(None);
                        }
                    })
                    .on_mouse(move |event: MouseEvent| {
                        if event.event_type == MouseEventType::Up {
                            if let Some(Dragging::Mover { mut context }) = dragging.deref() {
                                if let Some(graphic_index) = editing_image_index {
                                    context.end_global_xy = event.global_xy;

                                    let (_, graphic) = screen_graphics
                                        .iter()
                                        .find(|(index, _)| index == &graphic_index)
                                        .unwrap();
                                    let circumscribed =
                                        context.move_circumscribed(graphic.circumscribed());

                                    SEQUENCE_ATOM.mutate(|sequence| {
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
                    });
            });

        let graphic_clip_on_event = arc(move |e: graphic_clip::Event| {
            match e {
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
                            SEQUENCE_ATOM.mutate(|sequence| {
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
                    let image_width_per_height_ratio = graphic_wh.width / graphic_wh.height;

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

                    set_context_menu.set(Some({
                        let context_menu_builder =
                            use_context_menu(global_xy, arc(|| set_context_menu.set(None)));

                        context_menu_builder.add_button(
                            "Fit - contain",
                            arc(|| {
                                SEQUENCE_ATOM.mutate(|sequence| {
                                    sequence.update_cut(
                                        cut_id,
                                        CutUpdateAction::GraphicFitContain {
                                            graphic_index,
                                            image_width_per_height_ratio,
                                        },
                                    )
                                });
                            }),
                        );

                        context_menu_builder.add_button(
                            "Fit - cover",
                            arc(|| {
                                SEQUENCE_ATOM.mutate(|sequence| {
                                    sequence.update_cut(
                                        cut_id,
                                        CutUpdateAction::GraphicFitCover {
                                            graphic_index,
                                            image_width_per_height_ratio,
                                        },
                                    )
                                });
                            }),
                        );

                        match graphic {
                            ScreenGraphic::Cg(cg) => {
                                let cg_id = cg.id;
                                let on_click_character_edit = on_click_character_edit.clone();
                                context_menu_builder.add_button(
                                    "Edit character",
                                    arc(|| {
                                        on_click_character_edit(
                                            character_editor::EditTarget::ExistingCharacterPart {
                                                cut_id,
                                                cg_id,
                                                graphic_index,
                                            },
                                        );
                                    }),
                                );
                            }
                            ScreenGraphic::Image(_) => {}
                        }

                        context_menu_builder.build()
                    }));
                }
            }
        });

        ctx.add(background);
        // ctx.add(hooks::clip(
        //     PathBuilder::new().add_rect(Rect::from_xy_wh(Xy::zero(), wh)),
        //     ClipOp::Intersect,
        //     Zip::from_iter(screen_graphics.into_iter().map(
        //         move |&(graphic_index, ref screen_graphic)| graphic_clip::GraphicClip {
        //             cut_id,
        //             graphic_index,
        //             graphic: screen_graphic.clone(),
        //             is_editing_graphic: editing_image_index == &Some(graphic_index),
        //             project_id,
        //             wh,
        //             dragging: dragging.clone(),
        //             cg_files: cg_files.clone(),
        //             on_event: graphic_clip_on_event.clone(),
        //         },
        //     )),
        // ));

        ctx.add(render_grid_guide(wh));
        if let Some(context_menu) = context_menu.deref() {
            ctx.add(context_menu);
        }
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
