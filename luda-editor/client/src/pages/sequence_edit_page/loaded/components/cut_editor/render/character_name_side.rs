use super::*;

impl CutEditor {
    pub fn render_character_name_side(
        &self,
        wh: Wh<Px>,
        props: &Props,
        cut: &Cut,
    ) -> RenderingTree {
        let character_name = cut.character_name.clone();

        render([
            transparent_rect(wh),
            if props.is_focused && self.selected_target == Some(ClickTarget::CharacterName) {
                let cut_id = cut.id();
                let prev_cut_id = prev_cut_id(&props, cut_id);

                self.character_name_input
                    .render(auto_complete_text_input::Props {
                        text: character_name,
                        wh,
                        candidates: get_character_name_candidates(&props.cuts, &cut),
                        on_text_change: move |text| {
                            namui::event::send(Event::ChangeCharacterName { name: text, cut_id })
                        },
                        on_edit_done: move || {
                            namui::event::send(Event::Click {
                                target: ClickTarget::CutText,
                            })
                        },
                        on_key_down: move |event| {
                            if event.code == Code::Tab && !event.is_composing {
                                event.prevent_default();
                                if namui::keyboard::any_code_press([
                                    Code::ShiftLeft,
                                    Code::ShiftRight,
                                ]) {
                                    let Some(prev_cut_id) = prev_cut_id else {
                                        return
                                    };
                                    namui::event::send(Event::MoveCutRequest {
                                        cut_id: prev_cut_id,
                                        to_prev: true,
                                        focused: true,
                                    })
                                } else {
                                    namui::event::send(Event::Click {
                                        target: ClickTarget::CutText,
                                    })
                                }
                            } else if event.code == Code::Escape {
                                namui::event::send(InternalEvent::EscapeKeyDown)
                            }
                        },
                    })
            } else {
                text(TextParam {
                    text: character_name,
                    x: 0.px(),
                    y: wh.height / 2.0,
                    align: TextAlign::Left,
                    baseline: TextBaseline::Middle,
                    font_type: sequence_player::CHARACTER_NAME_FONT,
                    style: sequence_player::character_name_text_style(1.one_zero()),
                    max_width: Some(wh.width),
                })
            },
        ])
        .attach_event(|builder| {
            builder.on_mouse_down_in(|event: MouseEvent| {
                if event.button == Some(MouseButton::Left) {
                    namui::event::send(Event::Click {
                        target: ClickTarget::CharacterName,
                    })
                }
            });
        })
    }
}

fn get_character_name_candidates(cuts: &[Cut], current_cut: &Cut) -> Vec<String> {
    let character_name = &current_cut.character_name;

    let mut candidates = cuts
        .iter()
        .enumerate()
        .map(|(index, cut)| (index, cut.character_name.clone()))
        .filter(|(_, name)| name != character_name)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();

    let cut_index = cuts
        .iter()
        .position(|cut| cut.id() == current_cut.id())
        .unwrap();

    candidates.sort_by_key(|(index, _)| (cut_index as isize - *index as isize).abs());

    candidates
        .into_iter()
        .map(|(_, name)| name)
        .collect::<Vec<_>>()
}
