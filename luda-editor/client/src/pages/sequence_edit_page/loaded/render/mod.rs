mod line_list;
mod top_bar;

use super::*;
use namui_prebuilt::*;

pub struct Props {
    pub wh: Wh<Px>,
}

impl LoadedSequenceEditorPage {
    pub fn render(&self, props: Props) -> namui::RenderingTree {
        if let Some(sequence_player) = &self.sequence_player {
            return table::vertical([
                table::fixed(20.px(), |wh| self.render_top_bar_for_player(wh)),
                table::ratio(1.0, |wh| {
                    sequence_player.render(sequence_player::Props { wh })
                }),
            ])(props.wh);
        }

        let sequence = &self.sequence;
        let characters = &self.project_shared_data.characters;

        let modal = render([
            match &self.character_edit_modal {
                Some(character_edit_modal) => {
                    let character_cell_right = 40.px() * 2.0 / 3.0;
                    translate(
                        character_cell_right,
                        0.px(),
                        character_edit_modal.render(character_edit_modal::Props {
                            wh: props.wh,
                            characters: &characters,
                        }),
                    )
                }
                None => RenderingTree::Empty,
            },
            match &self.image_select_modal {
                Some(image_select_modal) => {
                    let modal_wh = props.wh * 2.0 / 3.0;
                    let xy = ((props.wh - modal_wh) / 2.0).as_xy();
                    translate(
                        xy.x,
                        xy.y,
                        image_select_modal.render(image_select_modal::Props {
                            wh: modal_wh,
                            recent_selected_image_ids: &self.recent_selected_image_ids,
                        }),
                    )
                }
                None => RenderingTree::Empty,
            },
        ]);

        render([
            table::vertical([
                table::fixed(20.px(), |wh| {
                    self.render_top_bar_for_editor(
                        wh,
                        &sequence,
                        self.sequence_syncer.get_sync_status(),
                    )
                }),
                table::ratio(
                    1.0,
                    table::horizontal([table::ratio(1.0, |wh| {
                        self.render_line_list(wh, &sequence, &characters)
                    })]),
                ),
            ])(props.wh),
            modal,
            self.context_menu
                .as_ref()
                .map(|context_menu| context_menu.render())
                .unwrap_or(RenderingTree::Empty),
        ])
    }
}
