pub mod image;
pub mod path;
pub mod text;
use super::skia::StrokeOptions;
use crate::*;
pub use image::ImageDrawCommand;
pub use path::PathDrawCommand;
use serde::Serialize;
pub use text::{TextAlign, TextBaseline, TextDrawCommand};

#[derive(Serialize, Clone, Debug, PartialEq)]
pub enum DrawCommand {
    Path(PathDrawCommand),
    Image(ImageDrawCommand),
    Text(TextDrawCommand),
}

#[derive(Serialize, Clone, Debug, PartialEq)]
pub struct DrawCall {
    pub commands: Vec<DrawCommand>,
}

impl DrawCall {
    pub fn draw(&self) {
        self.commands.iter().for_each(|command| {
            command.draw();
        });
    }

    pub(crate) fn get_bounding_box(&self) -> Option<Rect<Px>> {
        self.commands
            .iter()
            .map(|command| command.get_bounding_box())
            .filter_map(|bounding_box| bounding_box)
            .reduce(|acc, bounding_box| {
                crate::Rect::get_minimum_rectangle_containing(&acc, bounding_box)
            })
    }

    pub(crate) fn is_xy_in(&self, xy: Xy<Px>) -> bool {
        self.commands.iter().any(|command| command.is_xy_in(xy))
    }
}

impl DrawCommand {
    pub fn draw(&self) {
        match self {
            DrawCommand::Image(command) => {
                command.draw();
            }
            DrawCommand::Path(command) => {
                command.draw();
            }
            DrawCommand::Text(command) => {
                command.draw();
            }
        }
    }
    fn get_bounding_box(&self) -> Option<Rect<Px>> {
        match self {
            DrawCommand::Path(command) => command.get_bounding_box(),
            DrawCommand::Image(command) => command.get_bounding_box(),
            DrawCommand::Text(command) => command.get_bounding_box(),
        }
    }
    fn is_xy_in(&self, xy: Xy<Px>) -> bool {
        match self {
            DrawCommand::Path(command) => command.is_xy_in(xy),
            DrawCommand::Image(command) => command.is_xy_in(xy),
            DrawCommand::Text(command) => command.is_xy_in(xy),
        }
    }
}
