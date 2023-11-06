use namui::prelude::*;

pub static FOCUSED_COMPONENT: Atom<Option<FocusableComponent>> = Atom::uninitialized_new();

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FocusableComponent {
    CutListView,
    CutEditor,
}
