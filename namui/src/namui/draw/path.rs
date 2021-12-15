use super::PathDrawCommand;
use crate::namui::{self, NamuiContext};

pub fn draw_path(namui_context: &NamuiContext, command: &PathDrawCommand) {
    namui_context
        .surface
        .canvas()
        .draw_path(&command.path, &command.paint);
}
