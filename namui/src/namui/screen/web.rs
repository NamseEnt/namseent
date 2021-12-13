use crate::namui;

pub fn size() -> namui::Wh<i16> {
    let window = namui::window();
    namui::Wh {
        width: window.inner_width().unwrap().as_f64().unwrap() as i16,
        height: window.inner_height().unwrap().as_f64().unwrap() as i16,
    }
}
