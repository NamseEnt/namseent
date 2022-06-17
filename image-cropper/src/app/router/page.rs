use crate::app::{cropper::Cropper, file_selector::FileSelector};

pub enum Page {
    FileSelector(FileSelector),
    Cropper(Cropper),
}
