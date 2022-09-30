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

pub enum Event {}

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
    pub fn new(sequence: Sequence, project_shared_data: ProjectSharedData) -> Self {
        Self {
            sequence,
            project_shared_data,
            state: State::ShowingCut { cut_index: 0 },
        }
    }
    pub fn render(&self, props: Props) -> RenderingTree {
        match &self.state {
            &State::ShowingCut { cut_index } => {
                let cut = self.sequence.cuts.get(cut_index).unwrap();
                render([
                    self.render_images(props.wh, cut, 1.0.one_zero()),
                    self.render_text(props.wh, cut),
                    simple_rect(props.wh, Color::TRANSPARENT, 0.px(), Color::TRANSPARENT)
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
            } => self.render_transition(props.wh, from_cut_index, transition_progress),
        }
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

    fn render_text(&self, wh: Wh<Px>, cut: &Cut) -> RenderingTree {
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
                                typography::body::center(wh, character_name, Color::WHITE)
                            }
                            None => RenderingTree::Empty,
                        }
                    }),
                    table::ratio(3, |wh| {
                        typography::body::center(wh, &cut.line, Color::WHITE)
                    }),
                ]),
            ),
        ])(wh)
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

    fn render_transition(
        &self,
        wh: Wh<Px>,
        from_cut_index: usize,
        transition_progress: OneZero,
    ) -> RenderingTree {
        let from_cut = self.sequence.cuts.get(from_cut_index).unwrap();
        let to_cut = self.sequence.cuts.get(from_cut_index + 1).unwrap();

        let from_opacity = 1.0.one_zero() - transition_progress;
        let to_opacity = transition_progress;

        render([
            self.render_images(wh, from_cut, from_opacity),
            self.render_images(wh, to_cut, to_opacity),
        ])
    }
}
