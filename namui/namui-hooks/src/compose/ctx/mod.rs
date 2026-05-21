mod compose_command;
mod public;

use super::RtContainer;
use crate::*;
pub(crate) use compose_command::*;

pub struct ComposeCtx<'a, 'rt> {
    world: &'a World,
    composer: &'a Composer,
    rt_container: &'rt RtContainer<'a>,
    full_stack: Option<u32>,
    parent_stack: Option<u32>,
}

impl<'a, 'rt> ComposeCtx<'a, 'rt> {
    pub(crate) fn new(
        world: &'a World,
        composer: &'a Composer,
        rt_container: &'rt RtContainer<'a>,
        parent_stack: Option<u32>,
    ) -> ComposeCtx<'a, 'rt> {
        if composer.mark_rendered(world.frame()) {
            world.count_rendered_composer();
        }
        ComposeCtx {
            world,
            composer,
            rt_container,
            full_stack: parent_stack,
            parent_stack,
        }
    }

    pub(crate) fn push_command(&self, command: ComposeCommand) -> ComposeCtx<'a, 'rt> {
        let full_stack = Some(self.world.push_compose_command(self.full_stack, command));
        ComposeCtx {
            world: self.world,
            composer: self.composer,
            rt_container: self.rt_container,
            full_stack,
            parent_stack: self.parent_stack,
        }
    }

    fn apply_stack(&self, mut rendering_tree: RenderingTree) -> RenderingTree {
        let arena = self.world.compose_command_arena.borrow();
        let mut cursor = self.full_stack;

        while cursor != self.parent_stack {
            let node = &arena[cursor.unwrap() as usize];
            rendering_tree = match &node.command {
                ComposeCommand::Translate { xy } => {
                    RenderingTree::Special(SpecialRenderingNode::Translate(TranslateNode {
                        x: xy.x,
                        y: xy.y,
                        rendering_tree: arena_alloc(rendering_tree),
                    }))
                }
                ComposeCommand::Absolute { xy } => {
                    RenderingTree::Special(SpecialRenderingNode::Absolute(AbsoluteNode {
                        x: xy.x,
                        y: xy.y,
                        rendering_tree: arena_alloc(rendering_tree),
                    }))
                }
                ComposeCommand::Clip { path, clip_op } => {
                    RenderingTree::Special(SpecialRenderingNode::Clip(ClipNode {
                        path: arena_alloc(path.clone()),
                        clip_op: *clip_op,
                        rendering_tree: arena_alloc(rendering_tree),
                    }))
                }
                ComposeCommand::OnTop => {
                    RenderingTree::Special(SpecialRenderingNode::OnTop(OnTopNode {
                        rendering_tree: arena_alloc(rendering_tree),
                    }))
                }
                ComposeCommand::Rotate { angle } => {
                    RenderingTree::Special(SpecialRenderingNode::Rotate(RotateNode {
                        angle: *angle,
                        rendering_tree: arena_alloc(rendering_tree),
                    }))
                }
                ComposeCommand::Scale { scale_xy } => {
                    RenderingTree::Special(SpecialRenderingNode::Scale(ScaleNode {
                        x: scale_xy.x.into(),
                        y: scale_xy.y.into(),
                        rendering_tree: arena_alloc(rendering_tree),
                    }))
                }
                ComposeCommand::MouseCursor { cursor } => {
                    RenderingTree::Special(SpecialRenderingNode::MouseCursor(MouseCursorNode {
                        cursor: arena_alloc(cursor.clone()),
                        rendering_tree: arena_alloc(rendering_tree),
                    }))
                }
            };
            cursor = node.parent;
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

        let stack_applied = self.apply_stack(rendering_tree);

        self.rt_container.push(stack_applied);
    }

    fn add_rt_container(&self, rt_container: RtContainer<'a>) {
        if rt_container.is_empty() {
            return;
        }
        let rendering_tree = rt_container.into_rendering_tree();
        self.add_rendering_tree(rendering_tree)
    }

    fn collect_commands(&self, stack: Option<u32>) -> Vec<ComposeCommand> {
        let arena = self.world.compose_command_arena.borrow();
        let mut commands = vec![];
        let mut cursor = stack;
        while let Some(index) = cursor {
            let node = &arena[index as usize];
            commands.push(node.command.clone());
            cursor = node.parent;
        }
        commands.reverse();
        commands
    }

    fn full_stack_commands(&self) -> Vec<ComposeCommand> {
        self.collect_commands(self.full_stack)
    }

    fn parent_stack_commands(&self) -> Vec<ComposeCommand> {
        self.collect_commands(self.parent_stack)
    }
}
