use super::PathDrawCommand;
use crate::namui::{self, NamuiContext};

pub fn draw_path(namui_context: &NamuiContext, command: &PathDrawCommand) {
    let path = command.path_builder.build();
    let paint = command.paint_builder.build();
    namui_context.surface.canvas().draw_path(&path, &paint);
}
