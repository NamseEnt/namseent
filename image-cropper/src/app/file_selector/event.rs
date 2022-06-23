use namui::Image;
use std::sync::Arc;

pub enum FileSelectorEvent {
    FileSelectDialogOpenButtonClicked,
    NamuiImageMakeFailed(String),
    NamuiImagePrepared {
        image: Arc<Image>,
        url: String,
        name: String,
    },
}
