use super::InitResult;
use crate::*;
use std::sync::{Mutex, OnceLock};

// TODO: Make it as atom and return sig only to user
static SCREEN_SIZE: OnceLock<Mutex<Wh<Px>>> = OnceLock::new();

pub(super) async fn init() -> InitResult {
    SCREEN_SIZE
        .set(Mutex::new(web::initial_window_size()))
        .unwrap();

    Ok(())
}

pub fn size() -> crate::Wh<Px> {
    SCREEN_SIZE.get().unwrap().lock().unwrap().clone()
}

pub(crate) fn resize(wh: Wh<Px>) {
    *SCREEN_SIZE.get().unwrap().lock().unwrap() = wh;
}
