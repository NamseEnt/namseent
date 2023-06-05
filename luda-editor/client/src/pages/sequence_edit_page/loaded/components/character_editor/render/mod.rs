mod cg_picker;
mod part_picker;
mod text_content;
mod tooltip;

use super::*;
use crate::{color, pages::sequence_edit_page::cg_files_atom::CG_FILES_ATOM};
use namui_prebuilt::*;
use tooltip::*;

impl CharacterEditor {
    pub fn render(&self, props: Props) -> namui::RenderingTree {
        render([
            self.render_background(props.wh),
            self.render_content(props),
            self.render_tooltip(),
        ])
    }

    fn render_background(&self, wh: Wh<Px>) -> namui::RenderingTree {
        simple_rect(wh, color::STROKE_NORMAL, 1.px(), color::BACKGROUND)
            .attach_event(|builder| {
                builder.on_mouse_down_out(|_| {
                    namui::event::send(Event::MouseDownOutsideCharacterEditor)
                });
            })
            .with_tooltip_destroyer(self.tooltip.is_some())
    }

    fn render_content(&self, props: Props) -> namui::RenderingTree {
        let cg_file_list = CG_FILES_ATOM.get_unwrap();
        match self.edit_target {
            EditTarget::NewCharacter { .. } | EditTarget::ExistingCharacter { .. } => {
                self.render_cg_picker(props.wh, &cg_file_list, props.project_id)
            }
            EditTarget::ExistingCharacterPart {
                cg_id,
                cut_id,
                graphic_index,
            } => {
                let selected_cg_file = cg_file_list.iter().find(|cg_file| cg_file.id == cg_id);
                let selected_screen_graphic = props.cut.and_then(|cut| {
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
                    (Some(selected_cg_file), Some(ScreenGraphic::Cg(selected_screen_cg))) => self
                        .render_part_picker(
                            props.wh,
                            selected_cg_file,
                            props.project_id,
                            cut_id,
                            graphic_index,
                            selected_screen_cg,
                        ),
                    _ => self.render_text_content(
                        props.wh,
                        "Selected resource not found. Close character picker and try again.",
                    ),
                }
            }
        }
    }

    fn render_tooltip(&self) -> namui::RenderingTree {
        match &self.tooltip {
            Some(tooltip) => tooltip.render(),
            None => RenderingTree::Empty,
        }
    }
}
