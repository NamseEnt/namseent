mod background_with_event;
mod character_name_side;
mod cut_text_side;

use super::*;
use crate::{components::sequence_player, *};
use namui_prebuilt::*;
use std::collections::BTreeSet;

impl CutEditor {
    pub fn render(&self, props: Props) -> namui::RenderingTree {
        let Some(cut) = props.cut else {
            return RenderingTree::Empty;
        };

        let content_rect = sequence_player::get_inner_content_rect(props.wh);

        render([
            self.background_with_event(&props, cut),
            translate(
                content_rect.x(),
                content_rect.y(),
                render([
                    simple_rect(
                        content_rect.wh(),
                        color::STROKE_NORMAL,
                        1.px(),
                        color::BACKGROUND,
                    ),
                    props.cut.map_or(RenderingTree::Empty, |cut| {
                        self.image_wysiwyg_editor.render(wysiwyg_editor::Props {
                            wh: content_rect.wh(),
                            screen_graphics: &cut.screen_graphics,
                            project_id: props.project_id,
                            cut_id: cut.id,
                            cg_files: props.cg_files,
                        })
                    }),
                    sequence_player::render_text_box(content_rect.wh()),
                    sequence_player::render_over_text(
                        content_rect.wh(),
                        |wh| self.render_character_name_side(wh, &props, &cut),
                        |wh| self.render_cut_text_side(wh, &props, &cut),
                    ),
                ]),
            ),
            self.context_menu
                .as_ref()
                .map_or(RenderingTree::Empty, |context_menu| context_menu.render()),
        ])
    }
}

fn prev_cut_id(props: &Props, cut_id: Uuid) -> Option<Uuid> {
    props.cuts.iter().enumerate().find_map(|(i, cut)| {
        if cut.id == cut_id {
            if i == 0 {
                None
            } else {
                Some(props.cuts[i - 1].id)
            }
        } else {
            None
        }
    })
}

fn next_cut_id(props: &Props, cut_id: Uuid) -> Option<Uuid> {
    props.cuts.iter().enumerate().find_map(|(i, cut)| {
        if cut.id == cut_id {
            if i == props.cuts.len() - 1 {
                None
            } else {
                Some(props.cuts[i + 1].id)
            }
        } else {
            None
        }
    })
}
