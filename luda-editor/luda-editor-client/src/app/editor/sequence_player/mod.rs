mod buttons;
mod content_loader;
mod player_screen;

use self::content_loader::ContentLoader;
use crate::app::types::*;
use buttons::*;
use namui::prelude::*;
use player_screen::*;
use std::{collections::HashMap, sync::Arc};

pub struct SequencePlayer {
    id: String,
    is_paused: bool,
    sequence: Arc<Sequence>,
    content_loader: ContentLoader,
    started_at: Option<Time>,
    last_paused_playback_time: Time,
    camera_angle_image_loader: Arc<dyn CameraAngleImageLoader>,
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
    pub rect: &'a namui::Rect<Px>,
    pub language: Language,
    pub subtitle_play_duration_measurer: &'a dyn SubtitlePlayDurationMeasure,
    pub with_buttons: bool,
    pub subtitle_character_color_map: &'a HashMap<String, Color>,
}

#[cfg(test)]
use mockall::{automock, predicate::*};
#[cfg_attr(test, automock)]
pub trait SequencePlay {
    fn play(&mut self);
    fn pause(&mut self);
    fn toggle_playback(&mut self);
    fn seek(&mut self, time: Time);
    fn update_sequence(&mut self, sequence: Arc<Sequence>);
    fn get_playback_time(&self) -> Time;
    fn update(&mut self, event: &dyn std::any::Any);
    fn render<'a>(&self, props: &SequencePlayerProps<'a>) -> RenderingTree;
}

impl SequencePlayer {
    pub fn new(
        sequence: Arc<Sequence>,
        camera_angle_image_loader: Arc<dyn CameraAngleImageLoader>,
    ) -> Self {
        let id = namui::nanoid();
        let this = Self {
            id: id.clone(),
            is_paused: true,
            sequence: sequence.clone(),
            content_loader: ContentLoader::new(sequence.clone(), camera_angle_image_loader.clone()),
            started_at: None,
            camera_angle_image_loader,
            last_paused_playback_time: Time::Ms(0.0),
        };
        this.call_loading_timeout();
        this
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
    fn call_loading_timeout(&self) {
        let id = self.id.clone();
        namui::set_timeout(
            move || namui::event::send(SequencePlayerEvent::CheckLoading(id)),
            Time::Sec(1.0),
        );
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
}
impl SequencePlay for SequencePlayer {
    fn play(&mut self) {
        self.is_paused = false;
        self.start_play();
    }
    fn pause(&mut self) {
        self.last_paused_playback_time = self.get_playback_time();
        self.is_paused = true;
        self.started_at = None;
    }
    fn toggle_playback(&mut self) {
        if self.is_paused {
            self.play();
        } else {
            self.pause();
        }
    }
    fn seek(&mut self, time: Time) {
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
    fn update_sequence(&mut self, sequence: Arc<Sequence>) {
        self.sequence = sequence.clone();
        self.content_loader = ContentLoader::new(sequence, self.camera_angle_image_loader.clone());
    }
    fn update(&mut self, event: &dyn std::any::Any) {
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
    fn render<'a>(&self, props: &'a SequencePlayerProps) -> RenderingTree {
        let wh = props.rect.wh();
        // NOTE : will be translated by props.rect.x() y.

        let player_screen_rect = namui::Rect::Xywh {
            x: px(0.0),
            y: px(0.0),
            width: wh.width,
            height: match props.with_buttons {
                true => wh.height * (5.0 / 6.0),
                false => wh.height,
            },
        };

        let buttons_rect = namui::Rect::Xywh {
            x: px(0.0),
            y: wh.height * (5.0 / 6.0),
            width: wh.width,
            height: wh.height * (1.0 / 6.0),
        };

        let border = namui::rect(namui::RectParam {
            rect: Rect::Xywh {
                x: px(0.0),
                y: px(0.0),
                width: wh.width,
                height: wh.height,
            },
            style: namui::RectStyle {
                stroke: Some(namui::RectStroke {
                    color: Color::BLACK,
                    border_position: namui::BorderPosition::Inside,
                    width: px(1.0),
                }),
                ..Default::default()
            },
        });

        let playback_status = self.get_playback_status();

        namui::translate(
            props.rect.x(),
            props.rect.y(),
            namui::render([
                border,
                render_player_screen(&PlayerScreenProps {
                    rect: player_screen_rect,
                    sequence: &self.sequence,
                    playback_status: &playback_status,
                    camera_angle_image_loader: self.camera_angle_image_loader.clone(),
                    language: props.language,
                    subtitle_play_duration_measurer: props.subtitle_play_duration_measurer,
                    subtitle_character_color_map: props.subtitle_character_color_map,
                }),
                match props.with_buttons {
                    true => render_buttons(&ButtonsProps {
                        rect: buttons_rect,
                        playback_status: &playback_status,
                    }),
                    false => RenderingTree::Empty,
                },
            ]),
        )
    }
    fn get_playback_time(&self) -> Time {
        self.started_at
            .map_or(self.last_paused_playback_time, |start_at| {
                Time::now() - start_at + self.last_paused_playback_time
            })
    }
}
