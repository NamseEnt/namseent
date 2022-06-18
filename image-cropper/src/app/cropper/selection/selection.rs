use super::RectSelection;
use namui::{RenderingTree, Xy};

#[derive(Clone)]
pub enum Selection {
    RectSelection(RectSelection),
}
impl Selection {
    pub fn render(&self, scale: f32) -> RenderingTree {
        match self {
            Selection::RectSelection(selection) => selection.render(scale),
        }
    }

    pub fn get_polygon(&self) -> Vec<Xy<f32>> {
        match self {
            Selection::RectSelection(selection) => selection.get_polygon(),
        }
    }

    fn get_id(&self) -> &String {
        match self {
            Selection::RectSelection(selection) => selection.get_id(),
        }
    }
}
pub trait SelectionTrait {
    fn render(&self, scale: f32) -> RenderingTree;
    fn get_polygon(&self) -> Vec<Xy<f32>>;
    fn get_id(&self) -> &String;
}

pub trait SelectionListModify<M>
where
    M: FnMut(Selection) -> Selection,
{
    fn modify_selection(&self, id: impl AsRef<str>, modifier: M) -> Self;
}
impl<M> SelectionListModify<M> for Vec<Selection>
where
    M: FnMut(Selection) -> Selection,
{
    fn modify_selection(&self, id: impl AsRef<str>, mut modifier: M) -> Self {
        let new_selection_list = self.clone();
        new_selection_list
            .into_iter()
            .map(|selection| match selection.get_id() == id.as_ref() {
                true => modifier(selection),
                false => selection,
            })
            .collect()
    }
}
