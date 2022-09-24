use super::{PolySelection, RectSelection};
use namui::prelude::*;

#[derive(Clone)]
pub enum Selection {
    RectSelection(RectSelection),
    PolySelection(PolySelection),
}
impl Selection {
    pub fn render(&self, scale: f32) -> RenderingTree {
        match self {
            Selection::RectSelection(selection) => selection.render(scale),
            Selection::PolySelection(selection) => selection.render(scale),
        }
    }

    pub fn get_polygon(&self) -> Vec<Xy<Px>> {
        match self {
            Selection::RectSelection(selection) => selection.get_polygon(),
            Selection::PolySelection(selection) => selection.get_polygon(),
        }
    }

    pub fn get_id(&self) -> Uuid {
        match self {
            Selection::RectSelection(selection) => selection.get_id(),
            Selection::PolySelection(selection) => selection.get_id(),
        }
    }
}
pub trait SelectionTrait {
    fn render(&self, scale: f32) -> RenderingTree;
    fn get_polygon(&self) -> Vec<Xy<Px>>;
    fn get_id(&self) -> Uuid;
}

pub trait SelectionListModify<M>
where
    M: FnMut(Selection) -> Selection,
{
    fn modify_selection(&self, id: Uuid, modifier: M) -> Self;
}
impl<M> SelectionListModify<M> for Vec<Selection>
where
    M: FnMut(Selection) -> Selection,
{
    fn modify_selection(&self, id: Uuid, mut modifier: M) -> Self {
        let new_selection_list = self.clone();
        new_selection_list
            .into_iter()
            .map(|selection| match selection.get_id() == id {
                true => modifier(selection),
                false => selection,
            })
            .collect()
    }
}
