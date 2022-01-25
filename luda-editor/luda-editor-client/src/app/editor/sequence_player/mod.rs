use self::content_loader::ContentLoader;
use crate::app::types::*;
use namui::{Color, Language, RenderingTree};
use std::{sync::Arc, time::Duration};
mod content_loader;
mod player_screen;
use player_screen::*;
mod buttons;
use buttons::*;

pub struct SequencePlayer {
    id: String,
    is_paused: bool,
    sequence: Arc<Sequence>,
    content_loader: ContentLoader,
    started_at: Option<Time>,
    last_paused_playback_time: Time,
    camera_angle_image_loader: Box<dyn CameraAngleImageLoader>,
}

enum SequencePlayerEvent {
    CheckLoading(String),
    AnimationFrame(String),
}

enum PlaybackStatus {
    Loading,
    Paused(Time),
    Playing(Time),
}

pub struct SequencePlayerProps<'a> {
    pub xywh: &'a namui::XywhRect<f32>,
    pub language: Language,
    pub subtitle_play_duration_measurer: &'a SubtitlePlayDurationMeasurer,
}

impl SequencePlayer {
    pub fn new(
        sequence: Arc<Sequence>,
        camera_angle_image_loader: Box<dyn CameraAngleImageLoader>,
    ) -> Self {
        let id = namui::nanoid();
        let this = Self {
            id: id.clone(),
            is_paused: true,
            sequence: sequence.clone(),
            content_loader: ContentLoader::new(
                sequence.clone(),
                camera_angle_image_loader.as_ref(),
            ),
            started_at: None,
            camera_angle_image_loader,
            last_paused_playback_time: Time::zero(),
        };
        this.call_loading_timeout();
        this
    }
    pub fn play(&mut self) {
        self.is_paused = false;
        self.start_play();
    }
    pub fn pause(&mut self) {
        self.last_paused_playback_time = self.get_playback_time();
        self.is_paused = true;
        self.started_at = None;
    }
    pub fn seek(&mut self, time: Time) {
        match self.get_playback_status() {
            PlaybackStatus::Loading | PlaybackStatus::Paused(_) => {
                self.last_paused_playback_time = time;
            }
            PlaybackStatus::Playing(_) => {
                self.last_paused_playback_time = time;
                self.started_at = Some(Time::now());
            }
        }
    }
    pub fn update_sequence(&mut self, sequence: Arc<Sequence>) {
        self.sequence = sequence.clone();
        self.content_loader = ContentLoader::new(sequence, self.camera_angle_image_loader.as_ref());
    }
    pub fn update(&mut self, event: &dyn std::any::Any) {
        if let Some(event) = event.downcast_ref::<SequencePlayerEvent>() {
            match event {
                SequencePlayerEvent::CheckLoading(id) => {
                    if id.ne(&self.id) {
                        return;
                    }
                    match self.content_loader.is_loaded() {
                        false => self.call_loading_timeout(),
                        true => {
                            if !self.is_paused {
                                self.start_play()
                            }
                        }
                    }
                }
                SequencePlayerEvent::AnimationFrame(id) => {
                    if id.ne(&self.id) || self.is_paused {
                        return;
                    }
                    let id = id.clone();
                    namui::request_animation_frame(move || {
                        namui::event::send(SequencePlayerEvent::AnimationFrame(id))
                    });
                }
            }
        } else if let Some(event) = event.downcast_ref::<ButtonsEvent>() {
            match event {
                ButtonsEvent::PlayButtonClicked => self.play(),
                ButtonsEvent::PauseButtonClicked => self.pause(),
            }
        }
    }
    fn start_play(&mut self) {
        if self.is_paused || self.started_at.is_some() || !self.content_loader.is_loaded() {
            return;
        }

        self.started_at = Some(Time::now());
        let id = self.id.clone();
        namui::request_animation_frame(|| {
            namui::event::send(SequencePlayerEvent::AnimationFrame(id))
        });
    }
    pub fn render(&self, props: &SequencePlayerProps) -> RenderingTree {
        let wh = props.xywh.wh();
        // NOTE : will be translated by props.xywh.xy.

        let player_screen_xywh = namui::XywhRect {
            x: 0.0,
            y: 0.0,
            width: wh.width,
            height: wh.height * (5.0 / 6.0),
        };

        let buttons_xywh = namui::XywhRect {
            x: 0.0,
            y: wh.height * (5.0 / 6.0),
            width: wh.width,
            height: wh.height * (1.0 / 6.0),
        };

        let border = namui::rect(namui::RectParam {
            x: 0.0,
            y: 0.0,
            width: wh.width,
            height: wh.height,
            style: namui::RectStyle {
                stroke: Some(namui::RectStroke {
                    color: Color::BLACK,
                    border_position: namui::BorderPosition::Inside,
                    width: 1.0,
                }),
                ..Default::default()
            },
        });

        let playback_status = self.get_playback_status();

        namui::translate(
            props.xywh.x,
            props.xywh.y,
            namui::render![
                border,
                render_player_screen(&PlayerScreenProps {
                    xywh: &player_screen_xywh,
                    sequence: &self.sequence,
                    playback_status: &playback_status,
                    camera_angle_image_loader: self.camera_angle_image_loader.as_ref(),
                    language: props.language,
                    subtitle_play_duration_measurer: &props.subtitle_play_duration_measurer,
                }),
                render_buttons(&ButtonsProps {
                    xywh: &buttons_xywh,
                    playback_status: &playback_status,
                }),
            ],
        )
    }
    pub fn get_playback_time(&self) -> Time {
        self.started_at
            .map_or(self.last_paused_playback_time, |start_at| {
                Time::now() - start_at + self.last_paused_playback_time
            })
    }
    fn get_playback_status(&self) -> PlaybackStatus {
        if !self.content_loader.is_loaded() {
            return PlaybackStatus::Loading;
        }
        if self.is_paused {
            return PlaybackStatus::Paused(self.last_paused_playback_time);
        }
        PlaybackStatus::Playing(self.get_playback_time())
    }
    fn call_loading_timeout(&self) {
        let id = self.id.clone();
        namui::set_timeout(
            move || namui::event::send(SequencePlayerEvent::CheckLoading(id)),
            Duration::from_secs(1),
        );
    }
}
