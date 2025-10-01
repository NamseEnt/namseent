use crate::*;
use std::ops::ControlFlow;

impl XyIn for RenderingTree {
    fn xy_in(&self, calculator: &dyn SkCalculate, xy: Xy<Px>) -> bool {
        xy_in(self, calculator, xy, &[])
    }
}

impl XyIn for [&RenderingTree] {
    fn xy_in(&self, calculator: &dyn SkCalculate, xy: Xy<Px>) -> bool {
        self.iter().any(|node| node.xy_in(calculator, xy))
    }
}

pub struct VisitUtils<'a> {
    pub rendering_tree: &'a RenderingTree,
    pub ancestors: &'a [&'a RenderingTree],
}
impl VisitUtils<'_> {
    pub fn to_local_xy(&self, xy: Xy<Px>) -> Xy<Px> {
        self.rendering_tree.to_local_xy(xy, self.ancestors)
    }
    #[cfg(test)]
    fn with_ancestors(&self, mut func: impl FnMut(&[&RenderingTree])) {
        func(self.ancestors)
    }
}

pub trait Visit {
    fn visit_rln<F>(&self, callback: &mut F, ancestors: &[&Self]) -> ControlFlow<()>
    where
        F: FnMut(&Self, VisitUtils) -> ControlFlow<()>;
    fn to_local_xy(&self, xy: Xy<Px>, ancestors: &[&Self]) -> Xy<Px>;
    #[allow(dead_code)]
    fn get_xy(&self, ancestors: &[&RenderingTree]) -> Xy<Px>;
}

impl Visit for RenderingTree {
    fn visit_rln<F>(&self, callback: &mut F, ancestors: &[&Self]) -> ControlFlow<()>
    where
        F: FnMut(&Self, VisitUtils) -> ControlFlow<()>,
    {
        let mut next_ancestors = Vec::from(ancestors);
        next_ancestors.push(self);

        match self {
            RenderingTree::Children(children) => {
                for child in children.iter().rev() {
                    if let ControlFlow::Break(_) = child.visit_rln(callback, &next_ancestors) {
                        return ControlFlow::Break(());
                    }
                }
            }
            RenderingTree::Special(special) => {
                if let ControlFlow::Break(_) = special
                    .inner_rendering_tree_ref()
                    .visit_rln(callback, &next_ancestors)
                {
                    return ControlFlow::Break(());
                }
            }
            RenderingTree::Empty | RenderingTree::Node(_) => {}
        }

        let utils = VisitUtils {
            ancestors,
            rendering_tree: self,
        };
        callback(self, utils)
    }
    fn to_local_xy(&self, xy: Xy<Px>, ancestors: &[&Self]) -> Xy<Px> {
        let mut result_xy = xy;
        for ancestor in ancestors.iter() {
            if let RenderingTree::Special(special) = ancestor {
                match special {
                    SpecialRenderingNode::Translate(translate) => {
                        result_xy.x -= translate.x;
                        result_xy.y -= translate.y;
                    }
                    SpecialRenderingNode::Absolute(absolute) => {
                        result_xy = xy;
                        result_xy.x -= absolute.x;
                        result_xy.y -= absolute.y;
                    }
                    SpecialRenderingNode::Rotate(rotate) => {
                        result_xy = rotate.get_counter_wise_matrix().transform_xy(result_xy);
                    }
                    SpecialRenderingNode::Scale(scale) => {
                        result_xy.x /= *scale.x;
                        result_xy.y /= *scale.y;
                    }
                    SpecialRenderingNode::Transform(transform) => {
                        result_xy = transform.matrix.inverse().unwrap().transform_xy(result_xy);
                    }
                    SpecialRenderingNode::Clip(_)
                    | SpecialRenderingNode::OnTop(_)
                    | SpecialRenderingNode::MouseCursor(_) => {}
                }
            }
        }
        result_xy
    }
    #[allow(dead_code)]
    fn get_xy(&self, ancestors: &[&RenderingTree]) -> Xy<Px> {
        let mut xy = Xy {
            x: px(0.0),
            y: px(0.0),
        };
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
                        xy.x *= *scale.x;
                        xy.y *= *scale.y;
                    }
                    SpecialRenderingNode::Transform(transform) => {
                        xy = transform.matrix.transform_xy(xy);
                    }
                    SpecialRenderingNode::Clip(_)
                    | SpecialRenderingNode::OnTop(_)
                    | SpecialRenderingNode::MouseCursor(_) => {}
                }
            }
        }
        xy
    }
}

fn xy_in(
    rendering_tree: &RenderingTree,
    calculator: &dyn SkCalculate,
    xy: Xy<Px>,
    ancestors: &[&RenderingTree],
) -> bool {
    let mut result = false;
    let _ = rendering_tree.visit_rln(
        &mut |node, utils| {
            if let RenderingTree::Node(node) = node {
                let local_xy = utils.to_local_xy(xy);
                if node.xy_in(calculator, local_xy)
                    && is_xy_clip_in_by_ancestors(calculator, xy, utils.ancestors)
                {
                    result = true;
                    ControlFlow::Break(())
                } else {
                    ControlFlow::Continue(())
                }
            } else {
                ControlFlow::Continue(())
            }
        },
        ancestors,
    );

    result
}

fn is_xy_clip_in_by_ancestors(
    calculator: &dyn SkCalculate,
    xy: Xy<Px>,
    ancestors: &[&RenderingTree],
) -> bool {
    let mut ancestors = ancestors.to_vec();
    while let Some(closest_ancestor) = ancestors.pop() {
        if let RenderingTree::Special(special) = closest_ancestor {
            if let SpecialRenderingNode::Clip(clip) = special {
                let utils = VisitUtils {
                    ancestors: &ancestors,
                    rendering_tree: closest_ancestor,
                };
                let local_xy = utils.to_local_xy(xy);
                if !clip.clip_in(calculator, local_xy) {
                    return false;
                }
            } else if let SpecialRenderingNode::OnTop(_) = special {
                return true;
            }
        }
    }

    true
}

trait ClipIn {
    fn clip_in(&self, calculator: &dyn SkCalculate, xy: Xy<Px>) -> bool;
}

impl ClipIn for ClipNode {
    fn clip_in(&self, calculator: &dyn SkCalculate, xy: Xy<Px>) -> bool {
        let xy_in = self.path.xy_in(calculator, xy);

        match self.clip_op {
            ClipOp::Intersect => xy_in,
            ClipOp::Difference => !xy_in,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use float_cmp::assert_approx_eq;

    // Helper function to create a dummy leaf node
    fn dummy_leaf() -> RenderingTree {
        RenderingTree::Node(crate::DrawCommand::Path {
            command: Box::new(crate::PathDrawCommand {
                path: crate::Path::new(),
                paint: crate::Paint::default(),
            }),
        })
    }

    #[test]
    fn rln_visiting_order_should_be_rln() {
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
        let node_8 = RenderingTree::Empty;
        let node_7 = RenderingTree::Empty;
        let node_6 = RenderingTree::Children(vec![node_8.clone()]);
        let node_5 = RenderingTree::Empty;
        let node_4 = RenderingTree::Children(vec![node_7.clone()]);
        let node_3 = RenderingTree::Empty;
        let node_2 = RenderingTree::Children(vec![node_5.clone(), node_6.clone()]);
        let node_1 = RenderingTree::Children(vec![node_3.clone(), node_4.clone()]);
        let node_0 = RenderingTree::Children(vec![node_1.clone(), node_2.clone()]);

        let mut rendering_trees = vec![];
        let _ = node_0.visit_rln(
            &mut |rendering_tree, _| {
                rendering_trees.push(rendering_tree.clone());
                ControlFlow::Continue(())
            },
            &[],
        );

        assert_eq!(
            rendering_trees,
            vec![
                node_8, node_6, node_5, node_2, node_7, node_4, node_3, node_1, node_0,
            ]
        );
    }

    #[test]
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

        let node_10 = crate::translate(px(20.0), px(20.0), dummy_leaf());
        let node_9 = crate::scale(2.0, 2.0, RenderingTree::Children(vec![node_10.clone()]));
        let node_8 = crate::translate(px(20.0), px(20.0), dummy_leaf());
        let node_7 = crate::translate(px(10.0), px(20.0), dummy_leaf());
        let node_6 = crate::absolute(
            px(100.0),
            px(100.0),
            RenderingTree::Children(vec![node_8.clone()]),
        );
        let node_5 = crate::rotate(
            (std::f32::consts::PI / 2.0).rad(),
            RenderingTree::Children(vec![node_7.clone()]),
        );
        let node_4 = crate::translate(px(20.0), px(30.0), dummy_leaf());
        let node_3 = RenderingTree::Children(vec![node_9.clone()]);
        let node_2 = crate::translate(
            px(50.0),
            px(100.0),
            RenderingTree::Children(vec![node_5.clone(), node_6.clone()]),
        );
        let node_1 = crate::translate(
            px(100.0),
            px(200.0),
            RenderingTree::Children(vec![node_3.clone(), node_4.clone()]),
        );
        let node_0 = RenderingTree::Children(vec![node_1.clone(), node_2.clone()]);

        let mut call_count = 0;

        let answer = [
            Xy::new(-110.px(), -110.px()),
            Xy::new(-90.px(), -90.px()),
            Xy::new(-100.px(), 20.px()),
            Xy::new(-90.px(), 40.px()),
            Xy::new(-40.px(), -90.px()),
            Xy::new(-110.px(), -220.px()),
            Xy::new(-65.px(), -115.px()),
            Xy::new(-45.px(), -95.px()),
            Xy::new(-90.px(), -190.px()),
            Xy::new(-90.px(), -190.px()),
            Xy::new(10.px(), 10.px()),
        ];

        let _ = node_0.visit_rln(
            &mut |rendering_tree, utils| {
                if matches!(rendering_tree, RenderingTree::Empty)
                    || matches!(rendering_tree, RenderingTree::Special(_))
                {
                    return ControlFlow::Continue(());
                }

                let parent = utils.ancestors.last();
                let is_direct_child_of_special =
                    parent.is_some_and(|p| matches!(p, RenderingTree::Special(_)));
                let is_direct_child_of_children =
                    parent.is_some_and(|p| matches!(p, RenderingTree::Children(_)));
                let is_top_level = utils.ancestors.is_empty();

                if !is_direct_child_of_special && !is_direct_child_of_children && !is_top_level {
                    return ControlFlow::Continue(());
                }

                let xy = Xy {
                    x: px(10.0),
                    y: px(10.0),
                };

                let local_xy = utils.to_local_xy(xy);
                assert_approx_eq!(
                    f32,
                    local_xy.x.as_f32(),
                    answer[call_count].x.as_f32(),
                    ulps = 2
                );
                assert_approx_eq!(
                    f32,
                    local_xy.y.as_f32(),
                    answer[call_count].y.as_f32(),
                    ulps = 2
                );
                call_count += 1;

                ControlFlow::Continue(())
            },
            &[],
        );
        assert_eq!(call_count, 11);
    }
    #[test]
    fn to_local_xy_should_work_with_matrix_transform() {
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
        let node_10 = crate::transform(TransformMatrix::from_translate(20.0, 20.0), dummy_leaf());
        let node_9 = crate::transform(
            TransformMatrix::from_scale(2.0, 2.0),
            RenderingTree::wrap([node_10.clone()]),
        );
        let node_8 = crate::transform(TransformMatrix::from_translate(20.0, 20.0), dummy_leaf());
        let node_7 = crate::transform(TransformMatrix::from_translate(10.0, 20.0), dummy_leaf());
        let node_6 = crate::absolute(px(100.0), px(100.0), RenderingTree::wrap([node_8.clone()]));
        let node_5 = crate::transform(
            TransformMatrix::from_rotate(90.deg()),
            RenderingTree::wrap([node_7.clone()]),
        );
        let node_4 = crate::transform(TransformMatrix::from_translate(20.0, 30.0), dummy_leaf());
        let node_3 = RenderingTree::wrap([node_9.clone()]);
        let node_2 = crate::transform(
            TransformMatrix::from_translate(50.0, 100.0),
            RenderingTree::wrap([node_5.clone(), node_6.clone()]),
        );
        let node_1 = crate::transform(
            TransformMatrix::from_translate(100.0, 200.0),
            RenderingTree::wrap([node_3.clone(), node_4.clone()]),
        );
        let node_0 = RenderingTree::wrap([node_1.clone(), node_2.clone()]);

        let mut call_count = 0;

        let answer = [
            Xy::new(-90.px(), -90.px()),
            Xy::new(-40.px(), -90.px()),
            Xy::new(-90.px(), 40.px()),
            Xy::new(-40.px(), -90.px()),
            Xy::new(10.px(), 10.px()),
            Xy::new(-90.px(), -190.px()),
            Xy::new(-45.px(), -95.px()),
            Xy::new(-90.px(), -190.px()),
            Xy::new(10.px(), 10.px()),
        ];

        let _ = node_0.visit_rln(
            &mut |rendering_tree, utils| {
                if !matches!(rendering_tree, RenderingTree::Special(_)) {
                    return ControlFlow::Continue(());
                }

                let xy = Xy {
                    x: px(10.0),
                    y: px(10.0),
                };

                let local_xy = utils.to_local_xy(xy);
                assert_approx_eq!(
                    f32,
                    local_xy.x.as_f32(),
                    answer[call_count].x.as_f32(),
                    ulps = 2
                );
                assert_approx_eq!(
                    f32,
                    local_xy.y.as_f32(),
                    answer[call_count].y.as_f32(),
                    ulps = 2
                );
                call_count += 1;

                ControlFlow::Continue(())
            },
            &[],
        );
        assert_eq!(call_count, 9);
    }

    #[test]
    fn to_local_xy_translate_scale_translate_test() {
        let node_2 = crate::translate(px(2.0), px(2.0), dummy_leaf());
        let node_1 = crate::scale(2.0, 2.0, RenderingTree::wrap([node_2.clone()]));
        let node_0 = crate::translate(px(2.0), px(2.0), RenderingTree::wrap([node_1.clone()]));

        let mut call_count = 0;

        // (xy_0_0, xy_10_10)
        let answer = [
            (Xy::new(-1.px(), -1.px()), Xy::new(4.px(), 4.px())),
            (Xy::new(-2.px(), -2.px()), Xy::new(8.px(), 8.px())),
            (Xy::new(0.px(), 0.px()), Xy::new(10.px(), 10.px())),
        ];

        let _ = node_0.visit_rln(
            &mut |rendering_tree, utils| {
                if !matches!(rendering_tree, RenderingTree::Special(_)) {
                    return ControlFlow::Continue(());
                }

                let xy_0_0 = Xy {
                    x: px(0.0),
                    y: px(0.0),
                };
                let xy_10_10 = Xy {
                    x: px(10.0),
                    y: px(10.0),
                };

                let local_xy_0_0 = utils.to_local_xy(xy_0_0);
                let local_xy_10_10 = utils.to_local_xy(xy_10_10);

                assert_approx_eq!(
                    f32,
                    local_xy_0_0.x.as_f32(),
                    answer[call_count].0.x.as_f32(),
                    ulps = 2
                );
                assert_approx_eq!(
                    f32,
                    local_xy_0_0.y.as_f32(),
                    answer[call_count].0.y.as_f32(),
                    ulps = 2
                );
                assert_approx_eq!(
                    f32,
                    local_xy_10_10.x.as_f32(),
                    answer[call_count].1.x.as_f32(),
                    ulps = 2
                );
                assert_approx_eq!(
                    f32,
                    local_xy_10_10.y.as_f32(),
                    answer[call_count].1.y.as_f32(),
                    ulps = 2
                );
                call_count += 1;

                ControlFlow::Continue(())
            },
            &[],
        );
        assert_eq!(call_count, 3);
    }

    #[test]
    fn to_local_xy_translate_after_scale_test() {
        let node_1 = crate::translate(px(2.0), px(2.0), dummy_leaf());
        let node_0 = crate::scale(2.0, 2.0, RenderingTree::wrap([node_1.clone()]));

        let mut call_count = 0;

        // (xy_0_0, xy_10_10)
        let answer = [
            (Xy::new(0.px(), 0.px()), Xy::new(5.px(), 5.px())),
            (Xy::new(0.px(), 0.px()), Xy::new(10.px(), 10.px())),
        ];

        let _ = node_0.visit_rln(
            &mut |rendering_tree, utils| {
                if !matches!(rendering_tree, RenderingTree::Special(_)) {
                    return ControlFlow::Continue(());
                }

                let xy_0_0 = Xy {
                    x: px(0.0),
                    y: px(0.0),
                };
                let xy_10_10 = Xy {
                    x: px(10.0),
                    y: px(10.0),
                };

                let local_xy_0_0 = utils.to_local_xy(xy_0_0);
                let local_xy_10_10 = utils.to_local_xy(xy_10_10);

                assert_approx_eq!(
                    f32,
                    local_xy_0_0.x.as_f32(),
                    answer[call_count].0.x.as_f32(),
                    ulps = 2
                );
                assert_approx_eq!(
                    f32,
                    local_xy_0_0.y.as_f32(),
                    answer[call_count].0.y.as_f32(),
                    ulps = 2
                );
                assert_approx_eq!(
                    f32,
                    local_xy_10_10.x.as_f32(),
                    answer[call_count].1.x.as_f32(),
                    ulps = 2
                );
                assert_approx_eq!(
                    f32,
                    local_xy_10_10.y.as_f32(),
                    answer[call_count].1.y.as_f32(),
                    ulps = 2
                );
                call_count += 1;

                ControlFlow::Continue(())
            },
            &[],
        );
        assert_eq!(call_count, 2);
    }

    #[test]
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
        let node_5 = RenderingTree::Empty;
        let node_4 = RenderingTree::Empty;
        let node_3 = RenderingTree::Empty;
        let node_2 = RenderingTree::Children(vec![node_5.clone()]);
        let node_1 = RenderingTree::Children(vec![node_3.clone(), node_4.clone()]);
        let node_0 = RenderingTree::Children(vec![node_1.clone(), node_2.clone()]);

        let mut with_ancestors_call_count = 0;

        let expected_ancestors_list: Vec<Vec<&RenderingTree>> = vec![
            vec![&node_0, &node_2], // node_5
            vec![&node_0],          // node_2
            vec![&node_0, &node_1], // node_4
            vec![&node_0, &node_1], // node_3
            vec![&node_0],          // node_1
            vec![],                 // node_0
        ];

        let _ = node_0.visit_rln(
            &mut |_rendering_tree, utils| {
                utils.with_ancestors(|ancestors| {
                    assert_eq!(
                        ancestors,
                        expected_ancestors_list[with_ancestors_call_count]
                    );
                    with_ancestors_call_count += 1;
                });
                ControlFlow::Continue(())
            },
            &[],
        );
        assert_eq!(with_ancestors_call_count, 6);
    }
    #[test]
    fn clip_should_block_checking_xy_in() {
        // TODO: Test this. We cannot test it right now because test runtime doesn't load canvaskit.
    }
}
