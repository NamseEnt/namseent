use super::*;

pub struct VisitUtils<'a> {
    pub rendering_tree: &'a RenderingTree,
    pub ancestors: &'a [&'a RenderingTree],
}
impl VisitUtils<'_> {
    pub fn is_xy_in(&self, xy: Xy<f32>) -> bool {
        self.rendering_tree.is_xy_in(xy, self.ancestors)
    }
    pub fn to_local_xy(&self, xy: Xy<f32>) -> Xy<f32> {
        self.rendering_tree.to_local_xy(xy, self.ancestors)
    }
    pub fn get_xy(&self) -> Xy<f32> {
        self.rendering_tree.get_xy(self.ancestors)
    }
    pub fn with_ancestors(&self, mut func: impl FnMut(&[&RenderingTree])) {
        func(self.ancestors)
    }
}

impl RenderingTree {
    pub fn visit_rln<F>(&self, mut callback: F)
    where
        F: FnMut(&Self, VisitUtils) -> ControlFlow<()>,
    {
        self.try_visit_rln(&mut callback, &vec![]);
    }
    fn try_visit_rln<F>(&self, callback: &mut F, ancestors: &[&Self]) -> ControlFlow<()>
    where
        F: FnMut(&Self, VisitUtils) -> ControlFlow<()>,
    {
        let mut next_ancestors = Vec::from(ancestors);
        next_ancestors.push(self);

        match self {
            RenderingTree::Children(ref children) => {
                for child in children.iter().rev() {
                    match child.try_visit_rln(callback, &next_ancestors) {
                        ControlFlow::Break(_) => {
                            return ControlFlow::Break(());
                        }
                        _ => {}
                    }
                }
            }
            RenderingTree::Special(special) => {
                match special
                    .get_rendering_tree()
                    .try_visit_rln(callback, &next_ancestors)
                {
                    ControlFlow::Break(_) => {
                        return ControlFlow::Break(());
                    }
                    _ => {}
                }
            }
            _ => {}
        }

        let utils = VisitUtils {
            ancestors: ancestors,
            rendering_tree: self,
        };
        callback(self, utils)
    }
    fn to_local_xy(&self, xy: Xy<f32>, ancestors: &[&Self]) -> Xy<f32> {
        let mut xy = xy.clone();
        for ancestor in ancestors.iter() {
            match ancestor {
                RenderingTree::Special(special) => match special {
                    SpecialRenderingNode::Translate(translate) => {
                        xy.x -= translate.x;
                        xy.y -= translate.y;
                    }
                    SpecialRenderingNode::Absolute(absolute) => {
                        xy = xy.clone();
                        xy.x -= absolute.x;
                        xy.y -= absolute.y;
                    }
                    SpecialRenderingNode::Rotate(rotate) => {
                        xy = rotate.get_counter_wise_matrix().transform_xy(xy);
                    }
                    SpecialRenderingNode::Scale(scale) => {
                        xy.x /= scale.x;
                        xy.y /= scale.y;
                    }
                    _ => {}
                },
                _ => {}
            }
        }
        xy
    }
    fn is_xy_in(&self, xy: Xy<f32>, ancestors: &[&Self]) -> bool {
        let mut result = false;
        self.try_visit_rln(
            &mut |node, utils| match node {
                RenderingTree::Node(node) => {
                    let local_xy = utils.to_local_xy(xy);
                    if node.is_xy_in(local_xy) && is_xy_clip_in_by_ancestors(xy, utils.ancestors) {
                        result = true;
                        ControlFlow::Break(())
                    } else {
                        ControlFlow::Continue(())
                    }
                }
                _ => ControlFlow::Continue(()),
            },
            ancestors,
        );

        result
    }
    fn get_xy(&self, ancestors: &[&RenderingTree]) -> Xy<f32> {
        let mut xy = Xy { x: 0.0, y: 0.0 };
        for ancestor in ancestors.iter().rev() {
            if let RenderingTree::Special(special) = ancestor {
                match special {
                    SpecialRenderingNode::Translate(translate) => {
                        xy.x += translate.x;
                        xy.y += translate.y;
                    }
                    SpecialRenderingNode::Absolute(absolute) => {
                        xy.x += absolute.x;
                        xy.y += absolute.y;
                        break;
                    }
                    SpecialRenderingNode::Rotate(rotate) => {
                        let matrix = rotate.get_matrix();
                        xy = matrix.transform_xy(xy);
                    }
                    SpecialRenderingNode::Scale(scale) => {
                        xy.x *= scale.x;
                        xy.y *= scale.y;
                    }
                    _ => {}
                }
            }
        }
        xy
    }
}

fn is_xy_clip_in_by_ancestors(xy: Xy<f32>, ancestors: &[&RenderingTree]) -> bool {
    let mut ancestors = ancestors.to_vec();
    while let Some(closest_ancestor) = ancestors.pop() {
        if let RenderingTree::Special(special) = closest_ancestor {
            if let SpecialRenderingNode::Clip(clip) = special {
                let utils = VisitUtils {
                    ancestors: &ancestors,
                    rendering_tree: closest_ancestor,
                };
                let local_xy = utils.to_local_xy(xy);
                if !clip.is_clip_in(local_xy) {
                    return false;
                }
            }
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use float_cmp::assert_approx_eq;
    use wasm_bindgen_test::wasm_bindgen_test;

    #[test]
    #[wasm_bindgen_test]
    fn visiting_order_should_be_rln() {
        /*
            tree:
                 0
               /   \
              1     2
             / \   / \
            3   4 5   6
                |      \
                7       8

            rln order: 8, 6, 5, 2, 7, 4, 3, 1, 0
        */
        let node_8 = RenderingTree::Empty.with_id("8");
        let node_7 = RenderingTree::Empty.with_id("7");
        let node_6 = render([node_8]).with_id("6");
        let node_5 = RenderingTree::Empty.with_id("5");
        let node_4 = render([node_7]).with_id("4");
        let node_3 = RenderingTree::Empty.with_id("3");
        let node_2 = render([node_5, node_6]).with_id("2");
        let node_1 = render([node_3, node_4]).with_id("1");
        let node_0 = render([node_1, node_2]).with_id("0");

        let mut called_ids = vec![];
        node_0.visit_rln(|rendering_tree, _| {
            if let RenderingTree::Special(rendering_tree) = rendering_tree {
                if let SpecialRenderingNode::WithId(with_id) = rendering_tree {
                    called_ids.push(with_id.id.clone());
                }
            }
            ControlFlow::Continue(())
        });

        assert_eq!(
            called_ids,
            vec!["8", "6", "5", "2", "7", "4", "3", "1", "0"]
        );
    }

    #[test]
    #[wasm_bindgen_test]
    fn to_local_xy_should_work() {
        /*
            tree:
                   0
                 /   \
                1     2
               / \   / \
              3   4 5   6
             /       \   \
            9         7   8
            |
            10
        */
        let node_10 = crate::translate(20.0, 20.0, RenderingTree::Empty.with_id("10"));
        let node_9 = crate::scale(2.0, 2.0, render([node_10]).with_id("9"));
        let node_8 = crate::translate(20.0, 20.0, RenderingTree::Empty.with_id("8"));
        let node_7 = crate::translate(20.0, 20.0, RenderingTree::Empty.with_id("7"));
        let node_6 = crate::absolute(100.0, 100.0, render([node_8]).with_id("6"));
        let node_5 = crate::rotate(std::f32::consts::PI, render([node_7]).with_id("5"));
        let node_4 = crate::translate(20.0, 30.0, RenderingTree::Empty.with_id("4"));
        let node_3 = render([node_9]).with_id("3");
        let node_2 = render([node_5, node_6]).with_id("2");
        let node_1 = crate::translate(100.0, 200.0, render([node_3, node_4]).with_id("1"));
        let node_0 = render([node_1, node_2]).with_id("0");

        let mut call_count = 0;

        node_0.visit_rln(|rendering_tree, utils| {
            let xy = Xy { x: 10.0, y: 10.0 };
            if let RenderingTree::Special(rendering_tree) = rendering_tree {
                if let SpecialRenderingNode::WithId(with_id) = rendering_tree {
                    let local_xy = utils.to_local_xy(xy);
                    match with_id.id.as_str() {
                        "0" => {
                            assert_approx_eq!(f32, local_xy.x, 10.0, ulps = 2);
                            assert_approx_eq!(f32, local_xy.y, 10.0, ulps = 2);
                            call_count += 1;
                        }
                        "1" => {
                            assert_approx_eq!(f32, local_xy.x, -90.0, ulps = 2);
                            assert_approx_eq!(f32, local_xy.y, -190.0, ulps = 2);
                            call_count += 1;
                        }
                        "2" => {
                            assert_approx_eq!(f32, local_xy.x, 10.0, ulps = 2);
                            assert_approx_eq!(f32, local_xy.y, 10.0, ulps = 2);
                            call_count += 1;
                        }
                        "3" => {
                            assert_approx_eq!(f32, local_xy.x, -90.0, ulps = 2);
                            assert_approx_eq!(f32, local_xy.y, -190.0, ulps = 2);
                            call_count += 1;
                        }
                        "4" => {
                            assert_approx_eq!(f32, local_xy.x, -110.0, ulps = 2);
                            assert_approx_eq!(f32, local_xy.y, -220.0, ulps = 2);
                            call_count += 1;
                        }
                        "5" => {
                            assert_approx_eq!(f32, local_xy.x, -10.0, ulps = 2);
                            assert_approx_eq!(f32, local_xy.y, -10.0, ulps = 2);
                            call_count += 1;
                        }
                        "6" => {
                            assert_approx_eq!(f32, local_xy.x, -90.0, ulps = 2);
                            assert_approx_eq!(f32, local_xy.y, -90.0, ulps = 2);
                            call_count += 1;
                        }
                        "7" => {
                            assert_approx_eq!(f32, local_xy.x, -30.0, ulps = 2);
                            assert_approx_eq!(f32, local_xy.y, -30.0, ulps = 2);
                            call_count += 1;
                        }
                        "8" => {
                            assert_approx_eq!(f32, local_xy.x, -110.0, ulps = 2);
                            assert_approx_eq!(f32, local_xy.y, -110.0, ulps = 2);
                            call_count += 1;
                        }
                        "9" => {
                            assert_approx_eq!(f32, local_xy.x, -45.0, ulps = 2);
                            assert_approx_eq!(f32, local_xy.y, -95.0, ulps = 2);
                            call_count += 1;
                        }
                        "10" => {
                            assert_approx_eq!(f32, local_xy.x, -65.0, ulps = 2);
                            assert_approx_eq!(f32, local_xy.y, -115.0, ulps = 2);
                            call_count += 1;
                        }
                        _ => {}
                    }
                }
            }
            ControlFlow::Continue(())
        });
        assert_eq!(call_count, 11);
    }

    #[test]
    #[wasm_bindgen_test]
    fn to_local_xy_translate_scale_translate_test() {
        let node_2 = crate::translate(2.0, 2.0, render([]).with_id("2"));
        let node_1 = crate::scale(2.0, 2.0, render([node_2]).with_id("1"));
        let node_0 = crate::translate(2.0, 2.0, render([node_1]).with_id("0"));

        let mut call_count = 0;

        node_0.visit_rln(|rendering_tree, utils| {
            let xy_0_0 = Xy { x: 0.0, y: 0.0 };
            let xy_10_10 = Xy { x: 10.0, y: 10.0 };
            if let RenderingTree::Special(rendering_tree) = rendering_tree {
                if let SpecialRenderingNode::WithId(with_id) = rendering_tree {
                    let local_xy_0_0 = utils.to_local_xy(xy_0_0);
                    let local_xy_10_10 = utils.to_local_xy(xy_10_10);
                    match with_id.id.as_str() {
                        "0" => {
                            assert_approx_eq!(f32, local_xy_0_0.x, -2.0, ulps = 2);
                            assert_approx_eq!(f32, local_xy_0_0.y, -2.0, ulps = 2);
                            assert_approx_eq!(f32, local_xy_10_10.x, 8.0, ulps = 2);
                            assert_approx_eq!(f32, local_xy_10_10.y, 8.0, ulps = 2);
                            call_count += 1;
                        }
                        "1" => {
                            assert_approx_eq!(f32, local_xy_0_0.x, -1.0, ulps = 2);
                            assert_approx_eq!(f32, local_xy_0_0.y, -1.0, ulps = 2);
                            assert_approx_eq!(f32, local_xy_10_10.x, 4.0, ulps = 2);
                            assert_approx_eq!(f32, local_xy_10_10.y, 4.0, ulps = 2);
                            call_count += 1;
                        }
                        "2" => {
                            assert_approx_eq!(f32, local_xy_0_0.x, -3.0, ulps = 2);
                            assert_approx_eq!(f32, local_xy_0_0.y, -3.0, ulps = 2);
                            assert_approx_eq!(f32, local_xy_10_10.x, 2.0, ulps = 2);
                            assert_approx_eq!(f32, local_xy_10_10.y, 2.0, ulps = 2);
                            call_count += 1;
                        }
                        _ => {}
                    }
                }
            }
            ControlFlow::Continue(())
        });
        assert_eq!(call_count, 3);
    }

    #[test]
    #[wasm_bindgen_test]
    fn to_local_xy_translate_after_scale_test() {
        let node_1 = crate::translate(2.0, 2.0, render([]).with_id("1"));
        let node_0 = crate::scale(2.0, 2.0, render([node_1]).with_id("0"));

        let mut call_count = 0;

        node_0.visit_rln(|rendering_tree, utils| {
            let xy_0_0 = Xy { x: 0.0, y: 0.0 };
            let xy_10_10 = Xy { x: 10.0, y: 10.0 };
            if let RenderingTree::Special(rendering_tree) = rendering_tree {
                if let SpecialRenderingNode::WithId(with_id) = rendering_tree {
                    let local_xy_0_0 = utils.to_local_xy(xy_0_0);
                    let local_xy_10_10 = utils.to_local_xy(xy_10_10);
                    match with_id.id.as_str() {
                        "0" => {
                            assert_approx_eq!(f32, local_xy_0_0.x, 0.0, ulps = 2);
                            assert_approx_eq!(f32, local_xy_0_0.y, 0.0, ulps = 2);
                            assert_approx_eq!(f32, local_xy_10_10.x, 5.0, ulps = 2);
                            assert_approx_eq!(f32, local_xy_10_10.y, 5.0, ulps = 2);
                            call_count += 1;
                        }
                        "1" => {
                            assert_approx_eq!(f32, local_xy_0_0.x, -2.0, ulps = 2);
                            assert_approx_eq!(f32, local_xy_0_0.y, -2.0, ulps = 2);
                            assert_approx_eq!(f32, local_xy_10_10.x, 3.0, ulps = 2);
                            assert_approx_eq!(f32, local_xy_10_10.y, 3.0, ulps = 2);
                            call_count += 1;
                        }
                        _ => {}
                    }
                }
            }
            ControlFlow::Continue(())
        });
        assert_eq!(call_count, 2);
    }

    #[test]
    #[wasm_bindgen_test]
    fn with_ancestors_should_give_right_ancestors() {
        /*
            tree:
                 0
               /   \
              1     2
             / \   /
            3   4 5

            rln order: 5, 2, 4, 3, 1, 0
        */
        let node_5 = RenderingTree::Empty.with_id("5");
        let node_4 = RenderingTree::Empty.with_id("4");
        let node_3 = RenderingTree::Empty.with_id("3");
        let node_2 = render([node_5]).with_id("2");
        let node_1 = render([node_3, node_4]).with_id("1");
        let node_0 = render([node_1, node_2]).with_id("0");

        let mut with_ancestors_call_count = 0;

        fn get_ancestor_ids(ancestors: &[&RenderingTree]) -> Vec<String> {
            ancestors
                .iter()
                .filter_map(|node| {
                    if let RenderingTree::Special(rendering_tree) = node {
                        if let SpecialRenderingNode::WithId(with_id) = rendering_tree {
                            return Some(with_id.id.clone());
                        }
                    }
                    None
                })
                .collect()
        }

        node_0.visit_rln(|rendering_tree, utils| {
            if let RenderingTree::Special(rendering_tree) = rendering_tree {
                if let SpecialRenderingNode::WithId(with_id) = rendering_tree {
                    match with_id.id.as_str() {
                        "0" => {
                            utils.with_ancestors(|ancestors| {
                                let ancestors_ids = get_ancestor_ids(ancestors);
                                with_ancestors_call_count += 1;
                                assert_eq!(ancestors_ids, Vec::<String>::new());
                            });
                        }
                        "1" => utils.with_ancestors(|ancestors| {
                            let ancestors_ids = get_ancestor_ids(ancestors);
                            with_ancestors_call_count += 1;
                            assert_eq!(ancestors_ids, vec!["0"]);
                        }),
                        "2" => utils.with_ancestors(|ancestors| {
                            let ancestors_ids = get_ancestor_ids(ancestors);
                            with_ancestors_call_count += 1;
                            assert_eq!(ancestors_ids, vec!["0"]);
                        }),
                        "3" => utils.with_ancestors(|ancestors| {
                            let ancestors_ids = get_ancestor_ids(ancestors);
                            with_ancestors_call_count += 1;
                            assert_eq!(ancestors_ids, vec!["0", "1"]);
                        }),
                        "4" => utils.with_ancestors(|ancestors| {
                            let ancestors_ids = get_ancestor_ids(ancestors);
                            with_ancestors_call_count += 1;
                            assert_eq!(ancestors_ids, vec!["0", "1"]);
                        }),
                        "5" => utils.with_ancestors(|ancestors| {
                            let ancestors_ids = get_ancestor_ids(ancestors);
                            with_ancestors_call_count += 1;
                            assert_eq!(ancestors_ids, vec!["0", "2"]);
                        }),
                        _ => {}
                    }
                }
            }
            ControlFlow::Continue(())
        });
        assert_eq!(with_ancestors_call_count, 6);
    }
    #[test]
    #[wasm_bindgen_test]
    fn clip_should_block_checking_xy_in() {
        // TODO: Test this. We cannot test it right now because test runtime doesn't load canvaskit.
    }
}
