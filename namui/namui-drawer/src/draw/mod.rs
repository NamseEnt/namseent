mod atlas;
mod image;
mod path;
mod text;

use crate::*;

pub(crate) trait Draw {
    fn draw(self, skia: &mut NativeSkia);
}

impl Draw for RenderingTree {
    fn draw(self, skia: &mut NativeSkia) {
        struct RenderingTreeDrawContext {
            on_top_node_matrix_tuples: Vec<(OnTopNode, TransformMatrix)>,
        }
        fn draw_internal(
            skia: &mut NativeSkia,
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
                        skia.surface().canvas().scale(*scale.x, *scale.y);
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
                    SpecialRenderingNode::MouseCursor(_) => {
                        draw_internal(
                            skia,
                            special.inner_rendering_tree_ref(),
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
    fn draw(self, skia: &mut NativeSkia) {
        match self {
            DrawCommand::Path { command } => command.draw(skia),
            DrawCommand::Text { command } => command.draw(skia),
            DrawCommand::Image { command } => command.draw(skia),
            DrawCommand::Atlas { command } => command.draw(skia),
        }
    }
}

pub fn draw_mouse_cursor(
    skia: &mut NativeSkia,
    mouse_xy: Xy<Px>,
    mouse_cursor: MouseCursor,
    sprite_set: &StandardCursorSpriteSet,
) {
    skia.surface().canvas().save();
    skia.surface().canvas().translate(mouse_xy.x, mouse_xy.y);

    'draw: {
        match mouse_cursor {
            MouseCursor::Standard(standard_cursor) => {
                let Some(sprite) = sprite_set.sprites.get(&standard_cursor) else {
                    break 'draw;
                };
                let calculate_offset_xy = |index: usize| -> Xy<Px> {
                    let column = index % sprite_set.columns;
                    let row = index / sprite_set.columns;
                    sprite_set.cursor_wh.to_xy() * Xy::new(column, row)
                };
                let (index, hotspot_xy) = match *sprite {
                    CursorSprite::Static { index, hotspot_xy } => (index, hotspot_xy),
                    CursorSprite::Animated {
                        start_index,
                        hotspot_xy,
                        frame_count,
                        frame_duration,
                    } => {
                        static INSTANT_FOR_ANIMATION: std::sync::OnceLock<namui_type::Instant> =
                            std::sync::OnceLock::new();
                        let elapsed = INSTANT_FOR_ANIMATION
                            .get_or_init(namui_type::Instant::now)
                            .elapsed();

                        let frame_index = ((elapsed.as_millis() / frame_duration.as_millis())
                            % frame_count as i128)
                            as usize;
                        (start_index + frame_index, hotspot_xy)
                    }
                };
                let offset_xy = calculate_offset_xy(index);

                skia.surface().canvas().clip_path(
                    &Path::new().add_rect(Rect::from_xy_wh(-hotspot_xy, sprite_set.cursor_wh)),
                    ClipOp::Intersect,
                    false,
                );
                ImageDrawCommand {
                    rect: Rect::from_xy_wh(-offset_xy - hotspot_xy, sprite_set.sheet.info().wh()),
                    image: sprite_set.sheet,
                    fit: ImageFit::None,
                    paint: None,
                }
                .draw(skia);
            }
            MouseCursor::Custom(rendering_tree) => {
                rendering_tree.draw(skia);
            }
        }
    }

    skia.surface().canvas().restore();
}
