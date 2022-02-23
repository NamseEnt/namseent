use crate::app::types::*;
use namui::prelude::*;
use std::sync::Arc;

pub fn extract_camera_clip_ids(sequence: &Sequence) -> Vec<String> {
    sequence
        .tracks
        .iter()
        .filter_map::<Vec<String>, _>(|track| {
            if let Track::Camera(camera_track) = track.as_ref() {
                Some(
                    camera_track
                        .clips
                        .iter()
                        .map(|clip| clip.id.clone())
                        .collect(),
                )
            } else {
                None
            }
        })
        .flatten()
        .collect()
}

pub fn extract_camera_clips(sequence: &Sequence) -> Vec<Arc<CameraClip>> {
    sequence
        .tracks
        .iter()
        .filter_map::<Vec<Arc<CameraClip>>, _>(|track| {
            if let Track::Camera(camera_track) = track.as_ref() {
                Some(camera_track.clips.iter().map(|clip| clip.clone()).collect())
            } else {
                None
            }
        })
        .flatten()
        .collect()
}

pub fn extract_subtitle_clips(sequence: &Sequence) -> Vec<Arc<SubtitleClip>> {
    sequence
        .tracks
        .iter()
        .filter_map::<Vec<Arc<SubtitleClip>>, _>(|track| {
            if let Track::Subtitle(subtitle_track) = track.as_ref() {
                Some(
                    subtitle_track
                        .clips
                        .iter()
                        .map(|clip| clip.clone())
                        .collect(),
                )
            } else {
                None
            }
        })
        .flatten()
        .collect()
}

pub fn mock_sequence(camera_clip_ids: &[&str], subtitle_clip_ids: &[&str]) -> Sequence {
    let mut camera_clips = Vec::new();
    camera_clip_ids.iter().enumerate().for_each(|(index, id)| {
        let start_at = Time::from_ms(index as f32);
        let end_at = Time::from_ms((index + 1) as f32);
        camera_clips.push(mock_camera_clip(id, start_at, end_at));
    });

    let mut subtitle_clips = Vec::new();
    subtitle_clip_ids
        .iter()
        .enumerate()
        .for_each(|(index, id)| {
            let start_at = Time::from_ms(index as f32);
            subtitle_clips.push(mock_subtitle_clip(id, start_at));
        });

    Sequence {
        tracks: vec![
            Arc::new(Track::Camera(CameraTrack {
                id: "track-1".to_string(),
                clips: camera_clips.into(),
            })),
            Arc::new(Track::Subtitle(SubtitleTrack {
                id: "track-2".to_string(),
                clips: subtitle_clips.into(),
            })),
        ]
        .into(),
    }
}

pub fn mock_camera_clip(clip_id: &str, start_at: Time, end_at: Time) -> Arc<CameraClip> {
    Arc::new(CameraClip {
        id: clip_id.to_string(),
        start_at,
        end_at,
        camera_angle: CameraAngle {
            character_pose_emotion: CharacterPoseEmotion(
                "c".to_string(),
                "p".to_string(),
                "e".to_string(),
            ),
            source_01_circumscribed: Circumscribed {
                center: Xy { x: 0.0, y: 0.0 },
                radius: 1.0,
            },
            crop_screen_01_rect: LtrbRect {
                left: 0.0,
                top: 0.0,
                right: 1.0,
                bottom: 1.0,
            },
        },
    })
}

pub fn mock_subtitle_clip(clip_id: &str, start_at: Time) -> Arc<SubtitleClip> {
    Arc::new(SubtitleClip {
        id: clip_id.to_string(),
        start_at,
        subtitle: mock_subtitle(&clip_id),
        is_needed_to_update_position: false,
    })
}

pub fn mock_subtitle(clip_id: &str) -> Subtitle {
    Subtitle {
        id: clip_id.to_string(),
        speaker: "".to_string(),
        language_text_map: vec![(Language::Ko, "hello_world".to_string())]
            .into_iter()
            .collect(),
    }
}
