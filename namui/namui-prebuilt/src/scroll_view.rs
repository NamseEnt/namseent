use namui::*;

pub struct ScrollView<C: Component> {
    pub wh: Wh<Px>,
    pub scroll_bar_width: Px,
    pub content: C,
    pub scroll_y: Px,
    pub set_scroll_y: SetState<Px>,
}

impl<C: Component> Component for ScrollView<C> {
    fn render(self, ctx: &RenderCtx) {
        ctx.add(ScrollViewWithCtx {
            wh: self.wh,
            scroll_bar_width: self.scroll_bar_width,
            content: |ctx| {
                ctx.add(self.content);
            },
            scroll_y: self.scroll_y,
            set_scroll_y: self.set_scroll_y,
        });
    }
}

pub struct AutoScrollView<C: Component> {
    pub wh: Wh<Px>,
    pub scroll_bar_width: Px,
    pub content: C,
}

impl<C: Component> Component for AutoScrollView<C> {
    fn render(self, ctx: &RenderCtx) {
        let (scroll_y, set_scroll_y) = ctx.state(|| 0.px());

        ctx.add(ScrollView {
            wh: self.wh,
            scroll_bar_width: self.scroll_bar_width,
            content: self.content,
            scroll_y: *scroll_y,
            set_scroll_y,
        });
    }
}

pub struct AutoScrollViewWithCtx<Func: FnOnce(ComposeCtx)> {
    pub wh: Wh<Px>,
    pub scroll_bar_width: Px,
    pub content: Func,
}

impl<Func: FnOnce(ComposeCtx)> Component for AutoScrollViewWithCtx<Func> {
    fn render(self, ctx: &RenderCtx) {
        let (scroll_y, set_scroll_y) = ctx.state(|| 0.px());

        ctx.add(ScrollViewWithCtx {
            wh: self.wh,
            scroll_bar_width: self.scroll_bar_width,
            content: self.content,
            scroll_y: *scroll_y,
            set_scroll_y,
        });
    }
}

pub struct ScrollViewWithCtx<Func: FnOnce(ComposeCtx)> {
    pub wh: Wh<Px>,
    pub scroll_bar_width: Px,
    pub content: Func,
    pub scroll_y: Px,
    pub set_scroll_y: SetState<Px>,
}

impl<Func: FnOnce(ComposeCtx)> Component for ScrollViewWithCtx<Func> {
    fn render(self, ctx: &RenderCtx) {
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
                .ghost_compose(0_usize, content);
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

            let scroll_bar = |ctx: ComposeCtx| {
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
                .compose(|ctx| {
                    ctx.add(wheeler).attach_event(|event| {
                        if let Event::Wheel { event } = event
                            && event.is_local_xy_in()
                        {
                            let next_scroll_y = namui::math::num::clamp(
                                scroll_y + px(event.delta_xy.y),
                                px(0.0),
                                (px(0.0)).max(bounding_box.height() - height),
                            );

                            set_scroll_y.set(next_scroll_y);
                            event.stop_propagation();
                        }
                    });
                });
        });
    }
}
