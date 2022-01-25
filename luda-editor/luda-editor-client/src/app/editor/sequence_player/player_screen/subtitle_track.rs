use crate::app::types::*;
use namui::prelude::*;
use std::sync::Arc;

pub(super) fn render_subtitle_track_in_player_screen(
    track: &SubtitleTrack,
    screen_wh: &Wh<f32>,
    playback_time: &Time,
    language: Language,
    subtitle_play_duration_measurer: &SubtitlePlayDurationMeasurer,
) -> RenderingTree {
    let mut clips_in_playback_time = track
        .clips
        .iter()
        .filter(|clip| {
            clip.start_at <= playback_time
                && playback_time < clip.end_at(language, subtitle_play_duration_measurer)
        })
        .collect::<Vec<_>>();
    clips_in_playback_time.sort_by(|a, b| a.start_at.cmp(&b.start_at));

    RenderingTree::Children(
        clips_in_playback_time
            .iter()
            .map(|clip| {
                let line_index = get_line_index_pushing_up(
                    &track.clips,
                    playback_time,
                    clip,
                    language,
                    subtitle_play_duration_measurer,
                );
                render_subtitle(&clip.subtitle, screen_wh, line_index, language)
            })
            .collect(),
    )
}

/// NOTE : Below functions are in research.
/// https://docs.google.com/document/d/1MBLlg_g72LxW5TTknX-AX3CVlLwKjYsH-Yj0UBO9VVk/edit#
fn get_line_index_pushing_up(
    clips: &[Arc<SubtitleClip>],
    playback_time: &Time,
    target_clip: &SubtitleClip,
    language: Language,
    subtitle_play_duration_measurer: &SubtitlePlayDurationMeasurer,
) -> usize {
    let mut clips_come_after_target: Vec<_> = clips
        .iter()
        .filter(|clip| target_clip.start_at < clip.start_at)
        .collect();
    clips_come_after_target.sort_by(|a, b| a.start_at.cmp(&b.start_at));

    let mut line_index = 0;

    for clip in clips_come_after_target.iter() {
        if playback_time < &clip.start_at {
            break;
        }
        let number_of_clips_at_start = clips_come_after_target
            .iter()
            .filter(|clip_a| {
                clip_a.id == clip.id
                    || clip_a.start_at < clip.start_at
                        && clip.start_at < clip_a.end_at(language, subtitle_play_duration_measurer)
            })
            .count();
        line_index = std::cmp::max(line_index, number_of_clips_at_start);
    }
    line_index
}

fn render_subtitle(
    subtitle: &Subtitle,
    screen_wh: &Wh<f32>,
    line_index: usize,
    language: Language,
) -> RenderingTree {
    let screen_size_relative_ratio = screen_wh.width / 1080.0;
    const FONT_SIZE_AT_WIDTH_1080: i16 = 64;
    let font_size = (FONT_SIZE_AT_WIDTH_1080 as f32 * screen_size_relative_ratio) as i16;

    let last_subtitle_center_xy = Xy {
        x: screen_wh.width / 2.0,
        y: screen_wh.height * 0.8,
    };

    const LINE_HEIGHT_RATE: f32 = 1.5;

    let subtitle_center_xy = last_subtitle_center_xy
        + Xy {
            x: 0.0,
            y: -(font_size as f32 * LINE_HEIGHT_RATE * line_index as f32),
        };

    namui::clip(
        namui::PathBuilder::new().add_rect(&namui::LtrbRect {
            left: 0.0,
            top: 0.0,
            right: screen_wh.width,
            bottom: screen_wh.height,
        }),
        namui::ClipOp::Intersect,
        namui::text(namui::TextParam {
            x: subtitle_center_xy.x,
            y: subtitle_center_xy.y,
            align: namui::TextAlign::Center,
            baseline: namui::TextBaseline::Bottom,
            font_type: namui::FontType {
                font_weight: namui::FontWeight::REGULAR,
                size: font_size,
                language,
                serif: false,
            },
            style: namui::TextStyle {
                color: namui::Color::WHITE,
                background: Some(namui::TextStyleBackground {
                    color: namui::Color::BLACK,
                    margin: None,
                }),
                border: None,
                drop_shadow: None,
            },
            text: subtitle.language_text_map.get(&language).unwrap().clone(),
        }),
    )
}
