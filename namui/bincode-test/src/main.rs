use namui_skia::*;
use namui_type::*;

fn main() {
    let rendering_tree = RenderingTree::Node(DrawCommand::Path {
        command: Box::new(PathDrawCommand {
            path: Path::new(),
            paint: Paint::new(Color::WHITE),
        }),
    });

    let bytes = bincode::encode_to_vec(rendering_tree, bincode::config::standard()).unwrap();

    std::fs::write("rendering_tree.bin", bytes).unwrap();
}
