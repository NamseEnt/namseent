use crate::*;
use std::borrow::Borrow;

impl BoundingBox for &RenderingTree {
    fn bounding_box(self) -> Option<Rect<Px>> {
        static CACHE: LruCache<RenderingTree, Rect<Px>> = LruCache::new();

        if let Some(cached) = CACHE.get(self) {
            return Some(*cached);
        }

        struct BoundingBoxContext {
            bounding_boxes_on_top: Vec<Option<Rect<Px>>>,
        }
        fn get_bounding_box_with_matrix(
            rendering_tree: &RenderingTree,
            matrix: &TransformMatrix,
            bounding_box_context: &mut BoundingBoxContext,
        ) -> Option<Rect<Px>> {
            fn get_bounding_box_with_matrix_of_rendering_trees<'a>(
                rendering_trees: impl IntoIterator<Item = &'a RenderingTree>,
                matrix: &TransformMatrix,
                bounding_box_context: &mut BoundingBoxContext,
            ) -> Option<Rect<Px>> {
                rendering_trees
                    .into_iter()
                    .filter_map(|child| {
                        get_bounding_box_with_matrix(child, matrix, bounding_box_context)
                    })
                    .reduce(|acc, bounding_box| {
                        Rect::get_minimum_rectangle_containing(&acc, bounding_box)
                    })
            }

            match rendering_tree {
                RenderingTree::Children(children) => {
                    get_bounding_box_with_matrix_of_rendering_trees(
                        children,
                        matrix,
                        bounding_box_context,
                    )
                }
                RenderingTree::Node(draw_command) => draw_command
                    .bounding_box()
                    .map(|bounding_box| matrix.transform_rect(bounding_box)),
                RenderingTree::Special(special) => match special {
                    SpecialRenderingNode::Translate(translate) => {
                        let matrix = matrix * translate.get_matrix();
                        get_bounding_box_with_matrix_of_rendering_trees(
                            [translate.rendering_tree.borrow()],
                            &matrix,
                            bounding_box_context,
                        )
                    }
                    SpecialRenderingNode::Clip(clip) => {
                        get_bounding_box_with_matrix_of_rendering_trees(
                            [clip.rendering_tree.borrow()],
                            matrix,
                            bounding_box_context,
                        )
                        .and_then(|bounding_box| {
                            let clip_bounding_box = clip
                                .path
                                .bounding_box()
                                .map(|bounding_box| matrix.transform_rect(bounding_box));

                            match clip.clip_op {
                                ClipOp::Intersect => {
                                    clip_bounding_box.and_then(|clip_bounding_box| {
                                        bounding_box.intersect(clip_bounding_box)
                                    })
                                }
                                ClipOp::Difference => match clip_bounding_box {
                                    Some(clip_bounding_box) => {
                                        if bounding_box == clip_bounding_box {
                                            return None;
                                        }

                                        let xs = [
                                            bounding_box.left(),
                                            bounding_box.right(),
                                            clip_bounding_box.left(),
                                            clip_bounding_box.right(),
                                        ];
                                        let ys = [
                                            bounding_box.top(),
                                            bounding_box.bottom(),
                                            clip_bounding_box.top(),
                                            clip_bounding_box.bottom(),
                                        ];

                                        let sixteen_xys = xs
                                            .iter()
                                            .zip(ys.iter())
                                            .map(|(x, y)| Xy { x: *x, y: *y });

                                        let difference_area_xys = sixteen_xys.filter(|xy| {
                                            (clip_bounding_box.is_xy_outside(*xy)
                                                || clip_bounding_box.is_xy_on_border(*xy))
                                                && !bounding_box.is_xy_outside(*xy)
                                        });

                                        difference_area_xys.fold(None, |acc, xy| match acc {
                                            Some(rect) => Some(Rect::Ltrb {
                                                left: rect.left().min(xy.x),
                                                top: rect.top().min(xy.y),
                                                right: rect.right().max(xy.x),
                                                bottom: rect.bottom().max(xy.y),
                                            }),
                                            None => Some(Rect::Ltrb {
                                                left: xy.x,
                                                top: xy.y,
                                                right: xy.x,
                                                bottom: xy.y,
                                            }),
                                        })
                                    }
                                    None => Some(bounding_box),
                                },
                            }
                        })
                    }
                    SpecialRenderingNode::Absolute(absolute) => {
                        get_bounding_box_with_matrix_of_rendering_trees(
                            [absolute.rendering_tree.borrow()],
                            &absolute.get_matrix(),
                            bounding_box_context,
                        )
                    }
                    SpecialRenderingNode::Rotate(rotate) => {
                        let matrix = matrix * rotate.get_matrix();

                        get_bounding_box_with_matrix_of_rendering_trees(
                            [rotate.rendering_tree.borrow()],
                            &matrix,
                            bounding_box_context,
                        )
                    }
                    SpecialRenderingNode::Scale(scale) => {
                        let matrix = matrix * scale.get_matrix();

                        get_bounding_box_with_matrix_of_rendering_trees(
                            [scale.rendering_tree.borrow()],
                            &matrix,
                            bounding_box_context,
                        )
                    }
                    SpecialRenderingNode::Transform(transform) => {
                        let matrix = matrix * transform.matrix;

                        get_bounding_box_with_matrix_of_rendering_trees(
                            [transform.rendering_tree.borrow()],
                            &matrix,
                            bounding_box_context,
                        )
                    }
                    SpecialRenderingNode::OnTop(on_top) => {
                        let bounding_box = get_bounding_box_with_matrix_of_rendering_trees(
                            [on_top.rendering_tree.borrow()],
                            matrix,
                            bounding_box_context,
                        );
                        bounding_box_context
                            .bounding_boxes_on_top
                            .push(bounding_box);
                        bounding_box
                    }
                    SpecialRenderingNode::WithId(_) | SpecialRenderingNode::MouseCursor(_) => {
                        get_bounding_box_with_matrix_of_rendering_trees(
                            [special.inner_rendering_tree_ref()],
                            matrix,
                            bounding_box_context,
                        )
                    }
                },
                RenderingTree::Empty => None,
                RenderingTree::Boxed(boxed) => {
                    get_bounding_box_with_matrix(boxed.borrow(), matrix, bounding_box_context)
                }
                RenderingTree::BoxedChildren(children) => {
                    get_bounding_box_with_matrix_of_rendering_trees(
                        children.iter().map(|child| child.borrow()),
                        matrix,
                        bounding_box_context,
                    )
                }
            }
        }

        let mut bounding_box_context = BoundingBoxContext {
            bounding_boxes_on_top: vec![],
        };
        let bounding_box = get_bounding_box_with_matrix(
            self,
            &TransformMatrix::identity(),
            &mut bounding_box_context,
        );

        let bounding_box = bounding_box_context
            .bounding_boxes_on_top
            .into_iter()
            .flatten()
            .fold(bounding_box, |acc, bounding_box| {
                acc.map(|acc| Rect::get_minimum_rectangle_containing(&acc, bounding_box))
            });

        if let Some(bounding_box) = bounding_box {
            CACHE.put(self.clone(), bounding_box);
        }

        bounding_box
    }
}

impl<'a, T> BoundingBox for T
where
    T: Iterator<Item = &'a RenderingTree>,
{
    fn bounding_box(self) -> Option<Rect<Px>> {
        self.filter_map(|rendering_tree| rendering_tree.bounding_box())
            .reduce(|acc, bounding_box| Rect::get_minimum_rectangle_containing(&acc, bounding_box))
    }
}
