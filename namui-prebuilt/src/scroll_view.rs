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
        let (scroll_y, set_scroll_y) = ctx.state(|| 0.px());

        ctx.return_(ScrollView {
            xy: self.xy,
            scroll_bar_width: self.scroll_bar_width,
            height: self.height,
            content: self.content.clone(),
            scroll_y: *scroll_y,
            set_scroll_y,
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

        let content = ctx.ghost_render(content.as_ref());

        let Some(bounding_box) = content.get_bounding_box() else {
            return ctx.return_no();
        };

        let scroll_y = namui::math::num::clamp(
            scroll_y,
            px(0.0),
            px(0.0).max(bounding_box.height() - height),
        );

        let inner = clip(
            namui::PathBuilder::new().add_rect(Rect::Xywh {
                x: bounding_box.x(),
                y: bounding_box.y(),
                width: bounding_box.width(),
                height,
            }),
            namui::ClipOp::Intersect,
            translate(0.px(), -scroll_y.floor(), content),
        );

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

        ctx.return_(hooks::translate(
            xy.x,
            xy.y,
            (
                whole_rect.on_event(move |event| match event {
                    Event::Wheel { event } => {
                        let next_scroll_y = namui::math::num::clamp(
                            scroll_y + px(event.delta_xy.y),
                            px(0.0),
                            (px(0.0)).max(bounding_box.height() - height),
                        );

                        set_scroll_y.set(next_scroll_y);

                        event.stop_propagation();
                    }
                    _ => {}
                }),
                inner,
                scroll_bar,
            ),
        ))
    }
}
