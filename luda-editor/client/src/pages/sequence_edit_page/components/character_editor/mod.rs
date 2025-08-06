mod cg_picker;
mod part_picker;

use self::{cg_picker::CgPicker, part_picker::PartPicker};
use crate::{
    color,
    components::tool_tip::ToolTip,
    pages::sequence_edit_page::atom::{CG_FILES_ATOM, SEQUENCE_ATOM},
};
use namui::*;
use namui_prebuilt::*;
use rpc::data::{Cut, CutUpdateAction, ScreenCg, ScreenGraphic};
use std::{ops::Deref, sync::atomic::AtomicBool};

pub struct CharacterEditor<'a> {
    pub edit_target: EditTarget,
    pub wh: Wh<Px>,
    pub project_id: Uuid,
    pub cut: Option<&'a Cut>,
    pub on_event: Box<dyn 'a + Fn(Event)>,
}

pub enum Event {
    Close,
    CgChangeButtonClicked,
    ChangeEditTarget { edit_target: EditTarget },
}

impl Component for CharacterEditor<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            edit_target,
            wh,
            project_id,
            cut,
            on_event,
        } = self;
        let (tool_tip, set_tool_tip) = ctx.state::<Option<ToolTip>>(|| None);
        let (cg_file_list, _) = ctx.atom(&CG_FILES_ATOM);
        let tool_tip_setted: AtomicBool = Default::default();

        enum InternalEvent {
            MoveInCgFileThumbnail { global_xy: Xy<Px>, text: String },
        }

        let background = simple_rect(wh, color::STROKE_NORMAL, 1.px(), color::BACKGROUND)
            .attach_event(|event| match event {
                namui::Event::MouseDown { event } => {
                    if !event.is_local_xy_in() {
                        on_event(Event::Close);
                    }
                }
                namui::Event::MouseMove { .. } => {
                    if !tool_tip_setted.load(std::sync::atomic::Ordering::Relaxed)
                        && tool_tip.is_some()
                    {
                        set_tool_tip.set(None);
                    }
                }
                _ => {}
            });

        let on_internal_event = |event: InternalEvent| match event {
            InternalEvent::MoveInCgFileThumbnail { global_xy, text } => {
                tool_tip_setted.store(true, std::sync::atomic::Ordering::Relaxed);
                set_tool_tip.set(Some(ToolTip { global_xy, text }))
            }
        };

        ctx.compose(|ctx| {
            let Some(tool_tip) = tool_tip.deref() else {
                return;
            };
            ctx.add(tool_tip.clone());
        });

        ctx.compose(|ctx| {
            match edit_target {
                EditTarget::NewCharacter { .. } | EditTarget::ExistingCharacter { .. } => {
                    ctx.add(CgPicker {
                        wh,
                        project_id,
                        on_event: &|event| match event {
                            cg_picker::Event::MoveInCgFileThumbnail { global_xy, name } => {
                                on_internal_event(InternalEvent::MoveInCgFileThumbnail {
                                    global_xy,
                                    text: name,
                                })
                            }
                            cg_picker::Event::ClickCgFileThumbnail { cg_id } => match edit_target {
                                EditTarget::NewCharacter { cut_id } => {
                                    let cg_files = CG_FILES_ATOM.get();
                                    let Some(cg_file) =
                                        cg_files.iter().find(|file| file.id == cg_id)
                                    else {
                                        return;
                                    };

                                    let graphic_index: Uuid = Uuid::new_v4();

                                    SEQUENCE_ATOM.mutate(move |sequence| {
                                        sequence.update_cut(
                                            cut_id,
                                            CutUpdateAction::PushScreenGraphic {
                                                graphic_index,
                                                screen_graphic: ScreenGraphic::Cg(ScreenCg::new(
                                                    cg_file,
                                                )),
                                            },
                                        )
                                    });
                                    on_event(Event::ChangeEditTarget {
                                        edit_target: EditTarget::ExistingCharacterPart {
                                            cut_id,
                                            cg_id,
                                            graphic_index,
                                        },
                                    });
                                }
                                EditTarget::ExistingCharacter {
                                    cut_id,
                                    graphic_index,
                                } => {
                                    let cg_files = CG_FILES_ATOM.get();
                                    let Some(cg_file) =
                                        cg_files.iter().find(|file| file.id == cg_id)
                                    else {
                                        return;
                                    };

                                    SEQUENCE_ATOM.mutate(move |sequence| {
                                        sequence.update_cut(
                                            cut_id,
                                            CutUpdateAction::ChangeCgKeepCircumscribed {
                                                graphic_index,
                                                cg: ScreenCg::new(cg_file),
                                            },
                                        )
                                    });
                                    on_event(Event::ChangeEditTarget {
                                        edit_target: EditTarget::ExistingCharacterPart {
                                            cut_id,
                                            cg_id,
                                            graphic_index,
                                        },
                                    });
                                }
                                _ => {}
                            },
                        },
                    });
                }
                EditTarget::ExistingCharacterPart {
                    cg_id,
                    cut_id,
                    graphic_index,
                } => {
                    let selected_cg_file = cg_file_list.iter().find(|cg_file| cg_file.id == cg_id);
                    let selected_screen_graphic = cut.as_ref().and_then(|cut| {
                        cut.screen_graphics
                            .iter()
                            .find_map(|(index, screen_graphic)| {
                                if index == &graphic_index {
                                    Some(screen_graphic)
                                } else {
                                    None
                                }
                            })
                    });

                    match (selected_cg_file, selected_screen_graphic) {
                        (Some(selected_cg_file), Some(ScreenGraphic::Cg(selected_screen_cg))) => {
                            ctx.add(PartPicker {
                                wh,
                                cg_file: selected_cg_file,
                                project_id,
                                cut_id,
                                graphic_index,
                                screen_cg: selected_screen_cg,
                                on_event: Box::new(|event| match event {
                                    part_picker::Event::MoveInCgFileThumbnail {
                                        global_xy,
                                        name,
                                    } => on_internal_event(InternalEvent::MoveInCgFileThumbnail {
                                        global_xy,
                                        text: name,
                                    }),
                                    part_picker::Event::CgChangeButtonClicked => {
                                        on_event(Event::CgChangeButtonClicked)
                                    }
                                }),
                            });
                        }
                        _ => {
                            ctx.add(table::padding(8.px(), |wh| {
                                let text = "Selected resource not found. \
                                    Close character picker and try again.";
                                typography::body::center_top(wh.width, text, color::STROKE_NORMAL)
                            })(wh));
                        }
                    }
                }
            };
        });

        ctx.component(background);
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EditTarget {
    NewCharacter {
        cut_id: Uuid,
    },
    ExistingCharacter {
        cut_id: Uuid,
        graphic_index: Uuid,
    },
    ExistingCharacterPart {
        cut_id: Uuid,
        cg_id: Uuid,
        graphic_index: Uuid,
    },
}
