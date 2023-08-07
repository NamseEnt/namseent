mod bounding_box;
mod call_event;
mod visit;

use super::*;
use crate::namui::*;
pub use call_event::*;
use std::ops::ControlFlow;
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Default, Debug, Clone, serde::Serialize)]
pub struct RenderingData {
    pub draw_calls: Vec<DrawCall>,
}
#[derive(Debug, Clone, serde::Serialize)]
pub enum RenderingTree {
    Node(RenderingData),
    Children(Vec<RenderingTree>),
    Special(SpecialRenderingNode),
    Empty,
}

impl Default for RenderingTree {
    fn default() -> Self {
        RenderingTree::Empty
    }
}

// NOTE : to support putting MouseCursor into event.
unsafe impl Send for RenderingTree {}
unsafe impl Sync for RenderingTree {}

impl SpecialRenderingNode {
    pub(crate) fn get_rendering_tree(&self) -> Arc<RenderingTree> {
        match self {
            SpecialRenderingNode::Translate(node) => node.rendering_tree.clone(),
            SpecialRenderingNode::Clip(node) => node.rendering_tree.clone(),
            SpecialRenderingNode::MouseCursor(node) => node.rendering_tree.clone(),
            SpecialRenderingNode::WithId(node) => node.rendering_tree.clone(),
            SpecialRenderingNode::Absolute(node) => node.rendering_tree.clone(),
            SpecialRenderingNode::Rotate(node) => node.rendering_tree.clone(),
            SpecialRenderingNode::Custom(node) => node.rendering_tree.clone(),
            SpecialRenderingNode::Scale(node) => node.rendering_tree.clone(),
            SpecialRenderingNode::Transform(node) => node.rendering_tree.clone(),
            SpecialRenderingNode::OnTop(node) => node.rendering_tree.clone(),
        }
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = globalThis)]
    fn flushCanvas();
}

pub fn draw_rendering_tree(rendering_tree: &RenderingTree) {
    crate::graphics::surface()
        .canvas()
        .clear(Color::TRANSPARENT);

    rendering_tree.draw();

    crate::graphics::surface().flush();

    flushCanvas();
}

/// NOTE
/// Order of tree traversal is important.
/// - draw = pre-order dfs (NLR)
/// - events = Reverse post-order (RLN)
/// reference: https://en.wikipedia.org/wiki/Tree_traversal
impl RenderingTree {
    pub fn iter(&self) -> impl Iterator<Item = &RenderingTree> {
        let mut vec = vec![];
        match self {
            RenderingTree::Children(children) => {
                vec.extend(children.iter());
            }
            RenderingTree::Node(_) | RenderingTree::Special(_) => vec.push(self),
            RenderingTree::Empty => {}
        };

        vec.into_iter()
    }

    pub fn into_iter(self) -> impl Iterator<Item = RenderingTree> {
        let mut vec = vec![];
        match self {
            RenderingTree::Children(children) => {
                vec.extend(children.into_iter());
            }
            RenderingTree::Node(_) | RenderingTree::Special(_) => vec.push(self),
            RenderingTree::Empty => {}
        };

        vec.into_iter()
    }

    pub(crate) fn draw(&self) {
        struct DrawContext {
            on_top_node_matrix_tuples: Vec<(OnTopNode, Matrix3x3)>,
        }
        fn draw_internal(rendering_tree: &RenderingTree, draw_context: &mut DrawContext) {
            match rendering_tree {
                RenderingTree::Children(ref children) => {
                    for child in children.iter() {
                        draw_internal(child, draw_context);
                    }
                }
                RenderingTree::Node(rendering_data) => {
                    rendering_data.draw_calls.iter().for_each(|draw_call| {
                        draw_call.draw();
                    });
                }
                RenderingTree::Special(special) => match special {
                    SpecialRenderingNode::Translate(translate) => {
                        crate::graphics::surface().canvas().save();
                        crate::graphics::surface()
                            .canvas()
                            .translate(translate.x, translate.y);

                        draw_internal(&translate.rendering_tree, draw_context);
                        crate::graphics::surface().canvas().restore();
                    }
                    SpecialRenderingNode::Clip(clip) => {
                        crate::graphics::surface().canvas().save();
                        let path = clip.path_builder.build();
                        crate::graphics::surface().canvas().clip_path(
                            path.as_ref(),
                            &clip.clip_op,
                            true,
                        );
                        draw_internal(&clip.rendering_tree, draw_context);
                        crate::graphics::surface().canvas().restore();
                    }
                    SpecialRenderingNode::Absolute(absolute) => {
                        crate::graphics::surface().canvas().save();
                        crate::graphics::surface()
                            .canvas()
                            .set_matrix(Matrix3x3::from_slice([
                                [1.0, 0.0, absolute.x.as_f32()],
                                [0.0, 1.0, absolute.y.as_f32()],
                                [0.0, 0.0, 1.0],
                            ]));
                        draw_internal(&absolute.rendering_tree, draw_context);
                        crate::graphics::surface().canvas().restore();
                    }
                    SpecialRenderingNode::Rotate(rotate) => {
                        crate::graphics::surface().canvas().save();
                        crate::graphics::surface().canvas().rotate(rotate.angle);
                        draw_internal(&rotate.rendering_tree, draw_context);
                        crate::graphics::surface().canvas().restore();
                    }
                    SpecialRenderingNode::Scale(scale) => {
                        crate::graphics::surface().canvas().save();
                        crate::graphics::surface().canvas().scale(scale.x, scale.y);
                        draw_internal(&scale.rendering_tree, draw_context);
                        crate::graphics::surface().canvas().restore();
                    }
                    SpecialRenderingNode::Transform(transform) => {
                        crate::graphics::surface().canvas().save();
                        crate::graphics::surface()
                            .canvas()
                            .transform(transform.matrix);
                        draw_internal(&transform.rendering_tree, draw_context);
                        crate::graphics::surface().canvas().restore();
                    }
                    SpecialRenderingNode::OnTop(on_top) => {
                        let matrix = crate::graphics::surface().canvas().get_matrix();
                        draw_context
                            .on_top_node_matrix_tuples
                            .push((on_top.clone(), matrix));
                    }
                    SpecialRenderingNode::MouseCursor(_)
                    | SpecialRenderingNode::WithId(_)
                    | SpecialRenderingNode::Custom(_) => {
                        draw_internal(&special.get_rendering_tree(), draw_context);
                    }
                },
                RenderingTree::Empty => {}
            }
        }

        let mut draw_context = DrawContext {
            on_top_node_matrix_tuples: Vec::new(),
        };
        draw_internal(self, &mut draw_context);

        for (node, matrix) in draw_context.on_top_node_matrix_tuples {
            crate::graphics::surface().canvas().save();
            crate::graphics::surface().canvas().set_matrix(matrix);
            node.rendering_tree.draw();
            crate::graphics::surface().canvas().restore();
        }
    }

    // TODO
    // pub(crate) fn get_mouse_cursor(&self, xy: Xy<Px>) -> Option<MouseCursor> {
    //     let mut result = None;
    //     self.visit_rln(|node, utils| {
    //         if let RenderingTree::Special(special) = node {
    //             if let SpecialRenderingNode::MouseCursor(mouse_cursor) = special {
    //                 if utils.is_xy_in(xy) {
    //                     result = Some(*(mouse_cursor.cursor.clone()));
    //                     return ControlFlow::Break(());
    //                 }
    //             }
    //         }
    //         ControlFlow::Continue(())
    //     });
    //     result
    // }

    pub fn get_xy_by_id(&self, id: crate::Uuid) -> Option<Xy<Px>> {
        let mut result = None;
        self.visit_rln(|node, utils| {
            if let RenderingTree::Special(special) = node {
                if let SpecialRenderingNode::WithId(with_id) = special {
                    if with_id.id == id {
                        result = Some(utils.get_xy());
                        return ControlFlow::Break(());
                    }
                }
            }
            ControlFlow::Continue(())
        });
        result
    }
    pub fn get_xy_of_child(&self, child: &RenderingTree) -> Option<Xy<Px>> {
        let mut result = None;
        self.visit_rln(|node, utils| {
            if std::ptr::eq(node, child) {
                result = Some(utils.get_xy());
                return ControlFlow::Break(());
            }
            ControlFlow::Continue(())
        });
        result
    }
}

impl RenderingData {
    fn get_bounding_box(&self) -> Option<Rect<Px>> {
        self.draw_calls
            .iter()
            .map(|draw_call| draw_call.get_bounding_box())
            .filter_map(|bounding_box| bounding_box)
            .reduce(|acc, bounding_box| Rect::get_minimum_rectangle_containing(&acc, bounding_box))
    }
    fn is_xy_in(&self, xy: Xy<Px>) -> bool {
        self.draw_calls
            .iter()
            .any(|draw_call| draw_call.is_xy_in(xy))
    }
}
