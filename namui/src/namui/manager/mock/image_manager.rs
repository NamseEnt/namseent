use crate::namui::{self, skia::Image};
use dashmap::DashMap;
use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
};

pub struct ImageManager {
    image_map: DashMap<String, Arc<Image>>,
    image_requested_set: Mutex<HashSet<String>>,
}

impl ImageManager {
    pub fn new() -> Self {
        Self {
            image_map: DashMap::new(),
            image_requested_set: Mutex::new(HashSet::new()),
        }
    }
    pub fn try_load(&self, url: &String) -> Option<Arc<Image>> {
        todo!()
    }
}
