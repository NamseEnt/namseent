use super::cg_render;
use crate::storage::{get_project_cg_thumbnail_image_url, get_project_image_url};
use namui::*;
use namui_prebuilt::*;
use rpc::data::*;

pub struct SequencePlayer<'a> {
    pub wh: Wh<Px>,
    pub sequence: &'a Sequence,
    pub project_shared_data: &'a ProjectSharedData,
    /// NOTE: This `init_cut_index` is not setable after first render.
    pub init_cut_index: usize,
    pub cg_files: &'a Vec<CgFile>,
}

impl Component for SequencePlayer<'_> {
    fn render(self, ctx: &RenderCtx) {
        let (cut_index, _) = ctx.state(|| self.init_cut_index);

        #[derive(Debug)]
        enum State {
            ShowingCut {
                cut_index: usize,
            },
            Transitioning {
                from_cut_index: usize,
                transition_progress: OneZero,
                start_time: Time,
            },
        }
        let (state, set_state) = ctx.state(|| State::ShowingCut {
            cut_index: *cut_index,
        });

        if self.sequence.cuts.is_empty() {
            return;
        }

        let now = ctx.track_eq(&namui::now());

        ctx.effect("Play transition", move || match *state {
            State::ShowingCut { .. } => {}
            State::Transitioning {
                from_cut_index,
                transition_progress: _,
                start_time,
            } => {
                let transition_duration = 500.ms();

                let delta_time = *now - start_time;

                set_state.set(if delta_time > transition_duration {
                    State::ShowingCut {
                        cut_index: from_cut_index + 1,
                    }
                } else {
                    State::Transitioning {
                        from_cut_index,
                        transition_progress: (delta_time / transition_duration).one_zero(),
                        start_time,
                    }
                });
            }
        });

        let go_to_next_cut = |do_transition: bool| {
            if self.sequence.cuts.is_empty() {
                return;
            }

            match *state {
                State::ShowingCut { cut_index } => {
                    let next_cut_index = cut_index + 1;
                    if next_cut_index >= self.sequence.cuts.len() {
                        return;
                    }

                    set_state.set(if do_transition {
                        State::Transitioning {
                            from_cut_index: cut_index,
                            transition_progress: 0.0.one_zero(),
                            start_time: namui::now(),
                        }
                    } else {
                        State::ShowingCut {
                            cut_index: next_cut_index,
                        }
                    });
                }
                State::Transitioning { from_cut_index, .. } => {
                    if !do_transition {
                        set_state.set(State::ShowingCut {
                            cut_index: from_cut_index + 1,
                        });
                    }
                }
            }
        };

        let go_to_prev_cut = || {
            let prev_cut_index = match *state {
                State::ShowingCut { cut_index } => {
                    if cut_index == 0 {
                        return;
                    }
                    cut_index - 1
                }
                State::Transitioning { from_cut_index, .. } => from_cut_index,
            };
            set_state.set(State::ShowingCut {
                cut_index: prev_cut_index,
            });
        };

        let inner_content_rect = get_inner_content_rect(self.wh);

        ctx.compose(|ctx| {
            ctx.translate(inner_content_rect.xy())
                .compose(|ctx| match *state.as_ref() {
                    State::ShowingCut { cut_index } => {
                        let cut = self.sequence.cuts.get(cut_index).unwrap();
                        render_graphics(
                            ctx,
                            self.project_shared_data.id(),
                            inner_content_rect.wh(),
                            cut,
                            1.0.one_zero(),
                            self.cg_files,
                        );
                        ctx.add((
                            render_text(
                                self.project_shared_data,
                                inner_content_rect.wh(),
                                cut,
                                1.0.one_zero(),
                            ),
                            render_text_box(inner_content_rect.wh()),
                            simple_rect(
                                inner_content_rect.wh(),
                                Color::TRANSPARENT,
                                0.px(),
                                Color::TRANSPARENT,
                            )
                            .attach_event(|event| {
                                if let Event::MouseDown { event } = event {
                                    if event.is_local_xy_in() {
                                        go_to_next_cut(true)
                                    }
                                }
                            }),
                        ));
                    }
                    State::Transitioning {
                        from_cut_index,
                        transition_progress,
                        start_time: _start_time,
                    } => {
                        let from_cut = self.sequence.cuts.get(from_cut_index).unwrap();
                        self.render_transitioning_image(
                            ctx,
                            inner_content_rect.wh(),
                            from_cut_index,
                            transition_progress,
                            self.cg_files,
                        );
                        ctx.add((
                            render_text_box(inner_content_rect.wh()),
                            render_text(
                                self.project_shared_data,
                                inner_content_rect.wh(),
                                from_cut,
                                1.0.one_zero() - transition_progress,
                            ),
                        ));
                    }
                })
                .attach_event(|event| {
                    if let Event::KeyDown { event } = event {
                        if event.code == Code::ArrowLeft {
                            go_to_prev_cut()
                        } else if event.code == Code::ArrowRight {
                            go_to_next_cut(false)
                        }
                    }
                });
        });
    }
}

impl SequencePlayer<'_> {
    fn render_transitioning_image(
        &self,
        ctx: &mut ComposeCtx,
        wh: Wh<Px>,
        from_cut_index: usize,
        transition_progress: OneZero,
        cg_files: &Vec<CgFile>,
    ) {
        let from_cut = self.sequence.cuts.get(from_cut_index).unwrap();
        let to_cut = self.sequence.cuts.get(from_cut_index + 1).unwrap();

        let project_id = self.project_shared_data.id();

        if from_cut.screen_graphics == to_cut.screen_graphics {
            render_graphics(ctx, project_id, wh, from_cut, 1.0.one_zero(), cg_files)
        } else {
            let from_opacity = 1.0.one_zero() - transition_progress;
            let to_opacity = transition_progress;

            render_graphics(ctx, project_id, wh, from_cut, from_opacity, cg_files);
            render_graphics(ctx, project_id, wh, to_cut, to_opacity, cg_files);
        }
    }
}

pub fn get_inner_content_rect(wh: Wh<Px>) -> Rect<Px> {
    let width_per_height = 4.0 / 3.0;

    let ratio = wh.width / wh.height;

    if ratio == width_per_height {
        Rect::from_xy_wh(Xy::zero(), wh)
    } else if ratio > width_per_height {
        let result_wh = Wh::new(wh.height * width_per_height, wh.height);
        let result_xy = Xy::new((wh.width - result_wh.width) / 2.0, 0.px());
        Rect::from_xy_wh(result_xy, result_wh)
    } else {
        let result_wh = Wh::new(wh.width, wh.width / width_per_height);
        let result_xy = Xy::new(0.px(), (wh.height - result_wh.height) / 2.0);
        Rect::from_xy_wh(result_xy, result_wh)
    }
}

pub fn render_text_box(screen_wh: Wh<Px>) -> impl Component {
    table::vertical([
        table::ratio(3, |_wh| RenderingTree::Empty),
        table::ratio(1, |wh| {
            rect(RectParam {
                rect: Rect::from_xy_wh(Xy::zero(), wh),
                style: RectStyle {
                    stroke: Some(RectStroke {
                        color: Color::BLACK,
                        width: 1.px(),
                        border_position: BorderPosition::Inside,
                    }),
                    fill: Some(RectFill {
                        color: Color::from_f01(1.0, 1.0, 1.0, 0.3),
                    }),
                    round: Some(RectRound { radius: 8.px() }),
                },
            })
        }),
    ])(screen_wh)
    .attach_event(|event| {
        if let Event::MouseDown { event } = event {
            if event.is_local_xy_in() {
                event.stop_propagation();
            }
        }
    })
}

pub fn character_name_font() -> Font {
    Font {
        size: int_px(36),
        name: "NotoSansKR-Bold".to_string(),
    }
}
pub fn cut_text_font() -> Font {
    Font {
        size: int_px(24),
        name: "NotoSansKR-Bold".to_string(),
    }
}

pub fn character_name_text_style(opacity: OneZero) -> TextStyle {
    TextStyle {
        border: Some(TextStyleBorder {
            width: 4.px(),
            color: Color::from_f01(0.0, 0.0, 0.0, opacity.as_f32()),
        }),
        drop_shadow: Some(TextStyleDropShadow {
            x: 1.px(),
            y: 2.px(),
            color: Some(Color::from_f01(0.0, 0.0, 0.0, opacity.as_f32())),
        }),
        color: Color::from_f01(1.0, 1.0, 1.0, opacity.as_f32()),
        ..Default::default()
    }
}

pub fn cut_text_style(opacity: OneZero) -> TextStyle {
    TextStyle {
        border: Some(TextStyleBorder {
            width: 4.px(),
            color: Color::from_f01(0.0, 0.0, 0.0, opacity.as_f32()),
        }),
        drop_shadow: Some(TextStyleDropShadow {
            x: 1.px(),
            y: 2.px(),
            color: Some(Color::from_f01(0.0, 0.0, 0.0, opacity.as_f32())),
        }),
        color: Color::from_f01(1.0, 1.0, 1.0, opacity.as_f32()),
        line_height_percent: 150.percent(),
        ..Default::default()
    }
}

pub fn render_text(
    _project_shared_data: &ProjectSharedData,
    wh: Wh<Px>,
    cut: &Cut,
    opacity: OneZero,
) -> RenderingTree {
    render_over_text(
        wh,
        |wh| {
            text(TextParam {
                text: cut.character_name.clone(),
                x: 0.px(),
                y: wh.height / 2,
                align: TextAlign::Left,
                baseline: TextBaseline::Middle,
                font: character_name_font(),
                style: character_name_text_style(opacity),
                max_width: Some(wh.width),
            })
        },
        |wh| {
            text(TextParam {
                text: cut.line.clone(),
                x: 0.px(),
                y: 0.px(),
                align: TextAlign::Left,
                baseline: TextBaseline::Top,
                font: cut_text_font(),
                style: cut_text_style(opacity),
                max_width: Some(wh.width),
            })
        },
    )
}

pub fn render_over_text(
    wh: Wh<Px>,
    character_name_side: impl FnOnce(Wh<Px>) -> RenderingTree,
    cut_text_side: impl FnOnce(Wh<Px>) -> RenderingTree,
) -> RenderingTree {
    table::vertical([
        table::ratio(3, |_wh| RenderingTree::Empty),
        table::ratio(
            1,
            table::vertical([
                table::ratio(1, table::horizontal_padding(32.px(), character_name_side)),
                table::ratio(3, table::padding_no_clip(32.px(), cut_text_side)),
            ]),
        ),
    ])(wh)
}

pub fn render_over_text_hooks<'a>(
    ctx: &mut ComposeCtx,
    wh: Wh<Px>,
    character_name_side: impl 'a + Fn(Wh<Px>, &mut ComposeCtx),
    cut_text_side: impl 'a + Fn(Wh<Px>, &mut ComposeCtx),
) {
    table::vertical([
        table::ratio(3, |_wh, _ctx| {}),
        table::ratio(
            1,
            table::vertical([
                table::ratio(1, table::horizontal_padding(32.px(), character_name_side)),
                table::ratio(3, table::padding_no_clip(32.px(), cut_text_side)),
            ]),
        ),
    ])(wh, ctx);
}

pub fn calculate_graphic_wh_on_screen(
    original_graphic_size: Wh<Px>,
    container_wh: Wh<Px>,
    circumscribed: Circumscribed<Percent>,
) -> Wh<Px> {
    let screen_radius = container_wh.length() / 2;
    let image_radius_px = original_graphic_size.length() / 2;
    let radius_px = screen_radius * circumscribed.radius;

    original_graphic_size * (radius_px / image_radius_px)
}

pub fn calculate_graphic_rect_on_screen(
    original_graphic_size: Wh<Px>,
    container_wh: Wh<Px>,
    circumscribed: Circumscribed<Percent>,
) -> Rect<Px> {
    let wh = calculate_graphic_wh_on_screen(original_graphic_size, container_wh, circumscribed);
    let center_xy = container_wh.to_xy() * circumscribed.center_xy;

    let xy = center_xy - wh.to_xy() / 2.0;

    Rect::from_xy_wh(xy, wh)
}

fn render_graphics(
    ctx: &mut ComposeCtx,
    project_id: Uuid,
    wh: Wh<Px>,
    cut: &Cut,
    opacity: OneZero,
    cg_files: &Vec<CgFile>,
) {
    let paint = namui::Paint::new().set_color_filter(ColorFilter {
        color: Color::from_f01(1.0, 1.0, 1.0, opacity.as_f32()),
        blend_mode: BlendMode::DstIn,
    });

    ctx.compose(|ctx| {
        for (_, screen_graphic) in &cut.screen_graphics {
            ctx.add(SequencePlayerGraphic {
                project_id,
                wh,
                graphic: screen_graphic,
                paint: Some(paint.clone()),
                cg_files,
            });
        }
    });
}

pub struct SequencePlayerGraphic<'a> {
    project_id: Uuid,
    wh: Wh<Px>,
    graphic: &'a ScreenGraphic,
    paint: Option<Paint>,
    cg_files: &'a Vec<CgFile>,
}

impl Component for SequencePlayerGraphic<'_> {
    fn render(self, ctx: &RenderCtx) {
        let graphic = ctx.track_eq(self.graphic);
        let url = ctx.memo(|| match graphic.as_ref() {
            ScreenGraphic::Image(screen_image) => {
                get_project_image_url(self.project_id, screen_image.id).unwrap()
            }
            ScreenGraphic::Cg(screen_cg) => {
                get_project_cg_thumbnail_image_url(self.project_id, screen_cg.id).unwrap()
            }
        });
        let image = ctx.image(&url);
        let Some(image) = image.as_ref() else {
            return;
        };

        let Ok(image) = image else {
            namui::log!("Failed to load image: {:?}", url);
            return;
        };

        ctx.compose(|ctx| {
            let center_xy = image.wh.to_rect().center();
            let mut ctx = ctx
                .translate(center_xy)
                .rotate(graphic.rotation())
                .translate(center_xy * -1.0);
            match graphic.as_ref() {
                ScreenGraphic::Image(screen_image) => {
                    let rect = calculate_graphic_rect_on_screen(
                        image.wh,
                        self.wh,
                        screen_image.circumscribed,
                    );

                    ctx.add(namui::image(ImageParam {
                        rect,
                        source: ImageSource::Url {
                            url: url.clone_inner(),
                        },
                        style: ImageStyle {
                            fit: ImageFit::Fill,
                            paint: self.paint,
                        },
                    }));
                }
                ScreenGraphic::Cg(screen_cg) => {
                    let outer_rect = calculate_graphic_rect_on_screen(
                        image.wh,
                        self.wh,
                        screen_cg.circumscribed,
                    );

                    let cg_file = self
                        .cg_files
                        .iter()
                        .find(|cg_file| cg_file.name == screen_cg.name);

                    match cg_file {
                        Some(cg_file) => ctx.add(cg_render::CgRender {
                            project_id: self.project_id,
                            rect: outer_rect,
                            screen_cg,
                            cg_file,
                        }),
                        None => ctx.add(RenderingTree::Empty),
                    };
                }
            }
        })
        .done()
    }
}
