mod image;
mod path;
mod text;

use namui_skia::*;
use namui_type::*;

pub(crate) struct DrawContext<'a> {
    skia: &'a mut dyn SkSkia,
    start_load_image: &'a dyn Fn(&ImageSource),
}

impl<'a> DrawContext<'a> {
    pub fn new(skia: &'a mut dyn SkSkia, start_load_image: &'a dyn Fn(&ImageSource)) -> Self {
        Self {
            skia,
            start_load_image,
        }
    }
    pub fn surface(&mut self) -> &mut dyn SkSurface {
        self.skia.surface()
    }
    pub fn canvas(&mut self) -> &dyn SkCanvas {
        self.surface().canvas()
    }
}

pub(crate) trait Draw {
    fn draw(self, ctx: &mut DrawContext);
}

impl Draw for RenderingTree {
    fn draw(self, ctx: &mut DrawContext) {
        struct RenderingTreeDrawContext {
            on_top_node_matrix_tuples: Vec<(OnTopNode, TransformMatrix)>,
        }
        fn draw_internal(
            ctx: &mut DrawContext,
            rendering_tree: &RenderingTree,
            rendering_tree_draw_context: &mut RenderingTreeDrawContext,
        ) {
            match rendering_tree {
                RenderingTree::Children(children) => {
                    // NOTE: Children are drawn in reverse order. First(Left) child is drawn at the front.
                    for child in children.into_iter().rev() {
                        draw_internal(ctx, child, rendering_tree_draw_context);
                    }
                }
                RenderingTree::Node(draw_command) => {
                    draw_command.draw(ctx);
                }
                RenderingTree::Special(special) => match special {
                    SpecialRenderingNode::Translate(translate) => {
                        ctx.canvas().save();
                        ctx.canvas().translate(translate.x, translate.y);

                        draw_internal(ctx, &translate.rendering_tree, rendering_tree_draw_context);
                        ctx.canvas().restore();
                    }
                    SpecialRenderingNode::Clip(clip) => {
                        ctx.canvas().save();
                        ctx.canvas().clip_path(&clip.path, clip.clip_op, true);
                        draw_internal(ctx, &clip.rendering_tree, rendering_tree_draw_context);
                        ctx.canvas().restore();
                    }
                    SpecialRenderingNode::Absolute(absolute) => {
                        ctx.canvas().save();
                        ctx.canvas().set_matrix(TransformMatrix::from_slice([
                            [1.0, 0.0, absolute.x.as_f32()],
                            [0.0, 1.0, absolute.y.as_f32()],
                        ]));
                        draw_internal(ctx, &absolute.rendering_tree, rendering_tree_draw_context);
                        ctx.canvas().restore();
                    }
                    SpecialRenderingNode::Rotate(rotate) => {
                        ctx.canvas().save();
                        ctx.canvas().rotate(rotate.angle);
                        draw_internal(ctx, &rotate.rendering_tree, rendering_tree_draw_context);
                        ctx.canvas().restore();
                    }
                    SpecialRenderingNode::Scale(scale) => {
                        ctx.canvas().save();
                        ctx.canvas().scale(scale.x, scale.y);
                        draw_internal(ctx, &scale.rendering_tree, rendering_tree_draw_context);
                        ctx.canvas().restore();
                    }
                    SpecialRenderingNode::Transform(transform) => {
                        ctx.canvas().save();
                        ctx.canvas().transform(transform.matrix);
                        draw_internal(ctx, &transform.rendering_tree, rendering_tree_draw_context);
                        ctx.canvas().restore();
                    }
                    SpecialRenderingNode::OnTop(on_top) => {
                        let matrix = ctx.canvas().get_matrix();
                        rendering_tree_draw_context
                            .on_top_node_matrix_tuples
                            .push((on_top.clone(), matrix));
                    }
                    SpecialRenderingNode::WithId(_) => {
                        draw_internal(
                            ctx,
                            special.inner_rendering_tree_ref(),
                            rendering_tree_draw_context,
                        );
                    }
                },
                RenderingTree::Empty => {}
                RenderingTree::Boxed(boxed) => {
                    draw_internal(ctx, boxed.as_ref(), rendering_tree_draw_context);
                }
                RenderingTree::BoxedChildren(children) => {
                    for child in children {
                        draw_internal(ctx, child, rendering_tree_draw_context);
                    }
                }
            }
        }

        let mut draw_context = RenderingTreeDrawContext {
            on_top_node_matrix_tuples: Vec::new(),
        };
        draw_internal(ctx, &self, &mut draw_context);

        for (node, matrix) in draw_context.on_top_node_matrix_tuples {
            ctx.canvas().save();
            ctx.canvas().set_matrix(matrix);
            node.rendering_tree.draw(ctx);
            ctx.canvas().restore();
        }
    }
}

impl Draw for &DrawCommand {
    fn draw(self, ctx: &mut DrawContext) {
        match self {
            DrawCommand::Path { command } => command.draw(ctx),
            DrawCommand::Text { command } => command.draw(ctx),
            DrawCommand::Image { command } => command.draw(ctx),
        }
    }
}
