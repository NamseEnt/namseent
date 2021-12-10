use super::PathDrawCommand;
use crate::engine::{self, EngineContext};

pub fn draw_path(engine_context: &EngineContext, command: &PathDrawCommand) {
    engine::log(format!("draw_path"));
    engine_context
        .surface
        .canvas()
        .draw_path(&command.path, &command.paint);
}
