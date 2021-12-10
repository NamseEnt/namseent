use super::PathDrawCommand;
use crate::engine::{self, EngineContext};

pub fn draw_path<TState>(engine_context: &EngineContext<TState>, command: &PathDrawCommand) {
    engine::log(format!("draw_path"));
    engine_context
        .surface
        .canvas()
        .draw_path(&command.path, &command.paint);
}
