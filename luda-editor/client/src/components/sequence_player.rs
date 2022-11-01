use crate::storage::get_project_image_url;
use namui::prelude::*;
use namui_prebuilt::*;
use rpc::data::*;

pub struct SequencePlayer {
    sequence: Sequence,
    project_shared_data: ProjectSharedData,
    state: State,
}

pub struct Props {
    pub wh: Wh<Px>,
}

enum InternalEvent {
    NextCut,
}

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

impl SequencePlayer {
    pub fn new(
        sequence: Sequence,
        project_shared_data: ProjectSharedData,
        start_cut_index: usize,
    ) -> Self {
        Self {
            sequence,
            project_shared_data,
            state: State::ShowingCut {
                cut_index: start_cut_index,
            },
        }
    }
    pub fn render(&self, props: Props) -> RenderingTree {
        if self.sequence.cuts.is_empty() {
            return RenderingTree::Empty;
        }

        let inner_content_rect = get_inner_content_rect(props.wh);

        translate(
            inner_content_rect.x(),
            inner_content_rect.y(),
            match &self.state {
                &State::ShowingCut { cut_index } => {
                    let cut = self.sequence.cuts.get(cut_index).unwrap();
                    render([
                        self.render_images(inner_content_rect.wh(), cut, 1.0.one_zero()),
                        self.render_text_box(inner_content_rect.wh()),
                        self.render_text(inner_content_rect.wh(), cut, 1.0.one_zero()),
                        simple_rect(
                            inner_content_rect.wh(),
                            Color::TRANSPARENT,
                            0.px(),
                            Color::TRANSPARENT,
                        )
                        .attach_event(|builder| {
                            builder.on_mouse_down_in(|_| {
                                namui::event::send(InternalEvent::NextCut);
                            });
                        }),
                    ])
                }
                &State::Transitioning {
                    from_cut_index,
                    transition_progress,
                    start_time: _start_time,
                } => {
                    let from_cut = self.sequence.cuts.get(from_cut_index).unwrap();
                    render([
                        self.render_transitioning_image(
                            inner_content_rect.wh(),
                            from_cut_index,
                            transition_progress,
                        ),
                        self.render_text_box(inner_content_rect.wh()),
                        self.render_text(
                            inner_content_rect.wh(),
                            from_cut,
                            1.0.one_zero() - transition_progress,
                        ),
                    ])
                }
            },
        )
    }
    pub fn update(&mut self, event: &dyn std::any::Any) {
        match &mut self.state {
            State::ShowingCut { .. } => {}
            State::Transitioning {
                from_cut_index,
                transition_progress,
                start_time,
            } => {
                let transition_duration = 500.ms();

                let delta_time = Time::now() - *start_time;
                if delta_time > transition_duration {
                    self.state = State::ShowingCut {
                        cut_index: *from_cut_index + 1,
                    };
                } else {
                    *transition_progress = (delta_time / transition_duration).one_zero();
                }
            }
        }

        if let Some(event) = event.downcast_ref::<InternalEvent>() {
            match event {
                InternalEvent::NextCut => {
                    self.on_next_cut();
                }
            }
        }
    }

    fn render_text_box(&self, wh: Wh<Px>) -> RenderingTree {
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
        ])(wh)
    }

    fn render_text(&self, wh: Wh<Px>, cut: &Cut, opacity: OneZero) -> RenderingTree {
        table::vertical([
            table::ratio(3, |_wh| RenderingTree::Empty),
            table::ratio(
                1,
                table::vertical([
                    table::ratio(1, |wh| {
                        let character_name = cut
                            .character_id
                            .and_then(|character_id| {
                                self.project_shared_data
                                    .characters
                                    .iter()
                                    .find(|character| character.id() == character_id)
                            })
                            .map(|character| &character.name);

                        match character_name {
                            Some(character_name) => {
                                let margin = 32.px();
                                text(TextParam {
                                    text: character_name.clone(),
                                    x: margin,
                                    y: wh.height / 2,
                                    align: TextAlign::Left,
                                    baseline: TextBaseline::Middle,
                                    font_type: FontType {
                                        serif: false,
                                        size: 36.int_px(),
                                        language: Language::Ko,
                                        font_weight: FontWeight::BOLD,
                                    },
                                    style: TextStyle {
                                        border: Some(TextStyleBorder {
                                            width: 4.px(),
                                            color: Color::from_f01(0.0, 0.0, 0.0, opacity.as_f32()),
                                        }),
                                        drop_shadow: Some(TextStyleDropShadow {
                                            x: 1.px(),
                                            y: 2.px(),
                                            color: Some(Color::from_f01(
                                                0.0,
                                                0.0,
                                                0.0,
                                                opacity.as_f32(),
                                            )),
                                        }),
                                        color: Color::from_f01(1.0, 1.0, 1.0, opacity.as_f32()),
                                        background: None,
                                    },
                                    max_width: Some(wh.width - margin * 2),
                                })
                            }
                            None => RenderingTree::Empty,
                        }
                    }),
                    table::ratio(3, |wh| {
                        let margin = 32.px();
                        text(TextParam {
                            text: cut.line.clone(),
                            x: margin,
                            y: margin,
                            align: TextAlign::Left,
                            baseline: TextBaseline::Middle,
                            font_type: FontType {
                                serif: false,
                                size: 24.int_px(),
                                language: Language::Ko,
                                font_weight: FontWeight::BOLD,
                            },
                            style: TextStyle {
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
                                background: None,
                            },
                            max_width: Some(wh.width - margin * 2),
                        })
                    }),
                ]),
            ),
        ])(wh)
    }
    fn get_image_urls(&self, cut: &Cut) -> Vec<Url> {
        cut.screen_image_ids
            .into_iter()
            .filter_map(|image_id| image_id)
            .map(|image_id| get_project_image_url(self.project_shared_data.id(), image_id).unwrap())
            .collect::<Vec<_>>()
    }
    fn render_images(&self, wh: Wh<Px>, cut: &Cut, opacity: OneZero) -> RenderingTree {
        let image_urls = cut
            .screen_image_ids
            .into_iter()
            .filter_map(|image_id| image_id)
            .map(|image_id| get_project_image_url(self.project_shared_data.id(), image_id).unwrap())
            .collect::<Vec<_>>();

        let image_count = image_urls.len();

        let image_width = wh.width / image_count;

        let paint_builder = namui::PaintBuilder::new().set_color_filter(
            Color::from_f01(1.0, 1.0, 1.0, opacity.as_f32()),
            BlendMode::DstIn,
        );

        render(image_urls.into_iter().enumerate().map(|(i, image_url)| {
            namui::image(ImageParam {
                rect: Rect::from_xy_wh(
                    Xy::new(image_width * i, 0.px()),
                    Wh::new(image_width, wh.height),
                ),
                source: ImageSource::Url(image_url),
                style: ImageStyle {
                    fit: ImageFit::Contain,
                    paint_builder: Some(paint_builder.clone()),
                },
            })
        }))
    }

    fn on_next_cut(&mut self) {
        if self.sequence.cuts.len() == 0 {
            return;
        }

        match self.state {
            State::ShowingCut { cut_index } => {
                let next_cut_index = cut_index + 1;
                if next_cut_index >= self.sequence.cuts.len() {
                    return;
                }

                self.state = State::Transitioning {
                    from_cut_index: cut_index,
                    transition_progress: 0.0.one_zero(),
                    start_time: Time::now(),
                };
            }
            State::Transitioning { .. } => {
                return;
            }
        }
    }

    fn render_transitioning_image(
        &self,
        wh: Wh<Px>,
        from_cut_index: usize,
        transition_progress: OneZero,
    ) -> RenderingTree {
        let from_cut = self.sequence.cuts.get(from_cut_index).unwrap();
        let to_cut = self.sequence.cuts.get(from_cut_index + 1).unwrap();

        let from_cut_image_urls = self.get_image_urls(from_cut);
        let to_cut_image_urls = self.get_image_urls(to_cut);

        if from_cut_image_urls == to_cut_image_urls {
            self.render_images(wh, from_cut, 1.0.one_zero())
        } else {
            let from_opacity = 1.0.one_zero() - transition_progress;
            let to_opacity = transition_progress;

            render([
                self.render_images(wh, from_cut, from_opacity),
                self.render_images(wh, to_cut, to_opacity),
            ])
        }
    }
}

fn get_inner_content_rect(wh: Wh<Px>) -> Rect<Px> {
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
