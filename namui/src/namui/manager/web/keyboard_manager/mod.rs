use std::{
    collections::HashSet,
    str::FromStr,
    sync::{Arc, RwLock},
};
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{window, HtmlElement};
mod codes;
pub use codes::*;

use crate::namui;

pub struct KeyboardManager {
    pub pressing_code_set: Arc<RwLock<HashSet<Code>>>,
}

impl KeyboardManager {
    pub fn any_code_press(&self, codes: Vec<Code>) -> bool {
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

        let pressing_code_set_key_down = pressing_code_set.clone();
        let key_down_closure = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
            let code_string = event.code();
            let code = Code::from_str(&code_string).unwrap();
            pressing_code_set_key_down.write().unwrap().insert(code);
            namui::log(format!("key down: {}", code_string));
        }) as Box<dyn FnMut(_)>);

        let pressing_code_set_key_up = pressing_code_set.clone();
        let key_up_closure = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
            let code_string = event.code();
            let code = Code::from_str(&code_string).unwrap();
            pressing_code_set_key_up.write().unwrap().remove(&code);
            namui::log(format!("key up: {}", code_string));
        }) as Box<dyn FnMut(_)>);

        let pressing_code_set_clear = pressing_code_set.clone();
        let clear_closure = Closure::wrap(Box::new(move || {
            pressing_code_set_clear.write().unwrap().clear();
        }) as Box<dyn FnMut()>);

        let document = window().unwrap().document().unwrap();

        document
            .add_event_listener_with_callback("keydown", key_down_closure.as_ref().unchecked_ref())
            .unwrap();

        document
            .add_event_listener_with_callback("keyup", key_up_closure.as_ref().unchecked_ref())
            .unwrap();

        ["blur", "visibilitychange"].iter().for_each(|event_name| {
            document
                .add_event_listener_with_callback(
                    event_name,
                    clear_closure.as_ref().unchecked_ref(),
                )
                .unwrap();
        });

        key_down_closure.forget();
        key_up_closure.forget();
        clear_closure.forget();

        KeyboardManager {
            pressing_code_set: pressing_code_set.clone(),
        }
    }
}
