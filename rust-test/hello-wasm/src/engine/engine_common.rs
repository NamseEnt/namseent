use super::draw::{RenderingData, RenderingTree};
use super::manager::*;
use async_trait::*;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use strum_macros::EnumIter;

pub trait Surface {
    fn flush(&self);
}

pub trait Canvas {}

pub struct FpsInfo {
    pub fps: u16,
    pub frame_count: u16,
    pub last_60_frame_time: Duration,
}

pub struct EngineContext<TState> {
    pub state: TState,
    pub surface: Box<dyn Surface>,
    pub canvas: Box<dyn Canvas>,
    pub fps_info: FpsInfo,
    pub render: Render<TState>,
    pub mouse_manager: Box<dyn MouseManager>,
    pub typeface_manager: Box<dyn TypefaceManager>,
}

#[async_trait]
pub trait EngineImpl {
    async fn init<TState: std::marker::Send>(
        state: TState,
        render: Render<TState>,
    ) -> EngineContext<TState>;
    fn request_animation_frame(callback: Box<dyn FnOnce()>);
    fn log(format: String);
    fn now() -> Duration;
}

pub type Render<TState> = fn(&EngineState, &mut TState) -> Option<RenderingTree>;

#[macro_export]
macro_rules! render_func(
    ($_func_name:ident, $_state_type:ty, $_state_identity:ident, $body:expr) => (
        paste::item! {
            fn [<render_ $ _func_name>] ($_state_identity: &mut $_state_type) -> Option<RenderingTree> { $body }
        }
    )
);

pub trait ToTree {
    fn process(self) -> Option<RenderingTree>;
}

impl ToTree for Option<RenderingTree> {
    fn process(self) -> Option<RenderingTree> {
        self
    }
}

impl ToTree for RenderingData {
    fn process(self) -> Option<RenderingTree> {
        Some(RenderingTree::Node(self))
    }
}

#[macro_export]
macro_rules! render {
    ( $( $x:expr ),* ) => {
        {

            let mut temp_vec: Vec<Option<RenderingTree>> = Vec::new();
            $(
                let option_rendering_tree = ToTree::process($x);
                temp_vec.push(option_rendering_tree);
            )*
            Some(RenderingTree::Children(temp_vec))
        }
    };
}

pub type Rendering = Option<RenderingTree>;

#[derive(Debug, Clone, Copy)]
pub struct Xy<T> {
    pub x: T,
    pub y: T,
}

pub struct EngineState {
    pub mouse_position: Xy<i16>,
}
#[derive(Debug, Hash, Eq, PartialEq, EnumIter, Clone, Copy, Serialize, Deserialize)]
pub enum Language {
    Ko,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub struct FontWeight(pub u16);
impl FontWeight {
    pub const THIN: FontWeight = FontWeight(100);
    pub const LIGHT: FontWeight = FontWeight(300);
    pub const REGULAR: FontWeight = FontWeight(400);
    pub const MEDIUM: FontWeight = FontWeight(500);
    pub const BOLD: FontWeight = FontWeight(700);
    pub const BLACK: FontWeight = FontWeight(900);

    pub fn iter() -> impl Iterator<Item = FontWeight> {
        (1..=9).map(|x| FontWeight(x * 100))
    }
}

#[derive(Hash, Eq, PartialEq)]
pub struct FontType {
    serif: bool,
    size: i16,
    language: Language,
    font_weight: FontWeight,
}
pub trait Font {}

pub trait Typeface {}

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
pub struct TypefaceType {
    pub serif: bool,
    pub language: Language,
    pub font_weight: FontWeight,
}
