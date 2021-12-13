use super::namui_state::{update_namui_state, NamuiState};
use super::render::{RenderingData, RenderingTree};
use super::skia::*;
use serde::{Deserialize, Serialize};
use serde_repr::*;
use std::time::Duration;
use strum_macros::EnumIter;

pub struct FpsInfo {
    pub fps: u16,
    pub frame_count: u16,
    pub last_60_frame_time: Duration,
}

pub struct NamuiContext {
    pub surface: Surface,
    pub fps_info: FpsInfo,
}

pub trait NamuiImpl {
    fn init() -> NamuiContext;
    fn request_animation_frame(callback: Box<dyn FnOnce()>);
    fn log(format: String);
    fn now() -> Duration;
}

pub struct NamuiInternal {}
impl NamuiInternal {
    pub fn update_state(namui_state: NamuiState) {
        update_namui_state(namui_state);
    }
}

pub struct Namui;

impl RenderingTree {
    pub fn into_rendering_tree(self) -> RenderingTree {
        self
    }
}

impl RenderingData {
    pub fn into_rendering_tree(self) -> RenderingTree {
        RenderingTree::Node(self)
    }
}

/// $x type
/// - namui::RenderingTree
/// - namui::RenderingData
#[macro_export]
macro_rules! render {
    ( $( $x:expr ),+ $(,)? ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                let rendering_tree = $x.into_rendering_tree();
                temp_vec.push(rendering_tree);
            )*
            if temp_vec.len() == 1 {
                temp_vec.swap_remove(0)
            } else {
                $crate::RenderingTree::Children(temp_vec)
            }
        }
    };
    () => (
        $crate::RenderingTree::Empty
    );
}

pub use render;

pub type Rendering = RenderingTree;

#[derive(Debug, Clone, Copy)]
pub struct Xy<T> {
    pub x: T,
    pub y: T,
}

#[derive(Debug, Clone, Copy)]
pub struct Wh<T> {
    pub width: T,
    pub height: T,
}
impl<T> Wh<T> {
    pub fn new(width: T, height: T) -> Self {
        Self { width, height }
    }
}

#[derive(Debug, Hash, Eq, PartialEq, EnumIter, Clone, Copy, Serialize, Deserialize)]
pub enum Language {
    Ko,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy, Serialize_repr, Deserialize_repr)]
#[repr(u16)]
pub enum FontWeight {
    _100 = 100,
    _200 = 200,
    _300 = 300,
    _400 = 400,
    _500 = 500,
    _600 = 600,
    _700 = 700,
    _800 = 800,
    _900 = 900,
}
impl FontWeight {
    pub const THIN: FontWeight = FontWeight::_100;
    pub const LIGHT: FontWeight = FontWeight::_300;
    pub const REGULAR: FontWeight = FontWeight::_400;
    pub const MEDIUM: FontWeight = FontWeight::_500;
    pub const BOLD: FontWeight = FontWeight::_700;
    pub const BLACK: FontWeight = FontWeight::_900;

    pub fn iter() -> impl Iterator<Item = FontWeight> {
        vec![
            FontWeight::_100,
            FontWeight::_200,
            FontWeight::_300,
            FontWeight::_400,
            FontWeight::_500,
            FontWeight::_600,
            FontWeight::_700,
            FontWeight::_800,
            FontWeight::_900,
        ]
        .into_iter()
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy, Serialize)]
pub struct FontType {
    pub serif: bool,
    pub size: i16,
    pub language: Language,
    pub font_weight: FontWeight,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub struct TypefaceType {
    pub serif: bool,
    pub language: Language,
    pub font_weight: FontWeight,
}
