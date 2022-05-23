use super::*;

pub struct VisitUtils<'a> {
    pub is_xy_in: &'a dyn Fn(&Xy<f32>) -> bool,
}

impl RenderingTree {
    pub fn visit_rln<F>(&self, mut callback: F)
    where
        F: FnMut(&Self, VisitUtils) -> ControlFlow<()>,
    {
        self.try_visit_rln(&mut callback, &vec![]);
    }
    fn try_visit_rln<F>(&self, callback: &mut F, ancestors: &Vec<&Self>) -> ControlFlow<()>
    where
        F: FnMut(&Self, VisitUtils) -> ControlFlow<()>,
    {
        let mut next_ancestors = ancestors.clone();
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
                match special.get_child().try_visit_rln(callback, &next_ancestors) {
                    ControlFlow::Break(_) => {
                        return ControlFlow::Break(());
                    }
                    _ => {}
                }
            }
            _ => {}
        }

        let utils = VisitUtils {
            is_xy_in: &|xy| self.is_xy_in(xy, ancestors),
        };
        callback(self, utils)
    }
    fn is_xy_in(&self, xy: &Xy<f32>, ancestors: &Vec<&Self>) -> bool {
        let bounding_box = self.get_bounding_box_with_ancestors(ancestors);

        match bounding_box {
            Some(bounding_box) => bounding_box.is_xy_in(&xy),
            None => false,
        }
    }
    fn get_bounding_box_with_ancestors(&self, ancestors: &Vec<&Self>) -> Option<XywhRect<f32>> {
        let self_bounding_box = self.get_bounding_box();
        if self_bounding_box.is_none() {
            return None;
        }

        let mut bounding_box = self_bounding_box.unwrap();

        for ancestor in ancestors.iter().rev() {
            match ancestor {
                RenderingTree::Special(special) => match special {
                    SpecialRenderingNode::Translate(translate) => {
                        bounding_box.x += translate.x;
                        bounding_box.y += translate.y;
                    }
                    SpecialRenderingNode::Clip(clip) => {
                        let path = clip.path_builder.build();

                        let clip_bounding_box = path.get_bounding_box();

                        let bounding_box_ltrb = LtrbRect {
                            left: bounding_box.x,
                            top: bounding_box.y,
                            right: bounding_box.x + bounding_box.width,
                            bottom: bounding_box.y + bounding_box.height,
                        };

                        let bounding_box_ltrb = match clip.clip_op {
                            ClipOp::Intersect => clip_bounding_box.and_then(|clip_bounding_box| {
                                bounding_box_ltrb.intersect(&clip_bounding_box)
                            }),
                            ClipOp::Difference => match clip_bounding_box {
                                Some(clip_bounding_box) => {
                                    if bounding_box_ltrb == clip_bounding_box {
                                        return None;
                                    }

                                    let xs = [
                                        bounding_box_ltrb.left,
                                        bounding_box_ltrb.right,
                                        clip_bounding_box.left,
                                        clip_bounding_box.right,
                                    ];
                                    let ys = [
                                        bounding_box_ltrb.top,
                                        bounding_box_ltrb.bottom,
                                        clip_bounding_box.top,
                                        clip_bounding_box.bottom,
                                    ];

                                    let sixteen_xys =
                                        xs.iter().zip(ys.iter()).map(|(x, y)| Xy { x: *x, y: *y });

                                    let difference_area_xys = sixteen_xys.filter(|xy| {
                                        (clip_bounding_box.is_xy_outside(&xy)
                                            || clip_bounding_box.is_xy_on_border(&xy))
                                            && !bounding_box_ltrb.is_xy_outside(&xy)
                                    });

                                    difference_area_xys.fold(None, |acc: Option<LtrbRect>, xy| {
                                        match acc {
                                            Some(rect) => Some(LtrbRect {
                                                left: rect.left.min(xy.x),
                                                top: rect.top.min(xy.y),
                                                right: rect.right.max(xy.x),
                                                bottom: rect.bottom.max(xy.y),
                                            }),
                                            None => Some(LtrbRect {
                                                left: xy.x,
                                                top: xy.y,
                                                right: xy.x,
                                                bottom: xy.y,
                                            }),
                                        }
                                    })
                                }
                                None => Some(bounding_box_ltrb),
                            },
                        };

                        match bounding_box_ltrb {
                            Some(bounding_box_ltrb) => {
                                bounding_box = XywhRect {
                                    x: bounding_box_ltrb.left,
                                    y: bounding_box_ltrb.top,
                                    width: bounding_box_ltrb.right - bounding_box_ltrb.left,
                                    height: bounding_box_ltrb.bottom - bounding_box_ltrb.top,
                                };
                            }
                            None => return None,
                        };
                    }
                    SpecialRenderingNode::Absolute(absolute) => {
                        bounding_box.x = absolute.x;
                        bounding_box.y = absolute.y;
                    }
                    SpecialRenderingNode::Rotate(rotate) => {
                        let four_points = [
                            Xy {
                                x: bounding_box.x,
                                y: bounding_box.y,
                            },
                            Xy {
                                x: bounding_box.x + bounding_box.width,
                                y: bounding_box.y,
                            },
                            Xy {
                                x: bounding_box.x + bounding_box.width,
                                y: bounding_box.y + bounding_box.height,
                            },
                            Xy {
                                x: bounding_box.x,
                                y: bounding_box.y + bounding_box.height,
                            },
                        ];

                        let rotation_matrix = rotate.get_matrix();
                        let rotated_points = four_points
                            .iter()
                            .map(|xy| rotation_matrix.transform_xy(xy))
                            .collect::<Vec<_>>();

                        let (left, top, right, bottom) = rotated_points.iter().fold(
                            (std::f32::MAX, std::f32::MAX, std::f32::MIN, std::f32::MIN),
                            |(left, top, right, bottom), xy| {
                                (
                                    f32::min(left, xy.x),
                                    f32::min(top, xy.y),
                                    f32::max(right, xy.x),
                                    f32::max(bottom, xy.y),
                                )
                            },
                        );

                        bounding_box = XywhRect {
                            x: left,
                            y: top,
                            width: right - left,
                            height: bottom - top,
                        };
                    }
                    _ => {}
                },
                _ => {}
            }
        }
        Some(bounding_box)
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
