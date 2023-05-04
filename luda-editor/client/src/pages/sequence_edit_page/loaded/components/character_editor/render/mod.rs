mod cg_picker;
mod part_picker;
mod text_content;
mod tooltip;

use super::*;
use crate::color;
use namui_prebuilt::*;
use tooltip::*;

impl CharacterEditor {
    pub fn render(&self, props: Props) -> namui::RenderingTree {
        render([
            self.render_background(props),
            self.render_content(props),
            self.render_tooltip(),
        ])
    }

    fn render_background(&self, props: Props) -> namui::RenderingTree {
        simple_rect(props.wh, color::STROKE_NORMAL, 1.px(), color::BACKGROUND)
            .attach_event(|builder| {
                builder.on_mouse_down_out(|_| {
                    namui::event::send(Event::MouseDownOutsideCharacterEditor)
                });
            })
            .with_tooltip_destroyer(self.tooltip.is_some())
    }

    fn render_content(&self, props: Props) -> namui::RenderingTree {
        match &self.cg_file_load_state {
            CgFileLoadState::Loading => self.render_text_content(props.wh, "Loading..."),
            CgFileLoadState::Failed { error } => self.render_text_content(props.wh, error),
            CgFileLoadState::Loaded(cg_file_list) => match self.edit_target {
                EditTarget::NewCharacter { .. } | EditTarget::ExistingCharacter { .. } => {
                    self.render_cg_picker(props.wh, &cg_file_list, props.project_id)
                }
                EditTarget::NewCharacterPart { cg_id, .. }
                | EditTarget::ExistingCharacterPart { cg_id, .. } => {
                    let selected_cg_file = cg_file_list.iter().find(|cg_file| cg_file.id == cg_id);
                    match selected_cg_file {
                        Some(selected_cg_file) => {
                            self.render_part_picker(props.wh, selected_cg_file, props.project_id)
                        }
                        None => self.render_text_content(
                            props.wh,
                            "No CG file found. Please close character picker and try again.",
                        ),
                    }
                }
            },
        }
    }

    fn render_tooltip(&self) -> namui::RenderingTree {
        match &self.tooltip {
            Some(tooltip) => tooltip.render(),
            None => RenderingTree::Empty,
        }
    }
}
