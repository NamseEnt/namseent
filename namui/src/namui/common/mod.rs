use super::namui_state::{update_namui_state, NamuiState};
use super::render::{RenderingData, RenderingTree};
use super::skia::*;
use crate::event::EventReceiver;
use serde::{Deserialize, Serialize};
use serde_repr::*;
use std::collections::HashSet;
use std::time::Duration;
use strum_macros::EnumIter;
mod xy;
pub use xy::*;
mod set_timeout;
pub use set_timeout::*;
mod request_animation_frame;
pub use request_animation_frame::*;

pub struct FpsInfo {
    pub fps: u16,
    pub frame_count: u16,
    pub last_60_frame_time: Duration,
}

pub struct NamuiContext {
    pub(crate) surface: Surface,
    pub(crate) fps_info: FpsInfo,
    pub(crate) rendering_tree: RenderingTree,
    pub(crate) event_receiver: EventReceiver,
}
impl NamuiContext {
    pub fn get_rendering_tree_xy(&self, id: &str) -> Option<Xy<f32>> {
        self.rendering_tree.get_xy(id)
    }
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

impl std::convert::From<RenderingData> for RenderingTree {
    fn from(data: RenderingData) -> Self {
        RenderingTree::Node(data)
    }
}

impl std::convert::From<Vec<RenderingTree>> for RenderingTree {
    fn from(vector: Vec<RenderingTree>) -> Self {
        RenderingTree::Children(vector)
    }
}

#[macro_export]
macro_rules! __rust_force_expr {
    ($e:expr) => {
        $e
    };
}

/// $x type
/// - namui::RenderingTree
/// - namui::RenderingData
#[macro_export]
macro_rules! render {
    ( $( $x:expr ),+ $(,)? ) => (
        $crate::__rust_force_expr!(
            {
                let mut temp_vec = Vec::new();
                $(
                    let rendering_tree = $crate::RenderingTree::from($x);
                    temp_vec.push(rendering_tree);
                )*
                if temp_vec.len() == 1 {
                    temp_vec.swap_remove(0)
                } else {
                    $crate::RenderingTree::Children(temp_vec)
                }
            }
        )
    );
    () => (
        $crate::RenderingTree::Empty
    );
}

pub use render;

pub type Rendering = RenderingTree;

#[derive(Debug, Clone, Copy, Serialize)]
pub struct Wh<T> {
    pub width: T,
    pub height: T,
}
impl<T> Wh<T> {
    pub fn new(width: T, height: T) -> Self {
        Self { width, height }
    }
}
impl Wh<f32> {
    pub fn length(&self) -> f32 {
        (self.width * self.width + self.height * self.height).sqrt()
    }
}
impl Wh<f64> {
    pub fn length(&self) -> f64 {
        (self.width * self.width + self.height * self.height).sqrt()
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

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
}

pub struct RawMouseEvent {
    pub xy: Xy<f32>,
    pub buttons: HashSet<MouseButton>,
}
