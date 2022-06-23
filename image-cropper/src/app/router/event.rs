use crate::app::{cropper::Cropper, file_selector::FileSelector};

type PageInitializer<T> = Box<dyn Fn() -> T + Send + Sync>;

pub enum RouterEvent {
    PageChangeRequestedToFileSelectorEvent(PageInitializer<FileSelector>),
    PageChangeRequestedToCropperEvent(PageInitializer<Cropper>),
}
