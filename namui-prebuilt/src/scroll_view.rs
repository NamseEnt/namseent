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
    fn render<'a>(&'a self, ctx: &'a RenderCtx) -> RenderDone {
        let (scroll_y, set_scroll_y) = ctx.use_state(|| 0.px());

        ctx.use_children(|ctx| {
            ctx.add(ScrollView {
                xy: self.xy,
                scroll_bar_width: self.scroll_bar_width,
                height: self.height,
                content: self.content.clone(),
                scroll_y: *scroll_y,
                set_scroll_y,
            });

            ctx.done()
        })
    }
}

impl Component for ScrollView<'_> {
    fn render<'a>(&'a self, ctx: &'a RenderCtx) -> RenderDone {
        let &Self {
            xy,
            scroll_bar_width,
            height,
            ref content,
            scroll_y,
            set_scroll_y,
        } = self;

        ctx.use_children_with_rendering_tree(
            |ctx| {
                ctx.add(content.as_ref());

                ctx.done()
            },
            move |children| {
                let content = namui::render(children);

                let content_bounding_box = content.get_bounding_box();
                if content_bounding_box.is_none() {
                    return RenderingTree::Empty;
                }
                let content_bounding_box = content_bounding_box.unwrap();

                let scroll_y = namui::math::num::clamp(
                    scroll_y,
                    px(0.0),
                    px(0.0).max(content_bounding_box.height() - height),
                );

                let inner = namui::clip(
                    namui::PathBuilder::new().add_rect(Rect::Xywh {
                        x: content_bounding_box.x(),
                        y: content_bounding_box.y(),
                        width: content_bounding_box.width(),
                        height,
                    }),
                    namui::ClipOp::Intersect,
                    namui::translate(px(0.0), -scroll_y.floor(), content.clone()),
                );

                let scroll_bar_handle_height = height * (height / content_bounding_box.height());

                let scroll_bar_y = (height - scroll_bar_handle_height)
                    * (scroll_y / (content_bounding_box.height() - height));

                let scroll_bar = match content_bounding_box.height() > height {
                    true => rect(RectParam {
                        rect: Rect::Xywh {
                            x: content_bounding_box.width() - scroll_bar_width, // iOS Style!
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
                        width: content_bounding_box.width(),
                        height,
                    },
                    style: RectStyle {
                        fill: Some(RectFill {
                            color: Color::TRANSPARENT,
                        }),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .attach_event(move |builder| {
                    let height = height;
                    builder.on_wheel(move |event: WheelEvent| {
                        let next_scroll_y = namui::math::num::clamp(
                            scroll_y + px(event.delta_xy.y),
                            px(0.0),
                            (px(0.0)).max(content_bounding_box.height() - height),
                        );

                        set_scroll_y.set(next_scroll_y);

                        event.stop_propagation();
                    });
                });
                translate(xy.x, xy.y, namui::render([whole_rect, inner, scroll_bar]))
            },
        )
    }
}
