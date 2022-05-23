use super::*;

pub struct VisitUtils<'a> {
    pub is_xy_in: &'a dyn Fn(&Xy<f32>) -> bool,
}

impl RenderingTree {
    pub fn visit_rln<F>(&self, mut callback: F)
    where
        F: FnMut(&Self, VisitUtils) -> ControlFlow<()>,
    {
        self.try_visit_rln(&mut callback, &Matrix3x3::identity());
    }
    fn try_visit_rln<F>(&self, callback: &mut F, matrix: &Matrix3x3) -> ControlFlow<()>
    where
        F: FnMut(&Self, VisitUtils) -> ControlFlow<()>,
    {
        match self {
            RenderingTree::Children(ref children) => {
                for child in children.iter().rev() {
                    match child.try_visit_rln(callback, &matrix) {
                        ControlFlow::Break(_) => {
                            return ControlFlow::Break(());
                        }
                        _ => {}
                    }
                }
            }
            RenderingTree::Special(special) => {
                match special.get_child().try_visit_rln(callback, &matrix) {
                    ControlFlow::Break(_) => {
                        return ControlFlow::Break(());
                    }
                    _ => {}
                }
            }
            _ => {}
        }

        let utils = VisitUtils {
            is_xy_in: &|xy| self.is_point_in(&xy, &matrix),
        };
        callback(self, utils)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::wasm_bindgen_test;

    #[test]
    #[wasm_bindgen_test]
    fn visiting_order_should_be_rln() {
        /*
            tree:
                 0
               /   \
              1     2
             / \   /
            3   4 5

            rln order: 5, 2, 4, 3, 1, 0
        */

        let node_5 = (RenderingTree::Empty).with_id("5");
        let node_4 = (RenderingTree::Empty).with_id("4");
        let node_3 = (RenderingTree::Empty).with_id("3");
        let node_2 = (RenderingTree::Children(vec![node_5])).with_id("2");
        let node_1 = (RenderingTree::Children(vec![node_3, node_4])).with_id("1");
        let node_0 = (RenderingTree::Children(vec![node_1, node_2])).with_id("0");

        let mut called_ids = vec![];
        node_0.visit_rln(|rendering_tree, _| {
            if let RenderingTree::Special(rendering_tree) = rendering_tree {
                if let SpecialRenderingNode::WithId(with_id) = rendering_tree {
                    called_ids.push(with_id.id.clone());
                }
            }

            ControlFlow::Continue(())
        });

        assert_eq!(called_ids, vec!["5", "2", "4", "3", "1", "0"]);
    }
}
