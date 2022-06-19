use super::{RectSelectionCreate, RectSelectionResize};
use crate::app::cropper::selection::Selection;

pub enum Job {
    RectSelectionResize(RectSelectionResize),
    RectSelectionCreate(RectSelectionCreate),
}
impl Job {
    pub fn execute(&self, selection_list: Vec<Selection>) -> Vec<Selection> {
        match self {
            Job::RectSelectionResize(job) => job.execute(selection_list),
            Job::RectSelectionCreate(job) => job.execute(selection_list),
        }
    }
}

pub trait JobExecution {
    fn execute(&self, selection_list: Vec<Selection>) -> Vec<Selection>;
}
