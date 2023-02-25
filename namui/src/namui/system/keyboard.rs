use super::InitResult;
use crate::Code;
use std::{
    collections::HashSet,
    str::FromStr,
    sync::{Arc, RwLock},
};
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::window;

struct KeyboardSystem {
    pressing_code_set: Arc<RwLock<HashSet<Code>>>,
}

lazy_static::lazy_static! {
    static ref KEYBOARD_SYSTEM: Arc<KeyboardSystem> = Arc::new(KeyboardSystem::new());
}

pub(super) async fn init() -> InitResult {
    lazy_static::initialize(&KEYBOARD_SYSTEM);
    Ok(())
}

impl KeyboardSystem {
    pub fn new() -> Self {
        let pressing_code_set = Arc::new(RwLock::new(HashSet::new()));
        let document = window().unwrap().document().unwrap();

        document
            .add_event_listener_with_callback(
                "keydown",
                Closure::wrap(Box::new({
                    let pressing_code_set = pressing_code_set.clone();
                    move |event: web_sys::KeyboardEvent| {
                        let code = Code::from_str(&event.code()).unwrap();
                        record_key_down(code);

                        event.prevent_default();

                        crate::event::send(crate::NamuiEvent::KeyDown(crate::RawKeyboardEvent {
                            id: crate::uuid(),
                            code,
                            pressing_codes: pressing_code_set.read().unwrap().clone(),
                        }));
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
                        let code = Code::from_str(&event.code()).unwrap();
                        let mut pressing_code_set = pressing_code_set.write().unwrap();
                        pressing_code_set.remove(&code);

                        crate::event::send(crate::NamuiEvent::KeyUp(crate::RawKeyboardEvent {
                            id: crate::uuid(),
                            code,
                            pressing_codes: pressing_code_set.clone(),
                        }));
                    }
                }) as Box<dyn FnMut(_)>)
                .into_js_value()
                .unchecked_ref(),
            )
            .unwrap();

        let reset_pressing_code_set_closure = Closure::wrap(Box::new({
            let pressing_code_set = pressing_code_set.clone();
            move || {
                pressing_code_set.write().unwrap().clear();
            }
        }) as Box<dyn FnMut()>)
        .into_js_value();

        ["blur", "visibilitychange"].iter().for_each(|event_name| {
            document
                .add_event_listener_with_callback(
                    *event_name,
                    reset_pressing_code_set_closure.unchecked_ref(),
                )
                .unwrap();
        });
        window()
            .unwrap()
            .add_event_listener_with_callback(
                "blur",
                reset_pressing_code_set_closure.unchecked_ref(),
            )
            .unwrap();

        KeyboardSystem {
            pressing_code_set: pressing_code_set.clone(),
        }
    }
}

pub fn any_code_press(codes: impl IntoIterator<Item = Code>) -> bool {
    let pressing_code_set = KEYBOARD_SYSTEM.pressing_code_set.read().unwrap();
    for code in codes {
        if pressing_code_set.contains(&code) {
            return true;
        }
    }
    false
}

pub(crate) fn record_key_down(code: Code) {
    let mut pressing_code_set = KEYBOARD_SYSTEM.pressing_code_set.write().unwrap();
    pressing_code_set.insert(code);
}

pub fn shift_press() -> bool {
    any_code_press([Code::ShiftLeft, Code::ShiftRight])
}
pub fn ctrl_press() -> bool {
    any_code_press([Code::ControlLeft, Code::ControlRight])
}
pub fn alt_press() -> bool {
    any_code_press([Code::AltLeft, Code::AltRight])
}
