use crate::app::types::*;
use namui::prelude::*;
use std::{collections::HashMap, sync::Arc};

pub(super) fn render_subtitle_track_in_player_screen(
    track: &SubtitleTrack,
    screen_wh: &Wh<f32>,
    playback_time: &Time,
    language: Language,
    subtitle_play_duration_measurer: &dyn SubtitlePlayDurationMeasure,
    subtitle_character_color_map: &HashMap<String, Color>,
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
                render_subtitle(
                    &clip.subtitle,
                    screen_wh,
                    line_index,
                    language,
                    subtitle_character_color_map,
                )
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
    subtitle_play_duration_measurer: &dyn SubtitlePlayDurationMeasure,
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
    subtitle_character_color_map: &HashMap<String, Color>,
) -> RenderingTree {
    let screen_size_relative_ratio = screen_wh.width / 1080.0;
    const FONT_SIZE_AT_WIDTH_1080: i16 = 36;
    let font_size = (FONT_SIZE_AT_WIDTH_1080 as f32 * screen_size_relative_ratio) as i16;

    const BACKGROUND_MARGIN: LtrbRect = LtrbRect {
        left: 15.0,
        top: 2.0,
        right: 15.0,
        bottom: 2.0,
    };

    let last_subtitle_center_xy = Xy {
        x: screen_wh.width / 2.0,
        y: screen_wh.height * 0.8,
    };

    const LINE_HEIGHT_RATE: f32 = 1.5;

    let subtitle_center_xy = last_subtitle_center_xy
        + Xy {
            x: 0.0,
            y: -((font_size as f32 + BACKGROUND_MARGIN.top + BACKGROUND_MARGIN.bottom)
                * LINE_HEIGHT_RATE
                * line_index as f32),
        };

    let text_box_font_type = namui::FontType {
        font_weight: namui::FontWeight::REGULAR,
        size: font_size,
        language,
        serif: false,
    };

    let subtitle_text = subtitle.language_text_map.get(&language).unwrap().clone();
    let text_box_width = get_text_width(&subtitle_text, &text_box_font_type, None);

    let name = subtitle.speaker.clone();
    let is_no_name = name.is_empty();
    let name_box_width = if is_no_name {
        Some(0.0)
    } else {
        get_text_width(&name, &text_box_font_type, None)
    };

    if text_box_width.is_none() || name_box_width.is_none() {
        return RenderingTree::Empty;
    }

    let text_box_width = text_box_width.unwrap() + BACKGROUND_MARGIN.left + BACKGROUND_MARGIN.right;
    let name_box_width = if is_no_name {
        0.0
    } else {
        name_box_width.unwrap() + BACKGROUND_MARGIN.left + BACKGROUND_MARGIN.right
    };
    let text_box_center_x = subtitle_center_xy.x + name_box_width / 2.0;

    let name_box = if is_no_name {
        RenderingTree::Empty
    } else {
        let name_box_center_x = subtitle_center_xy.x - text_box_width / 2.0 + 1.0; // NOTE : 1.0 is for floating point error.

        let name_background_color = subtitle_character_color_map
            .get(&name)
            .unwrap_or_else(|| &Color::WHITE)
            .clone();

        let is_name_background_color_dark = (name_background_color.r as u32
            + name_background_color.g as u32
            + name_background_color.b as u32)
            / 3
            < 128;

        let name_text_color = if is_name_background_color_dark {
            Color::WHITE
        } else {
            Color::BLACK
        };
        namui::text(namui::TextParam {
            x: name_box_center_x,
            y: subtitle_center_xy.y,
            align: namui::TextAlign::Center,
            baseline: namui::TextBaseline::Bottom,
            font_type: text_box_font_type,
            style: namui::TextStyle {
                color: name_text_color,
                background: Some(namui::TextStyleBackground {
                    color: name_background_color,
                    margin: Some(BACKGROUND_MARGIN),
                }),
                border: None,
                drop_shadow: None,
            },
            text: name,
        })
    };

    let text_box = namui::text(namui::TextParam {
        x: text_box_center_x,
        y: subtitle_center_xy.y,
        align: namui::TextAlign::Center,
        baseline: namui::TextBaseline::Bottom,
        font_type: text_box_font_type,
        style: namui::TextStyle {
            color: namui::Color::WHITE,
            background: Some(namui::TextStyleBackground {
                color: namui::Color::BLACK,
                margin: Some(BACKGROUND_MARGIN),
            }),
            border: None,
            drop_shadow: None,
        },
        text: subtitle.language_text_map.get(&language).unwrap().clone(),
    });

    namui::clip(
        namui::PathBuilder::new().add_rect(&namui::LtrbRect {
            left: 0.0,
            top: 0.0,
            right: screen_wh.width,
            bottom: screen_wh.height,
        }),
        namui::ClipOp::Intersect,
        render![name_box, text_box],
    )
}
