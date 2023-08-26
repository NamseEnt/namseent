mod image;
mod path;
mod text;

use namui_skia::*;
use namui_type::*;
use std::sync::Arc;

pub(crate) struct DrawContext {
    pub(crate) skia: Arc<dyn SkSkia>,
}

impl DrawContext {
    pub fn new(skia: Arc<dyn SkSkia>) -> Self {
        Self { skia }
    }
    pub fn surface(&self) -> &dyn SkSurface {
        self.skia.surface()
    }
    pub fn canvas(&self) -> &dyn SkCanvas {
        self.surface().canvas()
    }
}

pub(crate) trait Draw {
    fn draw(self, ctx: &DrawContext);
}

impl Draw for RenderingTree {
    fn draw(self, ctx: &DrawContext) {
        struct RenderingTreeDrawContext {
            on_top_node_matrix_tuples: Vec<(OnTopNode, Matrix3x3)>,
        }
        fn draw_internal(
            ctx: &DrawContext,
            rendering_tree: RenderingTree,
            rendering_tree_draw_context: &mut RenderingTreeDrawContext,
        ) {
            match rendering_tree {
                RenderingTree::Children(children) => {
                    // NOTE: Children are drawn in reverse order. First(Left) child is drawn at the front.
                    for child in children.into_iter().rev() {
                        draw_internal(ctx, child, rendering_tree_draw_context);
                    }
                }
                RenderingTree::Node(rendering_data) => {
                    rendering_data.draw_calls.into_iter().for_each(|draw_call| {
                        draw_call.draw(ctx);
                    });
                }
                RenderingTree::Special(special) => match special {
                    SpecialRenderingNode::Translate(translate) => {
                        ctx.canvas().save();
                        ctx.canvas().translate(translate.x, translate.y);

                        draw_internal(ctx, *translate.rendering_tree, rendering_tree_draw_context);
                        ctx.canvas().restore();
                    }
                    SpecialRenderingNode::Clip(clip) => {
                        ctx.canvas().save();
                        ctx.canvas().clip_path(&clip.path, clip.clip_op, true);
                        draw_internal(ctx, *clip.rendering_tree, rendering_tree_draw_context);
                        ctx.canvas().restore();
                    }
                    SpecialRenderingNode::Absolute(absolute) => {
                        ctx.canvas().save();
                        ctx.canvas().set_matrix(Matrix3x3::from_slice([
                            [1.0, 0.0, absolute.x.as_f32()],
                            [0.0, 1.0, absolute.y.as_f32()],
                            [0.0, 0.0, 1.0],
                        ]));
                        draw_internal(ctx, *absolute.rendering_tree, rendering_tree_draw_context);
                        ctx.canvas().restore();
                    }
                    SpecialRenderingNode::Rotate(rotate) => {
                        ctx.canvas().save();
                        ctx.canvas().rotate(rotate.angle);
                        draw_internal(ctx, *rotate.rendering_tree, rendering_tree_draw_context);
                        ctx.canvas().restore();
                    }
                    SpecialRenderingNode::Scale(scale) => {
                        ctx.canvas().save();
                        ctx.canvas().scale(scale.x, scale.y);
                        draw_internal(ctx, *scale.rendering_tree, rendering_tree_draw_context);
                        ctx.canvas().restore();
                    }
                    SpecialRenderingNode::Transform(transform) => {
                        ctx.canvas().save();
                        ctx.canvas().transform(transform.matrix);
                        draw_internal(ctx, *transform.rendering_tree, rendering_tree_draw_context);
                        ctx.canvas().restore();
                    }
                    SpecialRenderingNode::OnTop(on_top) => {
                        let matrix = ctx.canvas().get_matrix();
                        rendering_tree_draw_context
                            .on_top_node_matrix_tuples
                            .push((on_top.clone(), matrix));
                    }
                    SpecialRenderingNode::MouseCursor(_) | SpecialRenderingNode::WithId(_) => {
                        draw_internal(
                            ctx,
                            special.inner_rendering_tree(),
                            rendering_tree_draw_context,
                        );
                    }
                },
                RenderingTree::Empty => {}
            }
        }

        let mut draw_context = RenderingTreeDrawContext {
            on_top_node_matrix_tuples: Vec::new(),
        };
        draw_internal(ctx, self, &mut draw_context);

        for (node, matrix) in draw_context.on_top_node_matrix_tuples {
            ctx.canvas().save();
            ctx.canvas().set_matrix(matrix);
            node.rendering_tree.draw(ctx);
            ctx.canvas().restore();
        }
    }
}

impl Draw for DrawCall {
    fn draw(self, ctx: &DrawContext) {
        self.commands
            .into_iter()
            .for_each(|command| command.draw(ctx));
    }
}

impl Draw for DrawCommand {
    fn draw(self, ctx: &DrawContext) {
        match self {
            DrawCommand::Path { command } => command.draw(ctx),
            DrawCommand::Text { command } => command.draw(ctx),
            DrawCommand::Image { command } => command.draw(ctx),
        }
    }
}
