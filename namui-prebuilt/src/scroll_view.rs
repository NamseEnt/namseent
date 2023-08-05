use namui::prelude::*;
use std::sync::Arc;

#[component]
pub struct ScrollView<'a> {
    pub xy: Xy<Px>,
    pub scroll_bar_width: Px,
    pub height: Px,
    pub content: Arc<dyn 'a + Component>,
    pub scroll_y: Px,
    pub set_scroll_y: SetState<Px>,
}

#[component]
pub struct AutoScrollView<'a> {
    pub xy: Xy<Px>,
    pub scroll_bar_width: Px,
    pub height: Px,
    pub content: Arc<dyn 'a + Component>,
}

impl Component for AutoScrollView<'_> {
    fn render<'a>(&'a self, ctx: &'a RenderCtx) {
        let (scroll_y, set_scroll_y) = ctx.state(|| 0.px());

        ctx.add(ScrollView {
            xy: self.xy,
            scroll_bar_width: self.scroll_bar_width,
            height: self.height,
            content: self.content.clone(),
            scroll_y: *scroll_y,
            set_scroll_y,
        });
    }
}

impl Component for ScrollView<'_> {
    fn render<'a>(&'a self, ctx: &'a RenderCtx) {
        let &Self {
            xy,
            scroll_bar_width,
            height,
            ref content,
            scroll_y,
            set_scroll_y,
        } = self;

        let (bounding_box, instance) = ctx.test_bounding_box(content.as_ref());
        let Some(bounding_box) = bounding_box else {
            return;
        };

        let scroll_y = namui::math::num::clamp(
            scroll_y,
            px(0.0),
            px(0.0).max(bounding_box.height() - height),
        );

        let inner = ctx.later_once(|ctx| {
            ctx.clip(
                namui::PathBuilder::new().add_rect(Rect::Xywh {
                    x: bounding_box.x(),
                    y: bounding_box.y(),
                    width: bounding_box.width(),
                    height,
                }),
                namui::ClipOp::Intersect,
            )
            .translate(Xy::new(0.px(), -scroll_y.floor()))
            .add_with_instance(content.as_ref(), instance);
        });

        let scroll_bar_handle_height = height * (height / bounding_box.height());

        let scroll_bar_y =
            (height - scroll_bar_handle_height) * (scroll_y / (bounding_box.height() - height));

        let scroll_bar = match bounding_box.height() > height {
            true => rect(RectParam {
                rect: Rect::Xywh {
                    x: bounding_box.width() - scroll_bar_width, // iOS Style!
                    y: scroll_bar_y,
                    width: scroll_bar_width,
                    height: scroll_bar_handle_height,
                },
                style: RectStyle {
                    fill: Some(RectFill {
                        color: Color::grayscale_f01(0.5),
                    }),
                    ..Default::default()
                },
                ..Default::default()
            }),
            false => RenderingTree::Empty,
        };
        let whole_rect = rect(RectParam {
            rect: Rect::Xywh {
                x: px(0.0),
                y: px(0.0),
                width: bounding_box.width(),
                height,
            },
            style: RectStyle {
                fill: Some(RectFill {
                    color: Color::TRANSPARENT,
                }),
                ..Default::default()
            },
            ..Default::default()
        });

        ctx.translate(xy)
            .branch(|ctx| {
                ctx.add(whole_rect).on_wheel(move |event: WheelEvent| {
                    let next_scroll_y = namui::math::num::clamp(
                        scroll_y + px(event.delta_xy.y),
                        px(0.0),
                        (px(0.0)).max(bounding_box.height() - height),
                    );

                    set_scroll_y.set(next_scroll_y);

                    event.stop_propagation();
                });
            })
            .add(inner)
            .add(scroll_bar);

        // ctx.return_(translate(
        //     xy,
        //     (
        //         whole_rect.on_wheel(move |event: WheelEvent| {
        //             let next_scroll_y = namui::math::num::clamp(
        //                 scroll_y + px(event.delta_xy.y),
        //                 px(0.0),
        //                 (px(0.0)).max(bounding_box.height() - height),
        //             );

        //             set_scroll_y.set(next_scroll_y);

        //             event.stop_propagation();
        //         }),
        //         inner,
        //         scroll_bar,
        //     ),
        // ));
    }
}
