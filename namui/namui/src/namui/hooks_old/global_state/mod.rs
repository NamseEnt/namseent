mod iter_on_compose;

use crate::*;
use elsa::FrozenIndexSet;
pub(crate) use iter_on_compose::*;
use slab::Slab;

struct GlobalState {
    tree_ctx: TreeContext,
    clippings: Vec<Clipping>,
    saved_clippings: Option<Vec<Clipping>>,
    raw_event: Option<RawEvent>,
    /// pre-calculated matrix stack
    matrix_stack: Vec<TransformMatrix>,
    updated_sigs: FrozenIndexSet<SigId>,
    construct_tree: Slab<ConstructTreeNode>,
    construct_tree_node_id: usize,
}

static mut GLOBAL_STATE: Option<GlobalState> = None;

pub(crate) fn init(tree_ctx: TreeContext) {
    set_up_global_state(tree_ctx, 0);
}
pub(crate) fn reset() {
    unsafe {
        let global_state = GLOBAL_STATE.take().unwrap();
        set_up_global_state(
            global_state.tree_ctx,
            global_state.construct_tree.capacity(),
        );
    }
}

fn set_up_global_state(tree_ctx: TreeContext, construct_tree_capacity: usize) {
    let mut construct_tree = Slab::with_capacity(construct_tree_capacity);
    let entry = construct_tree.vacant_entry();
    let construct_tree_root_id = entry.key();
    entry.insert(ConstructTreeNode {
        construct_type: ConstructTreeNodeType::Root,
        children_ids: smallvec::smallvec![],
        parent_id: construct_tree_root_id,
    });

    unsafe {
        GLOBAL_STATE = Some(GlobalState {
            tree_ctx,
            clippings: Default::default(),
            saved_clippings: Default::default(),
            raw_event: Default::default(),
            matrix_stack: vec![TransformMatrix::identity()],
            updated_sigs: Default::default(),
            construct_tree,
            construct_tree_node_id: construct_tree_root_id,
        });
    }
}

pub(crate) fn set_raw_event(raw_event: RawEvent) {
    unsafe {
        GLOBAL_STATE.as_mut().unwrap().raw_event = Some(raw_event);
    }
}

fn push_construct_node(construct_type: ConstructTreeNodeType) {
    unsafe {
        let global_state = GLOBAL_STATE.as_mut().unwrap();
        let key = global_state.construct_tree.vacant_key();

        let parent_node = global_state
            .construct_tree
            .get_mut(global_state.construct_tree_node_id)
            .unwrap();
        parent_node.children_ids.push(key);

        global_state.construct_tree.insert(ConstructTreeNode {
            construct_type,
            children_ids: smallvec::smallvec![],
            parent_id: global_state.construct_tree_node_id,
        });
    }
}

fn pop_construct_node() {
    unsafe {
        // let global_state = GLOBAL_STATE.as_mut().unwrap();

        // let node = global_state
        //     .construct_tree
        //     .get_mut(global_state.construct_tree_node_id)
        //     .unwrap();

        // if node.children_ids.is_empty() {
        //     return;
        // }

        // // let mut vec: Vec<_> = Vec::with_capacity(node.children_ids.len());

        // let mut i = 0;

        // for child_id in node.children_ids.iter() {
        //     let global_state = GLOBAL_STATE.as_mut().unwrap();
        //     let node = global_state.construct_tree.get_mut(*child_id).unwrap();
        //     if let ConstructTreeNodeType::RenderingTree { rendering_tree } = &node.construct_type {
        //         if rendering_tree != &RenderingTree::Empty {
        //             // vec.push(RenderingTree::Static(rendering_tree));
        //             i += 1;
        //         }
        //     }
        // }

        // if i > 0 {
        //     println!("i: {}", i);
        // }

        // if vec.is_empty() {
        //     return;
        // }

        // let children = RenderingTree::Children(vec);

        // let rendering_tree = match &node.construct_type {
        //     ConstructTreeNodeType::Root => unreachable!(),
        //     ConstructTreeNodeType::Compose | ConstructTreeNodeType::Component => children,
        //     ConstructTreeNodeType::Top => crate::on_top(children),
        //     ConstructTreeNodeType::Translate { xy } => crate::translate(xy.x, xy.y, children),
        //     ConstructTreeNodeType::Absolute { xy } => crate::absolute(xy.x, xy.y, children),
        //     ConstructTreeNodeType::Clip { clipping } => {
        //         RenderingTree::Special(SpecialRenderingNode::Clip(ClipNode {
        //             path: clipping.path.clone(), // remove this clone
        //             clip_op: clipping.clip_op,
        //             rendering_tree: children.into(),
        //         }))
        //     }
        //     ConstructTreeNodeType::Rotate { angle } => crate::rotate(*angle, children),
        //     ConstructTreeNodeType::Scale { scale_xy } => {
        //         crate::scale(scale_xy.x, scale_xy.y, children)
        //     }
        //     ConstructTreeNodeType::RenderingTree { rendering_tree } => {
        //         assert_eq!(rendering_tree, &RenderingTree::Empty);
        //         children
        //     }
        // };

        // node.construct_type = ConstructTreeNodeType::RenderingTree { rendering_tree };
    }
}

#[derive(Debug)]
struct ConstructTreeNode {
    construct_type: ConstructTreeNodeType,
    children_ids: smallvec::SmallVec<[usize; 4]>,
    parent_id: usize,
}

#[derive(Debug)]
enum ConstructTreeNodeType {
    Root,
    Top,
    Compose,
    Component,
    Translate { xy: Xy<Px> },
    Absolute { xy: Xy<Px> },
    Clip { clipping: Clipping },
    Rotate { angle: Angle },
    Scale { scale_xy: Xy<f32> },
    RenderingTree { rendering_tree: RenderingTree },
}

pub(crate) enum GlobalStatePop {
    Compose,
    Top,
    Translate { xy: Xy<Px> },
    Absolute { xy: Xy<Px> },
    Clip { clipping: Clipping },
    Rotate { angle: Angle },
    Scale { scale_xy: Xy<f32> },
}

impl Drop for GlobalStatePop {
    fn drop(&mut self) {
        unsafe {
            let global_state = GLOBAL_STATE.as_mut().unwrap();

            match self {
                GlobalStatePop::Compose => {}
                GlobalStatePop::Top => {
                    global_state.clippings = global_state.saved_clippings.take().unwrap();
                }
                GlobalStatePop::Clip { .. } => {
                    global_state.clippings.pop().unwrap();
                }
                GlobalStatePop::Translate { .. }
                | GlobalStatePop::Absolute { .. }
                | GlobalStatePop::Rotate { .. }
                | GlobalStatePop::Scale { .. } => {
                    global_state.matrix_stack.pop().unwrap();
                }
            }

            pop_construct_node();
        }
    }
}

impl GlobalStatePop {
    pub(crate) fn compose() -> Self {
        push_construct_node(ConstructTreeNodeType::Compose);
        GlobalStatePop::Compose
    }

    pub(crate) fn top() -> Self {
        unsafe {
            let global_state = GLOBAL_STATE.as_mut().unwrap();

            global_state.saved_clippings = Some(std::mem::take(&mut global_state.clippings));
        }
        push_construct_node(ConstructTreeNodeType::Top);
        GlobalStatePop::Top
    }

    pub(crate) fn translate(xy: Xy<Px>) -> Self {
        unsafe {
            let global_state = GLOBAL_STATE.as_mut().unwrap();

            global_state.matrix_stack.push({
                let mut next = *global_state.matrix_stack.last().unwrap();
                next.translate(xy.x.as_f32(), xy.y.as_f32());
                next
            });
        }
        push_construct_node(ConstructTreeNodeType::Translate { xy });
        GlobalStatePop::Translate { xy }
    }

    pub(crate) fn absolute(xy: Xy<Px>) -> GlobalStatePop {
        unsafe {
            let global_state = GLOBAL_STATE.as_mut().unwrap();

            global_state
                .matrix_stack
                .push(TransformMatrix::from_translate(
                    xy.x.as_f32(),
                    xy.y.as_f32(),
                ));
        }
        push_construct_node(ConstructTreeNodeType::Absolute { xy });
        GlobalStatePop::Absolute { xy }
    }

    pub(crate) fn clip(clipping: Clipping) -> GlobalStatePop {
        unsafe {
            let global_state = GLOBAL_STATE.as_mut().unwrap();

            global_state.clippings.push(clipping.clone());
        }
        GlobalStatePop::Clip { clipping }
    }

    pub(crate) fn rotate(angle: Angle) -> GlobalStatePop {
        unsafe {
            let global_state = GLOBAL_STATE.as_mut().unwrap();

            global_state.matrix_stack.push({
                let mut next = *global_state.matrix_stack.last().unwrap();
                next.rotate(angle);
                next
            });
        }
        push_construct_node(ConstructTreeNodeType::Rotate { angle });
        GlobalStatePop::Rotate { angle }
    }

    pub(crate) fn scale(scale_xy: Xy<f32>) -> Self {
        unsafe {
            let global_state = GLOBAL_STATE.as_mut().unwrap();

            global_state.matrix_stack.push({
                let mut next = *global_state.matrix_stack.last().unwrap();
                next.scale(scale_xy.x, scale_xy.y);
                next
            });
        }
        push_construct_node(ConstructTreeNodeType::Scale { scale_xy });
        GlobalStatePop::Scale { scale_xy }
    }
}

// gets

pub(crate) fn clippings() -> &'static Vec<Clipping> {
    unsafe { &GLOBAL_STATE.as_ref().unwrap().clippings }
}
pub(crate) fn raw_event() -> &'static Option<RawEvent> {
    unsafe { &GLOBAL_STATE.as_ref().unwrap().raw_event }
}
pub(crate) fn tree_ctx() -> &'static TreeContext {
    unsafe { &GLOBAL_STATE.as_ref().unwrap().tree_ctx }
}
pub(crate) fn matrix() -> &'static TransformMatrix {
    unsafe { GLOBAL_STATE.as_ref().unwrap().matrix_stack.last().unwrap() }
}
pub(crate) fn updated_sigs() -> &'static FrozenIndexSet<SigId> {
    unsafe { &GLOBAL_STATE.as_ref().unwrap().updated_sigs }
}

pub(crate) fn take_last_rendering_tree() -> RenderingTree {
    unsafe {
        let global_state = GLOBAL_STATE.as_mut().unwrap();

        let node = global_state
            .construct_tree
            .get_mut(global_state.construct_tree_node_id)
            .unwrap();

        let last_id = node.children_ids.pop().unwrap();

        let ConstructTreeNodeType::RenderingTree { rendering_tree } =
            global_state.construct_tree.remove(last_id).construct_type
        else {
            return RenderingTree::Empty;
        };

        rendering_tree
    }
}
pub(crate) fn add_child(rendering_tree: RenderingTree) {
    unsafe {
        let global_state = GLOBAL_STATE.as_mut().unwrap();
        let child_id = global_state.construct_tree.vacant_key();

        let node = global_state
            .construct_tree
            .get_mut(global_state.construct_tree_node_id)
            .unwrap();

        node.children_ids.push(child_id);

        global_state.construct_tree.insert(ConstructTreeNode {
            construct_type: ConstructTreeNodeType::RenderingTree { rendering_tree },
            children_ids: smallvec::smallvec![],
            parent_id: global_state.construct_tree_node_id,
        });
    }
}

pub(crate) fn before_render_component() {
    push_construct_node(ConstructTreeNodeType::Component);
}

pub(crate) fn done_render_component() -> RenderingTree {
    pop_construct_node();
    take_last_rendering_tree()
}
