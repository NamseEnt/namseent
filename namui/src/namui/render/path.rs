use crate::namui::{self, *};

pub fn path(path_builder: PathBuilder, paint: Paint) -> RenderingTree {
    RenderingTree::Node(RenderingData {
        draw_calls: vec![DrawCall {
            commands: vec![DrawCommand::Path(PathDrawCommand {
                path_builder,
                paint,
            })],
        }],
        ..Default::default()
    })
}
