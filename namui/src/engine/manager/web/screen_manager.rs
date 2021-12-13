use crate::engine::{self};
use std::sync::{Arc, RwLock};
use wasm_bindgen::{prelude::Closure, JsCast};

pub struct ScreenManager {
    screen_size: Arc<RwLock<(i16, i16)>>,
}

impl ScreenManager {
    pub fn screen_size(&self) -> (i16, i16) {
        (*self.screen_size).read().unwrap().clone()
    }
    pub fn new() -> Self {
        let window = engine::window();
        let screen_size = Arc::new(RwLock::new((
            window.inner_width().unwrap().as_f64().unwrap() as i16,
            window.inner_height().unwrap().as_f64().unwrap() as i16,
        )));

        let manager = ScreenManager {
            screen_size: screen_size.clone(),
        };

        let closure = Closure::wrap(Box::new(move || {
            let window = engine::window();
            let mut screen_size = screen_size.write().unwrap();
            *screen_size = (
                window.inner_width().unwrap().as_f64().unwrap() as i16,
                window.inner_height().unwrap().as_f64().unwrap() as i16,
            );
            engine::event::send(Box::new(engine::event::EngineEvent::ScreenResize(
                engine::Wh {
                    width: screen_size.0,
                    height: screen_size.1,
                },
            )));
        }) as Box<dyn FnMut()>);

        window
            .add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref())
            .unwrap();

        closure.forget();

        manager
    }
}
