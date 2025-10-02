use crate::*;

pub fn path(path: Path, paint: Paint) -> RenderingTree {
    RenderingTree::Node(DrawCommand::Path {
        command: PathDrawCommand { path, paint }.into(),
    })
}
