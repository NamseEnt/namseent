mod image_cell;
mod text_cell;

use super::*;
pub use image_cell::*;
pub use text_cell::*;

pub trait Cell {
    fn render(&self, props: Props) -> RenderingTree;
    fn borders(&self) -> &Borders;
}

pub trait Border {
    fn border(self, side: Side, line: Line) -> Self;
}

#[derive(Clone, Copy, Debug)]
pub enum Side {
    Left,
    Top,
    Right,
    Bottom,
    LeftRight,
    TopBottom,
    All,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Line {
    None,
    Single,
    Double,
    BoldSingle,
}

///
/// border
/// border starts center pixel between cell and cell.
/// So, there iis 1px extra pixel for border.
///
/// ```text
/// ┌──────┐│┌──────┐
/// │ cell │││ cell │
/// └──────┘│└──────┘
///         ^This is the 1px for start of border.
///         Every cell's border starts from here and goes to inside of cell
/// ```
///
pub struct Borders {
    pub(crate) left: Line,
    pub(crate) top: Line,
    pub(crate) right: Line,
    pub(crate) bottom: Line,
}

impl Borders {
    pub fn new() -> Self {
        Self {
            left: Line::None,
            top: Line::None,
            right: Line::None,
            bottom: Line::None,
        }
    }

    fn add(&mut self, side: Side, line: Line) {
        match side {
            Side::Left => self.left = line,
            Side::Top => self.top = line,
            Side::Right => self.right = line,
            Side::Bottom => self.bottom = line,
            Side::LeftRight => {
                self.left = line;
                self.right = line;
            }
            Side::TopBottom => {
                self.top = line;
                self.bottom = line;
            }
            Side::All => {
                self.left = line;
                self.top = line;
                self.right = line;
                self.bottom = line;
            }
        }
    }
}

pub struct Props<'a> {
    pub wh: Wh<Px>,
    pub is_editing: bool,
    pub is_selected: bool,
    pub text_input: &'a TextInput,
    pub color_palette: ColorPalette,
}

pub struct EmptyCell {
    on_edit: Option<Box<dyn Fn()>>,
    borders: Borders,
}
pub fn empty() -> EmptyCell {
    EmptyCell {
        on_edit: None,
        borders: Borders::new(),
    }
}
impl Cell for EmptyCell {
    fn render(&self, _props: Props) -> RenderingTree {
        RenderingTree::Empty
    }

    fn borders(&self) -> &Borders {
        &self.borders
    }
}
impl EmptyCell {
    pub fn borders(mut self, side: Side, line: Line) -> Self {
        self.borders.add(side, line);
        self
    }
}
impl Into<Box<dyn Cell>> for EmptyCell {
    fn into(self) -> Box<dyn Cell> {
        Box::new(self)
    }
}
