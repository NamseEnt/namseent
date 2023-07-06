use super::*;
use crate::RPC;
use namui_prebuilt::*;

pub struct Props {
    pub wh: Wh<Px>,
}

impl LoadedSequenceEditorPage {
    pub fn render(&self, props: Props) -> namui::RenderingTree {
        let sequence = SEQUENCE_ATOM.get_unwrap();
        let cg_files = CG_FILES_ATOM.get_unwrap();
        let context_menu = match self.context_menu.as_ref() {
            Some(context_menu) => context_menu.render(),
            None => RenderingTree::Empty,
        };
        let selected_cut = self
            .selected_cut_id
            .and_then(|id| sequence.cuts.iter().find(|c| c.id == id));

        let memos_of_selected_cut = selected_cut.and_then(|cut| self.cut_id_memo_map.get(&cut.id));

        render([
            table::horizontal([
                table::fixed(220.px(), |wh| {
                    self.cut_list_view.render(cut_list_view::Props {
                        wh,
                        cuts: &sequence.cuts,
                        is_focused: self.focused_component == Some(FocusableComponent::CutListView),
                        selected_cut_id: self.selected_cut_id,
                        cut_id_memo_map: &self.cut_id_memo_map,
                    })
                }),
                table::ratio(4, |wh| {
                    self.cut_editor.render(cut_editor::Props {
                        wh,
                        cut: selected_cut,
                        is_focused: self.focused_component == Some(FocusableComponent::CutEditor),
                        cuts: &sequence.cuts,
                        project_id: self.project_id(),
                        cg_files: &cg_files,
                    })
                }),
                self.render_character_editor(selected_cut),
                self.render_memo_list_view(memos_of_selected_cut),
            ])(props.wh),
            context_menu,
            self.render_memo_editor(),
        ])
        .attach_event(|builder| {
            builder.on_file_drop(move |event| {
                for file in event.files.iter() {
                    if file.name() == "memos.json" {
                        namui::log!("{}", file.name());
                        let file = file.clone();
                        spawn_local(async move {
                            let memos: HashMap<usize, Vec<String>> =
                                serde_json::from_slice(&file.content().await).unwrap();
                            let sequence = SEQUENCE_ATOM.get_unwrap();
                            let total_length = memos.len();
                            const INFO_PRINT_INTERVAL: namui::Time = Time::Sec(1.0);
                            let mut last_info_printed_time = namui::now();
                            for (memos_index, (cut_index, memos)) in memos.into_iter().enumerate() {
                                let now = namui::now();
                                if now - last_info_printed_time >= INFO_PRINT_INTERVAL {
                                    namui::log!("memo uploading {memos_index}/{total_length}");
                                    last_info_printed_time = now;
                                }
                                if let Some(cut) = sequence.cuts.get(cut_index) {
                                    let cut_id = cut.id;
                                    for memo in memos {
                                        RPC.create_memo(rpc::create_memo::Request {
                                            sequence_id: sequence.id,
                                            cut_id,
                                            content: memo,
                                        })
                                        .await
                                        .unwrap();
                                    }
                                }
                            }
                        })
                    }
                }
            });
        })
    }

    fn render_character_editor<'a>(&'a self, cut: Option<&'a Cut>) -> table::TableCell {
        const CHARACTER_EDITOR_WIDTH: Px = px(496.0);

        match &self.character_editor {
            Some(character_editor) => table::fixed(CHARACTER_EDITOR_WIDTH, move |wh| {
                character_editor.render(character_editor::Props {
                    wh,
                    project_id: self.project_id(),
                    cut,
                })
            }),
            None => table::fixed(0.px(), |_| RenderingTree::Empty),
        }
    }

    fn render_memo_list_view<'a>(&'a self, memos: Option<&'a Vec<Memo>>) -> table::TableCell {
        const MEMO_WINDOW_WIDTH: Px = px(256.0);
        let sequence_id = SEQUENCE_ATOM.get_unwrap().id;

        if let Some(memos) = memos {
            if !memos.is_empty() {
                return table::fixed(MEMO_WINDOW_WIDTH, move |wh| {
                    self.memo_list_view
                        .render(components::memo_list_view::Props {
                            wh,
                            memos,
                            sequence_id,
                            user_id: self.user_id,
                        })
                });
            }
        }

        table::fixed(0.px(), |_| RenderingTree::Empty)
    }

    fn render_memo_editor(&self) -> RenderingTree {
        const MEMO_EDITOR_WH: Wh<Px> = Wh {
            width: px(512.0),
            height: px(256.0),
        };

        self.memo_editor
            .as_ref()
            .map_or(RenderingTree::Empty, |memo_editor| {
                memo_editor.render(components::memo_editor::Props { wh: MEMO_EDITOR_WH })
            })
    }
}
