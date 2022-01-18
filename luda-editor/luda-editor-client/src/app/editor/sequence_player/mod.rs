use self::content_loader::ContentLoader;
use crate::app::types::*;
use namui::{Color, Language, RenderingTree};
use std::{rc::Rc, time::Duration};
mod content_loader;
mod player_screen;
use player_screen::*;

pub struct SequencePlayer {
    id: String,
    is_playing: bool,
    sequence: Rc<Sequence>,
    content_loader: ContentLoader,
    playback_time: Time,
    camera_angle_image_loader: Box<dyn CameraAngleImageLoader>,
}

enum SequencePlayerEvent {
    CheckLoading(String),
}

pub struct SequencePlayerProps<'a> {
    pub xywh: &'a namui::XywhRect<f32>,
    pub language: Language,
    pub subtitle_play_duration_measurer: &'a SubtitlePlayDurationMeasurer,
}

impl SequencePlayer {
    pub fn new(
        sequence: Rc<Sequence>,
        camera_angle_image_loader: Box<dyn CameraAngleImageLoader>,
    ) -> Self {
        let id = namui::nanoid();
        let this = Self {
            id: id.clone(),
            is_playing: false,
            sequence: sequence.clone(),
            content_loader: ContentLoader::new(
                sequence.clone(),
                camera_angle_image_loader.as_ref(),
            ),
            playback_time: Time::zero(),
            camera_angle_image_loader,
        };
        this.call_loading_timeout();
        this
    }
    pub fn play(&mut self) {
        self.is_playing = true;
    }
    pub fn stop(&mut self) {
        self.is_playing = false;
    }
    pub fn seek(&mut self, time: Time) {
        todo!()
    }
    pub fn update_sequence(
        &mut self,
        sequence: Rc<Sequence>,
        camera_angle_image_loader: &dyn CameraAngleImageLoader,
    ) {
        self.sequence = sequence.clone();
        self.content_loader = ContentLoader::new(sequence, camera_angle_image_loader);
    }
    pub fn update(&mut self, event: &dyn std::any::Any) {
        if let Some(event) = event.downcast_ref::<SequencePlayerEvent>() {
            match event {
                SequencePlayerEvent::CheckLoading(id) => {
                    if id.ne(&self.id) {
                        return;
                    }
                    match self.content_loader.is_loaded() {
                        false => {
                            namui::log!("SequencePlayer::update: loading not yet");
                            self.call_loading_timeout()
                        }
                        true => {
                            namui::log!("SequencePlayer::update: loaded");
                        }
                    }
                }
            }
        }
    }
    pub fn render(&self, props: &SequencePlayerProps) -> RenderingTree {
        let wh = props.xywh.wh();
        // NOTE : will be translated by xy.

        let title_header_center_y = wh.height * (0.5 / 6.0);
        let title_header_center_x = wh.width * 0.5;
        let title_font_size = title_header_center_y.floor() as i16;

        let player_screen_xywh = namui::XywhRect {
            x: 0.0,
            y: wh.height * (1.0 / 6.0),
            width: wh.width,
            height: wh.height * (4.0 / 6.0),
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
        let title_header = namui::text(namui::TextParam {
            x: title_header_center_x,
            y: title_header_center_y,
            align: namui::TextAlign::Center,
            baseline: namui::TextBaseline::Middle,
            font_type: namui::FontType {
                font_weight: namui::FontWeight::BOLD,
                size: title_font_size,
                language: namui::Language::Ko,
                serif: false,
            },
            style: namui::TextStyle {
                color: namui::Color::BLACK,
                ..Default::default()
            },
            text: "[Sequence Player]".to_string(),
        });

        namui::translate(
            props.xywh.x,
            props.xywh.y,
            namui::render![
                border,
                title_header,
                render_player_screen(&PlayerScreenProps {
                    xywh: &player_screen_xywh,
                    is_loading: !self.content_loader.is_loaded(),
                    sequence: &self.sequence,
                    playback_time: &self.playback_time,
                    camera_angle_image_loader: self.camera_angle_image_loader.as_ref(),
                    language: props.language,
                    subtitle_play_duration_measurer: &props.subtitle_play_duration_measurer,
                }),
                // TODO : Buttons
            ],
        )
    }
    fn call_loading_timeout(&self) {
        let id = self.id.clone();
        namui::set_timeout(
            move || namui::event::send(SequencePlayerEvent::CheckLoading(id)),
            Duration::from_secs(1),
        );
    }
}
