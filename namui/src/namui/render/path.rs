use crate::namui::*;

pub fn path(path_builder: PathBuilder, paint_builder: PaintBuilder) -> RenderingTree {
    RenderingTree::Node(RenderingData {
        draw_calls: vec![DrawCall {
            commands: vec![DrawCommand::Path(PathDrawCommand {
                path_builder,
                paint_builder,
            })],
        }],
        ..Default::default()
    })
}
