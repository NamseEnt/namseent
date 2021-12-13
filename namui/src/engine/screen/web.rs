use crate::engine;

pub fn size() -> engine::Wh<i16> {
    let window = engine::window();
    engine::Wh {
        width: window.inner_width().unwrap().as_f64().unwrap() as i16,
        height: window.inner_height().unwrap().as_f64().unwrap() as i16,
    }
}
