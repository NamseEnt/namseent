use crate::app::types::*;
use namui::{Language, RenderingTree, Wh, Xy, XywhRect};
mod subtitle_track;
use super::PlaybackStatus;
use subtitle_track::*;

pub(super) struct PlayerScreenProps<'a> {
    pub playback_status: &'a PlaybackStatus,
    pub xywh: &'a XywhRect<f32>,
    pub sequence: &'a Sequence,
    pub camera_angle_image_loader: &'a dyn CameraAngleImageLoader,
    pub language: Language,
    pub subtitle_play_duration_measurer: &'a dyn SubtitlePlayDurationMeasure,
}

pub(super) fn render_player_screen(props: &PlayerScreenProps) -> RenderingTree {
    match props.playback_status {
        PlaybackStatus::Loading => {
            let center = props.xywh.center();
            let font_size = (props.xywh.height * 0.2).floor() as i16;
            return namui::text(namui::TextParam {
                x: center.x,
                y: center.y,
                align: namui::TextAlign::Center,
                baseline: namui::TextBaseline::Middle,
                font_type: namui::FontType {
                    font_weight: namui::FontWeight::BOLD,
                    size: font_size,
                    language: namui::Language::Ko,
                    serif: false,
                },
                style: namui::TextStyle {
                    color: namui::Color::BLACK,
                    ..Default::default()
                },
                text: "Loading...".to_string(),
            });
        }
        PlaybackStatus::Paused(playback_time) | PlaybackStatus::Playing(playback_time) => {
            const SCREEN_WH_RATIO: f32 = 16.0 / 9.0;

            let screen_height = props.xywh.width / SCREEN_WH_RATIO;
            let screen_top_margin = props.xywh.height / 2.0 - screen_height / 2.0;

            let screen_wh = Wh {
                width: props.xywh.width,
                height: screen_height,
            };

            let translated_xy = props.xywh.xy()
                + Xy {
                    x: 0.0,
                    y: screen_top_margin,
                };
            namui::translate(
                translated_xy.x,
                translated_xy.y,
                namui::render![
                    namui::rect(namui::RectParam {
                        x: 0.0,
                        y: 0.0,
                        width: screen_wh.width,
                        height: screen_wh.height,
                        style: namui::RectStyle {
                            stroke: Some(namui::RectStroke {
                                border_position: namui::BorderPosition::Outside,
                                color: namui::Color::BLACK,
                                width: 1.0,
                            }),
                            ..Default::default()
                        },
                    }),
                    render_sequence_in_player_screen(
                        props.sequence,
                        &screen_wh,
                        playback_time,
                        props.camera_angle_image_loader,
                        props.language,
                        props.subtitle_play_duration_measurer,
                    ),
                ],
            )
        }
    }
}

fn render_sequence_in_player_screen(
    sequence: &Sequence,
    screen_wh: &Wh<f32>,
    playback_time: &Time,
    camera_angle_image_loader: &dyn CameraAngleImageLoader,
    language: Language,
    subtitle_play_duration_measurer: &dyn SubtitlePlayDurationMeasure,
) -> RenderingTree {
    RenderingTree::Children(
        sequence
            .tracks
            .iter()
            .map(|track| match track.as_ref() {
                Track::Camera(camera_track) => camera_track
                    .get_clip_at_time(playback_time)
                    .map(|clip| {
                        render_camera_clip_in_player_screen(
                            clip,
                            screen_wh,
                            camera_angle_image_loader,
                        )
                    })
                    .unwrap_or_else(|| RenderingTree::Empty),
                Track::Subtitle(subtitle_track) => render_subtitle_track_in_player_screen(
                    &subtitle_track,
                    screen_wh,
                    playback_time,
                    language,
                    subtitle_play_duration_measurer,
                ),
            })
            .collect(),
    )
}

fn render_camera_clip_in_player_screen(
    clip: &CameraClip,
    screen_wh: &Wh<f32>,
    camera_angle_image_loader: &dyn CameraAngleImageLoader,
) -> RenderingTree {
    clip.camera_angle
        .render(screen_wh, camera_angle_image_loader)
}
