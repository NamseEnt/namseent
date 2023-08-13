use crate::*;

pub fn bounding_box(rendering_tree: &RenderingTree) -> Option<Rect<Px>> {
    todo!()
    // let mut bounding_box = None;

    // rendering_tree.visit(&mut |node| {
    //     if let Some(node_bounding_box) = node.get_bounding_box() {
    //         bounding_box = bounding_box
    //             .map(|bounding_box| bounding_box.union(node_bounding_box))
    //             .or(Some(node_bounding_box));
    //     }
    // });

    // bounding_box
}
