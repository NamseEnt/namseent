mod image;
mod path;
mod text;

use namui_skia::*;
use namui_type::*;

pub(crate) trait Draw {
    fn draw(self, skia: &mut impl SkSkia);
}

impl Draw for RenderingTree {
    fn draw(self, skia: &mut impl SkSkia) {
        struct RenderingTreeDrawContext {
            on_top_node_matrix_tuples: Vec<(OnTopNode, TransformMatrix)>,
        }
        fn draw_internal(
            skia: &mut impl SkSkia,
            rendering_tree: &RenderingTree,
            rendering_tree_draw_context: &mut RenderingTreeDrawContext,
        ) {
            match rendering_tree {
                RenderingTree::Children(children) => {
                    // NOTE: Children are drawn in reverse order. First(Left) child is drawn at the front.
                    for child in children.iter().rev() {
                        draw_internal(skia, child, rendering_tree_draw_context);
                    }
                }
                RenderingTree::Node(draw_command) => {
                    draw_command.draw(skia);
                }
                RenderingTree::Special(special) => match special {
                    SpecialRenderingNode::Translate(translate) => {
                        skia.surface().canvas().save();
                        skia.surface().canvas().translate(translate.x, translate.y);

                        draw_internal(skia, &translate.rendering_tree, rendering_tree_draw_context);
                        skia.surface().canvas().restore();
                    }
                    SpecialRenderingNode::Clip(clip) => {
                        skia.surface().canvas().save();
                        skia.surface()
                            .canvas()
                            .clip_path(&clip.path, clip.clip_op, true);
                        draw_internal(skia, &clip.rendering_tree, rendering_tree_draw_context);
                        skia.surface().canvas().restore();
                    }
                    SpecialRenderingNode::Absolute(absolute) => {
                        skia.surface().canvas().save();
                        skia.surface()
                            .canvas()
                            .set_matrix(TransformMatrix::from_slice([
                                [1.0, 0.0, absolute.x.as_f32()],
                                [0.0, 1.0, absolute.y.as_f32()],
                            ]));
                        draw_internal(skia, &absolute.rendering_tree, rendering_tree_draw_context);
                        skia.surface().canvas().restore();
                    }
                    SpecialRenderingNode::Rotate(rotate) => {
                        skia.surface().canvas().save();
                        skia.surface().canvas().rotate(rotate.angle);
                        draw_internal(skia, &rotate.rendering_tree, rendering_tree_draw_context);
                        skia.surface().canvas().restore();
                    }
                    SpecialRenderingNode::Scale(scale) => {
                        skia.surface().canvas().save();
                        skia.surface()
                            .canvas()
                            .scale(scale.x.into(), scale.y.into());
                        draw_internal(skia, &scale.rendering_tree, rendering_tree_draw_context);
                        skia.surface().canvas().restore();
                    }
                    SpecialRenderingNode::Transform(transform) => {
                        skia.surface().canvas().save();
                        skia.surface().canvas().transform(transform.matrix);
                        draw_internal(skia, &transform.rendering_tree, rendering_tree_draw_context);
                        skia.surface().canvas().restore();
                    }
                    SpecialRenderingNode::OnTop(on_top) => {
                        let matrix = skia.surface().canvas().get_matrix();
                        rendering_tree_draw_context
                            .on_top_node_matrix_tuples
                            .push((on_top.clone(), matrix));
                    }
                    SpecialRenderingNode::WithId(_) | SpecialRenderingNode::MouseCursor(_) => {
                        draw_internal(
                            skia,
                            special.inner_rendering_tree_ref(),
                            rendering_tree_draw_context,
                        );
                    }
                },
                RenderingTree::Empty => {}
                RenderingTree::Boxed(boxed) => {
                    draw_internal(skia, boxed.as_ref(), rendering_tree_draw_context);
                }
                RenderingTree::BoxedChildren(children) => {
                    for child in children.iter().rev() {
                        draw_internal(skia, child, rendering_tree_draw_context);
                    }
                }
            }
        }

        let mut draw_context = RenderingTreeDrawContext {
            on_top_node_matrix_tuples: Vec::new(),
        };
        draw_internal(skia, &self, &mut draw_context);

        for (node, matrix) in draw_context.on_top_node_matrix_tuples {
            skia.surface().canvas().save();
            skia.surface().canvas().set_matrix(matrix);
            node.rendering_tree.draw(skia);
            skia.surface().canvas().restore();
        }
    }
}

impl Draw for &DrawCommand {
    fn draw(self, skia: &mut impl SkSkia) {
        match self {
            DrawCommand::Path { command } => command.draw(skia),
            DrawCommand::Text { command } => command.draw(skia),
            DrawCommand::Image { command } => command.draw(skia),
        }
    }
}

impl Draw for MouseCursor {
    fn draw(self, skia: &mut impl SkSkia) {
        match self {
            MouseCursor::TopBottomResize => todo!(),
            MouseCursor::LeftRightResize => todo!(),
            MouseCursor::LeftTopRightBottomResize => todo!(),
            MouseCursor::RightTopLeftBottomResize => todo!(),
            MouseCursor::Default => {
                let path = Path::new()
                    .line_to(0.px(), 15.px())
                    .line_to(4.px(), 12.px())
                    .line_to(7.px(), 19.px())
                    .line_to(9.px(), 18.px())
                    .line_to(6.px(), 11.px())
                    .line_to(11.px(), 11.px())
                    .close();

                let fill_paint = Paint::new(Color::WHITE).set_style(PaintStyle::Fill);
                let stroke_paint = Paint::new(Color::BLACK)
                    .set_style(PaintStyle::Stroke)
                    .set_stroke_width(1.px());

                let fill_command = PathDrawCommand {
                    path: path.clone(),
                    paint: fill_paint,
                };
                let stroke_command = PathDrawCommand {
                    path,
                    paint: stroke_paint,
                };

                (&fill_command).draw(skia);
                (&stroke_command).draw(skia);
            }
            MouseCursor::Text => todo!(),
            MouseCursor::Grab => todo!(),
            MouseCursor::Grabbing => todo!(),
            MouseCursor::Move => todo!(),
            MouseCursor::Pointer => todo!(),
            MouseCursor::Crosshair => todo!(),
            MouseCursor::Custom(_rendering_tree) => todo!(),
        }
    }
}
