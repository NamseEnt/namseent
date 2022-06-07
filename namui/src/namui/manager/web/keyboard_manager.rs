use crate::Code;
use std::{
    collections::HashSet,
    str::FromStr,
    sync::{Arc, RwLock},
};
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::window;

pub struct KeyboardManager {
    pressing_code_set: Arc<RwLock<HashSet<Code>>>,
}

impl KeyboardManager {
    pub fn any_code_press(&self, codes: impl IntoIterator<Item = Code>) -> bool {
        let pressing_code_set = self.pressing_code_set.read().unwrap();
        for code in codes {
            if pressing_code_set.contains(&code) {
                return true;
            }
        }
        false
    }
    pub fn new() -> Self {
        let pressing_code_set = Arc::new(RwLock::new(HashSet::new()));
        let document = window().unwrap().document().unwrap();

        document
            .add_event_listener_with_callback(
                "keydown",
                Closure::wrap(Box::new({
                    let pressing_code_set = pressing_code_set.clone();
                    move |event: web_sys::KeyboardEvent| {
                        let code_string = event.code();
                        let code = Code::from_str(&code_string);
                        if code.is_err() {
                            crate::log!(
                                "[DEBUG] Fail to get code from key_down callback {}",
                                code_string
                            );
                            return;
                        }
                        let code = code.unwrap();
                        pressing_code_set.write().unwrap().insert(code);

                        match code {
                            Code::Space
                            | Code::ArrowLeft
                            | Code::ArrowRight
                            | Code::ArrowUp
                            | Code::ArrowDown
                            | Code::AltLeft
                            | Code::AltRight => {
                                event.prevent_default();
                            }
                            _ => {}
                        }

                        crate::event::send(crate::NamuiEvent::KeyDown(crate::KeyEvent { code }));
                    }
                }) as Box<dyn FnMut(_)>)
                .into_js_value()
                .unchecked_ref(),
            )
            .unwrap();

        document
            .add_event_listener_with_callback(
                "keyup",
                Closure::wrap(Box::new({
                    let pressing_code_set = pressing_code_set.clone();
                    move |event: web_sys::KeyboardEvent| {
                        let code_string = event.code();
                        let code = Code::from_str(&code_string);
                        if code.is_err() {
                            crate::log!(
                                "[DEBUG] Fail to get code from key_up callback {}",
                                code_string
                            );
                            return;
                        }
                        let code = code.unwrap();
                        pressing_code_set.write().unwrap().remove(&code);
                    }
                }) as Box<dyn FnMut(_)>)
                .into_js_value()
                .unchecked_ref(),
            )
            .unwrap();

        ["blur", "visibilitychange"].iter().for_each(|event_name| {
            document
                .add_event_listener_with_callback(
                    *event_name,
                    Closure::wrap(Box::new({
                        let pressing_code_set = pressing_code_set.clone();
                        move || {
                            pressing_code_set.write().unwrap().clear();
                        }
                    }) as Box<dyn FnMut()>)
                    .into_js_value()
                    .unchecked_ref(),
                )
                .unwrap();
        });

        KeyboardManager {
            pressing_code_set: pressing_code_set.clone(),
        }
    }
}
