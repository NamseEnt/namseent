use super::EditingTarget;
use crate::types::{ActionTicket, AnimationHistory};
use namui::{
    prelude::*,
    types::{Angle, Px, Time},
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

pub struct Props {
    pub wh: Wh<Px>,
    pub editing_target: Option<EditingTarget>,
}

enum Event {}
