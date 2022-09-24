use std::sync::Arc;

use super::EditingTarget;
use crate::types::AnimationHistory;
use namui::{
    animation::{Animation, ImageInterpolation, Layer},
    prelude::*,
    types::Px,
};
use namui_prebuilt::*;

mod render;
mod update;

pub struct LineEditWindow {
    animation_history: AnimationHistory,
}

impl LineEditWindow {
    pub fn new(animation_history: AnimationHistory) -> Self {
        Self { animation_history }
    }
}

pub struct Props<'a> {
    pub wh: Wh<Px>,
    pub editing_target: Option<EditingTarget>,
    pub selected_layer: Option<&'a Layer>,
}

enum Event {
    SelectItem {
        line: ImageInterpolation,
        layer_id: namui::Uuid,
        point_id: namui::Uuid,
    },
    UpdateLine {
        layer_id: namui::Uuid,
        point_id: namui::Uuid,
        func: Arc<dyn Fn(&mut ImageInterpolation) + Send + Sync>,
    },
}
