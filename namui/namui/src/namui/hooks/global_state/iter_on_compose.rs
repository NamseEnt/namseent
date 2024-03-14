use super::*;

pub(crate) fn iter_last_rendering_tree() -> IterOnCompose<'static> {
    unsafe {
        let global_state = GLOBAL_STATE.as_mut().unwrap();
        let nearest_compose_node_id = {
            let mut node_id = global_state.construct_tree_node_id;
            loop {
                let node = global_state.construct_tree.get(node_id).unwrap();
                if let ConstructTreeNodeType::Compose = node.construct_type {
                    break node_id;
                }
                node_id = node.parent_id;
            }
        };

        IterOnCompose {
            children_ids: global_state
                .construct_tree
                .get(nearest_compose_node_id)
                .unwrap()
                .children_ids
                .as_ref(),
        }
    }
}

pub struct IterOnCompose<'a> {
    children_ids: &'a [usize],
}

impl IterOnCompose<'_> {
    fn iter(&self) -> impl Iterator<Item = &RenderingTree> {
        self.children_ids.iter().filter_map(|id| {
            let global_state = unsafe { GLOBAL_STATE.as_ref().unwrap() };

            let node = global_state.construct_tree.get(*id).unwrap();
            if let ConstructTreeNodeType::RenderingTree { rendering_tree } = &node.construct_type {
                Some(rendering_tree)
            } else {
                None
            }
        })
    }
}

impl<'a> BoundingBox for IterOnCompose<'a> {
    fn xy_in(&self, xy: Xy<Px>) -> bool {
        self.iter().any(|rendering_tree| rendering_tree.xy_in(xy))
    }

    fn bounding_box(&self) -> Option<Rect<Px>> {
        self.iter()
            .filter_map(|child| child.bounding_box())
            .reduce(|acc, bounding_box| Rect::get_minimum_rectangle_containing(&acc, bounding_box))
    }
}
