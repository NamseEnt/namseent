use super::*;
use namui_prebuilt::*;

pub struct Props {
    pub wh: Wh<Px>,
}

impl LoadedSequenceEditorPage {
    pub fn render(&self, props: Props) -> namui::RenderingTree {
        let context_menu = match self.context_menu.as_ref() {
            Some(context_menu) => context_menu.render(),
            None => RenderingTree::Empty,
        };
        let selected_cut = self
            .selected_cut_id
            .and_then(|id| self.sequence.cuts.iter().find(|c| c.id() == id));

        render([
            table::horizontal([
                table::fixed(220.px(), |wh| {
                    self.cut_list_view.render(cut_list_view::Props {
                        wh,
                        cuts: &self.sequence.cuts,
                        is_focused: self.focused_component == Some(FocusableComponent::CutListView),
                        selected_cut_id: self.selected_cut_id,
                    })
                }),
                table::ratio(4, |wh| {
                    self.cut_editor.render(cut_editor::Props {
                        wh,
                        cut: selected_cut,
                        is_focused: self.focused_component == Some(FocusableComponent::CutEditor),
                        cuts: &self.sequence.cuts,
                        project_id: self.project_id(),
                    })
                }),
                self.render_character_editor(selected_cut),
                self.render_memo_list_view(selected_cut),
            ])(props.wh),
            context_menu,
            self.render_memo_editor(),
        ])
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

    fn render_memo_list_view<'a>(&'a self, cut: Option<&'a Cut>) -> table::TableCell {
        const MEMO_WINDOW_WIDTH: Px = px(256.0);

        if let Some(cut) = cut {
            if !cut.memos.is_empty() {
                return table::fixed(MEMO_WINDOW_WIDTH, move |wh| {
                    self.memo_list_view
                        .render(components::memo_list_view::Props {
                            wh,
                            cut_id: cut.id(),
                            memos: &cut.memos,
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
