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

        let is_any_line_text_input_focused = self
            .line_text_inputs
            .iter()
            .any(|(_, text_input)| text_input.is_focused());

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
                    if let Some(cut) = self
                        .sequence
                        .cuts
                        .iter()
                        .find(|cut| cut.id() == image_select_modal.cut_id)
                    {
                        image_select_modal.render(image_select_modal::Props {
                            wh: props.wh,
                            recent_selected_image_ids: &self.recent_selected_image_ids,
                            cut,
                            project_shared_data: &self.project_shared_data,
                        })
                    } else {
                        RenderingTree::Empty
                    }
                }
                None => RenderingTree::Empty,
            },
            match &self.image_manager_modal {
                Some(image_manager_modal) => {
                    image_manager_modal.render(image_manager_modal::Props { wh: props.wh })
                }
                None => RenderingTree::Empty,
            },
        ])
        .attach_event(|builder| {
            builder
                .on_mouse_down_in(|event| {
                    event.stop_propagation();
                })
                .on_key_down(move |event| {
                    if code_composites_on(
                        event,
                        [
                            vec![Code::ControlLeft, Code::KeyY],
                            vec![Code::ControlLeft, Code::ShiftLeft, Code::KeyZ],
                        ],
                    ) && !is_any_line_text_input_focused
                    {
                        namui::event::send(Event::RedoSequenceChange);
                    } else if code_composites_on(event, [vec![Code::ControlLeft, Code::KeyZ]])
                        && !is_any_line_text_input_focused
                    {
                        namui::event::send(Event::UndoSequenceChange);
                    } else if code_composites_on(event, [vec![Code::Escape]]) {
                        namui::event::send(Event::EscapeKeyDown);
                    } else if code_composites_on(event, [vec![Code::ControlLeft, Code::Enter]])
                        && is_any_line_text_input_focused
                    {
                        namui::event::send(Event::CtrlEnterKeyDown);
                    }

                    fn code_composites_on(
                        event: &KeyboardEvent,
                        iter: impl IntoIterator<Item = Vec<Code>>,
                    ) -> bool {
                        iter.into_iter().any(|codes| {
                            codes
                                .into_iter()
                                .all(|code| event.pressing_codes.contains(&code))
                        })
                    }
                });
        });

        render([
            table::horizontal([
                table::ratio(1, |_wh| RenderingTree::Empty),
                table::ratio(
                    4,
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
                    ]),
                ),
                table::ratio(1, |_wh| RenderingTree::Empty),
            ])(props.wh),
            modal,
            self.context_menu
                .as_ref()
                .map(|context_menu| context_menu.render())
                .unwrap_or(RenderingTree::Empty),
        ])
    }
}
