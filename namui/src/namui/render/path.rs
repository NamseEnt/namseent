use crate::namui::{self, *};

pub fn path(path: Path, paint: Paint) -> RenderingTree {
    RenderingTree::Node(RenderingData {
        draw_calls: vec![DrawCall {
            commands: vec![DrawCommand::Path(PathDrawCommand { path, paint })],
        }],
        ..Default::default()
    })
}
