use super::*;
use rpc::data::ScreenCg;

impl CharacterEditor {
    pub fn update(&mut self, event: &namui::Event) {
        event.is::<InternalEvent>(|event| {
            match event {
                InternalEvent::OpenTooltip { global_xy, text } => {
                    self.tooltip = Some(Tooltip {
                        global_xy: *global_xy,
                        text: text.clone(),
                    });
                }
                InternalEvent::CloseTooltip => {
                    self.tooltip = None;
                }
                InternalEvent::CgChangeButtonClicked => {
                    if let EditTarget::ExistingCharacterPart {
                        cut_id,
                        graphic_index,
                        ..
                    } = self.edit_target
                    {
                        self.edit_target = EditTarget::ExistingCharacter {
                            cut_id,
                            graphic_index,
                        };
                    }
                }
                InternalEvent::CgFileLoadStateChanged(cg_file_load_state) => {
                    self.cg_file_load_state = cg_file_load_state.clone();
                }
                &InternalEvent::CgThumbnailClicked { cg_id } => match self.edit_target {
                    EditTarget::NewCharacter { cut_id } => {
                        namui::event::send(Event::UpdateCutGraphics {
                            cut_id,
                            callback: Box::new(move |graphics| {
                                let graphic_index = graphics.len();
                                graphics.push(ScreenGraphic::Cg(ScreenCg::new(cg_id, vec![])));
                                namui::event::send(InternalEvent::FocusCg {
                                    cut_id,
                                    cg_id,
                                    graphic_index,
                                })
                            }),
                        });
                    }
                    EditTarget::ExistingCharacter {
                        cut_id,
                        graphic_index,
                    } => {
                        self.edit_target = EditTarget::ExistingCharacterPart {
                            cut_id,
                            graphic_index,
                            cg_id,
                        };
                    }
                    _ => {}
                },
                &InternalEvent::FocusCg {
                    cut_id,
                    cg_id,
                    graphic_index,
                } => {
                    self.edit_target = EditTarget::ExistingCharacterPart {
                        cut_id,
                        cg_id,
                        graphic_index,
                    };
                }
            };
        });
        self.scroll_view.update(event);
    }
}
