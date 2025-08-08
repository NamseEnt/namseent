mod compose_command;
mod public;

use super::RtContainer;
use crate::*;
use compose_command::*;
use std::borrow::Cow;

pub struct ComposeCtx<'a, 'rt> {
    world: &'a World,
    composer: &'a Composer,
    rt_container: &'rt RtContainer,
    full_stack: CowFullStack<'rt>,
    stack_parent_len: usize,
}

pub(crate) type CowFullStack<'a> = Cow<'a, Vec<ComposeCommand>>;

impl<'a, 'rt> ComposeCtx<'a, 'rt> {
    pub(crate) fn new(
        world: &'a World,
        composer: &'a Composer,
        rt_container: &'rt RtContainer,
        full_stack: CowFullStack<'rt>,
    ) -> ComposeCtx<'a, 'rt> {
        composer.set_rendered_flag();
        ComposeCtx {
            world,
            composer,
            rt_container,
            stack_parent_len: full_stack.len(),
            full_stack,
        }
    }

    fn apply_stack(&self, mut rendering_tree: RenderingTree) -> RenderingTree {
        for command in self.full_stack.iter().skip(self.stack_parent_len).rev() {
            rendering_tree = match command {
                ComposeCommand::Translate { xy } => {
                    RenderingTree::Special(SpecialRenderingNode::Translate(TranslateNode {
                        x: xy.x,
                        y: xy.y,
                        rendering_tree: rendering_tree.into(),
                    }))
                }
                ComposeCommand::Absolute { xy } => {
                    RenderingTree::Special(SpecialRenderingNode::Absolute(AbsoluteNode {
                        x: xy.x,
                        y: xy.y,
                        rendering_tree: rendering_tree.into(),
                    }))
                }
                ComposeCommand::Clip { path, clip_op } => {
                    RenderingTree::Special(SpecialRenderingNode::Clip(ClipNode {
                        path: path.clone(),
                        clip_op: *clip_op,
                        rendering_tree: rendering_tree.into(),
                    }))
                }
                ComposeCommand::OnTop => {
                    RenderingTree::Special(SpecialRenderingNode::OnTop(OnTopNode {
                        rendering_tree: rendering_tree.into(),
                    }))
                }
                ComposeCommand::Rotate { angle } => {
                    RenderingTree::Special(SpecialRenderingNode::Rotate(RotateNode {
                        angle: *angle,
                        rendering_tree: rendering_tree.into(),
                    }))
                }
                ComposeCommand::Scale { scale_xy } => {
                    RenderingTree::Special(SpecialRenderingNode::Scale(ScaleNode {
                        x: scale_xy.x.into(),
                        y: scale_xy.y.into(),
                        rendering_tree: rendering_tree.into(),
                    }))
                }
                ComposeCommand::MouseCursor { cursor } => {
                    RenderingTree::Special(SpecialRenderingNode::MouseCursor(MouseCursorNode {
                        cursor: Box::new(cursor.clone()),
                        rendering_tree: rendering_tree.into(),
                    }))
                }
            }
        }

        rendering_tree
    }

    fn add_rendering_tree(&self, rendering_tree: RenderingTree) {
        if rendering_tree == RenderingTree::Empty {
            return;
        }

        if let RenderingTree::Children(children) = &rendering_tree
            && children.is_empty()
        {
            return;
        }

        if let RenderingTree::BoxedChildren(children) = &rendering_tree
            && children.is_empty()
        {
            return;
        }

        let stack_applied = self.apply_stack(rendering_tree);

        self.rt_container.push(stack_applied.into());
    }

    fn add_rt_container(&self, rt_container: RtContainer) {
        if rt_container.is_empty() {
            return;
        }
        let rendering_tree = rt_container.into();
        self.add_rendering_tree(rendering_tree)
    }

    fn parent_stack(&self) -> impl Iterator<Item = &ComposeCommand> {
        self.full_stack.iter().take(self.stack_parent_len)
    }
}
