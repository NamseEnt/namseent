use namui::prelude::*;

#[component]
pub struct ScrollView<C: Component> {
    pub xy: Xy<Px>,
    pub scroll_bar_width: Px,
    pub height: Px,
    pub content: C,
    pub scroll_y: Px,
    pub set_scroll_y: SetState<Px>,
}

#[component]
pub struct AutoScrollView<C: Component> {
    pub xy: Xy<Px>,
    pub scroll_bar_width: Px,
    pub height: Px,
    pub content: C,
}

impl<C: Component> Component for AutoScrollView<C> {
    fn render<'a>(self, ctx: &'a RenderCtx) -> RenderDone {
        let (scroll_y, set_scroll_y) = ctx.state(|| 0.px());

        ctx.component(ScrollView {
            xy: self.xy,
            scroll_bar_width: self.scroll_bar_width,
            height: self.height,
            content: self.content,
            scroll_y: *scroll_y,
            set_scroll_y,
        });

        ctx.done()
    }
}

impl<C: Component> Component for ScrollView<C> {
    fn render<'a>(self, ctx: &'a RenderCtx) -> RenderDone {
        let Self {
            xy,
            scroll_bar_width,
            height,
            content,
            scroll_y,
            set_scroll_y,
        } = self;
        let (bounding_box, set_bounding_box) = ctx.state(|| None);

        let Some(bounding_box) = *bounding_box else  {
            let content = ctx.ghost_render(content);
            let bounding_box = content.bounding_box();

            if bounding_box.is_some()  {
                set_bounding_box.set(bounding_box);
            };
            return ctx.done();
        };

        let scroll_y = namui::math::num::clamp(
            scroll_y,
            px(0.0),
            px(0.0).max(bounding_box.height() - height),
        );

        let inner = |ctx: &mut ComposeCtx| {
            ctx.clip(
                namui::Path::new().add_rect(Rect::Xywh {
                    x: bounding_box.x(),
                    y: bounding_box.y(),
                    width: bounding_box.width(),
                    height,
                }),
                namui::ClipOp::Intersect,
            )
            .translate((0.px(), -scroll_y.floor()))
            .add(content);
        };

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

        ctx.compose(|ctx| {
            ctx.translate(xy)
                .add(whole_rect.attach_event(move |event| match event {
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
                }))
                .compose(inner)
                .add(scroll_bar);
        })
        .done()
    }
}

#[component]
pub struct AutoScrollViewWithCtx<Func: FnOnce(&mut ComposeCtx)> {
    pub xy: Xy<Px>,
    pub scroll_bar_width: Px,
    pub height: Px,
    #[skip_debug]
    pub content: Func,
}

impl<Func: FnOnce(&mut ComposeCtx)> Component for AutoScrollViewWithCtx<Func> {
    fn render<'a>(self, ctx: &'a RenderCtx) -> RenderDone {
        let (scroll_y, set_scroll_y) = ctx.state(|| 0.px());

        ctx.component(ScrollViewWithCtx {
            xy: self.xy,
            scroll_bar_width: self.scroll_bar_width,
            height: self.height,
            content: self.content,
            scroll_y: *scroll_y,
            set_scroll_y,
        });

        ctx.done()
    }
}

#[component]
pub struct ScrollViewWithCtx<Func: FnOnce(&mut ComposeCtx)> {
    pub xy: Xy<Px>,
    pub scroll_bar_width: Px,
    pub height: Px,
    #[skip_debug]
    pub content: Func,
    pub scroll_y: Px,
    pub set_scroll_y: SetState<Px>,
}

impl<Func: FnOnce(&mut ComposeCtx)> Component for ScrollViewWithCtx<Func> {
    fn render<'a>(self, ctx: &'a RenderCtx) -> RenderDone {
        let Self {
            xy,
            scroll_bar_width,
            height,
            content,
            scroll_y,
            set_scroll_y,
        } = self;
        let (bounding_box, set_bounding_box) = ctx.state(|| None);

        let Some(bounding_box) = *bounding_box else  {
            let content = ctx.ghost_render_with_ctx(content);

            if let Some(bounding_box) = content.bounding_box() {
                set_bounding_box.set(Some(bounding_box));
            };
            return ctx.done();
        };

        let scroll_y = namui::math::num::clamp(
            scroll_y,
            px(0.0),
            px(0.0).max(bounding_box.height() - height),
        );

        let inner = |ctx: &mut ComposeCtx| {
            content(
                &mut ctx
                    .clip(
                        namui::Path::new().add_rect(Rect::Xywh {
                            x: bounding_box.x(),
                            y: bounding_box.y(),
                            width: bounding_box.width(),
                            height,
                        }),
                        namui::ClipOp::Intersect,
                    )
                    .translate((0.px(), -scroll_y.floor())),
            );
        };

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

        ctx.compose(|ctx| {
            ctx.translate(xy)
                .add(whole_rect.attach_event(|event| match event {
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
                }))
                .compose(inner)
                .add(scroll_bar);
        })
        .done()
    }
}
