use namui::*;
use rpc::data::ImageWithLabels;

pub static IMAGES_ATOM: Atom<Vec<ImageWithLabels>> = Atom::uninitialized_new();
