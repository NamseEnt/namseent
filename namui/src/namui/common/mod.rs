mod codes;
mod open_external;
mod request_animation_frame;
mod set_timeout;
pub mod types;

use super::render::{RenderingData, RenderingTree};
use crate::*;
pub use codes::*;
pub use open_external::*;
pub use request_animation_frame::*;
use serde::{Deserialize, Serialize};
use serde_repr::*;
pub use set_timeout::*;
use std::collections::HashSet;
pub use types::*;

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
macro_rules! render_macro {
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

pub use render_macro as render;

pub fn render(rendering_trees: impl IntoIterator<Item = RenderingTree>) -> RenderingTree {
    RenderingTree::Children(rendering_trees.into_iter().collect())
}

pub fn try_render(func: impl FnOnce() -> Option<RenderingTree>) -> RenderingTree {
    func().unwrap_or(RenderingTree::Empty)
}

pub type Rendering = RenderingTree;

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy, Serialize, Deserialize)]
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
    pub size: IntPx,
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

#[derive(Debug)]
pub struct RawMouseEvent {
    pub id: String,
    pub xy: Xy<Px>,
    pub pressing_buttons: HashSet<MouseButton>,
    pub button: Option<MouseButton>,
}

#[derive(Debug)]
pub struct RawWheelEvent {
    pub id: String,
    /// NOTE: https://devblogs.microsoft.com/oldnewthing/20130123-00/?p=5473
    pub delta_xy: Xy<f32>,
}

#[derive(Debug)]
pub struct RawKeyboardEvent {
    pub id: String,
    pub code: Code,
    pub pressing_codes: HashSet<Code>,
}

#[derive(Debug)]
pub struct DeepLinkOpenedEvent {
    pub url: String,
}
