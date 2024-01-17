use web_sys::Window;

pub fn window() -> Window {
    web_sys::window().unwrap()
}

pub fn document() -> web_sys::Document {
    window().document().unwrap()
}
