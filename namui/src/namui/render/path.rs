use crate::namui::*;

pub fn path(path: Path, paint: Paint) -> RenderingTree {
    RenderingTree::Node(RenderingData {
        draw_calls: vec![DrawCall {
            commands: vec![DrawCommand::Path {
                command: PathDrawCommand { path, paint },
            }],
        }],
    })
}
