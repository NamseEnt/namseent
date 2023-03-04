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
                        cut: self
                            .selected_cut_id
                            .and_then(|id| self.sequence.cuts.iter().find(|c| c.id() == id)),
                        is_focused: self.focused_component == Some(FocusableComponent::CutEditor),
                        cuts: &self.sequence.cuts,
                        project_id: self.project_id(),
                    })
                }),
            ])(props.wh),
            context_menu,
        ])
    }
}
