use namui::prelude::*;

#[component]
pub struct ScrollView<C: Component> {
    pub wh: Wh<Px>,
    pub scroll_bar_width: Px,
    pub content: C,
    pub scroll_y: Px,
    pub set_scroll_y: SetState<Px>,
}

impl<C: Component> Component for ScrollView<C> {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        ctx.component(ScrollViewWithCtx {
            wh: self.wh,
            scroll_bar_width: self.scroll_bar_width,
            content: |ctx| {
                ctx.add(self.content);
            },
            scroll_y: self.scroll_y,
            set_scroll_y: self.set_scroll_y,
        })
        .done()
    }
}

#[component]
pub struct AutoScrollView<C: Component> {
    pub wh: Wh<Px>,
    pub scroll_bar_width: Px,
    pub content: C,
}

impl<C: Component> Component for AutoScrollView<C> {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        let (scroll_y, set_scroll_y) = ctx.state(|| 0.px());

        ctx.component(ScrollView {
            wh: self.wh,
            scroll_bar_width: self.scroll_bar_width,
            content: self.content,
            scroll_y: *scroll_y,
            set_scroll_y,
        });

        ctx.done()
    }
}

#[component]
pub struct AutoScrollViewWithCtx<Func: FnOnce(&mut ComposeCtx)> {
    pub wh: Wh<Px>,
    pub scroll_bar_width: Px,
    #[skip_debug]
    pub content: Func,
}

impl<Func: FnOnce(&mut ComposeCtx)> Component for AutoScrollViewWithCtx<Func> {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        let (scroll_y, set_scroll_y) = ctx.state(|| 0.px());

        ctx.component(ScrollViewWithCtx {
            wh: self.wh,
            scroll_bar_width: self.scroll_bar_width,
            content: self.content,
            scroll_y: *scroll_y,
            set_scroll_y,
        });

        ctx.done()
    }
}

#[component]
pub struct ScrollViewWithCtx<Func: FnOnce(&mut ComposeCtx)> {
    pub wh: Wh<Px>,
    pub scroll_bar_width: Px,
    #[skip_debug]
    pub content: Func,
    pub scroll_y: Px,
    pub set_scroll_y: SetState<Px>,
}

impl<Func: FnOnce(&mut ComposeCtx)> Component for ScrollViewWithCtx<Func> {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        let Self {
            wh,
            scroll_bar_width,
            content,
            scroll_y,
            set_scroll_y,
        } = self;

        let height = wh.height;

        ctx.compose(|ctx| {
            let rendering_tree = ctx
                .clip(
                    namui::Path::new().add_rect(wh.to_rect()),
                    namui::ClipOp::Intersect,
                )
                .translate((0.px(), -scroll_y.floor()))
                .ghost_compose(None, content);
            let Some(bounding_box) = rendering_tree.bounding_box() else {
                return;
            };

            let clamped_scroll_y = namui::math::num::clamp(
                scroll_y,
                px(0.0),
                px(0.0).max(bounding_box.height() - height),
            );

            if clamped_scroll_y != scroll_y {
                set_scroll_y.set(clamped_scroll_y);
            }

            let scroll_bar = |ctx: &mut ComposeCtx| {
                if bounding_box.height() > height {
                    let scroll_bar_handle_height = height * (height / bounding_box.height());

                    let scroll_bar_y = (height - scroll_bar_handle_height)
                        * (scroll_y / (bounding_box.height() - height));

                    ctx.add(namui::rect(RectParam {
                        rect: Rect::Xywh {
                            x: wh.width - scroll_bar_width, // iOS Style!
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
                    }));
                }
            };

            let wheeler = namui::rect(RectParam {
                rect: wh.to_rect(),
                style: RectStyle {
                    fill: Some(RectFill {
                        color: Color::TRANSPARENT,
                    }),
                    ..Default::default()
                },
            })
            .attach_event(|event| {
                if let Event::Wheel { event } = event {
                    if event.is_local_xy_in() {
                        let next_scroll_y = namui::math::num::clamp(
                            scroll_y + px(event.delta_xy.y),
                            px(0.0),
                            (px(0.0)).max(bounding_box.height() - height),
                        );

                        set_scroll_y.set(next_scroll_y);
                        event.stop_propagation();
                    }
                }
            });

            ctx.compose(scroll_bar)
                .compose(|ctx| {
                    ctx.clip(
                        namui::Path::new().add_rect(wh.to_rect()),
                        namui::ClipOp::Intersect,
                    )
                    .translate((0.px(), -scroll_y.floor()))
                    .add(rendering_tree);
                })
                .add(wheeler);
        })
        .done()
    }
}
